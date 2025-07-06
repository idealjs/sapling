use crate::jsx_template::create_solidjs_call_with_tracker::create_solidjs_call_with_tracker;
use biome_js_syntax::*;
use biome_js_factory::make::*;
use biome_rowan::AstNode;
use crate::jsx_template::HelperUsageTracker;
use crate::transformations::jsx_template::create_text_insert_statement::create_text_insert_statement;
use crate::transformations::jsx_template::create_expression_insert_statement::create_expression_insert_statement;
use crate::transformations::jsx_template::create_child_insert_statement::create_child_insert_statement;

pub fn create_native_element(jsx_element: &JsxElement, tag_name: &str, tracker: &mut HelperUsageTracker) -> Option<AnyJsExpression> {
    // 创建语句列表
    let mut statements = Vec::<AnyJsStatement>::new();
    
    // 生成唯一的变量名
    static mut COUNTER: u32 = 0;
    unsafe { COUNTER += 1; }
    let el_var_name = format!("_el${}", unsafe { COUNTER });
    
    // 1. var _el$ = _$createElement("tagName");
    let el_var_token = JsSyntaxToken::new_detached(T![ident], &el_var_name, Vec::new(), Vec::new());
    let el_var_ident = js_identifier_expression(js_reference_identifier(el_var_token.clone()));
    
    tracker.create_element = true;
    
    let create_element_call = js_call_expression(
        js_identifier_expression(js_reference_identifier(
            JsSyntaxToken::new_detached(T![ident], "_$createElement", Vec::new(), Vec::new())
        )).into(),
        js_call_arguments(
            token(T!['(']),
            js_call_argument_list(
                vec![AnyJsCallArgument::AnyJsExpression(
                    AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(
                            js_string_literal_expression(js_string_literal(tag_name)),
                        ),
                    ),
                )],
                vec![],
            ),
            token(T![')']),
        ),
    ).build();

    let declarator = js_variable_declarator(
        AnyJsBindingPattern::AnyJsBinding(
            AnyJsBinding::JsIdentifierBinding(js_identifier_binding(el_var_token.clone()))
        )
    ).with_initializer(
        js_initializer_clause(token(T![=]), create_element_call.into()).into(),
    ).build();

    let var_decl = js_variable_statement(
        js_variable_declaration(
            token(T![var]),
            js_variable_declarator_list(vec![declarator], vec![]),
        ).build(),
    ).build();
    
    statements.push(var_decl.into());

    // 1.5. 处理属性（包括ref属性）
    let opening_element = jsx_element.opening_element().ok()?;
    for attr in opening_element.attributes() {
        match attr {
            AnyJsxAttribute::JsxAttribute(jsx_attr) => {
                if let Ok(name) = jsx_attr.name() {
                    if let Ok(name_token) = name.name_token() {
                        let attr_name = name_token.text_trimmed();
                        
                        // 处理 ref 属性
                        if attr_name == "ref" {
                            if let Some(initializer) = jsx_attr.initializer() {
                                if let Ok(value) = initializer.value() {
                                    if let AnyJsxAttributeValue::JsxExpressionAttributeValue(expr_attr) = value {
                                        if let Ok(ref_expr) = expr_attr.expression() {
                                            tracker.use_ref = true;
                                            
                                            // 生成 ref 处理逻辑
                                            // var _ref$ = props.ref;
                                            // typeof _ref$ === "function" ? _$use(_ref$, _el$) : props.ref = _el$;
                                            
                                            let ref_var_name = format!("_ref${}", unsafe { COUNTER });
                                            let ref_var_token = JsSyntaxToken::new_detached(T![ident], &ref_var_name, Vec::new(), Vec::new());
                                            
                                            let ref_declarator = js_variable_declarator(
                                                AnyJsBindingPattern::AnyJsBinding(
                                                    AnyJsBinding::JsIdentifierBinding(js_identifier_binding(ref_var_token.clone()))
                                                )
                                            ).with_initializer(
                                                js_initializer_clause(token(T![=]), ref_expr.clone()).into(),
                                            ).build();
                                            
                                            let ref_var_decl = js_variable_statement(
                                                js_variable_declaration(
                                                    token(T![var]),
                                                    js_variable_declarator_list(vec![ref_declarator], vec![]),
                                                ).build(),
                                            ).build();
                                            
                                            statements.push(ref_var_decl.into());
                                            
                                            // typeof _ref$ === "function" ? _$use(_ref$, _el$) : props.ref = _el$;
                                            // typeof _ref$ === "function"
                                            let typeof_expr = AnyJsExpression::JsUnaryExpression(
                                                js_unary_expression(
                                                    token(T![typeof]),
                                                    js_identifier_expression(js_reference_identifier(ref_var_token.clone())).into()
                                                )
                                            );
                                            let typeof_check = AnyJsExpression::JsBinaryExpression(
                                                js_binary_expression(
                                                    typeof_expr,
                                                    token(T![===]),
                                                    AnyJsExpression::AnyJsLiteralExpression(
                                                        AnyJsLiteralExpression::JsStringLiteralExpression(
                                                            js_string_literal_expression(js_string_literal("function"))
                                                        )
                                                    )
                                                )
                                            );
                                            // _$use(_ref$, _el$)
                                            let use_call = AnyJsExpression::JsCallExpression(
                                                js_call_expression(
                                                    js_identifier_expression(js_reference_identifier(
                                                        JsSyntaxToken::new_detached(T![ident], "_$use", Vec::new(), Vec::new())
                                                    )).into(),
                                                    js_call_arguments(
                                                        token(T!['(']),
                                                        js_call_argument_list(
                                                            vec![
                                                                AnyJsCallArgument::AnyJsExpression(
                                                                    js_identifier_expression(js_reference_identifier(ref_var_token)).into()
                                                                ),
                                                                AnyJsCallArgument::AnyJsExpression(el_var_ident.clone().into()),
                                                            ],
                                                            vec![token(T![,])],
                                                        ),
                                                        token(T![')']),
                                                    ),
                                                ).build()
                                            );
                                            // props.ref = _el$
                                            let assign = match ref_expr {
                                                AnyJsExpression::JsStaticMemberExpression(expr) => {
                                                    AnyJsAssignment::cast(expr.syntax().clone())
                                                        .or_else(|| biome_js_syntax::JsStaticMemberAssignment::cast(expr.syntax().clone()).map(AnyJsAssignment::JsStaticMemberAssignment))
                                                }
                                                AnyJsExpression::JsIdentifierExpression(expr) => {
                                                    AnyJsAssignment::cast(expr.syntax().clone())
                                                        .or_else(|| biome_js_syntax::JsIdentifierAssignment::cast(expr.syntax().clone()).map(AnyJsAssignment::JsIdentifierAssignment))
                                                }
                                                _ => {
                                                    // fallback: 构造一个 props_ref = _el$ 的赋值
                                                    let fallback_token = JsSyntaxToken::new_detached(T![ident], "props_ref", Vec::new(), Vec::new());
                                                    Some(AnyJsAssignment::JsIdentifierAssignment(
                                                        biome_js_factory::make::js_identifier_assignment(
                                                            fallback_token
                                                        )
                                                    ))
                                                }
                                            };
                                            if let Some(assign) = assign {
                                            let assignment = AnyJsExpression::JsAssignmentExpression(
                                                js_assignment_expression(
                                                    AnyJsAssignmentPattern::AnyJsAssignment(assign),
                                                    token(T![=]),
                                                    el_var_ident.clone().into()
                                                )
                                            );
                                            // typeof_check ? use_call : assignment
                                            let conditional = AnyJsExpression::JsConditionalExpression(
                                                js_conditional_expression(
                                                    typeof_check,
                                                    token(T![?]),
                                                    use_call,
                                                    token(T![:]),
                                                    assignment
                                                )
                                            );
                                            let conditional_stmt = js_expression_statement(
                                                conditional
                                            ).with_semicolon_token(token(T![;])).build();
                                            statements.push(conditional_stmt.into());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // 2. 处理子元素
    let children = jsx_element.children();
    for child in children {
        match child {
            AnyJsxChild::JsxText(text_node) => {
                let text_token = text_node.value_token().ok()?;
                let text_content = text_token.text_trimmed();
                if !text_content.trim().is_empty() {
                    tracker.insert_node = true;
                    tracker.create_text_node = true;
                    let stmt = create_text_insert_statement(&el_var_name, text_content)?;
                    statements.push(stmt);
                }
            }
            AnyJsxChild::JsxExpressionChild(expr_child) => {
                if let Some(expr) = expr_child.expression() {
                    tracker.insert = true;
                    let stmt = create_expression_insert_statement(&el_var_name, expr)?;
                    statements.push(stmt);
                }
            }
            AnyJsxChild::JsxElement(child_element) => {
                if let Some(child_expr) = create_solidjs_call_with_tracker(&child_element, tracker) {
                    tracker.insert_node = true;
                    let stmt = create_child_insert_statement(&el_var_name, child_expr)?;
                    statements.push(stmt);
                }
            }
            _ => {}
        }
    }

    // 3. return _el$;
    let return_stmt = js_return_statement(token(T![return]))
        .with_argument(el_var_ident.into())
        .with_semicolon_token(token(T![;]))
        .build();
    statements.push(return_stmt.into());

    // 创建 IIFE
    let function_body = js_function_body(
        token(T!['{']),
        js_directive_list(vec![]),
        js_statement_list(statements),
        token(T!['}']),
    );

    let arrow_fn = js_arrow_function_expression(
        js_parameters(
            token(T!['(']),
            js_parameter_list(vec![], vec![]),
            token(T![')']),
        ).into(),
        token(T![=>]),
        function_body.into(),
    ).build();

    let iife = js_call_expression(
        js_parenthesized_expression(
            token(T!['(']),
            AnyJsExpression::JsArrowFunctionExpression(arrow_fn),
            token(T![')']),
        ).into(),
        js_call_arguments(
            token(T!['(']),
            js_call_argument_list(vec![], vec![]),
            token(T![')']),
        ),
    ).build();

    Some(AnyJsExpression::JsCallExpression(iife))
}