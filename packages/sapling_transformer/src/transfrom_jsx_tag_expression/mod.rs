use crate::{
    SaplingTransformer, TransformJsxElementToStatementsOptions, jsx_element_name_to_string,
};
use biome_js_factory::make::{js_directive_list, js_statement_list};
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsxTag, JsxElement, JsxFragment, JsxSelfClosingElement,
    JsxTagExpression,
};
use sapling_transformation::helpers::jsx_template::{make_array, make_iife};

impl SaplingTransformer {
    // main entry
    pub fn transform_jsx_tag_expression(
        &mut self,
        node: &JsxTagExpression,
    ) -> Option<AnyJsExpression> {
        let tag = node.tag().ok()?;
        match tag {
            AnyJsxTag::JsxElement(node) => self.transform_jsx_element(&node),
            AnyJsxTag::JsxFragment(node) => self.transform_jsx_fragment(&node),
            AnyJsxTag::JsxSelfClosingElement(node) => {
                self.transform_jsx_self_closing_element(&node)
            }
        }
    }
    pub fn transform_jsx_element(&mut self, node: &JsxElement) -> Option<AnyJsExpression> {
        let Some((statements, _)) = self.transform_jsx_element_to_statements(
            node,
            TransformJsxElementToStatementsOptions { need_return: true },
        ) else {
            return None;
        };
        let iife = make_iife(vec![], statements);

        Some(AnyJsExpression::JsCallExpression(iife))
    }
    pub fn transform_jsx_fragment(&mut self, node: &JsxFragment) -> Option<AnyJsExpression> {
        let mut elements = vec![];
        node.children().into_iter().for_each(|node| {
            let Some(expression) = self.transform_any_jsx_child(&node) else {
                return;
            };
            elements.push(AnyJsArrayElement::AnyJsExpression(expression));
        });
        match elements.len() {
            1 => {
                if let AnyJsArrayElement::AnyJsExpression(expr) =
                    elements.into_iter().next().unwrap()
                {
                    Some(expr)
                } else {
                    None
                }
            }
            _ => Some(AnyJsExpression::JsArrayExpression(make_array(elements))),
        }
    }
    pub fn transform_jsx_self_closing_element(
        &self,
        node: &JsxSelfClosingElement,
    ) -> Option<AnyJsExpression> {
        None
    }
}
