use biome_js_syntax::*;
use crate::jsx_template::{create_solidjs_call, transform_arrow_function};

pub fn transform_expression(expr: &AnyJsExpression) -> Option<AnyJsExpression> {
    match expr {
        AnyJsExpression::JsxTagExpression(jsx_tag) => {
            if let Ok(jsx_element_any) = jsx_tag.tag() {
                if let AnyJsxTag::JsxElement(jsx_element) = jsx_element_any {
                    return create_solidjs_call(&jsx_element);
                }
            }
            None
        },
        AnyJsExpression::JsArrowFunctionExpression(arrow_fn) => {
            transform_arrow_function(arrow_fn)
        },
        _ => None,
    }
}