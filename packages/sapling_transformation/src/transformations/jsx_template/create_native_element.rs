use crate::jsx_template::create_solidjs_call_with_tracker::create_solidjs_call_with_tracker;
use biome_js_syntax::*;
use biome_js_factory::make::*;
use crate::jsx_template::HelperUsageTracker;
use crate::transformations::jsx_template::create_text_insert_statement::create_text_insert_statement;
use crate::transformations::jsx_template::create_expression_insert_statement::create_expression_insert_statement;
use crate::transformations::jsx_template::create_child_insert_statement::create_child_insert_statement;

pub fn create_native_element(jsx_element: &JsxElement, tag_name: &str, tracker: &mut HelperUsageTracker) -> Option<AnyJsExpression> {
    // 创建语句列表
    let mut statements = Vec::<AnyJsStatement>::new();
    
    // 生成唯一的变量名
    static mut COUNTER: u32 = 0;
    unsafe { COUNTER += 1; }
    let el_var_name = format!("_el${}", unsafe { COUNTER });
    
    // 1. var _el$ = _$createElement("tagName");
    let el_var_token = JsSyntaxToken::new_detached(T![ident], &el_var_name, Vec::new(), Vec::new());
    let el_var_ident = js_identifier_expression(js_reference_identifier(el_var_token.clone()));
    
    tracker.create_element = true;
    
    let create_element_call = js_call_expression(
        js_identifier_expression(js_reference_identifier(
            JsSyntaxToken::new_detached(T![ident], "_$createElement", Vec::new(), Vec::new())
        )).into(),
        js_call_arguments(
            token(T!['(']),
            js_call_argument_list(
                vec![AnyJsCallArgument::AnyJsExpression(
                    AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(
                            js_string_literal_expression(js_string_literal(tag_name)),
                        ),
                    ),
                )],
                vec![],
            ),
            token(T![')']),
        ),
    ).build();

    let declarator = js_variable_declarator(
        AnyJsBindingPattern::AnyJsBinding(
            AnyJsBinding::JsIdentifierBinding(js_identifier_binding(el_var_token.clone()))
        )
    ).with_initializer(
        js_initializer_clause(token(T![=]), create_element_call.into()).into(),
    ).build();

    let var_decl = js_variable_statement(
        js_variable_declaration(
            token(T![var]),
            js_variable_declarator_list(vec![declarator], vec![]),
        ).build(),
    ).build();
    
    statements.push(var_decl.into());

    // 2. 处理子元素
    let children = jsx_element.children();
    for child in children {
        match child {
            AnyJsxChild::JsxText(text_node) => {
                let text_token = text_node.value_token().ok()?;
                let text_content = text_token.text_trimmed();
                if !text_content.trim().is_empty() {
                    tracker.insert_node = true;
                    tracker.create_text_node = true;
                    
                    let stmt = create_text_insert_statement(&el_var_name, text_content)?;
                    statements.push(stmt);
                }
            }
            AnyJsxChild::JsxExpressionChild(expr_child) => {
                if let Some(expr) = expr_child.expression() {
                    tracker.insert = true;
                    
                    let stmt = create_expression_insert_statement(&el_var_name, expr)?;
                    statements.push(stmt);
                }
            }
            AnyJsxChild::JsxElement(child_element) => {
                if let Some(child_expr) = create_solidjs_call_with_tracker(&child_element, tracker) {
                    tracker.insert_node = true;
                    
                    let stmt = create_child_insert_statement(&el_var_name, child_expr)?;
                    statements.push(stmt);
                }
            }
            _ => {}
        }
    }

    // 3. return _el$;
    let return_stmt = js_return_statement(token(T![return]))
        .with_argument(el_var_ident.into())
        .with_semicolon_token(token(T![;]))
        .build();
    statements.push(return_stmt.into());

    // 创建 IIFE
    let function_body = js_function_body(
        token(T!['{']),
        js_directive_list(vec![]),
        js_statement_list(statements),
        token(T!['}']),
    );

    let arrow_fn = js_arrow_function_expression(
        js_parameters(
            token(T!['(']),
            js_parameter_list(vec![], vec![]),
            token(T![')']),
        ).into(),
        token(T![=>]),
        function_body.into(),
    ).build();

    let iife = js_call_expression(
        js_parenthesized_expression(
            token(T!['(']),
            AnyJsExpression::JsArrowFunctionExpression(arrow_fn),
            token(T![')']),
        ).into(),
        js_call_arguments(
            token(T!['(']),
            js_call_argument_list(vec![], vec![]),
            token(T![')']),
        ),
    ).build();

    Some(AnyJsExpression::JsCallExpression(iife))
}