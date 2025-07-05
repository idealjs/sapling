use biome_js_syntax::*;
use biome_js_factory::make::*;
use crate::jsx_template::{handle_jsx_attributes};
use crate::create_insert_text_node_with_tracker;
use crate::create_insert_expression_node_with_tracker;

use crate::jsx_template::HelperUsageTracker;
pub fn create_solidjs_call_with_tracker(jsx_element: &JsxElement, tracker: &mut HelperUsageTracker) -> Option<AnyJsExpression> {
    // 获取元素名称
    let opening_element = jsx_element.opening_element().ok()?;
    let element_name = opening_element.name().ok()?;
    let jsx_name = element_name.as_jsx_name()?;
    let tag_token = jsx_name.value_token().ok()?;
    let tag_name = tag_token.text_trimmed();
    // 创建语句列表
    let mut statements = Vec::<AnyJsStatement>::new();
    // 1. var _el$ = _$createElement("tagName");
    let el_var_token = JsSyntaxToken::new_detached(T![ident], "_el$", Vec::new(), Vec::new());
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
    statements.push(var_decl.into());
    // 2. 处理属性（框架）
    if let Some(attr_stmts) = handle_jsx_attributes(opening_element) {
        statements.extend(attr_stmts);
    }
    // 3. 处理子元素
    let children = jsx_element.children();
    for child in children {
        match child {
            AnyJsxChild::JsxText(text_node) => {
                // 处理文本节点
                let text_token = text_node.value_token().ok()?;
                let text_content = text_token.text_trimmed();
                if !text_content.trim().is_empty() {
                    if let Some(stmt) = create_insert_text_node_with_tracker(&text_content, tracker) {
                        statements.push(stmt);
                    }
                }
            }
            AnyJsxChild::JsxExpressionChild(expr_child) => {
                // 处理表达式子元素
                if let Some(stmt) = create_insert_expression_node_with_tracker(&expr_child, tracker) {
                    statements.push(stmt);
                }
            }
            _ => {
                // 暂时跳过其他类型的子元素
            }
        }
    }
    // 4. return _el$;
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