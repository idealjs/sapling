use biome_js_factory::make::{
    ident, js_call_expression, js_identifier_binding, js_identifier_expression,
    js_initializer_clause, js_reference_identifier, js_string_literal,
    js_string_literal_expression, js_variable_declaration, js_variable_declarator,
    js_variable_declarator_list, js_variable_statement, token,
};
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument, AnyJsExpression, AnyJsStatement, T,
};
use biome_rowan::TriviaPieceKind;

use crate::make_js_call_arguments;

/// 生成 let id = _$createElement(tag_name); 的语句
pub fn make_create_element(id: &str, tag_name: &str) -> AnyJsStatement {
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
    let let_token_with_space = let_token.with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);

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

#[test]
fn test_make_create_element() {
    let stmt1 = make_create_element("_el1$", "div");
    let stmt2 = make_create_element("_el2$", "div");
    insta::assert_snapshot!(format!("{}\n{}", stmt1.to_string(), stmt2.to_string()));
}
