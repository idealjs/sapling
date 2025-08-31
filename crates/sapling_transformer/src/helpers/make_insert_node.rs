use biome_js_factory::make::{
    ident, js_call_expression, js_identifier_expression, js_reference_identifier, token,
};
use biome_js_syntax::{AnyJsCallArgument, AnyJsExpression, JsCallExpression, T};

use crate::make_js_call_arguments;

pub fn make_insert_node(parent_id: &str, child_id: &str) -> JsCallExpression {
    let callee = js_identifier_expression(js_reference_identifier(ident("_$insertNode")));
    let arg1 = AnyJsCallArgument::AnyJsExpression(AnyJsExpression::JsIdentifierExpression(
        js_identifier_expression(js_reference_identifier(ident(parent_id))),
    ));
    let arg2 = AnyJsCallArgument::AnyJsExpression(AnyJsExpression::JsIdentifierExpression(
        js_identifier_expression(js_reference_identifier(ident(child_id))),
    ));
    js_call_expression(
        callee.into(),
        make_js_call_arguments(vec![arg1, arg2], vec![token(T!(,))]),
    )
    .build()
}

#[test]
fn test_make_insert_node() {
    let stmt = make_insert_node("_el$", "_child$");

    insta::assert_snapshot!(stmt.to_string());
}
