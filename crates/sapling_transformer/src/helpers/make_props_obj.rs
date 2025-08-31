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

/// 将 JSX 的 attributes 与 children 转换为一组 props 键值对（Vec<(key, value)>）
///
/// 说明：项目中尚未找到统一的“创建对象字面量”的工厂调用点（js_object_expression 等在本仓库未直接使用）；
/// 为避免改变公共 API 或引入不确定的工厂函数，本函数返回一个键值对数组，调用者可以据此生成对象字面量或逐个 setProp。
///
/// 转换规则（与 transformer 中其它地方保持一致）：
/// - 属性名支持普通 name 与 namespace:name（例如 foo:bar 会被转换为 "foo:bar"）
/// - 字符串属性（title="abc"）会被转为字符串字面量表达式
/// - 表达式属性（title={expr}）会提取内部表达式作为值
/// - 忽略无法转换的属性值（例如复杂的 JsxTag 直接忽略）
/// - children 会被收集为数组并作为 "children" 属性加入（仅当 children 能被转换为表达式时才加入）
///
/// 示例（转换前 -> 由本函数输出的键值对表示）：
/// 转换前 JSX:
/// <Comp id="a" value={x}>
///   {y}
///   <Child />
/// </Comp>
///
/// 转换后（由本函数返回的 Vec）类似于:
/// [("id", js string "a"), ("value", <expr x>), ("children", [ <expr y>, <jsx child expr> ])]
///
/// 返回类型：Vec<(String, AnyJsExpression)> — 便于调用方选择生成对象字面量或逐个调用 make_set_prop。
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

    // 处理 attributes（与原实现类似，但构建为对象成员）
    for attribute in attributes.into_iter() {
        let attr = match attribute {
            AnyJsxAttribute::JsxAttribute(a) => a,
            _ => continue,
        };

        // 提取 name（支持 namespace）
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

        // 提取 value
        if let Some(init) = attr.initializer() {
            if let Ok(value_node) = init.value() {
                let expr_opt: Option<AnyJsExpression> = match value_node {
                    // 对于 JSX 字符串属性，需要去掉 token 上的外层引号后创建字面量表达式
                    AnyJsxAttributeValue::JsxString(str_val) => {
                        if let Some(tok) = str_val.value_token().ok() {
                            let s = tok.text_trimmed().to_string();
                            // 去掉包裹的单/双引号（安全处理）
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
                    // 表达式属性直接取内部表达式
                    AnyJsxAttributeValue::JsxExpressionAttributeValue(expr_val) => {
                        expr_val.expression().ok()
                    }
                    // 如果属性值是一个嵌套的 JSX 标签（复杂），忽略
                    AnyJsxAttributeValue::AnyJsxTag(_) => None,
                };

                if let Some(expr) = expr_opt {
                    // 构建对象成员：使用 identifier 作为成员名（若不合法则使用字符串字面量）
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

    // 处理 children：支持 ExpressionChild、Element、SelfClosingElement、Fragment、Text
    let mut elements: Vec<AnyJsArrayElement> = vec![];
    jsx_child_list.into_iter().for_each(|node| {
        let el_opt: Option<AnyJsExpression> = match node {
            AnyJsxChild::JsMetavariable(_) => None,
            // <Child>...</Child>
            AnyJsxChild::JsxElement(node) => Some(AnyJsExpression::JsxTagExpression(
                jsx_tag_expression(AnyJsxTag::JsxElement(node)),
            )),
            // 自闭合元素 <Child />
            AnyJsxChild::JsxSelfClosingElement(node) => Some(AnyJsExpression::JsxTagExpression(
                jsx_tag_expression(AnyJsxTag::JsxSelfClosingElement(node)),
            )),
            // 表达式子节点 { expr }
            AnyJsxChild::JsxExpressionChild(expr_child) => expr_child.expression(),
            // 片段转换为 JsxTagExpression (保留片段语义)
            AnyJsxChild::JsxFragment(node) => Some(AnyJsExpression::JsxTagExpression(
                jsx_tag_expression(AnyJsxTag::JsxFragment(node)),
            )),
            // 展开子节点忽略
            AnyJsxChild::JsxSpreadChild(_) => None,
            // 文本节点转换为字符串字面量（去除仅 whitespace 的文本）
            AnyJsxChild::JsxText(text_node) => {
                if let Some(tok) = text_node.value_token().ok() {
                    let txt = tok.text_trimmed();
                    // 跳过纯空白文本
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

    // 构建对象字面量并返回
    // 构建对象字面量并返回
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

    // 构造包含少量属性的 JsxAttributeList（id="a", title={"bar"}），children 保持空
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
                // 使用表达式属性包装字符串字面量
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

    // 构造一个表达式子节点 {a}
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
