use crate::jsx_element_name_to_string;
use biome_js_factory::make::{
    ident, js_call_expression, js_expression_statement, js_identifier_binding,
    js_identifier_expression, js_initializer_clause, js_reference_identifier, js_string_literal,
    js_string_literal_expression, js_variable_declaration, js_variable_declarator,
    js_variable_declarator_list, js_variable_statement, token,
};
use biome_js_semantic::{Scope, SemanticModel};
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression,
    AnyJsStatement, AnyJsxAttributeName, AnyJsxAttributeValue, AnyJsxChild, AnyJsxTag,
    JsArrayExpression, JsArrowFunctionExpression, JsAssignmentExpression, JsAwaitExpression,
    JsBinaryExpression, JsBogusExpression, JsCallExpression, JsClassExpression,
    JsComputedMemberExpression, JsConditionalExpression, JsFunctionExpression,
    JsIdentifierExpression, JsImportCallExpression, JsImportMetaExpression, JsInExpression,
    JsInstanceofExpression, JsLanguage, JsLogicalExpression, JsMetavariable, JsModule,
    JsNewExpression, JsNewTargetExpression, JsObjectExpression, JsParenthesizedExpression,
    JsPostUpdateExpression, JsPreUpdateExpression, JsSequenceExpression, JsStaticMemberExpression,
    JsSuperExpression, JsSyntaxKind, JsTemplateExpression, JsThisExpression, JsUnaryExpression,
    JsYieldExpression, JsxElement, JsxExpressionChild, JsxFragment, JsxSelfClosingElement,
    JsxSpreadChild, JsxTagExpression, JsxText, T, TsAsExpression, TsInstantiationExpression,
    TsNonNullAssertionExpression, TsSatisfiesExpression, TsTypeAssertionExpression,
};
use biome_js_syntax::{AnyJsxAttribute, TextRange};
use biome_rowan::{AstNode, SyntaxToken, TriviaPiece};
use biome_rowan::{BatchMutation, SyntaxNode, SyntaxNodeCast, TriviaPieceKind};
use sapling_transformation::helpers::jsx_template::{
    make_js_arrow_function_expression, make_js_call_arguments, make_js_function_body,
    make_js_parameters, make_js_return_statement,
};
use std::str::FromStr;
use std::{
    collections::{HashMap, HashSet},
    default,
};

impl SaplingTransformer {
    pub fn generate_unique_identifier(&mut self, scope: &Scope, base: &str) -> String {
        let mut name = base.to_string();
        let mut counter = 0;
        let range = scope.range();
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

#[derive(Debug, Clone, Default)]

pub struct TraverseResult {
    pub statments: Vec<AnyJsStatement>,
}

impl SaplingTransformer {
    pub fn transform(&mut self) {
        let syntax_node = self.js_module.syntax();

        self.traverse_syntax_node(syntax_node.clone());
    }

    pub fn traverse_syntax_node(&mut self, syntax_node: SyntaxNode<JsLanguage>) -> Option<()> {
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
            AnyJsxTag::JsxElement(node) => {
                self.transform_jsx_element(&node);
            }
            AnyJsxTag::JsxFragment(node) => {
                self.transform_jsx_fragment(&node);
            }
            AnyJsxTag::JsxSelfClosingElement(node) => {
                self.transform_jsx_self_closing_element(&node);
            }
        }
        None
    }

    pub fn transform_jsx_element(&mut self, node: &JsxElement) -> Option<()> {
        let tag_name = jsx_element_name_to_string(&node.opening_element().ok()?.name().ok()?)?;

        let scope = self.semantic_model.scope(node.syntax());
        let id = self.generate_unique_identifier(&scope, "_el$");
        let js_tag_statement = self.create_js_tag_statement(id.as_str(), tag_name.as_str());
        self.traverse_result.statments.push(js_tag_statement);

        let attributes = node.opening_element().ok()?.attributes();
        attributes.into_iter().for_each(|attribute| {
            let set_prop_statement = self.create_set_prop_statement(id.as_str(), attribute);
            // self.traverse_result.statments.push(set_prop_statement);
        });

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
}

impl SaplingTransformer {
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
