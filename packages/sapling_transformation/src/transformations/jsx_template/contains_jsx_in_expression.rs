use biome_js_syntax::*;
use crate::JsxElementInfo;

pub fn contains_jsx_in_expression(expr: &AnyJsExpression) -> bool {
    match expr {
        AnyJsExpression::JsxTagExpression(_) => true,
        AnyJsExpression::JsArrowFunctionExpression(arrow_fn) => {
            if let Ok(body) = arrow_fn.body() {
                if let AnyJsFunctionBody::AnyJsExpression(expr) = body {
                    contains_jsx_in_expression(&expr)
                } else {
                    false
                }
            } else {
                false
            }
        },
        AnyJsExpression::JsParenthesizedExpression(paren_expr) => {
            if let Ok(inner_expr) = paren_expr.expression() {
                contains_jsx_in_expression(&inner_expr)
            } else {
                false
            }
        },
        _ => false,
    }
}