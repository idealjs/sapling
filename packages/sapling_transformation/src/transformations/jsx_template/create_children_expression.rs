use crate::jsx_template::create_solidjs_call_with_tracker::create_solidjs_call_with_tracker;
use biome_js_syntax::*;
use biome_js_factory::make::*;
use crate::jsx_template::HelperUsageTracker;

pub fn create_children_expression(children: &JsxChildList, tracker: &mut HelperUsageTracker) -> Option<AnyJsExpression> {
    let mut child_expressions = Vec::new();

    for child in children {
        match child {
            AnyJsxChild::JsxText(text_node) => {
                let text_token = text_node.value_token().ok()?;
                let text_content = text_token.text_trimmed();
                if !text_content.trim().is_empty() {
                    child_expressions.push(AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(
                            js_string_literal_expression(js_string_literal(text_content))
                        )
                    ));
                }
            }
            AnyJsxChild::JsxExpressionChild(expr_child) => {
                if let Some(expr) = expr_child.expression() {
                    child_expressions.push(expr);
                }
            }
            AnyJsxChild::JsxElement(jsx_element) => {
                if let Some(child_expr) = create_solidjs_call_with_tracker(&jsx_element, tracker) {
                    child_expressions.push(child_expr);
                }
            }
            _ => {}
        }
    }

    if child_expressions.len() == 1 {
        Some(child_expressions.into_iter().next().unwrap())
    } else if child_expressions.len() > 1 {
        let array = js_array_expression(
            token(T!['[']),
            js_array_element_list(
                child_expressions.into_iter().map(|expr| AnyJsArrayElement::AnyJsExpression(expr)).collect::<Vec<_>>(),
                vec![],
            ),
            token(T![']']),
        );
        Some(AnyJsExpression::JsArrayExpression(array))
    } else {
        None
    }
}