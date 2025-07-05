use crate::jsx_template::create_solidjs_call_with_tracker::create_solidjs_call_with_tracker;
use biome_js_syntax::*;
use biome_js_factory::make::js_parenthesized_expression;
use crate::transform_arrow_function;
use crate::jsx_template::create_solidjs_call_with_tracker;
use crate::jsx_template::create_solidjs_call_with_tracker_self_closing;

use crate::jsx_template::HelperUsageTracker;
pub fn transform_expression_with_tracker(expr: &AnyJsExpression, tracker: &mut HelperUsageTracker) -> Option<AnyJsExpression> {
    match expr {
        AnyJsExpression::JsxTagExpression(jsx_tag) => {
            if let Ok(jsx_element_any) = jsx_tag.tag() {
                match jsx_element_any {
                    AnyJsxTag::JsxElement(jsx_element) => {
                        tracker.create_element = true;
                        return create_solidjs_call_with_tracker(&jsx_element, tracker);
                    }
                    AnyJsxTag::JsxSelfClosingElement(self_closing) => {
                        tracker.create_element = true;
                        if let Some(expr) = crate::transformations::jsx_template::create_solidjs_call_with_tracker_self_closing(&self_closing, tracker) {
                            return Some(expr);
                        }
                    }
                    AnyJsxTag::JsxFragment(fragment) => {
                        // Fragment 需要转换为包含多个 IIFE 的数组表达式
                        let mut elements = Vec::new();
                        for child in fragment.children() {
                            match child {
                                AnyJsxChild::JsxElement(el) => {
                                    if let Some(transformed) = create_solidjs_call_with_tracker(&el, tracker) {
                                        elements.push(transformed);
                                    }
                                }
                                AnyJsxChild::JsxExpressionChild(expr_child) => {
                                    if let Some(expr) = expr_child.expression() {
                                        if let Some(transformed_expr) = transform_expression_with_tracker(&expr, tracker) {
                                            elements.push(transformed_expr);
                                        } else {
                                            elements.push(expr);
                                        }
                                    }
                                }
                                AnyJsxChild::JsxText(text_node) => {
                                    if let Ok(text_token) = text_node.value_token() {
                                        let text_content = text_token.text_trimmed();
                                        if !text_content.trim().is_empty() {
                                            let text_expr = AnyJsExpression::AnyJsLiteralExpression(
                                                biome_js_syntax::AnyJsLiteralExpression::JsStringLiteralExpression(
                                                    biome_js_factory::make::js_string_literal_expression(
                                                        biome_js_factory::make::js_string_literal(text_content)
                                                    )
                                                )
                                            );
                                            elements.push(text_expr);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        if !elements.is_empty() {
                            // 构造包含逗号分隔符的数组表达式，每个元素之间用逗号分隔
                            let separators = if elements.len() > 1 {
                                (0..elements.len()-1).map(|_| biome_js_factory::make::token(T![,])).collect()
                            } else {
                                vec![]
                            };
                            
                            let arr = biome_js_factory::make::js_array_expression(
                                biome_js_factory::make::token(T!['[']),
                                biome_js_factory::make::js_array_element_list(
                                    elements.into_iter().map(AnyJsArrayElement::AnyJsExpression).collect::<Vec<_>>(),
                                    separators
                                ),
                                biome_js_factory::make::token(T![']']),
                            );
                            return Some(AnyJsExpression::JsArrayExpression(arr));
                        }
                    }
                    _ => {}
                }
            }
            None
        },
        AnyJsExpression::JsParenthesizedExpression(paren_expr) => {
            if let Ok(inner_expr) = paren_expr.expression() {
                if let Some(transformed_inner) = transform_expression_with_tracker(&inner_expr, tracker) {
                    let new_paren = js_parenthesized_expression(
                        paren_expr.l_paren_token().expect("Missing ("),
                        transformed_inner,
                        paren_expr.r_paren_token().expect("Missing )"),
                    );
                    return Some(AnyJsExpression::JsParenthesizedExpression(new_paren));
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