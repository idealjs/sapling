use biome_js_factory::make::{js_function_body, js_parameters, js_parenthesized_expression};
use biome_js_syntax::AnyJsCallArgument;
use biome_js_syntax::JsCallArguments;
use std::vec;

use biome_js_factory::make::{
    js_arrow_function_expression, js_call_argument_list, js_call_arguments, js_call_expression,
    js_return_statement, token,
};
use biome_js_syntax::{
    AnyJsExpression, JsArrowFunctionExpression, JsCallExpression, JsDirectiveList, JsFunctionBody,
    JsParameterList, JsParameters, JsReturnStatement, JsStatementList, JsSyntaxToken, T,
};

pub mod jsx_element_name_to_string;
pub mod make_array;
pub mod make_arrow_function_from_statements;
pub mod make_create_element;
pub mod make_create_jsx_tag_element;
pub mod make_create_text_node;
pub mod make_effect;
pub mod make_insert;
pub mod make_insert_node;
pub mod make_set_prop;

pub use jsx_element_name_to_string::*;
pub use make_array::*;
pub use make_arrow_function_from_statements::*;
pub use make_create_element::*;
pub use make_create_jsx_tag_element::*;
pub use make_create_text_node::*;
pub use make_effect::*;
pub use make_insert::*;
pub use make_insert_node::*;
pub use make_set_prop::*;

pub fn make_js_call_expression(
    arrow_function_expression: JsArrowFunctionExpression,
) -> JsCallExpression {
    let callee = js_parenthesized_expression(
        token(T!['(']),
        AnyJsExpression::JsArrowFunctionExpression(arrow_function_expression),
        token(T![')']),
    );

    let arguments = make_js_call_arguments(vec![], vec![]);

    let expression = js_call_expression(
        AnyJsExpression::JsParenthesizedExpression(callee),
        arguments,
    )
    .build();

    expression
}

pub fn make_js_call_arguments(
    arguments: Vec<AnyJsCallArgument>,
    separators: Vec<JsSyntaxToken>,
) -> JsCallArguments {
    js_call_arguments(
        token(T!['(']),
        js_call_argument_list(arguments, separators),
        token(T![')']),
    )
}

pub fn make_js_arrow_function_expression(
    params: JsParameters,
    body: JsFunctionBody,
) -> JsArrowFunctionExpression {
    use biome_js_syntax::{AnyJsArrowFunctionParameters, AnyJsFunctionBody, T};

    let params = AnyJsArrowFunctionParameters::JsParameters(params);
    let body: AnyJsFunctionBody = AnyJsFunctionBody::JsFunctionBody(body);
    js_arrow_function_expression(params, token(T![=>]), body).build()
}

pub fn make_js_parameters(
    js_parameter_list: JsParameterList, // parameters: Vec<AnyJsParameter>,
                                        // separators: Vec<JsSyntaxToken>,
) -> JsParameters {
    js_parameters(token(T!['(']), js_parameter_list, token(T![')']))
}

pub fn make_js_return_statement(any_js_expression: AnyJsExpression) -> JsReturnStatement {
    js_return_statement(token(T![return]))
        .with_argument(any_js_expression)
        .with_semicolon_token(token(T![;]))
        .build()
}

pub fn make_js_function_body(
    directives: JsDirectiveList,
    statements: JsStatementList,
) -> JsFunctionBody {
    js_function_body(token(T!['{']), directives, statements, token(T!['}']))
}
