use crate::{SaplingTransformer, jsx_element_name_to_string};
use biome_js_syntax::{
    AnyJsxTag, JsxElement, JsxFragment, JsxSelfClosingElement, JsxTagExpression,
};
use biome_rowan::AstNode;

impl SaplingTransformer {
    // main entry
    pub fn transform_jsx_tag_expression(
        &mut self,
        node: &JsxTagExpression,
    ) -> Option<JsxTagExpression> {
        let tag = node.tag().ok()?;
        match tag {
            AnyJsxTag::JsxElement(node) => self.transform_jsx_element(&node),
            AnyJsxTag::JsxFragment(node) => self.transform_jsx_fragment(&node),
            AnyJsxTag::JsxSelfClosingElement(node) => {
                self.transform_jsx_self_closing_element(&node)
            }
        }
    }
    pub fn transform_jsx_element(&mut self, node: &JsxElement) -> Option<JsxTagExpression> {
        let tag_name = jsx_element_name_to_string(&node.opening_element().ok()?.name().ok()?)?;

        let scope = self.semantic_model.scope(node.syntax());
        let id = self.generate_unique_identifier(&scope, "_el$");
        let js_tag_statement = self.create_js_tag_statement(id.as_str(), tag_name.as_str());
        self.transform_result.statments.push(js_tag_statement);

        let attributes = node.opening_element().ok()?.attributes();
        attributes.into_iter().for_each(|attribute| {
            let set_prop_statement = self.create_set_prop_statement(id.as_str(), attribute);
            match set_prop_statement {
                Some(set_prop_statement) => {
                    self.transform_result.statments.push(set_prop_statement);
                }
                None => {
                    return;
                }
            }
        });

        node.children().into_iter().for_each(|node| {
            self.transform_any_jsx_child(&node);
        });
        // jsx_element(opening_element, jsx_child_list(items), closing_element)
        None
    }
    pub fn transform_jsx_fragment(&self, node: &JsxFragment) -> Option<JsxTagExpression> {
        todo!()
    }
    pub fn transform_jsx_self_closing_element(
        &self,
        node: &JsxSelfClosingElement,
    ) -> Option<JsxTagExpression> {
        todo!()
    }
}
