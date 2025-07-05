use biome_js_syntax::*;
use biome_js_factory::make::*;

pub fn create_insert_text_node(text: &str) -> Option<AnyJsStatement> {
    let el_var_token = JsSyntaxToken::new_detached(T![ident], "_el$", Vec::new(), Vec::new());
    let insert_node_token =
        JsSyntaxToken::new_detached(T![ident], "_$insertNode", Vec::new(), Vec::new());
    let create_text_token =
        JsSyntaxToken::new_detached(T![ident], "_$createTextNode", Vec::new(), Vec::new());

    // _$insertNode(_el$, _$createTextNode(`text`))
    let create_text_call = js_call_expression(
        js_identifier_expression(js_reference_identifier(create_text_token)).into(),
        js_call_arguments(
            token(T!['(']),
            js_call_argument_list(
                vec![AnyJsCallArgument::AnyJsExpression(
                    AnyJsExpression::AnyJsLiteralExpression(
                        biome_js_syntax::AnyJsLiteralExpression::JsStringLiteralExpression(
                            js_string_literal_expression(js_string_literal(text.as_ref())),
                        ),
                    ),
                )],
                vec![],
            ),
            token(T![')']),
        ),
    )
    .build();
    let insert_call = js_call_expression(
        js_identifier_expression(js_reference_identifier(insert_node_token)).into(),
        js_call_arguments(
            token(T!['(']),
            js_call_argument_list(
                vec![
                    AnyJsCallArgument::AnyJsExpression(
                        js_identifier_expression(js_reference_identifier(el_var_token)).into(),
                    ),
                    AnyJsCallArgument::AnyJsExpression(AnyJsExpression::JsCallExpression(
                        create_text_call,
                    )),
                ],
                vec![token(T![,])],
            ),
            token(T![')']),
        ),
    )
    .build();

    let stmt = js_expression_statement(AnyJsExpression::JsCallExpression(insert_call))
        .with_semicolon_token(token(T![;]))
        .build();

    Some(stmt.into())
}