use crate::{SaplingTransformer, jsx_element_name_to_string};
use biome_js_factory::make::{
    ident, js_identifier_expression, js_reference_identifier, js_return_statement, token,
};
use biome_js_syntax::{
    AnyJsStatement, AnyJsxChild, AnyJsxTag, JsIdentifierExpression, JsMetavariable, JsxElement,
    JsxExpressionChild, JsxFragment, JsxSelfClosingElement, JsxSpreadChild, JsxTagExpression,
    JsxText, T,
};
use biome_rowan::AstNode;

impl SaplingTransformer {
    pub fn transform_jsx_tag_expression_to_statements(
        &mut self,
        node: &JsxTagExpression,
    ) -> Option<Vec<AnyJsStatement>> {
        let tag = node.tag().ok()?;
        match tag {
            AnyJsxTag::JsxElement(node) => self.transform_jsx_element_to_statements(&node),
            AnyJsxTag::JsxFragment(node) => self.transform_jsx_fragment_to_statements(&node),
            AnyJsxTag::JsxSelfClosingElement(node) => {
                self.transform_jsx_self_closing_element_to_statements(&node)
            }
        }
    }
    pub fn transform_jsx_element_to_statements(
        &mut self,
        node: &JsxElement,
    ) -> Option<Vec<AnyJsStatement>> {
        let mut statments: Vec<AnyJsStatement> = vec![];
        let tag_name = jsx_element_name_to_string(&node.opening_element().ok()?.name().ok()?)?;
        let scope = self.semantic_model.scope(node.syntax());
        let id = self.generate_unique_identifier(&scope, "_el$");
        let js_tag_statement = self.create_js_tag_statement(id.as_str(), tag_name.as_str());
        statments.push(js_tag_statement);

        // Handle children
        let children = node.children();
        children.into_iter().for_each(|node| {
            let Some(statements) = self.transform_any_jsx_child_to_statements(&node) else {
                return;
            };
            statments.extend(statements);
        });

        let return_stmt = AnyJsStatement::JsReturnStatement(
            js_return_statement(token(T![return]))
                .with_argument(js_identifier_expression(js_reference_identifier(ident(&id))).into())
                .with_semicolon_token(token(T![;]))
                .build(),
        );
        statments.push(return_stmt);

        Some(statments)
    }
    pub fn transform_jsx_fragment_to_statements(
        &self,
        node: &JsxFragment,
    ) -> Option<Vec<AnyJsStatement>> {
        None
    }
    pub fn transform_jsx_self_closing_element_to_statements(
        &self,
        node: &JsxSelfClosingElement,
    ) -> Option<Vec<AnyJsStatement>> {
        None
    }
    pub fn transform_any_jsx_child_to_statements(
        &mut self,
        node: &AnyJsxChild,
    ) -> Option<Vec<AnyJsStatement>> {
        match node {
            AnyJsxChild::JsMetavariable(node) => self.transform_js_metavariable_to_statements(node),
            AnyJsxChild::JsxElement(node) => self.transform_jsx_element_to_statements(node),
            AnyJsxChild::JsxExpressionChild(node) => {
                self.transform_jsx_expression_child_to_statements(node)
            }
            AnyJsxChild::JsxFragment(node) => self.transform_jsx_fragment_to_statements(node),
            AnyJsxChild::JsxSelfClosingElement(node) => {
                self.transform_jsx_self_closing_element_to_statements(node)
            }
            AnyJsxChild::JsxSpreadChild(node) => {
                self.transform_jsx_spread_child_to_statements(node)
            }
            AnyJsxChild::JsxText(node) => self.transform_jsx_text_to_statements(node),
        }
    }
    pub fn transform_js_metavariable_to_statements(
        &self,
        node: &JsMetavariable,
    ) -> Option<Vec<AnyJsStatement>> {
        None
    }
    pub fn transform_jsx_element_to_to_statements(
        &self,
        node: &JsxElement,
    ) -> Option<Vec<AnyJsStatement>> {
        None
    }
    pub fn transform_jsx_expression_child_to_statements(
        &self,
        node: &JsxExpressionChild,
    ) -> Option<Vec<AnyJsStatement>> {
        None
    }

    pub fn transform_jsx_spread_child_to_statements(
        &self,
        node: &JsxSpreadChild,
    ) -> Option<Vec<AnyJsStatement>> {
        None
    }
    pub fn transform_jsx_text_to_statements(&self, node: &JsxText) -> Option<Vec<AnyJsStatement>> {
        None
    }
}
