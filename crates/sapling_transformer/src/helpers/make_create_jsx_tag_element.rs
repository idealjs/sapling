use biome_js_factory::make::{
    js_arrow_function_expression, js_call_argument_list, js_call_arguments, js_call_expression,
    js_directive_list, js_function_body, js_identifier_expression, js_parameter_list,
    js_parameters, js_reference_identifier, token,
};
use biome_js_syntax::{
    AnyJsExpression, AnyJsStatement, JsCallExpression, JsDirective, JsSyntaxToken, T,
};

pub fn make_create_jsx_tag_element(
    directives: Vec<JsDirective>,
    statements: Vec<AnyJsStatement>,
) -> JsCallExpression {
    let function_body = js_function_body(
        token(T!['{']),
        js_directive_list(directives),
        biome_js_factory::make::js_statement_list(statements),
        token(T!['}']),
    );
    let params = js_parameters(
        token(T!['(']),
        js_parameter_list(vec![], vec![]),
        token(T![')']),
    );
    let arrow_fn =
        js_arrow_function_expression(params.into(), token(T![=>]), function_body.into()).build();

    let create_jsx_tag_element_token =
        JsSyntaxToken::new_detached(T![ident], "_$createJsxTagElement", Vec::new(), Vec::new());

    js_call_expression(
        js_identifier_expression(js_reference_identifier(create_jsx_tag_element_token)).into(),
        js_call_arguments(
            token(T!['(']),
            js_call_argument_list(
                vec![biome_js_syntax::AnyJsCallArgument::AnyJsExpression(
                    AnyJsExpression::JsArrowFunctionExpression(arrow_fn),
                )],
                vec![],
            ),
            token(T![')']),
        ),
    )
    .build()
}
