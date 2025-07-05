use biome_js_syntax::*;
use biome_js_factory::make::*;

pub fn create_text_insert_statement(el_var_name: &str, text: &str) -> Option<AnyJsStatement> {
    let insert_call = js_call_expression(
        js_identifier_expression(js_reference_identifier(
            JsSyntaxToken::new_detached(T![ident], "_$insertNode", Vec::new(), Vec::new())
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
                    AnyJsCallArgument::AnyJsExpression(
                        js_call_expression(
                            js_identifier_expression(js_reference_identifier(
                                JsSyntaxToken::new_detached(T![ident], "_$createTextNode", Vec::new(), Vec::new())
                            )).into(),
                            js_call_arguments(
                                token(T!['(']),
                                js_call_argument_list(
                                    vec![AnyJsCallArgument::AnyJsExpression(
                                        AnyJsExpression::AnyJsLiteralExpression(
                                            AnyJsLiteralExpression::JsStringLiteralExpression(
                                                js_string_literal_expression(js_string_literal(text))
                                            )
                                        )
                                    )],
                                    vec![],
                                ),
                                token(T![')']),
                            ),
                        ).build().into()
                    ),
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