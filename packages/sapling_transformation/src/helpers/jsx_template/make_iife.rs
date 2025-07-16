use biome_js_factory::make::{
    js_arrow_function_expression, js_call_argument_list, js_call_arguments, js_call_expression,
    js_directive_list, js_function_body, js_parameter_list, js_parameters,
    js_parenthesized_expression, js_statement_list, token,
};
use biome_js_syntax::{AnyJsExpression, AnyJsStatement, JsCallExpression, JsDirective, T};

pub fn make_iife(
    directives: Vec<JsDirective>,
    statements: Vec<AnyJsStatement>,
) -> JsCallExpression {
    let function_body = js_function_body(
        token(T!['{']),
        js_directive_list(directives),
        js_statement_list(statements),
        token(T!['}']),
    );
    let params = js_parameters(
        token(T!['(']),
        js_parameter_list(vec![], vec![]),
        token(T![')']),
    );
    let arrow_fn =
        js_arrow_function_expression(params.into(), token(T![=>]), function_body.into()).build();
    js_call_expression(
        js_parenthesized_expression(
            token(T!['(']),
            AnyJsExpression::JsArrowFunctionExpression(arrow_fn),
            token(T![')']),
        )
        .into(),
        js_call_arguments(
            token(T!['(']),
            js_call_argument_list(vec![], vec![]),
            token(T![')']),
        ),
    )
    .build()
}
