use biome_js_factory::make::{
    js_call_expression, js_identifier_expression, js_reference_identifier, token,
};
use biome_js_syntax::{AnyJsCallArgument, AnyJsExpression, JsCallExpression, T};

use crate::make_js_call_arguments;

pub fn make_effect(expr: AnyJsExpression, listener_key: Vec<usize>) -> JsCallExpression {
    use biome_js_factory::make::js_number_literal_expression;
    use biome_js_syntax::{AnyJsArrayElement, AnyJsExpression, AnyJsLiteralExpression};

    let callee = js_identifier_expression(js_reference_identifier(biome_js_factory::make::ident(
        "_$effect",
    )));

    // 将 deps: Vec<usize> 转为 Vec<AnyJsArrayElement>
    let deps_elements: Vec<AnyJsArrayElement> = listener_key
        .into_iter()
        .map(|v| {
            AnyJsArrayElement::AnyJsExpression(AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(js_number_literal_expression(
                    biome_js_factory::make::js_number_literal(&v.to_string()),
                )),
            ))
        })
        .collect();

    let deps_array = crate::helpers::make_array::make_array(deps_elements);

    js_call_expression(
        callee.into(),
        make_js_call_arguments(
            vec![
                AnyJsCallArgument::AnyJsExpression(expr),
                AnyJsCallArgument::AnyJsExpression(deps_array.into()),
            ],
            vec![token(T!(,))],
        ),
    )
    .build()
}
