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
    prop_key: &str,
    prop_value: AnyJsExpression,
) -> Option<AnyJsStatement> {
    let callee = js_identifier_expression(js_reference_identifier(ident("_$setProp")));

    // 1. 第一个参数：id 转为 AST 表达式节点
    let el_ident = js_identifier_expression(js_reference_identifier(ident(id)));
    let mut args = vec![AnyJsCallArgument::AnyJsExpression(el_ident.into())];
    let mut separators = vec![];

    // 2. 属性名作为第二个参数（字符串）
    separators.push(token(T!(,)).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]));
    args.push(AnyJsCallArgument::AnyJsExpression(
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(js_string_literal_expression(
                js_string_literal(prop_key),
            )),
        ),
    ));

    // 3. 处理属性值（作为第三个参数）
    // 现在 prop_value 已经是 AnyJsExpression，直接使用即可
    let value_expr = prop_value;
    separators.push(token(T!(,)).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]));
    args.push(AnyJsCallArgument::AnyJsExpression(value_expr));

    let call_expr =
        js_call_expression(callee.into(), make_js_call_arguments(args, separators)).build();

    let stmt = AnyJsStatement::JsExpressionStatement(
        js_expression_statement(AnyJsExpression::JsCallExpression(call_expr)).build(),
    );

    Some(stmt)
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

    // 从构造的 AnyJsxAttribute 中提取 value 并调用新的签名（将 AnyJsxAttributeValue 转为 AnyJsExpression）
    let id_value = match &id_attr {
        AnyJsxAttribute::JsxAttribute(a) => {
            let raw = a.initializer().unwrap().value().ok().unwrap();
            match raw {
                AnyJsxAttributeValue::JsxString(str_val) => AnyJsExpression::AnyJsLiteralExpression(
                    js_string_literal_expression(str_val.value_token().ok().unwrap()).into(),
                ),
                AnyJsxAttributeValue::JsxExpressionAttributeValue(expr_val) => {
                    expr_val.expression().ok().unwrap()
                }
                AnyJsxAttributeValue::AnyJsxTag(_) => panic!(),
            }
        }
        _ => panic!(),
    };
    let stmt1 = make_set_prop("_el$", "id", id_value).expect("stmt1 is None");
    
    let title_value = match &title_attr {
        AnyJsxAttribute::JsxAttribute(a) => {
            let raw = a.initializer().unwrap().value().ok().unwrap();
            match raw {
                AnyJsxAttributeValue::JsxString(str_val) => AnyJsExpression::AnyJsLiteralExpression(
                    js_string_literal_expression(str_val.value_token().ok().unwrap()).into(),
                ),
                AnyJsxAttributeValue::JsxExpressionAttributeValue(expr_val) => {
                    expr_val.expression().ok().unwrap()
                }
                AnyJsxAttributeValue::AnyJsxTag(_) => panic!(),
            }
        }
        _ => panic!(),
    };
    let stmt2 = make_set_prop("_el$", "title", title_value).expect("stmt2 is None");
    
    let foo_value = match &foo_attr {
        AnyJsxAttribute::JsxAttribute(a) => {
            let raw = a.initializer().unwrap().value().ok().unwrap();
            match raw {
                AnyJsxAttributeValue::JsxString(str_val) => AnyJsExpression::AnyJsLiteralExpression(
                    js_string_literal_expression(str_val.value_token().ok().unwrap()).into(),
                ),
                AnyJsxAttributeValue::JsxExpressionAttributeValue(expr_val) => {
                    expr_val.expression().ok().unwrap()
                }
                AnyJsxAttributeValue::AnyJsxTag(_) => panic!(),
            }
        }
        _ => panic!(),
    };
    let stmt3 = make_set_prop("_el$", "foo:some", foo_value).expect("stmt3 is None");

    insta::assert_snapshot!(format!(
        "{}\n{}\n{}",
        stmt1.to_string(),
        stmt2.to_string(),
        stmt3.to_string()
    ));
}
