use biome_js_syntax::*;
use crate::JsxElementInfo;

pub fn collect_jsx_from_expression(expr: &AnyJsExpression, jsx_elements: &mut Vec<JsxElementInfo>, in_function: bool) {
    match expr {
        AnyJsExpression::JsxTagExpression(jsx_tag) => {
            if let Ok(jsx_element_any) = jsx_tag.tag() {
                match jsx_element_any {
                    AnyJsxTag::JsxElement(element) => {
                        if let Ok(opening) = element.opening_element() {
                            if let Ok(name) = opening.name() {
                                if let Some(jsx_name) = name.as_jsx_name() {
                                    if let Ok(name_token) = jsx_name.value_token() {
                                        let tag_name = name_token.text_trimmed().to_string();
                                        jsx_elements.push(JsxElementInfo {
                                            tag_name,
                                            position: jsx_elements.len(),
                                            is_self_closing: false,
                                            in_function,
                                        });
                                    }
                                }
                            }
                        }
                    },
                    AnyJsxTag::JsxSelfClosingElement(self_closing) => {
                        if let Ok(name) = self_closing.name() {
                            if let Some(jsx_name) = name.as_jsx_name() {
                                if let Ok(name_token) = jsx_name.value_token() {
                                    let tag_name = name_token.text_trimmed().to_string();
                                    jsx_elements.push(JsxElementInfo {
                                        tag_name,
                                        position: jsx_elements.len(),
                                        is_self_closing: true,
                                        in_function,
                                    });
                                }
                            }
                        }
                    },
                    _ => {},
                }
            }
        },
        AnyJsExpression::JsArrowFunctionExpression(arrow_fn) => {
            if let Ok(body) = arrow_fn.body() {
                if let AnyJsFunctionBody::AnyJsExpression(expr) = body {
                    collect_jsx_from_expression(&expr, jsx_elements, true);
                }
            }
        },
        AnyJsExpression::JsParenthesizedExpression(paren_expr) => {
            if let Ok(inner_expr) = paren_expr.expression() {
                collect_jsx_from_expression(&inner_expr, jsx_elements, in_function);
            }
        },
        _ => {},
    }
}