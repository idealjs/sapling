use crate::{
    CreateTemplate, DomTemplate, SsrTemplate, TemplateInput, UniversalTemplate,
    convert_component_identifier, get_name_from_any_js_expression, get_tag_name, is_component,
    is_valid_html_nesting::is_valid_html_nesting, jsx_element_name_to_string,
};
use biome_js_factory::make::{
    ident, js_arrow_function_expression, js_call_arguments, js_call_expression, js_decorator_list,
    js_directive_list, js_formal_parameter, js_identifier_binding, js_identifier_expression,
    js_initializer_clause, js_parameter_list, js_parameters, js_reference_identifier,
    js_return_statement, js_statement_list, js_string_literal, js_string_literal_expression,
    js_variable_declaration, js_variable_declarator, js_variable_declarator_list,
    js_variable_statement, token,
};
use biome_js_semantic::{Scope, SemanticModel};
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument,
    AnyJsDeclaration, AnyJsDecorator, AnyJsExpression, AnyJsFormalParameter, AnyJsFunctionBody,
    AnyJsLiteralExpression, AnyJsParameter, AnyJsStatement, AnyJsxChild, AnyJsxTag,
    JsArrayExpression, JsArrowFunctionExpression, JsAssignmentExpression, JsAwaitExpression,
    JsBinaryExpression, JsBogusExpression, JsCallExpression, JsClassExpression,
    JsComputedMemberExpression, JsConditionalExpression, JsFunctionExpression, JsIdentifierBinding,
    JsIdentifierExpression, JsImportCallExpression, JsImportMetaExpression, JsInExpression,
    JsInstanceofExpression, JsLanguage, JsLogicalExpression, JsMetavariable, JsModule,
    JsNewExpression, JsNewTargetExpression, JsObjectExpression, JsParenthesizedExpression,
    JsPostUpdateExpression, JsPreUpdateExpression, JsReferenceIdentifier, JsSequenceExpression,
    JsStaticMemberExpression, JsSuperExpression, JsSyntaxKind, JsTemplateExpression,
    JsThisExpression, JsUnaryExpression, JsVariableDeclaration, JsYieldExpression, JsxElement,
    JsxExpressionChild, JsxFragment, JsxSelfClosingElement, JsxSpreadChild, JsxTagExpression,
    JsxText, T, TsAsExpression, TsInstantiationExpression, TsNonNullAssertionExpression,
    TsSatisfiesExpression, TsTypeAssertionExpression,
};
use biome_rowan::{
    AstNode, BatchMutation, BatchMutationExt, SyntaxNode, SyntaxNodeCast, SyntaxNodeChildren,
    SyntaxNodeOptionExt, TriviaPiece, TriviaPieceKind,
};

use biome_js_syntax::TextRange;
use sapling_transformation::helpers::jsx_template::{
    make_js_arrow_function_expression, make_js_call_arguments, make_js_function_body,
    make_js_parameters, make_js_return_statement,
};
use std::{
    collections::{HashMap, HashSet},
    default,
};

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
    pub traverse_result: TraverseResult,
}

// impl Default for SaplingTransformer {
//     fn default() -> Self {
//         Self {
//             mutation: Default::default(),
//             js_module: Default::default(),
//             pre_process_errors: Default::default(),
//             semantic_model: Default::default(),
//             scope_generated_identifiers: Default::default(),
//             config: Default::default(),
//             traverse_result: Default::default(),
//         }
//     }
// }

pub struct TransformNodePathInfo {
    pub top_level: bool,
    pub last_element: bool,
    pub do_not_escape: bool,
    pub skip_id: bool,
    pub component_child: bool,
    pub fragment_child: bool,
}

#[derive(Debug, Clone, Default)]

pub struct TraverseResult {
    pub statments: Vec<AnyJsStatement>,
}

impl SaplingTransformer {
    pub fn transform(&mut self) {
        self.mutation = self.js_module.clone().begin();
        self.pre_process();

        let syntax_node = self.js_module.syntax();

        self.traverse_syntax_node(syntax_node.clone());

        self.post_process();
    }

    pub fn traverse_syntax_node(&mut self, syntax_node: SyntaxNode<JsLanguage>) -> Option<()> {
        // let node = if let Ok(node) = JsxTagExpression::try_cast(syntax_node) {
        //     node
        // } else {
        //     syntax_node.children().for_each(|syntax_node| {
        //         self.traverse_syntax_node(syntax_node);
        //     });
        //     return;
        // };

        if matches!(syntax_node.kind(), JsSyntaxKind::JSX_TAG_EXPRESSION) {
            let node = syntax_node.cast::<JsxTagExpression>()?;
            self.transform_jsx_tag_expression(&node);
            None
        } else {
            syntax_node.children().for_each(|syntax_node| {
                self.traverse_syntax_node(syntax_node);
            });
            None
        }
    }

    pub fn transform_jsx_tag_expression(&mut self, node: &JsxTagExpression) -> Option<()> {
        let tag = node.tag().ok()?;
        match tag {
            AnyJsxTag::JsxElement(node) => {}
            AnyJsxTag::JsxFragment(node) => {}
            AnyJsxTag::JsxSelfClosingElement(node) => {}
        }
        // let children = node.syntax().children();
        None
    }

    pub fn transform_jsx_element(&mut self, node: &JsxElement) -> Option<()> {
        let name = jsx_element_name_to_string(&node.opening_element().ok()?.name().ok()?)?;

        //  opening_element.name().ok()
        // let _el$ = _$createElement("div");

        node.children().into_iter().for_each(|node| {
            self.transform_any_jsx_child(&node);
        });
        None
    }

    pub fn transform_jsx_fragment(&mut self, node: &JsxFragment) -> Option<()> {
        None
    }
    pub fn transform_jsx_self_closing_element(
        &mut self,
        node: &JsxSelfClosingElement,
    ) -> Option<()> {
        None
    }
    pub fn transform_any_jsx_child(&mut self, node: &AnyJsxChild) -> Option<()> {
        match node {
            AnyJsxChild::JsMetavariable(node) => {
                self.transform_js_metavariable(node);
            }
            AnyJsxChild::JsxElement(node) => {
                self.transform_jsx_element(node);
            }
            AnyJsxChild::JsxExpressionChild(node) => {
                self.transform_jsx_expression_child(node);
            }
            AnyJsxChild::JsxFragment(node) => {
                self.transform_jsx_fragment(node);
            }
            AnyJsxChild::JsxSelfClosingElement(node) => {
                self.transform_jsx_self_closing_element(node);
            }
            AnyJsxChild::JsxSpreadChild(node) => {
                self.transform_jsx_spread_child(node);
            }
            AnyJsxChild::JsxText(node) => {
                self.transform_jsx_text(node);
            }
        }
        None
    }
    pub fn transform_js_metavariable(&mut self, node: &JsMetavariable) -> Option<()> {
        None
    }

    pub fn transform_jsx_expression_child(&mut self, node: &JsxExpressionChild) -> Option<()> {
        let node = node.expression()?;
        match node {
            AnyJsExpression::AnyJsLiteralExpression(node) => {
                self.transform_any_js_literal_expression(&node);
            }
            AnyJsExpression::JsArrayExpression(node) => {
                self.transform_js_array_expression(&node);
            }
            AnyJsExpression::JsArrowFunctionExpression(node) => {
                self.transform_js_arrow_function_expression(&node);
            }
            AnyJsExpression::JsAssignmentExpression(node) => {
                self.transform_js_assignment_expression(&node);
            }
            AnyJsExpression::JsAwaitExpression(node) => {
                self.transform_js_await_expression(&node);
            }
            AnyJsExpression::JsBinaryExpression(node) => {
                self.transform_js_binary_expression(&node);
            }
            AnyJsExpression::JsBogusExpression(node) => {
                self.transform_js_bogus_expression(&node);
            }
            AnyJsExpression::JsCallExpression(node) => {
                self.transform_js_call_expression(&node);
            }
            AnyJsExpression::JsClassExpression(node) => {
                self.transform_js_class_expression(&node);
            }
            AnyJsExpression::JsComputedMemberExpression(node) => {
                self.transform_js_computed_member_expression(&node);
            }
            AnyJsExpression::JsConditionalExpression(node) => {
                self.transform_js_conditional_expression(&node);
            }
            AnyJsExpression::JsFunctionExpression(node) => {
                self.transform_js_function_expression(&node);
            }
            AnyJsExpression::JsIdentifierExpression(node) => {
                self.transform_js_identifier_expression(&node);
            }
            AnyJsExpression::JsImportCallExpression(node) => {
                self.transform_js_import_call_expression(&node);
            }
            AnyJsExpression::JsImportMetaExpression(node) => {
                self.transform_js_import_meta_expression(&node);
            }
            AnyJsExpression::JsInExpression(node) => {
                self.transform_js_in_expression(&node);
            }
            AnyJsExpression::JsInstanceofExpression(node) => {
                self.transform_js_instanceof_expression(&node);
            }
            AnyJsExpression::JsLogicalExpression(node) => {
                self.transform_js_logical_expression(&node);
            }
            AnyJsExpression::JsMetavariable(node) => {
                self.transform_js_metavariable(&node);
            }
            AnyJsExpression::JsNewExpression(node) => {
                self.transform_js_new_expression(&node);
            }
            AnyJsExpression::JsNewTargetExpression(node) => {
                self.transform_js_new_target_expression(&node);
            }
            AnyJsExpression::JsObjectExpression(node) => {
                self.transform_js_object_expression(&node);
            }
            AnyJsExpression::JsParenthesizedExpression(node) => {
                self.transform_js_parenthesized_expression(&node);
            }
            AnyJsExpression::JsPostUpdateExpression(node) => {
                self.transform_js_post_update_expression(&node);
            }
            AnyJsExpression::JsPreUpdateExpression(node) => {
                self.transform_js_pre_update_expression(&node);
            }
            AnyJsExpression::JsSequenceExpression(node) => {
                self.transform_js_sequence_expression(&node);
            }
            AnyJsExpression::JsStaticMemberExpression(node) => {
                self.transform_js_static_member_expression(&node);
            }
            AnyJsExpression::JsSuperExpression(node) => {
                self.transform_js_super_expression(&node);
            }
            AnyJsExpression::JsTemplateExpression(node) => {
                self.transform_js_template_expression(&node);
            }
            AnyJsExpression::JsThisExpression(node) => {
                self.transform_js_this_expression(&node);
            }
            AnyJsExpression::JsUnaryExpression(node) => {
                self.transform_js_unary_expression(&node);
            }
            AnyJsExpression::JsYieldExpression(node) => {
                self.transform_js_yield_expression(&node);
            }
            AnyJsExpression::JsxTagExpression(node) => {
                self.transform_jsx_tag_expression(&node);
            }
            AnyJsExpression::TsAsExpression(node) => {
                self.transform_ts_as_expression(&node);
            }
            AnyJsExpression::TsInstantiationExpression(node) => {
                self.transform_ts_instantiation_expression(&node);
            }
            AnyJsExpression::TsNonNullAssertionExpression(node) => {
                self.transform_ts_non_null_assertion_expression(&node);
            }
            AnyJsExpression::TsSatisfiesExpression(node) => {
                self.transform_ts_satisfies_expression(&node);
            }
            AnyJsExpression::TsTypeAssertionExpression(node) => {
                self.transform_ts_type_assertion_expression(&node);
            }
        }
        None
    }
    pub fn transform_jsx_spread_child(&mut self, node: &JsxSpreadChild) -> Option<()> {
        None
    }
    pub fn transform_jsx_text(&mut self, node: &JsxText) -> Option<()> {
        None
    }
    pub fn transform_any_js_literal_expression(
        &mut self,
        node: &AnyJsLiteralExpression,
    ) -> Option<()> {
        None
    }
    pub fn transform_js_array_expression(&mut self, node: &JsArrayExpression) -> Option<()> {
        None
    }
    pub fn transform_js_arrow_function_expression(
        &mut self,
        node: &JsArrowFunctionExpression,
    ) -> Option<()> {
        None
    }
    pub fn transform_js_assignment_expression(
        &mut self,
        node: &JsAssignmentExpression,
    ) -> Option<()> {
        None
    }
    pub fn transform_js_await_expression(&mut self, node: &JsAwaitExpression) -> Option<()> {
        None
    }
    pub fn transform_js_binary_expression(&mut self, node: &JsBinaryExpression) -> Option<()> {
        None
    }
    pub fn transform_js_bogus_expression(&mut self, node: &JsBogusExpression) -> Option<()> {
        None
    }
    pub fn transform_js_call_expression(&mut self, node: &JsCallExpression) -> Option<()> {
        None
    }
    pub fn transform_js_class_expression(&mut self, node: &JsClassExpression) -> Option<()> {
        None
    }
    pub fn transform_js_computed_member_expression(
        &mut self,
        node: &JsComputedMemberExpression,
    ) -> Option<()> {
        None
    }
    pub fn transform_js_conditional_expression(
        &mut self,
        node: &JsConditionalExpression,
    ) -> Option<()> {
        None
    }
    pub fn transform_js_function_expression(&mut self, node: &JsFunctionExpression) -> Option<()> {
        None
    }
    pub fn transform_js_identifier_expression(
        &mut self,
        node: &JsIdentifierExpression,
    ) -> Option<()> {
        None
    }
    pub fn transform_js_import_call_expression(
        &mut self,
        node: &JsImportCallExpression,
    ) -> Option<()> {
        None
    }
    pub fn transform_js_import_meta_expression(
        &mut self,
        node: &JsImportMetaExpression,
    ) -> Option<()> {
        None
    }
    pub fn transform_js_in_expression(&mut self, node: &JsInExpression) -> Option<()> {
        None
    }
    pub fn transform_js_instanceof_expression(
        &mut self,
        node: &JsInstanceofExpression,
    ) -> Option<()> {
        None
    }
    pub fn transform_js_logical_expression(&mut self, node: &JsLogicalExpression) -> Option<()> {
        None
    }
    pub fn transform_js_new_expression(&mut self, node: &JsNewExpression) -> Option<()> {
        None
    }
    pub fn transform_js_new_target_expression(
        &mut self,
        node: &JsNewTargetExpression,
    ) -> Option<()> {
        None
    }
    pub fn transform_js_object_expression(&mut self, node: &JsObjectExpression) -> Option<()> {
        None
    }
    pub fn transform_js_parenthesized_expression(
        &mut self,
        node: &JsParenthesizedExpression,
    ) -> Option<()> {
        None
    }
    pub fn transform_js_post_update_expression(
        &mut self,
        node: &JsPostUpdateExpression,
    ) -> Option<()> {
        None
    }
    pub fn transform_js_pre_update_expression(
        &mut self,
        node: &JsPreUpdateExpression,
    ) -> Option<()> {
        None
    }
    pub fn transform_js_sequence_expression(&mut self, node: &JsSequenceExpression) -> Option<()> {
        None
    }
    pub fn transform_js_static_member_expression(
        &mut self,
        node: &JsStaticMemberExpression,
    ) -> Option<()> {
        None
    }
    pub fn transform_js_super_expression(&mut self, node: &JsSuperExpression) -> Option<()> {
        None
    }
    pub fn transform_js_template_expression(&mut self, node: &JsTemplateExpression) -> Option<()> {
        None
    }
    pub fn transform_js_this_expression(&mut self, node: &JsThisExpression) -> Option<()> {
        None
    }
    pub fn transform_js_unary_expression(&mut self, node: &JsUnaryExpression) -> Option<()> {
        None
    }
    pub fn transform_js_yield_expression(&mut self, node: &JsYieldExpression) -> Option<()> {
        None
    }
    pub fn transform_ts_as_expression(&mut self, node: &TsAsExpression) -> Option<()> {
        None
    }
    pub fn transform_ts_instantiation_expression(
        &mut self,
        node: &TsInstantiationExpression,
    ) -> Option<()> {
        None
    }
    pub fn transform_ts_non_null_assertion_expression(
        &mut self,
        node: &TsNonNullAssertionExpression,
    ) -> Option<()> {
        None
    }
    pub fn transform_ts_satisfies_expression(
        &mut self,
        node: &TsSatisfiesExpression,
    ) -> Option<()> {
        None
    }
    pub fn transform_ts_type_assertion_expression(
        &mut self,
        node: &TsTypeAssertionExpression,
    ) -> Option<()> {
        None
    }
    pub fn create_js_tag_statement(&mut self, scope: &Scope, tag_name: &str) -> AnyJsStatement {
        let id = self.generate_unique_identifier(scope, "_el$");
        // 构造 let _el$ = _$createElement("div");
        let callee = js_identifier_expression(js_reference_identifier(ident("_$createElement")));
        let arg = AnyJsCallArgument::AnyJsExpression(AnyJsExpression::AnyJsLiteralExpression(
            js_string_literal_expression(js_string_literal(tag_name)).into(),
        ));
        let call_expr =
            js_call_expression(callee.into(), make_js_call_arguments(vec![arg], vec![])).build();

        let binding = js_identifier_binding(ident(id.as_str()));
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

        // tag_id = convertComponentIdentifier(path.node.openingElement.name)
        let tag_id = {
            // 获取 openingElement.name
            let opening_element: biome_js_syntax::AnyJsxElementName = node_path
                .clone()
                .cast::<JsxElement>()?
                .opening_element()
                .ok()?
                .name()
                .ok()?;
            convert_component_identifier(&opening_element)?
        };

        // 返回结构体，确保与 JS 逻辑一致
        Some(TemplateInput {
            exprs,
            tag_name: Some(tag_name),
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

    pub fn register_import_method(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
        name: &str,
        module_namee: &str,
    ) -> JsIdentifierBinding {
        todo!()
    }
}

pub struct CheckDynamicOptions {
    pub check_member: bool,
    pub check_tags: bool,
    pub check_call_expressions: bool,
    pub native: bool,
}

// 注册 import 方法，返回新的 tag_id（仅声明，具体实现可后续完善）
pub struct TagId {
    pub name: String,
    // 可扩展其他字段
}
