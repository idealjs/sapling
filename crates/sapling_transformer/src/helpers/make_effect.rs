use biome_js_factory::make::{
    js_call_expression, js_identifier_expression, js_reference_identifier,
};
use biome_js_syntax::{AnyJsCallArgument, AnyJsExpression, JsCallExpression};

use crate::make_js_call_arguments;

pub fn make_effect(expr: AnyJsExpression) -> JsCallExpression {
    let callee = js_identifier_expression(js_reference_identifier(biome_js_factory::make::ident(
        "_$effect",
    )));
    js_call_expression(
        callee.into(),
        make_js_call_arguments(vec![AnyJsCallArgument::AnyJsExpression(expr)], vec![]),
    )
    .build()
}
