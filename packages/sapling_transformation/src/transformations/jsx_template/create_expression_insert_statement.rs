use biome_js_syntax::*;
use biome_js_factory::make::*;

pub fn create_expression_insert_statement(el_var_name: &str, expr: AnyJsExpression) -> Option<AnyJsStatement> {
    let insert_call = js_call_expression(
        js_identifier_expression(js_reference_identifier(
            JsSyntaxToken::new_detached(T![ident], "_$insert", Vec::new(), Vec::new())
        )).into(),
        js_call_arguments(
            token(T!['(']),
            js_call_argument_list(
                vec![
                    AnyJsCallArgument::AnyJsExpression(
                        js_identifier_expression(js_reference_identifier(
                            JsSyntaxToken::new_detached(T![ident], el_var_name, Vec::new(), Vec::new())
                        )).into()
                    ),
                    AnyJsCallArgument::AnyJsExpression(expr),
                ],
                vec![token(T![,])],
            ),
            token(T![')']),
        ),
    ).build();

    let stmt = js_expression_statement(AnyJsExpression::JsCallExpression(insert_call))
        .with_semicolon_token(token(T![;]))
        .build();

    Some(stmt.into())
}