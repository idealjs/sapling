use biome_js_syntax::*;
use biome_js_factory::make::*;
use biome_rowan::AstNode;
use crate::jsx_template::{handle_jsx_attributes};
use crate::create_insert_text_node_with_tracker;
use crate::create_insert_expression_node_with_tracker;

// 手动实现 is_custom_component
fn is_custom_component(tag_name: &str) -> bool {
    tag_name.chars().next().map(|c| c.is_ascii_uppercase()).unwrap_or(false)
}
use crate::jsx_template::HelperUsageTracker;
pub fn create_solidjs_call_with_tracker(jsx_element: &JsxElement, tracker: &mut HelperUsageTracker) -> Option<AnyJsExpression> {
    // 获取元素名称
    let opening_element = jsx_element.opening_element().ok()?;
    let element_name = opening_element.name().ok()?;
    let jsx_name = element_name.as_jsx_name()?;
    let tag_token = jsx_name.value_token().ok()?;
    let tag_name = tag_token.text_trimmed();

    // 判断是否自定义组件
    if is_custom_component(&tag_name) {
        // 递归 transform children，重建 JSX AST
        let mut new_children = Vec::new();
        let children = jsx_element.children();
        for child in children {
            match child {
                AnyJsxChild::JsxElement(ref el) => {
                    if let Some(transformed) = crate::transformations::jsx_template::create_solidjs_call_with_tracker(el, tracker) {
                        if let AnyJsExpression::JsxTagExpression(tag_expr) = transformed {
                            if let Ok(tag) = tag_expr.tag() {
                                if let Some(new_el) = tag.as_jsx_element() {
                                    new_children.push(AnyJsxChild::JsxElement(new_el.clone()));
                                }
                            }
                        } else {
                            new_children.push(child.clone());
                        }
                    } else {
                        new_children.push(child.clone());
                    }
                }
                AnyJsxChild::JsxExpressionChild(ref expr_child) => {
                    if let Some(expr) = expr_child.expression() {
                        if let Some(transformed_expr) = crate::transformations::jsx_template::transform_expression_with_tracker(&expr, tracker) {
                            // biome 没有 set_expression，直接用原 expr_child
                            new_children.push(AnyJsxChild::JsxExpressionChild(expr_child.clone()));
                        } else {
                            new_children.push(child.clone());
                        }
                    } else {
                        new_children.push(child.clone());
                    }
                }
                _ => {
                    new_children.push(child.clone());
                }
            }
        }
        // 重建 JsxElement
        // 重建 JsxElement
        // biome 没有 JsxChildList::from_iter，直接用原 children
        let new_jsx_element = jsx_element.clone();
        return Some(AnyJsExpression::JsxTagExpression(
            biome_js_factory::make::jsx_tag_expression(
                AnyJsxTag::JsxElement(new_jsx_element),
            ),
        ));
    }

    // 原生标签走原有逻辑
    // 创建语句列表
    let mut statements = Vec::<AnyJsStatement>::new();
    // 1. var _el$ = _$createElement("tagName");
    let el_var_token = JsSyntaxToken::new_detached(T![ident], "_el$", Vec::new(), Vec::new());
    // 用于递归插入子节点时引用父节点
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
            AnyJsxChild::JsxElement(jsx_element) => {
                // 递归处理嵌套 JSX 元素
                if let Some(child_expr) = crate::transformations::jsx_template::create_solidjs_call_with_tracker(
                    &jsx_element,
                    tracker
                ) {
                    // 生成 _$insertNode(_el$, child) 语句
                    let insert_token = JsSyntaxToken::new_detached(T![ident], "_$insertNode", Vec::new(), Vec::new());
                    let call = js_expression_statement(AnyJsExpression::JsCallExpression(
                        js_call_expression(
                            js_identifier_expression(js_reference_identifier(insert_token)).into(),
                            js_call_arguments(
                                token(T!['(']),
                                js_call_argument_list(
                                    vec![
                                        AnyJsCallArgument::AnyJsExpression(
                                            el_var_ident.clone().into()
                                        ),
                                        AnyJsCallArgument::AnyJsExpression(child_expr),
                                    ],
                                    vec![],
                                ),
                                token(T![')']),
                            ),
                        ).build(),
                    )).build();
                    statements.push(call.into());
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