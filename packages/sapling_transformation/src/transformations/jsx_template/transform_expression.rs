use biome_js_syntax::*;
use crate::transform_arrow_function;
use crate::jsx_template::create_solidjs_call_with_tracker;

use crate::jsx_template::HelperUsageTracker;
pub fn transform_expression_with_tracker(expr: &AnyJsExpression, tracker: &mut HelperUsageTracker) -> Option<AnyJsExpression> {
    match expr {
        AnyJsExpression::JsxTagExpression(jsx_tag) => {
            if let Ok(jsx_element_any) = jsx_tag.tag() {
                if let AnyJsxTag::JsxElement(jsx_element) = jsx_element_any {
                    tracker.create_element = true;
                    return create_solidjs_call_with_tracker(&jsx_element, tracker);
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