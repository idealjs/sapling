use biome_js_syntax::{
    AnyJsExpression, AnyJsxChild, JsMetavariable, JsxElement, JsxExpressionChild, JsxFragment,
    JsxSelfClosingElement, JsxSpreadChild, JsxText,
};

use crate::SaplingTransformer;

impl SaplingTransformer {
    // main entry
    pub fn transform_any_jsx_child(&mut self, node: &AnyJsxChild) -> Option<AnyJsExpression> {
        match node {
            AnyJsxChild::JsMetavariable(node) => self.transform_js_metavariable(node),
            AnyJsxChild::JsxElement(node) => self.transform_jsx_element(node),
            AnyJsxChild::JsxExpressionChild(node) => self.transform_jsx_expression_child(node),
            AnyJsxChild::JsxFragment(node) => self.transform_jsx_fragment(node),
            AnyJsxChild::JsxSelfClosingElement(node) => {
                self.transform_jsx_self_closing_element(node)
            }
            AnyJsxChild::JsxSpreadChild(node) => self.transform_jsx_spread_child(node),
            AnyJsxChild::JsxText(node) => self.transform_jsx_text(node),
        }
    }
    pub fn transform_js_metavariable(&self, node: &JsMetavariable) -> Option<AnyJsExpression> {
        None
    }
    pub fn transform_jsx_expression_child(
        &self,
        node: &JsxExpressionChild,
    ) -> Option<AnyJsExpression> {
        None
    }
    pub fn transform_jsx_spread_child(&self, node: &JsxSpreadChild) -> Option<AnyJsExpression> {
        None
    }
    pub fn transform_jsx_text(&self, node: &JsxText) -> Option<AnyJsExpression> {
        None
    }
}
