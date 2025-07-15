use crate::SaplingTransformer;
use biome_js_factory::make::{
    ident, js_call_expression, js_expression_statement, js_identifier_binding,
    js_identifier_expression, js_initializer_clause, js_reference_identifier, js_string_literal,
    js_string_literal_expression, js_variable_declaration, js_variable_declarator,
    js_variable_declarator_list, js_variable_statement, token,
};
use biome_js_syntax::AnyJsxAttribute;
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression,
    AnyJsStatement, AnyJsxAttributeName, AnyJsxAttributeValue, T,
};
use biome_rowan::TriviaPieceKind;
use sapling_transformation::helpers::jsx_template::make_js_call_arguments;
use std::str::FromStr;

impl SaplingTransformer {
    pub fn create_js_tag_statement(&self, id: &str, tag_name: &str) -> AnyJsStatement {
        // 构造 let _el$ = _$createElement("div");
        let callee = js_identifier_expression(js_reference_identifier(ident("_$createElement")));
        let arg = AnyJsCallArgument::AnyJsExpression(AnyJsExpression::AnyJsLiteralExpression(
            js_string_literal_expression(js_string_literal(tag_name)).into(),
        ));
        let call_expr =
            js_call_expression(callee.into(), make_js_call_arguments(vec![arg], vec![])).build();

        let binding = js_identifier_binding(ident(id));
        let declarator = js_variable_declarator(AnyJsBindingPattern::AnyJsBinding(
            AnyJsBinding::JsIdentifierBinding(binding),
        ))
        .with_initializer(js_initializer_clause(
            token(T![=])
                .with_leading_trivia([(TriviaPieceKind::Whitespace, " ")])
                .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            call_expr.into(),
        ))
        .build();

        // 让 let 和变量名之间有空格
        let let_token = token(T![let]);
        let let_token_with_space =
            let_token.with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);

        let var_decl = js_variable_declaration(
            let_token_with_space,
            js_variable_declarator_list([declarator], []),
        )
        .build();

        // 添加分号
        let semicolon_token = token(T![;]);
        let var_stmt = js_variable_statement(var_decl)
            .with_semicolon_token(semicolon_token)
            .build();

        AnyJsStatement::JsVariableStatement(var_stmt)
    }
    pub fn create_set_prop_statement(
        &self,
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

            separators
                .push(token(T!(,)).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]));

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
            separators
                .push(token(T!(,)).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]));
            args.push(AnyJsCallArgument::AnyJsExpression(value_expr));
        }

        let call_expr =
            js_call_expression(callee.into(), make_js_call_arguments(args, separators)).build();

        Some(AnyJsStatement::JsExpressionStatement(
            js_expression_statement(AnyJsExpression::JsCallExpression(call_expr)).build(),
        ))
    }
}
