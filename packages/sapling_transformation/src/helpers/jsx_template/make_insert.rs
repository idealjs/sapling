// 用于生成 _$insert(parent_id, expr) 的辅助函数
//
// # 参数
// - parent_id: 父节点标识符（字符串）
// - expr: 需要插入的表达式（AnyJsExpression）
//
// # 返回
// 返回 JsCallExpression，表示 _$insert(parent_id, expr) 的调用。
// 用于 JSX 转换流程中插入子节点或表达式。
//
// # 用法示例
// let call_expr = make_insert("_el$", some_expr);

use crate::helpers::jsx_template::make_js_call_arguments;
use biome_js_factory::make::{
    js_call_expression, js_identifier_expression, js_reference_identifier, token,
};
use biome_js_syntax::{AnyJsCallArgument, AnyJsExpression, JsCallExpression, T};

/// 生成 _$insert(parent_id, expr) 的 JsCallExpression
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
