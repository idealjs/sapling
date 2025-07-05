use biome_js_syntax::*;
use biome_js_factory::make::*;
use crate::jsx_template::transform_expression;

pub fn transform_statement(stmt: &AnyJsStatement) -> AnyJsStatement {
    match stmt {
        AnyJsStatement::JsExpressionStatement(expr_stmt) => {
            if let Ok(expr) = expr_stmt.expression() {
                if let Some(transformed_expr) = transform_expression(&expr) {
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
                if let Some(transformed_expr) = transform_expression(&expr) {
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
                let new_stmts: Vec<AnyJsStatement> = stmts.iter().map(|s| transform_statement(&s)).collect();
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
    _ => stmt.clone(),
}
}