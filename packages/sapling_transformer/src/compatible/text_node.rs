use biome_js_factory::make::{
    ident, js_call_expression, js_identifier_expression, js_reference_identifier,
    js_string_literal, js_string_literal_expression,
};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression, JsCallExpression,
};

use crate::make_js_call_arguments;

pub fn generate_create_text_node_expr(text: &str) -> JsCallExpression {
    let callee = js_identifier_expression(js_reference_identifier(ident("_$createTextNode")));
    let arg = AnyJsCallArgument::AnyJsExpression(AnyJsExpression::AnyJsLiteralExpression(
        AnyJsLiteralExpression::JsStringLiteralExpression(js_string_literal_expression(
            js_string_literal(text),
        )),
    ));
    js_call_expression(callee.into(), make_js_call_arguments(vec![arg], vec![])).build()
}

#[test]
fn test_generate_create_text_node_expr() {
    let expr = generate_create_text_node_expr("hello");
    // 只做快照测试，确保语法结构正确
    insta::assert_snapshot!(expr.to_string());
}
