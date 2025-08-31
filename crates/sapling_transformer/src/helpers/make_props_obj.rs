use biome_js_factory::make::{
    ident, js_literal_member_name, js_object_expression, js_object_member_list,
    js_property_object_member, js_string_literal, js_string_literal_expression, jsx_tag_expression,
    token,
};
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsLiteralExpression, AnyJsxAttribute,
    AnyJsxAttributeName, AnyJsxAttributeValue, AnyJsxChild, AnyJsxTag, JsxAttributeList,
    JsxChildList,
};
use biome_js_syntax::{AnyJsObjectMember, JsObjectExpression, T};

use crate::make_array;

pub fn make_props_obj(
    attributes: JsxAttributeList,
    jsx_child_list: JsxChildList,
) -> Option<JsObjectExpression> {
    fn is_valid_ident(s: &str) -> bool {
        let mut chars = s.chars();
        match chars.next() {
            Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
            }
            _ => false,
        }
    }

    let mut members: Vec<AnyJsObjectMember> = vec![];

    for attribute in attributes.into_iter() {
        let attr = match attribute {
            AnyJsxAttribute::JsxAttribute(a) => a,
            _ => continue,
        };

        let name_token = match attr.name().ok() {
            Some(name_node) => match name_node {
                AnyJsxAttributeName::JsxName(name) => {
                    if let Some(tok) = name.value_token().ok() {
                        tok.text_trimmed().to_string()
                    } else {
                        continue;
                    }
                }
                AnyJsxAttributeName::JsxNamespaceName(ns_name) => {
                    let ns = match ns_name.namespace().ok() {
                        Some(n) => match n.value_token().ok() {
                            Some(t) => t.text_trimmed().to_string(),
                            None => continue,
                        },
                        None => continue,
                    };
                    let nm = match ns_name.name().ok() {
                        Some(nv) => match nv.value_token().ok() {
                            Some(t) => t.text_trimmed().to_string(),
                            None => continue,
                        },
                        None => continue,
                    };
                    format!("{ns}:{nm}")
                }
            },
            None => continue,
        };

        if let Some(init) = attr.initializer() {
            if let Ok(value_node) = init.value() {
                let expr_opt: Option<AnyJsExpression> = match value_node {
                    AnyJsxAttributeValue::JsxString(str_val) => {
                        if let Some(tok) = str_val.value_token().ok() {
                            let s = tok.text_trimmed().to_string();

                            let unquoted = s.trim_matches(|c| c == '"' || c == '\'').to_string();
                            Some(AnyJsExpression::AnyJsLiteralExpression(
                                AnyJsLiteralExpression::JsStringLiteralExpression(
                                    js_string_literal_expression(js_string_literal(&unquoted)),
                                ),
                            ))
                        } else {
                            None
                        }
                    }

                    AnyJsxAttributeValue::JsxExpressionAttributeValue(expr_val) => {
                        expr_val.expression().ok()
                    }

                    AnyJsxAttributeValue::AnyJsxTag(_) => None,
                };

                if let Some(expr) = expr_opt {
                    let name_member = if is_valid_ident(&name_token) {
                        js_literal_member_name(ident(&name_token))
                    } else {
                        js_literal_member_name(js_string_literal(&name_token))
                    };
                    let member = js_property_object_member(
                        biome_js_syntax::AnyJsObjectMemberName::JsLiteralMemberName(name_member),
                        token(T![:]),
                        expr,
                    );
                    members.push(AnyJsObjectMember::JsPropertyObjectMember(member));
                }
            }
        }
    }

    let mut elements: Vec<AnyJsArrayElement> = vec![];
    jsx_child_list.into_iter().for_each(|node| {
        let el_opt: Option<AnyJsExpression> = match node {
            AnyJsxChild::JsMetavariable(_) => None,

            AnyJsxChild::JsxElement(node) => Some(AnyJsExpression::JsxTagExpression(
                jsx_tag_expression(AnyJsxTag::JsxElement(node)),
            )),

            AnyJsxChild::JsxSelfClosingElement(node) => Some(AnyJsExpression::JsxTagExpression(
                jsx_tag_expression(AnyJsxTag::JsxSelfClosingElement(node)),
            )),

            AnyJsxChild::JsxExpressionChild(expr_child) => expr_child.expression(),

            AnyJsxChild::JsxFragment(node) => Some(AnyJsExpression::JsxTagExpression(
                jsx_tag_expression(AnyJsxTag::JsxFragment(node)),
            )),

            AnyJsxChild::JsxSpreadChild(_) => None,

            AnyJsxChild::JsxText(text_node) => {
                if let Some(tok) = text_node.value_token().ok() {
                    let txt = tok.text_trimmed();

                    if txt.trim().is_empty() {
                        None
                    } else {
                        let content = txt.to_string();
                        Some(AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsStringLiteralExpression(
                                js_string_literal_expression(js_string_literal(&content)),
                            ),
                        ))
                    }
                } else {
                    None
                }
            }
        };

        if let Some(el) = el_opt {
            elements.push(AnyJsArrayElement::AnyJsExpression(el));
        }
    });

    if !elements.is_empty() {
        let arr_expr = AnyJsExpression::JsArrayExpression(make_array(elements));
        let name_member = js_literal_member_name(ident("children"));
        let member = js_property_object_member(
            biome_js_syntax::AnyJsObjectMemberName::JsLiteralMemberName(name_member),
            token(T![:]),
            arr_expr,
        );
        members.push(AnyJsObjectMember::JsPropertyObjectMember(member));
    }

    Some(js_object_expression(
        token(T!['{']),
        js_object_member_list(
            members.clone(),
            vec![token(T![,]); members.len().saturating_sub(1)],
        ),
        token(T!['}']),
    ))
}
#[test]
fn test_make_props_obj_empty() {
    use biome_js_factory::make::{
        ident, js_identifier_expression, js_reference_identifier, js_string_literal_expression,
        jsx_attribute, jsx_attribute_initializer_clause, jsx_attribute_list, jsx_child_list,
        jsx_expression_attribute_value, jsx_expression_child, jsx_name, jsx_string,
        jsx_string_literal, token,
    };
    use biome_js_syntax::{AnyJsExpression, AnyJsLiteralExpression, AnyJsxAttribute, T};
    use insta::assert_snapshot;

    let id_attr = AnyJsxAttribute::from(
        jsx_attribute(jsx_name(ident("id")).into())
            .with_initializer(jsx_attribute_initializer_clause(
                token(T![=]),
                jsx_string(jsx_string_literal("a")).into(),
            ))
            .build(),
    );
    let title_attr = AnyJsxAttribute::from(
        jsx_attribute(jsx_name(ident("title")).into())
            .with_initializer(jsx_attribute_initializer_clause(
                token(T![=]),
                jsx_expression_attribute_value(
                    token(T!['{']),
                    AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(
                            js_string_literal_expression(jsx_string_literal("bar")),
                        ),
                    ),
                    token(T!['}']),
                )
                .into(),
            ))
            .build(),
    );

    let attrs = jsx_attribute_list(vec![id_attr, title_attr]);

    let expr = AnyJsExpression::JsIdentifierExpression(js_identifier_expression(
        js_reference_identifier(ident("a")),
    ));
    let child = jsx_expression_child(token(T!['{']), token(T!['}']))
        .with_expression(expr)
        .build()
        .into();
    let children = jsx_child_list(vec![child]);

    let props = make_props_obj(attrs, children);
    assert_snapshot!(format!("{:#?}", props));
}
