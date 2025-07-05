use biome_js_syntax::*;
use biome_js_factory::make::*;
use crate::jsx_template::transform_expression;

pub fn transform_arrow_function(arrow_fn: &JsArrowFunctionExpression) -> Option<AnyJsExpression> {
    if let Ok(body) = arrow_fn.body() {
        if let AnyJsFunctionBody::AnyJsExpression(expr) = body {
            if let Some(transformed_expr) = transform_expression(&expr) {
                if let Ok(params) = arrow_fn.parameters() {
                    let new_arrow_fn = js_arrow_function_expression(
                        params,
                        arrow_fn.fat_arrow_token().ok()?,
                        AnyJsFunctionBody::AnyJsExpression(transformed_expr),
                    )
                    .build();
                    return Some(AnyJsExpression::JsArrowFunctionExpression(new_arrow_fn));
                }
            }
        }
    }
    None
}