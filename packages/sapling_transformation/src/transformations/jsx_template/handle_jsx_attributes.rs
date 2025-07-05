use biome_js_syntax::*;
use biome_js_factory::make::*;

/// 简单实现：将静态属性、动态属性、事件分别生成 setAttribute/property/事件绑定语句
pub fn handle_jsx_attributes(
    opening_element: JsxOpeningElement,
) -> Option<Vec<AnyJsStatement>> {
    let mut stmts = Vec::new();
    let el_var_token = JsSyntaxToken::new_detached(T![ident], "_el$", Vec::new(), Vec::new());
    let el_var_ident = js_identifier_expression(js_reference_identifier(el_var_token.clone()));
    for attr_result in opening_element.attributes() {
        let attr = attr_result;
        match attr {
            AnyJsxAttribute::JsxAttribute(js_attr) => {
                if let (Ok(name), Some(value)) = (js_attr.name(), js_attr.initializer()) {
                    let name_token = name.name_token().ok();
                    if let Some(name_token) = name_token {
                        if let Some(expr) = value.value().ok() {
                            // _el$.setAttribute("name", value)
                            let value_expr = if let AnyJsxAttributeValue::JsxExpressionAttributeValue(e) = expr {
                                // JsxExpressionAttributeValue 实际表达式节点需进一步 .expression().ok() 获取
                                if let Ok(inner_expr) = e.expression() {
                                    inner_expr
                                } else {
                                    continue;
                                }
                            } else if let AnyJsxAttributeValue::JsxString(e) = expr {
                                let token = e.value_token().ok()?;
                                let s = token.text_trimmed();
                                AnyJsExpression::AnyJsLiteralExpression(
                                    biome_js_syntax::AnyJsLiteralExpression::JsStringLiteralExpression(
                                        js_string_literal_expression(js_string_literal(s))
                                    )
                                )
                            } else {
                                continue;
                            };
                            let set_attr_call = js_expression_statement(
                                AnyJsExpression::JsCallExpression(
                                    js_call_expression(
                                        js_static_member_expression(
                                            el_var_ident.clone().into(),
                                            token(T![.]),
                                            biome_js_syntax::AnyJsName::JsName(
                                                js_name(JsSyntaxToken::new_detached(T![ident], "setAttribute", Vec::new(), Vec::new()))
                                            ),
                                        ).into(),
                                        js_call_arguments(
                                            token(T!['(']),
                                            js_call_argument_list(
                                                vec![
                                                    AnyJsCallArgument::AnyJsExpression(
                                                        AnyJsExpression::AnyJsLiteralExpression(
                                                            biome_js_syntax::AnyJsLiteralExpression::JsStringLiteralExpression(
                                                                js_string_literal_expression(js_string_literal(name_token.text_trimmed()))
                                                            )
                                                        )
                                                    ),
                                                    AnyJsCallArgument::AnyJsExpression(
                                                        value_expr
                                                    ),
                                                ],
                                                vec![],
                                            ),
                                            token(T![')']),
                                        ),
                                    ).build()
                                )
                            ).build();
                            stmts.push(set_attr_call.into());
                        }
                    }
                } else if let Ok(name) = js_attr.name() {
                    let name_token = name.name_token().ok();
                    if let Some(name_token) = name_token {
                        // _el$.setAttribute("name", true)
                        let set_attr_call = js_expression_statement(
                            AnyJsExpression::JsCallExpression(
                                js_call_expression(
                                    js_static_member_expression(
                                        el_var_ident.clone().into(),
                                        token(T![.]),
                                        biome_js_syntax::AnyJsName::JsName(
                                            js_name(JsSyntaxToken::new_detached(T![ident], "setAttribute", Vec::new(), Vec::new()))
                                        ),
                                    ).into(),
                                    js_call_arguments(
                                        token(T!['(']),
                                        js_call_argument_list(
                                            vec![
                                                AnyJsCallArgument::AnyJsExpression(
                                                    AnyJsExpression::AnyJsLiteralExpression(
                                                        biome_js_syntax::AnyJsLiteralExpression::JsStringLiteralExpression(
                                                            js_string_literal_expression(js_string_literal(name_token.text_trimmed()))
                                                        )
                                                    )
                                                ),
                                                AnyJsCallArgument::AnyJsExpression(
                                                    AnyJsExpression::AnyJsLiteralExpression(
                                                        biome_js_syntax::AnyJsLiteralExpression::JsBooleanLiteralExpression(
                                                            js_boolean_literal_expression(token(T![true]))
                                                        )
                                                    )
                                                ),
                                            ],
                                            vec![],
                                        ),
                                        token(T![')']),
                                    ),
                                ).build()
                            )
                        ).build();
                        stmts.push(set_attr_call.into());
                    }
                }
            }
            AnyJsxAttribute::JsxSpreadAttribute(js_spread) => {
                if let Ok(expr) = js_spread.argument() {
                    let assign = js_expression_statement(
                        AnyJsExpression::JsCallExpression(
                            js_call_expression(
                                js_identifier_expression(
                                    js_reference_identifier(
                                        JsSyntaxToken::new_detached(T![ident], "Object.assign", Vec::new(), Vec::new())
                                    )
                                ).into(),
                                js_call_arguments(
                                    token(T!['(']),
                                    js_call_argument_list(
                                        vec![
                                            AnyJsCallArgument::AnyJsExpression(el_var_ident.clone().into()),
                                            AnyJsCallArgument::AnyJsExpression(expr),
                                        ],
                                        vec![],
                                    ),
                                    token(T![')']),
                                ),
                            ).build()
                        )
                    ).build();
                    stmts.push(assign.into());
                }
            }
            _ => {}
        }
    }
    if stmts.is_empty() {
        None
    } else {
        Some(stmts)
    }
}