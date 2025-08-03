use biome_js_factory::make::{
    ident, js_call_expression, js_identifier_expression, js_reference_identifier, token,
};
use biome_js_syntax::{AnyJsCallArgument, AnyJsExpression, JsCallExpression, T};
use sapling_transformation::helpers::jsx_template::make_js_call_arguments;

/// 生成 _$insertNode(parent_id, child_id) 的表达式语句
pub fn generate_insert_node_expr(parent_id: &str, child_id: &str) -> JsCallExpression {
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
fn test_generate_insert_node_expr() {
    let stmt = generate_insert_node_expr("_el$", "_child$");
    // 只做快照测试，确保语法结构正确
    insta::assert_snapshot!(stmt.to_string());
}
