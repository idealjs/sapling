use biome_js_syntax::*;
use biome_js_syntax::JsSyntaxToken;
use biome_js_syntax::AnyJsBindingPattern;
use biome_js_syntax::AnyJsFormalParameter;
use biome_js_factory::make::*;
use super::transform_expression_with_tracker;

use crate::jsx_template::HelperUsageTracker;
pub fn transform_any_js_statement_with_tracker(stmt: &AnyJsStatement, tracker: &mut HelperUsageTracker) -> AnyJsStatement {
    match stmt {
        AnyJsStatement::JsExpressionStatement(expr_stmt) => {
            if let Ok(expr) = expr_stmt.expression() {
                if let Some(transformed_expr) = transform_expression_with_tracker(&expr, tracker) {
                    let new_expr_stmt = js_expression_statement(transformed_expr)
                        .with_semicolon_token(expr_stmt.semicolon_token().expect("Missing semicolon"))
                        .build();
                    return AnyJsStatement::JsExpressionStatement(new_expr_stmt);
                }
            }
            stmt.clone()
        },
        AnyJsStatement::JsReturnStatement(return_stmt) => {
            if let Some(expr) = return_stmt.argument() {
                if let Some(transformed_expr) = transform_expression_with_tracker(&expr, tracker) {
                    let new_return_stmt = js_return_statement(return_stmt.return_token().expect("Missing return token"))
                        .with_argument(transformed_expr)
                        .with_semicolon_token(return_stmt.semicolon_token().expect("Missing semicolon"))
                        .build();
                    return AnyJsStatement::JsReturnStatement(new_return_stmt);
                }
            }
            stmt.clone()
        },
        AnyJsStatement::JsFunctionDeclaration(func_decl) => {
            // 递归转换函数体内所有语句
            if let Ok(body) = func_decl.body() {
                use biome_rowan::AstNodeList;
                use crate::helpers::jsx_template::make_js_function_body;
                let stmts = body.statements();
                let new_stmts: Vec<AnyJsStatement> = stmts.iter().map(|s| transform_any_js_statement_with_tracker(&s, tracker)).collect();
                let new_body = make_js_function_body(
                    js_directive_list(vec![]),
                    js_statement_list(new_stmts),
                );
                let new_func = js_function_declaration(
                    func_decl.function_token().expect("Missing function token"),
                    func_decl.id().expect("Missing id"),
                    func_decl.parameters().expect("Missing parameters"),
                    new_body,
                ).build();
                return AnyJsStatement::JsFunctionDeclaration(new_func);
            }
            stmt.clone()
        },
        AnyJsStatement::JsVariableStatement(var_stmt) => {
            // 只处理包含 JSX 的变量声明
            if let Ok(decl_list) = var_stmt.declaration() {
                let declarators = decl_list.declarators();
                let mut changed = false;
                let mut new_declarators = Vec::new();
                for decl in declarators {
                    if let Ok(decl) = decl {
                        if let Some(init) = decl.initializer() {
                            if let Ok(expr) = init.expression() {
                                if crate::transformations::jsx_template::contains_jsx_in_expression(&expr) {
                                    // 包裹为 IIFE
                                    use crate::helpers::jsx_template::*;
                                    let props_token = JsSyntaxToken::new_detached(T![ident], "props", Vec::new(), Vec::new());
                                    let binding = js_identifier_binding(props_token);
                                    let binding_pattern = AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(binding));
                                    let param = js_formal_parameter(js_decorator_list(vec![]), binding_pattern).build();
                                    let params = make_js_parameters(js_parameter_list(vec![AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(param))], vec![]));
                                    let body = make_js_function_body(
                                        js_directive_list(vec![]),
                                        js_statement_list(vec![
                                            // 递归转换 initializer 内部的表达式
                                            if let Some(transformed_expr) = transform_expression_with_tracker(&expr, tracker) {
                                                make_js_return_statement(transformed_expr).into()
                                            } else {
                                                make_js_return_statement(expr.clone().into()).into()
                                            }
                                        ]),
                                    );
                                    let arrow_fn = make_js_arrow_function_expression(params, body);
                                    let call_expr = make_js_call_expression(arrow_fn);
                                    let new_init = js_initializer_clause(token(T![=]), call_expr.into());
                                    let new_decl = decl.clone().with_initializer(Some(new_init));
                                    new_declarators.push(new_decl);
                                    changed = true;
                                    continue;
                                }
                                // 新增：递归 transform 箭头函数表达式
                                if let AnyJsExpression::JsArrowFunctionExpression(arrow_fn) = &expr {
                                    if let Some(transformed_arrow) = crate::transformations::jsx_template::transform_arrow_function(arrow_fn) {
                                        let new_init = js_initializer_clause(token(T![=]), transformed_arrow);
                                        let new_decl = decl.clone().with_initializer(Some(new_init));
                                        new_declarators.push(new_decl);
                                        changed = true;
                                        continue;
                                    }
                                }
                                // 新增：递归 transform 其它表达式（如 JSX 变量、Fragment 等）
                                if let Some(transformed_expr) = transform_expression_with_tracker(&expr, tracker) {
                                    let new_init = js_initializer_clause(token(T![=]), transformed_expr);
                                    let new_decl = decl.clone().with_initializer(Some(new_init));
                                    new_declarators.push(new_decl);
                                    changed = true;
                                    continue;
                                }
                            }
                        }
                        new_declarators.push(decl.clone());
                    }
                }
                if changed {
                    let new_decl_list = js_variable_declarator_list(new_declarators, vec![]);
                    let new_var_stmt = js_variable_statement(
                        js_variable_declaration(
                            token(T![var]),
                            new_decl_list,
                        ).build(),
                    ).build();
                    return AnyJsStatement::JsVariableStatement(new_var_stmt);
                }
            }
            stmt.clone()
        },
        _ => stmt.clone(),
    }
}