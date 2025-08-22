use biome_js_factory::make::{
    ident, js_call_expression, js_expression_statement, js_identifier_expression,
    js_reference_identifier, js_string_literal, js_string_literal_expression, token,
};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression, AnyJsStatement, AnyJsxAttribute,
    AnyJsxAttributeName, AnyJsxAttributeValue, T,
};
use biome_rowan::TriviaPieceKind;
use std::str::FromStr;

use crate::make_js_call_arguments;

/// 生成 _$setProp(el, name, value) 的表达式语句
pub fn make_set_prop(
    id: &str,
    any_js_attribute: AnyJsxAttribute,
) -> Option<AnyJsStatement> {
    let callee = js_identifier_expression(js_reference_identifier(ident("_$setProp")));

    // 1. 第一个参数：id 转为 AST 表达式节点
    let el_ident = js_identifier_expression(js_reference_identifier(ident(id)));
    let mut args = vec![AnyJsCallArgument::AnyJsExpression(el_ident.into())];
    let mut separators = vec![];
    // 2. 处理属性名和属性值
    if let AnyJsxAttribute::JsxAttribute(attr) = any_js_attribute {
        let name = attr.name().ok()?;
        let name_token = match name {
            AnyJsxAttributeName::JsxName(name) => {
                String::from_str(name.value_token().ok()?.text()).ok()?
            }
            AnyJsxAttributeName::JsxNamespaceName(name) => {
                let ns = name.namespace().ok()?;
                let ns_token = ns.value_token().ok()?;
                let ns = ns_token.text();
                let name_val = name.name().ok()?;
                let nm_token = name_val.value_token().ok()?;
                let nm = nm_token.text();
                format!("{ns}:{nm}")
            }
        };

        separators.push(token(T!(,)).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]));

        args.push(AnyJsCallArgument::AnyJsExpression(
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(js_string_literal_expression(
                    js_string_literal(name_token.as_str()),
                )),
            ),
        ));

        let value = attr.initializer().and_then(|init| init.value().ok())?;

        let value_expr = match value {
            AnyJsxAttributeValue::JsxString(str_val) => AnyJsExpression::AnyJsLiteralExpression(
                js_string_literal_expression(str_val.value_token().ok()?).into(),
            ),
            AnyJsxAttributeValue::JsxExpressionAttributeValue(expr_val) => {
                expr_val.expression().ok()?
            }
            AnyJsxAttributeValue::AnyJsxTag(_) => {
                todo!()
            }
        };
        separators.push(token(T!(,)).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]));
        args.push(AnyJsCallArgument::AnyJsExpression(value_expr));
    }

    let call_expr =
        js_call_expression(callee.into(), make_js_call_arguments(args, separators)).build();

    Some(AnyJsStatement::JsExpressionStatement(
        js_expression_statement(AnyJsExpression::JsCallExpression(call_expr)).build(),
    ))
}

#[test]
fn test_make_set_prop() {
    use biome_js_factory::make::ident;
    use biome_js_factory::make::jsx_attribute;
    use biome_js_factory::make::jsx_attribute_initializer_clause;
    use biome_js_factory::make::jsx_expression_attribute_value;
    use biome_js_factory::make::jsx_name;
    use biome_js_factory::make::jsx_namespace_name;
    use biome_js_factory::make::jsx_string;
    use biome_js_factory::make::jsx_string_literal;
    use biome_js_factory::make::token;
    use biome_js_factory::make::{
        js_number_literal, js_number_literal_expression, js_string_literal_expression,
    };
    use biome_js_syntax::AnyJsExpression;
    use biome_js_syntax::AnyJsLiteralExpression;
    use biome_js_syntax::AnyJsxAttribute;
    use biome_js_syntax::T;

    // id="foo"
    let id_attr = AnyJsxAttribute::from(
        jsx_attribute(jsx_name(ident("id")).into())
            .with_initializer(jsx_attribute_initializer_clause(
                token(T![=]),
                jsx_string(jsx_string_literal("foo")).into(),
            ))
            .build(),
    );

    // title={"bar"}
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

    // foo:some={0}
    let foo_attr = AnyJsxAttribute::from(
        jsx_attribute(
            jsx_namespace_name(
                jsx_name(ident("foo")),
                token(T![:]),
                jsx_name(ident("some")),
            )
            .into(),
        )
        .with_initializer(jsx_attribute_initializer_clause(
            token(T![=]),
            jsx_expression_attribute_value(
                token(T!['{']),
                AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsNumberLiteralExpression(
                        js_number_literal_expression(js_number_literal("0")),
                    ),
                ),
                token(T!['}']),
            )
            .into(),
        ))
        .build(),
    );

    let stmt1 = make_set_prop("_el$", id_attr).expect("stmt1 is None");
    let stmt2 = make_set_prop("_el$", title_attr).expect("stmt2 is None");
    let stmt3 = make_set_prop("_el$", foo_attr).expect("stmt3 is None");

    insta::assert_snapshot!(format!(
        "{}\n{}\n{}",
        stmt1.to_string(),
        stmt2.to_string(),
        stmt3.to_string()
    ));
}
