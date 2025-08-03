use biome_js_factory::make::{
    ident, js_call_expression, js_expression_statement, js_identifier_expression, js_reference_identifier, js_string_literal,
    js_string_literal_expression, token,
};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression, AnyJsStatement, AnyJsxAttribute,
    AnyJsxAttributeName, AnyJsxAttributeValue, T,
};
use biome_rowan::TriviaPieceKind;
use sapling_transformation::helpers::jsx_template::make_js_call_arguments;
use std::str::FromStr;

/// 生成 _$setProp(el, name, value) 的表达式语句
pub fn generate_set_prop_statement(
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
                AnyJsLiteralExpression::JsStringLiteralExpression(
                    js_string_literal_expression(js_string_literal(name_token.as_str())),
                ),
            ),
        ));

        let value = attr.initializer().and_then(|init| init.value().ok())?;

        let value_expr = match value {
            AnyJsxAttributeValue::JsxString(str_val) => {
                AnyJsExpression::AnyJsLiteralExpression(
                    js_string_literal_expression(str_val.value_token().ok()?).into(),
                )
            }
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