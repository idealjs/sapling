use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule};
use biome_js_factory::make::{
    js_call_argument_list, js_call_arguments, js_call_expression, js_directive_list,
    js_expression_statement, js_function_body, js_identifier_binding, js_identifier_expression,
    js_initializer_clause, js_parameter_list, js_parameters, js_parenthesized_expression,
    js_reference_identifier, js_return_statement, js_statement_list, js_string_literal_expression,
    js_variable_declaration, js_variable_declarator,
    js_variable_declarator_list, js_variable_statement, token, js_string_literal,
};
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument, AnyJsExpression, AnyJsStatement,
    AnyJsxChild, JsArrowFunctionExpression, JsCallExpression, JsFunctionBody, JsParameters,
    JsReturnStatement, JsStatementList, JsxElement, JsxExpressionChild, JsxTagExpression, T,
};
use biome_rowan::{AstNode, SyntaxToken, BatchMutationExt};

use crate::{declare_transformation, JsBatchMutation};

declare_transformation! {
    /// Transform JSX elements to SolidJS-style runtime calls
    pub(crate) JsxTemplate {
        version: "0.1.0",
        name: "jsx_template",
        language: "js",
    }
}

impl Rule for JsxTemplate {
    type Query = Ast<JsxTagExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let _node = ctx.query();
        // 总是返回 Some(()) 来触发转换
        Some(())
    }

    fn transform(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsBatchMutation> {
        let node = ctx.query();
        
        // 获取 JSX 元素
       let tag = node.tag().ok()?;
       let jsx_element = tag.as_jsx_element()?;
       let transformed_expr = create_solidjs_call(&jsx_element)?;

       let mutation = {
           use crate::helpers::jsx_template::replace;
           let expr_clone = transformed_expr.clone();
           replace(
               node,
               |node| {
                   let parent = node.syntax().parent().expect("Expected parent node");
                   biome_js_syntax::JsReturnStatement::cast(parent).expect("Expected parent to be a JsReturnStatement")
               },
               |_parent| {
                   biome_js_factory::make::js_return_statement(token(T![return]))
                       .with_argument(expr_clone.clone())
                       .with_semicolon_token(token(T![;]))
                       .build()
               }
           )
       };
       Some(mutation)
    }
}

fn create_solidjs_call(jsx_element: &JsxElement) -> Option<AnyJsExpression> {
    // 获取元素名称
    let opening_element = jsx_element.opening_element().ok()?;
    let element_name = opening_element.name().ok()?;
    let jsx_name = element_name.as_jsx_name()?;
    let tag_token = jsx_name.value_token().ok()?;
    let tag_name = tag_token.text_trimmed();
    
    // 创建语句列表
    let mut statements = Vec::<AnyJsStatement>::new();
    
    // 1. var _el$ = _$createElement("tagName");
    let el_var_token = SyntaxToken::new_detached(T![ident], "_el$", Vec::new(), Vec::new());
    let create_element_token = SyntaxToken::new_detached(T![ident], "_$createElement", Vec::new(), Vec::new());
    
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
                                    js_string_literal_expression(js_string_literal(tag_name.as_ref()))
                                )
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
    
    // 2. 处理子元素
    let children = jsx_element.children();
    for child in children {
        match child {
            AnyJsxChild::JsxText(text_node) => {
                // 处理文本节点
                let text_token = text_node.value_token().ok()?;
                let text_content = text_token.text_trimmed();
                if !text_content.trim().is_empty() {
                    statements.push(create_insert_text_node(&text_content)?);
                }
            }
            AnyJsxChild::JsxExpressionChild(expr_child) => {
                // 处理表达式子元素
                statements.push(create_insert_expression_node(&expr_child)?);
            }
            _ => {
                // 暂时跳过其他类型的子元素
            }
        }
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
    
    let arrow_fn = biome_js_factory::make::js_arrow_function_expression(
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
        ).into(),
        js_call_arguments(
            token(T!['(']),
            js_call_argument_list(vec![], vec![]),
            token(T![')']),
        ),
    )
    .build();
    
    Some(AnyJsExpression::JsCallExpression(iife))
}

fn create_insert_text_node(text: &str) -> Option<AnyJsStatement> {
    let el_var_token = SyntaxToken::new_detached(T![ident], "_el$", Vec::new(), Vec::new());
    let insert_node_token = SyntaxToken::new_detached(T![ident], "_$insertNode", Vec::new(), Vec::new());
    let create_text_token = SyntaxToken::new_detached(T![ident], "_$createTextNode", Vec::new(), Vec::new());
    
    // _$insertNode(_el$, _$createTextNode(`text`))
    // _$insertNode(_el$, _$createTextNode(`text`))
    let create_text_call = js_call_expression(
        js_identifier_expression(js_reference_identifier(create_text_token)).into(),
        js_call_arguments(
            token(T!['(']),
            js_call_argument_list(
                vec![AnyJsCallArgument::AnyJsExpression(
                    AnyJsExpression::AnyJsLiteralExpression(
                        biome_js_syntax::AnyJsLiteralExpression::JsStringLiteralExpression(
                            js_string_literal_expression(js_string_literal(text.as_ref()))
                        )
                    ),
                )],
                vec![],
            ),
            token(T![')']),
        ),
    )
    .build();
    let insert_call = js_call_expression(
        js_identifier_expression(js_reference_identifier(insert_node_token)).into(),
        js_call_arguments(
            token(T!['(']),
            js_call_argument_list(
                vec![
                    AnyJsCallArgument::AnyJsExpression(
                        js_identifier_expression(js_reference_identifier(el_var_token)).into(),
                    ),
                    AnyJsCallArgument::AnyJsExpression(
                        AnyJsExpression::JsCallExpression(create_text_call),
                    ),
                ],
                vec![token(T![,])],
            ),
            token(T![')']),
        ),
    )
    .build();
    
    let stmt = js_expression_statement(AnyJsExpression::JsCallExpression(insert_call))
        .with_semicolon_token(token(T![;]))
        .build();
    
    Some(stmt.into())
}

fn create_insert_expression_node(expr_child: &JsxExpressionChild) -> Option<AnyJsStatement> {
    let el_var_token = SyntaxToken::new_detached(T![ident], "_el$", Vec::new(), Vec::new());
    let insert_node_token = SyntaxToken::new_detached(T![ident], "_$insertNode", Vec::new(), Vec::new());
    let create_text_token = SyntaxToken::new_detached(T![ident], "_$createTextNode", Vec::new(), Vec::new());
    
    // 获取表达式内容
    let expression = expr_child.expression()?;
    
    // 对于简单的标识符表达式，创建模板字符串
    if let AnyJsExpression::JsIdentifierExpression(ident_expr) = &expression {
        if let Ok(name_ref) = ident_expr.name() {
            if let Ok(name_token) = name_ref.value_token() {
                let var_name = name_token.text_trimmed();
                
                // _$insertNode(_el$, _$createTextNode(`${varName}`))
                let create_text_call = js_call_expression(
                    js_identifier_expression(js_reference_identifier(create_text_token)).into(),
                    js_call_arguments(
                        token(T!['(']),
                        js_call_argument_list(
                            vec![AnyJsCallArgument::AnyJsExpression(
                                AnyJsExpression::AnyJsLiteralExpression(
                                    biome_js_syntax::AnyJsLiteralExpression::JsStringLiteralExpression(
                                        js_string_literal_expression(js_string_literal(var_name.as_ref()))
                                    )
                                ),
                            )],
                            vec![],
                        ),
                        token(T![')']),
                    ),
                )
                .build();
                
                let insert_call = js_call_expression(
                    js_identifier_expression(js_reference_identifier(insert_node_token)).into(),
                    js_call_arguments(
                        token(T!['(']),
                        js_call_argument_list(
                            vec![
                                AnyJsCallArgument::AnyJsExpression(
                                    js_identifier_expression(js_reference_identifier(el_var_token)).into(),
                                ),
                                AnyJsCallArgument::AnyJsExpression(
                                    AnyJsExpression::JsCallExpression(create_text_call),
                                ),
                            ],
                            vec![token(T![,])],
                        ),
                        token(T![')']),
                    ),
                )
                .build();
                
                let stmt = js_expression_statement(AnyJsExpression::JsCallExpression(insert_call))
                    .with_semicolon_token(token(T![;]))
                    .build();
                
                return Some(stmt.into());
            }
        }
    }
    
    None
}
