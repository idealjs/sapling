use std::any::Any;

use biome_js_factory::make::{
    js_arrow_function_expression, js_call_argument_list, js_call_arguments, js_call_expression, js_directive_list, js_expression_statement, js_function_body, js_function_expression, js_parameter_list, js_parameters, js_parenthesized_expression, js_return_statement, js_statement_list, token
};
use biome_js_syntax::{
    AnyJsExpression, AnyJsStatement, JsArrowFunctionExpression, JsCallExpression, JsExpressionStatement, JsFunctionExpression, JsLanguage, JsReturnStatement, JsStatementList, T
};
use biome_rowan::SyntaxToken;

pub fn make_js_call_expr() -> JsCallExpression {
    let callee = js_parenthesized_expression(
        token(T!['(']),
        AnyJsExpression::JsArrowFunctionExpression(make_arrow_function()),
        token(T![')']),
    );

    let arguments = js_call_arguments(
        token(T!['(']),
        js_call_argument_list([], []),
        token(T![')']),
    );

    let expression = js_call_expression(
        AnyJsExpression::JsParenthesizedExpression(callee),
        arguments,
    )
    .build();

    expression
}

pub fn make_arrow_function() -> JsArrowFunctionExpression {
    use biome_js_factory::make::{
        js_arrow_function_expression, js_parameters, js_parameter_list, js_function_body,
        js_directive_list, js_statement_list, token,
    };
    use biome_js_syntax::{AnyJsArrowFunctionParameters, AnyJsFunctionBody, T};

    let params = AnyJsArrowFunctionParameters::JsParameters(
        js_parameters(token(T!['(']), js_parameter_list(vec![], vec![]), token(T![')']))
    );
    let body = AnyJsFunctionBody::JsFunctionBody(
        js_function_body(
            token(T!['{']),
            js_directive_list(vec![]),
            js_statement_list(vec![]),
            token(T!['}']),
        )
    );
    js_arrow_function_expression(
        params,
        token(T![=>]),
        body
    ).build()
}

pub fn make_members() -> JsStatementList {
    js_statement_list(vec![])
}

pub fn make_js_return_statement(any_js_expression: AnyJsExpression) -> JsReturnStatement {
    js_return_statement(token(T![return]))
        .with_argument(any_js_expression)
        .with_semicolon_token(token(T![;]))
        .build()
}
