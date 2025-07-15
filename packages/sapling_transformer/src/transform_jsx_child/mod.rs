use biome_js_syntax::{
    AnyJsxChild, JsMetavariable, JsxElement, JsxExpressionChild, JsxFragment,
    JsxSelfClosingElement, JsxSpreadChild, JsxText,
};

use crate::SaplingTransformer;

impl SaplingTransformer {
    // main entry
    pub fn transform_any_jsx_child(&mut self, node: &AnyJsxChild) -> Option<AnyJsxChild> {
        match node {
            AnyJsxChild::JsMetavariable(node) => self.transform_js_metavariable_to_jsx_child(node),
            AnyJsxChild::JsxElement(node) => self.transform_jsx_element_to_jsx_child(node),
            AnyJsxChild::JsxExpressionChild(node) => self.transform_jsx_expression_child(node),
            AnyJsxChild::JsxFragment(node) => self.transform_jsx_fragment_to_jsx_child(node),
            AnyJsxChild::JsxSelfClosingElement(node) => {
                self.transform_jsx_self_closing_element_to_jsx_child(node)
            }
            AnyJsxChild::JsxSpreadChild(node) => self.transform_jsx_spread_child(node),
            AnyJsxChild::JsxText(node) => self.transform_jsx_text(node),
        }
    }
    pub fn transform_js_metavariable_to_jsx_child(
        &self,
        node: &JsMetavariable,
    ) -> Option<AnyJsxChild> {
        todo!()
    }
    pub fn transform_jsx_element_to_jsx_child(&self, node: &JsxElement) -> Option<AnyJsxChild> {
        todo!()
    }
    pub fn transform_jsx_expression_child(&self, node: &JsxExpressionChild) -> Option<AnyJsxChild> {
        todo!()
    }
    pub fn transform_jsx_fragment_to_jsx_child(&self, node: &JsxFragment) -> Option<AnyJsxChild> {
        todo!()
    }
    pub fn transform_jsx_self_closing_element_to_jsx_child(
        &self,
        node: &JsxSelfClosingElement,
    ) -> Option<AnyJsxChild> {
        todo!()
    }
    pub fn transform_jsx_spread_child(&self, node: &JsxSpreadChild) -> Option<AnyJsxChild> {
        todo!()
    }
    pub fn transform_jsx_text(&self, node: &JsxText) -> Option<AnyJsxChild> {
        todo!()
    }
}
