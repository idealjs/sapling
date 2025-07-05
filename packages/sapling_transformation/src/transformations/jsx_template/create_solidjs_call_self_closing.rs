use biome_js_syntax::*;
use biome_js_factory::make::*;
use crate::jsx_template::HelperUsageTracker;
use crate::jsx_template::handle_jsx_self_closing_attributes::handle_jsx_self_closing_attributes;

/// 处理自闭合 JSX 标签的 SolidJS 转换
pub fn create_solidjs_call_with_tracker_self_closing(
    jsx_element: &JsxSelfClosingElement,
    tracker: &mut HelperUsageTracker,
) -> Option<AnyJsExpression> {
    // 获取元素名称
    let element_name = jsx_element.name().ok()?;
    let jsx_name = element_name.as_jsx_name()?;
    let tag_token = jsx_name.value_token().ok()?;
    let tag_name = tag_token.text_trimmed();

    // 1. var _el$ = _$createElement("tagName");
    let el_var_token = JsSyntaxToken::new_detached(T![ident], "_el$", Vec::new(), Vec::new());
    let el_var_ident = js_identifier_expression(js_reference_identifier(el_var_token.clone()));
    let create_element_token =
        JsSyntaxToken::new_detached(T![ident], "_$createElement", Vec::new(), Vec::new());
    tracker.create_element = true;
    let declarator = js_variable_declarator(AnyJsBindingPattern::AnyJsBinding(
        AnyJsBinding::JsIdentifierBinding(js_identifier_binding(el_var_token.clone())),
    ))
    .with_initializer(
        js_initializer_clause(
            token(T![=]),
            js_call_expression(
                js_identifier_expression(js_reference_identifier(create_element_token)).into(),
                js_call_arguments(
                    token(T!['(']),
                    js_call_argument_list(
                        vec![AnyJsCallArgument::AnyJsExpression(
                            AnyJsExpression::AnyJsLiteralExpression(
                                biome_js_syntax::AnyJsLiteralExpression::JsStringLiteralExpression(
                                    js_string_literal_expression(js_string_literal(
                                        tag_name.as_ref(),
                                    )),
                                ),
                            ),
                        )],
                        vec![],
                    ),
                    token(T![')']),
                ),
            )
            .build()
            .into(),
        )
        .into(),
    )
    .build();
    let var_decl = js_variable_statement(
        js_variable_declaration(
            token(T![var]),
            js_variable_declarator_list(vec![declarator], vec![]),
        )
        .build(),
    )
    .build();

    let mut statements = Vec::<AnyJsStatement>::new();
    statements.push(var_decl.into());

    // 2. 处理属性
    // 2. 处理属性
    if let Some(attr_stmts) = handle_jsx_self_closing_attributes(jsx_element) {
        statements.extend(attr_stmts);
    }
    // 3. return _el$;
    let return_stmt = js_return_statement(token(T![return]))
        .with_argument(js_identifier_expression(js_reference_identifier(el_var_token)).into())
        .with_semicolon_token(token(T![;]))
        .build();
    statements.push(return_stmt.into());

    // 创建箭头函数
    let function_body = js_function_body(
        token(T!['{']),
        js_directive_list(vec![]),
        js_statement_list(statements),
        token(T!['}']),
    );
    let params = js_parameters(
        token(T!['(']),
        js_parameter_list(vec![], vec![]),
        token(T![')']),
    );
    let arrow_fn = js_arrow_function_expression(
        params.into(),
        token(T![=>]),
        function_body.into(),
    )
    .build();

    // 创建立即执行函数调用
    let iife = js_call_expression(
        js_parenthesized_expression(
            token(T!['(']),
            AnyJsExpression::JsArrowFunctionExpression(arrow_fn),
            token(T![')']),
        )
        .into(),
        js_call_arguments(
            token(T!['(']),
            js_call_argument_list(vec![], vec![]),
            token(T![')']),
        ),
    ).build();

    Some(AnyJsExpression::JsCallExpression(iife))
}