use crate::{
    CreateTemplate, DomTemplate, SsrTemplate, TemplateInput, UniversalTemplate, get_tag_name,
    is_component, is_valid_html_nesting::is_valid_html_nesting, jsx_element_name_to_string,
};
use biome_js_factory::make::{
    js_arrow_function_expression, js_call_arguments, js_call_expression, js_decorator_list,
    js_directive_list, js_formal_parameter, js_parameter_list, js_parameters, js_return_statement,
    js_statement_list, js_variable_statement, token,
};
use biome_js_semantic::{Scope, SemanticModel};
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsDeclaration, AnyJsDecorator, AnyJsExpression,
    AnyJsFormalParameter, AnyJsFunctionBody, AnyJsParameter, AnyJsStatement, AnyJsxTag,
    JsArrowFunctionExpression, JsLanguage, JsModule, JsSyntaxKind, JsVariableDeclaration,
    JsxElement, JsxExpressionChild, JsxSpreadChild, T,
};
use biome_rowan::{
    AstNode, BatchMutation, BatchMutationExt, SyntaxNode, SyntaxNodeCast, SyntaxNodeChildren,
    SyntaxNodeOptionExt, TriviaPieceKind,
};

use biome_js_syntax::TextRange;
use sapling_transformation::helpers::jsx_template::{
    make_js_arrow_function_expression, make_js_call_arguments, make_js_function_body,
    make_js_parameters, make_js_return_statement,
};
use std::collections::{HashMap, HashSet};

impl SaplingTransformer {
    /// 基于 biome scope 和 scope_generated_identifiers，生成唯一 identifier 名称
    pub fn generate_unique_identifier(&mut self, scope: &Scope, base: &str) -> String {
        let mut name = base.to_string();
        let mut counter = 0;
        let range = scope.range();
        // 获取当前作用域下已生成的 identifiers
        let used = self
            .scope_generated_identifiers
            .entry(range)
            .or_insert_with(HashSet::new);
        while scope.get_binding(&name).is_some() || used.contains(&name) {
            counter += 1;
            name = format!("{}{}", base, counter);
        }
        used.insert(name.clone());
        name
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub module_name: String,
    pub generate: String,
    pub hydratable: bool,
    pub delegate_events: bool,
    pub delegated_events: Vec<String>,
    pub built_ins: Vec<String>,
    pub require_import_source: bool,
    pub wrap_conditionals: bool,
    pub omit_nested_closing_tags: bool,
    pub omit_last_closing_tag: bool,
    pub omit_quotes: bool,
    pub context_to_custom_elements: bool,
    pub static_marker: String,
    pub effect_wrapper: String,
    pub memo_wrapper: String,
    pub validate: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            module_name: "dom".to_string(),
            generate: "dom".to_string(),
            hydratable: false,
            delegate_events: true,
            delegated_events: vec![],
            built_ins: vec![],
            require_import_source: false,
            wrap_conditionals: true,
            omit_nested_closing_tags: false,
            omit_last_closing_tag: true,
            omit_quotes: true,
            context_to_custom_elements: false,
            static_marker: "@once".to_string(),
            effect_wrapper: "effect".to_string(),
            memo_wrapper: "memo".to_string(),
            validate: true,
        }
    }
}

pub struct SaplingTransformer {
    pub mutation: BatchMutation<JsLanguage>,
    pub js_module: JsModule,
    pub pre_process_errors: Vec<String>,
    pub semantic_model: SemanticModel,
    pub scope_generated_identifiers: HashMap<TextRange, HashSet<String>>,
    pub config: Config,
}

pub struct TransformNodePathInfo {
    pub top_level: bool,
    pub last_element: bool,
    pub do_not_escape: bool,
    pub skip_id: bool,
    pub component_child: bool,
    pub fragment_child: bool,
}

impl SaplingTransformer {
    pub fn transform(&mut self) {
        self.mutation = self.js_module.clone().begin();
        self.pre_process();
        let descendants = self.js_module.syntax().descendants();

        descendants
            .filter(|node| matches!(node.kind(), JsSyntaxKind::JSX_TAG_EXPRESSION))
            .filter(|node| {
                let next = !node
                    .parent()
                    .map_or(false, |p| AnyJsxTag::can_cast(p.kind()));
                println!(
                    "Node: {:?}, Parent: {:?}, Next: {}",
                    node.kind(),
                    node.parent().map(|p| p.kind()),
                    next
                );
                next
            })
            .for_each(|node| self.transform_jsx(node));

        self.post_process();
    }
}

impl SaplingTransformer {
    pub fn pre_process(&mut self) {
        let descendants = self.js_module.syntax().descendants();
        descendants.for_each(|node| match node.kind() {
            JsSyntaxKind::JSX_ELEMENT => {
                let Some(node) = node.cast::<JsxElement>() else {
                    return;
                };
                let Some(parent) = node
                    .syntax()
                    .parent()
                    .and_then(|parent| parent.cast::<JsxElement>())
                else {
                    return;
                };

                let (Ok(el_tag), Ok(parent_tag)) = (
                    node.opening_element()
                        .and_then(|v| v.name())
                        .and_then(|v| v.name_value_token())
                        .and_then(|v| Ok(v.text().to_string())),
                    parent
                        .opening_element()
                        .and_then(|v| v.name())
                        .and_then(|v| v.name_value_token())
                        .and_then(|v| Ok(v.text().to_string())),
                ) else {
                    return;
                };

                let (el_tag, parent_tag) = (el_tag.as_str(), parent_tag.as_str());

                if is_component(el_tag) || is_component(parent_tag) {
                    if is_valid_html_nesting(parent_tag, el_tag) {
                        self.pre_process_errors.push(format!(
                            "Invalid JSX: <{}> cannot be child of <{}>",
                            el_tag, parent_tag
                        ));
                    }
                }
            }
            _ => {}
        });
    }

    pub fn post_process(&mut self) {}

    pub fn transform_jsx(&mut self, node_path: SyntaxNode<JsLanguage>) {
        // AnyJsxTag
        let info = if node_path.kind() == JsSyntaxKind::JSX_FRAGMENT {
            TransformNodePathInfo {
                top_level: false,
                last_element: false,
                do_not_escape: false,
                skip_id: false,
                component_child: false,
                fragment_child: false,
            }
        } else {
            TransformNodePathInfo {
                top_level: true,
                last_element: true,
                do_not_escape: false,
                skip_id: false,
                component_child: false,
                fragment_child: false,
            }
        };
        let update_template = self.get_update_template(&node_path);
        let Some(result) = self.transform_node_path(&node_path, info) else {
            return;
        };
        let mut create_template = self.get_create_template(&result);
        let template = create_template.create_template(&result, Some(false));
        let new_node = update_template(template);

        println!("Parent Kind: {:?}", node_path.parent().kind());
        println!("Node Kind: {:?}", node_path.kind());
        println!("Node Kind: {:?}", node_path);
        println!("New node: {:?}", new_node);
        self.mutation
            .replace_element(node_path.into(), new_node.into());
    }

    pub fn get_update_template(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
    ) -> impl Fn(AnyJsExpression) -> AnyJsExpression + 'static {
        let value: bool;

        let node_path_ref = node_path.clone();
        let update_template = |node: AnyJsExpression| -> AnyJsExpression { node };
        update_template
    }

    pub fn get_create_template(&self, result: &TemplateInput) -> Box<dyn CreateTemplate> {
        if result.tag_name.is_some() && result.renderer.as_deref() == Some("dom") {
            return Box::new(DomTemplate {});
        }

        if result.renderer.as_deref() == Some("ssr") {
            return Box::new(SsrTemplate {});
        }
        Box::new(UniversalTemplate {})
    }

    pub fn transform_node_path(
        &mut self,
        node_path: &SyntaxNode<JsLanguage>,
        info: TransformNodePathInfo,
    ) -> Option<TemplateInput> {
        let result: Option<TemplateInput> = match node_path.kind() {
            JsSyntaxKind::JSX_ELEMENT => self.transform_element(node_path, &info),
            JsSyntaxKind::JSX_FRAGMENT => {
                let mut result = TemplateInput {
                    id: None,
                    declarations: vec![],
                    exprs: vec![],
                    dynamics: vec![],
                    post_exprs: vec![],
                    tag_name: None,
                    template: None,
                    dynamic: false,
                    renderer: None,
                    text: false,
                };
                self.transform_fragment_children(node_path, &mut result);
                Some(result)
            }
            JsSyntaxKind::JSX_TEXT => {
                let mut result = TemplateInput {
                    declarations: vec![],
                    exprs: vec![],
                    dynamics: vec![],
                    post_exprs: vec![],
                    text: true,
                    id: None,
                    tag_name: None,
                    template: None,
                    dynamic: false,
                    renderer: None,
                };
                let scope = self.semantic_model.scope(node_path);
                let id = self.generate_unique_identifier(&scope, "el$");
                result.id = Some(id);
                Some(result)
            }
            JsSyntaxKind::JSX_EXPRESSION_CHILD => {
                let node = node_path.clone().cast::<JsxExpressionChild>()?;
                if node.expression().is_none() {
                    return None;
                }

                let exprs = node.expression()?;
                if self.is_dynamic(
                    exprs.syntax(),
                    CheckDynamicOptions {
                        check_member: true,
                        check_tags: info.component_child,
                        check_call_expressions: true,
                        native: !info.component_child,
                    },
                ) {
                    return Some(TemplateInput {
                        exprs: vec![exprs],
                        template: Some(String::new()),
                        ..Default::default()
                    });
                }

                // 优化后的表达式分支，提升可读性
                let is_logic_or_conditional = matches!(
                    exprs,
                    AnyJsExpression::JsLogicalExpression(_)
                        | AnyJsExpression::JsConditionalExpression(_)
                );

                let is_fragment_call = !info.component_child
                    && info.fragment_child
                    && matches!(exprs, AnyJsExpression::JsCallExpression(_))
                    && !matches!(
                        exprs.as_js_call_expression()?.callee().ok()?,
                        AnyJsExpression::JsCallExpression(_)
                    )
                    && !matches!(
                        exprs.as_js_call_expression()?.callee().ok()?,
                        AnyJsExpression::JsStaticMemberExpression(_)
                    );

                let expr = if is_logic_or_conditional {
                    self.transform_condition(
                        exprs.syntax(),
                        info.component_child || info.fragment_child,
                        false,
                    )
                } else if is_fragment_call {
                    (Some(exprs.as_js_call_expression()?.callee().ok()?), None)
                } else {
                    (
                        Some(
                            js_arrow_function_expression(
                                make_js_parameters(js_parameter_list(vec![], vec![])).into(),
                                token(T![=>])
                                    .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                                AnyJsFunctionBody::AnyJsExpression(exprs.into()),
                            )
                            .build()
                            .into(),
                        ),
                        None,
                    )
                };

                let exprs = match expr {
                    (Some(val), None) => Some(vec![val]),
                    (None, Some((declaration, expression))) => {
                        Some(vec![AnyJsExpression::JsCallExpression(
                            js_call_expression(
                                make_js_arrow_function_expression(
                                    make_js_parameters(js_parameter_list(vec![], vec![])),
                                    make_js_function_body(
                                        js_directive_list(vec![]),
                                        js_statement_list(vec![
                                            AnyJsStatement::JsVariableStatement(
                                                js_variable_statement(declaration).build(),
                                            ),
                                            AnyJsStatement::JsReturnStatement(
                                                make_js_return_statement(expression.into()),
                                            ),
                                        ]),
                                    ),
                                )
                                .into(),
                                make_js_call_arguments(vec![], vec![]),
                            )
                            .build(),
                        )])
                    }
                    _ => None,
                }?;

                Some(TemplateInput {
                    exprs,
                    dynamic: true,
                    template: Some("".to_string()),
                    ..Default::default()
                })
            }
            JsSyntaxKind::JSX_SPREAD_CHILD => {
                let node = node_path.clone().cast::<JsxSpreadChild>()?;
                let expr = node.expression().ok()?;
                if !self.is_dynamic(
                    expr.syntax(),
                    CheckDynamicOptions {
                        check_member: true,
                        check_tags: false,
                        check_call_expressions: false,
                        native: !info.component_child,
                    },
                ) {
                    return Some(TemplateInput {
                        exprs: vec![expr],
                        template: Some(String::new()),
                        ..Default::default()
                    });
                }
                let arrow = js_arrow_function_expression(
                    make_js_parameters(js_parameter_list(vec![], vec![])).into(),
                    token(T![=>]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                    AnyJsFunctionBody::AnyJsExpression(expr.into()),
                )
                .build()
                .into();
                Some(TemplateInput {
                    exprs: vec![arrow],
                    template: Some(String::new()),
                    dynamic: true,
                    ..Default::default()
                })
            }
            _ => {
                return None;
            }
        };

        result
    }

    pub fn transform_fragment_children(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
        result: &mut TemplateInput,
    ) {
    }

    pub fn transform_element(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
        info: &TransformNodePathInfo,
    ) -> Option<TemplateInput> {
        let tag_name = get_tag_name(node_path)?;

        if is_component(&tag_name) {
            return self.transform_component(node_path);
        }

        let generate = self.config.generate.as_str();
        if generate == "dom" {
            return self.transform_element_dom(node_path, info);
        }
        if generate == "ssr" {
            return self.transform_element_ssr(node_path, info);
        }

        self.transform_element_universal(node_path, info)
    }

    pub fn transform_component(&self, node_path: &SyntaxNode<JsLanguage>) -> Option<TemplateInput> {
        // 变量初始化
        let mut exprs = vec![];
        let config = &self.config;
        let tag_name = get_tag_name(node_path)?;
        // TODO: convertComponentIdentifier 逻辑
        // 基础遍历 openingElement.attributes
        // if let Some(opening_element) = node_path.children().find_map(|c| c.try_to::<JsxElement>()) {
        //     if let Some(attributes) = opening_element.opening_element().and_then(|oe| oe.attributes()) {
        //         for attr in attributes {
        //             // 这里只做类型识别，后续细化
        //             let kind = attr.kind();
        //             println!("Found attribute kind: {:?}", kind);
        //         }
        //     }
        // }
        // 基础 children 处理
        // 这里只做 children 节点遍历，后续细化
        for child in node_path.children() {
            println!("Found child kind: {:?}", child.kind());
        }
        // 基础 props 合并与 dynamicSpread 逻辑
        // 这里只做 props 向量和 dynamic_spread 标志初始化，后续细化
        // let mut props = vec![];
        // let mut dynamic_spread = false;
        // 基础组件表达式生成与条件提升
        // 这里只做 createComponent 调用表达式的占位，后续细化
        // 假设 create_component_expr 为最终表达式
        // exprs.push(create_component_expr);

        // TODO: 返回结构体

        // 返回结构体，确保与 JS 逻辑一致
        Some(TemplateInput {
            exprs,
            tag_name: Some(tag_name),
            // 其它字段后续细化
            ..Default::default()
        })
    }

    pub fn transform_element_dom(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
        info: &TransformNodePathInfo,
    ) -> Option<TemplateInput> {
        None
    }

    pub fn transform_element_ssr(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
        info: &TransformNodePathInfo,
    ) -> Option<TemplateInput> {
        None
    }

    pub fn transform_element_universal(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
        info: &TransformNodePathInfo,
    ) -> Option<TemplateInput> {
        None
    }

    pub fn transform_condition(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
        inline: bool,
        deep: bool,
    ) -> (
        Option<AnyJsExpression>,
        Option<(JsVariableDeclaration, JsArrowFunctionExpression)>,
    ) {
        todo!()
        // node_path.children().for_each(|node| {});
    }

    pub fn transform_component_children(&self, children: SyntaxNodeChildren<JsLanguage>) {}

    pub fn is_dynamic(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
        options: CheckDynamicOptions,
    ) -> bool {
        false
    }
}

pub struct CheckDynamicOptions {
    pub check_member: bool,
    pub check_tags: bool,
    pub check_call_expressions: bool,
    pub native: bool,
}
