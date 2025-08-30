use biome_js_factory::make::{
    js_call_expression, js_identifier_expression, js_reference_identifier, token,
};
use biome_js_syntax::{AnyJsCallArgument, AnyJsExpression, JsCallExpression, T};

use crate::make_js_call_arguments;

pub fn make_effect(expr: AnyJsExpression, listener_key: Vec<AnyJsExpression>) -> JsCallExpression {
    use biome_js_syntax::{AnyJsArrayElement, AnyJsStatement};

    let callee = js_identifier_expression(js_reference_identifier(biome_js_factory::make::ident(
        "_$effect",
    )));

    let deps_elements: Vec<AnyJsArrayElement> = listener_key
        .into_iter()
        .map(|v| AnyJsArrayElement::AnyJsExpression(v))
        .collect();

    let deps_array = crate::helpers::make_array::make_array(deps_elements);

    let return_stmt = AnyJsStatement::JsReturnStatement(crate::helpers::make_js_return_statement(
        deps_array.into(),
    ));
    let arrow_fn = crate::helpers::make_arrow_function_from_statement(return_stmt);

    js_call_expression(
        callee.into(),
        make_js_call_arguments(
            vec![
                AnyJsCallArgument::AnyJsExpression(expr),
                AnyJsCallArgument::AnyJsExpression(AnyJsExpression::JsArrowFunctionExpression(
                    arrow_fn,
                )),
            ],
            vec![token(T!(,))],
        ),
    )
    .build()
}
