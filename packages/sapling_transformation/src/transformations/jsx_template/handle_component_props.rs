use biome_js_syntax::*;
use biome_js_factory::make::*;
use biome_rowan::AstNodeList;
use crate::jsx_template::{handle_jsx_attributes};
use crate::jsx_template::HelperUsageTracker;
use crate::transformations::jsx_template::create_children_expression::create_children_expression;

pub fn handle_component_props(
    opening_element: &JsxOpeningElement,
    jsx_element: &JsxElement,
    tracker: &mut HelperUsageTracker,
) -> Option<AnyJsExpression> {
    let mut props = Vec::new();

    let attributes = opening_element.attributes();
    for attr in attributes {
        if let AnyJsxAttribute::JsxAttribute(jsx_attr) = attr {
            let name = match jsx_attr.name() {
                Ok(n) => n,
                Err(_) => continue,
            };
            let value = match jsx_attr.initializer() {
                Some(v) => v,
                None => continue,
            };
            let attr_value = match value.value() {
                Ok(v) => v,
                Err(_) => continue,
            };
            let prop_name = match name {
                AnyJsxAttributeName::JsxName(jsx_name) => {
                    jsx_name.value_token().ok()?.text_trimmed().to_string()
                }
                _ => continue,
            };

            let prop_value = match attr_value {
                AnyJsxAttributeValue::JsxString(jsx_string) => {
                    AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(
                            js_string_literal_expression(jsx_string.value_token().ok()?),
                        ),
                    )
                }
                AnyJsxAttributeValue::JsxExpressionAttributeValue(expr_attr) => {
                    match expr_attr.expression() {
                        Ok(expr) => expr,
                        Err(_) => continue,
                    }
                }
                _ => continue,
            };

            let prop = js_property_object_member(
                AnyJsObjectMemberName::JsLiteralMemberName(
                    js_literal_member_name(
                        JsSyntaxToken::new_detached(T![ident], &prop_name, Vec::new(), Vec::new()),
                    ),
                ),
                token(T![:]),
                prop_value,
            );

            props.push(AnyJsObjectMember::JsPropertyObjectMember(prop));
        }
    }

    // 处理 children
    let children = jsx_element.children();
    if AstNodeList::is_empty(&children) == false {
        let children_expr = create_children_expression(&children, tracker)?;
        
        let children_prop = js_property_object_member(
            AnyJsObjectMemberName::JsLiteralMemberName(
                js_literal_member_name(
                    JsSyntaxToken::new_detached(T![ident], "children", Vec::new(), Vec::new())
                )
            ),
            token(T![:]),
            children_expr,
        );

        props.push(AnyJsObjectMember::JsPropertyObjectMember(children_prop));
    }

    // 创建 props 对象
    let props_obj = js_object_expression(
        token(T!['{']),
        js_object_member_list(props, vec![]),
        token(T!['}']),
    );

    Some(AnyJsExpression::JsObjectExpression(props_obj))
}