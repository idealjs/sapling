use biome_js_factory::make::{
    js_call_expression, js_identifier_expression, js_reference_identifier, token,
};
use biome_js_syntax::{AnyJsCallArgument, AnyJsExpression, JsCallExpression, T};

use crate::make_js_call_arguments;

pub fn make_insert(parent_id: &str, expr: AnyJsExpression) -> JsCallExpression {
    let callee = js_identifier_expression(js_reference_identifier(biome_js_factory::make::ident(
        "_$insert",
    )));
    let arg1 = AnyJsCallArgument::AnyJsExpression(AnyJsExpression::JsIdentifierExpression(
        js_identifier_expression(js_reference_identifier(biome_js_factory::make::ident(
            parent_id,
        ))),
    ));
    let arg2 = AnyJsCallArgument::AnyJsExpression(expr);
    js_call_expression(
        callee.into(),
        make_js_call_arguments(vec![arg1, arg2], vec![token(T!(,))]),
    )
    .build()
}
