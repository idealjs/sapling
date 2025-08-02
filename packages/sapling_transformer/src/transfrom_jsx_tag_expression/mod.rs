use crate::{
    SaplingTransformer, TransformAnyJsxChildOptions, TransformJsxElementToStatementsOptions,
    jsx_element_name_to_string,
};
use biome_js_factory::make::{js_directive_list, js_statement_list};
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsxTag, JsxElement, JsxFragment, JsxSelfClosingElement,
    JsxTagExpression,
};
use sapling_transformation::helpers::jsx_template::{make_array, make_iife};

pub struct TransformAnyJsxFragmentOptions {
    pub parent_id: Option<String>,
}

pub struct TransformAnyJsxTagExpressionOptions {
    pub parent_id: Option<String>,
}

impl SaplingTransformer {
    // main entry
    pub fn transform_jsx_tag_expression(
        &mut self,
        node: &JsxTagExpression,
        transform_options: TransformAnyJsxTagExpressionOptions,
    ) -> Option<(Option<AnyJsExpression>, Option<String>)> {
        let tag = node.tag().ok()?;
        match tag {
            AnyJsxTag::JsxElement(node) => self.transform_jsx_element(&node),
            AnyJsxTag::JsxFragment(node) => self.transform_jsx_fragment(
                &node,
                TransformAnyJsxFragmentOptions {
                    parent_id: transform_options.parent_id,
                },
            ),
            AnyJsxTag::JsxSelfClosingElement(node) => {
                self.transform_jsx_self_closing_element(&node)
            }
        }
    }
    pub fn transform_jsx_element(
        &mut self,
        node: &JsxElement,
    ) -> Option<(Option<AnyJsExpression>, Option<String>)> {
        let (statements, _) = self.transform_jsx_element_to_statements(
            node,
            TransformJsxElementToStatementsOptions { need_return: true },
        )?;
        let iife = make_iife(vec![], statements);

        Some((Some(AnyJsExpression::JsCallExpression(iife)), None))
    }
    pub fn transform_jsx_fragment(
        &mut self,
        node: &JsxFragment,
        transform_options: TransformAnyJsxFragmentOptions,
    ) -> Option<(Option<AnyJsExpression>, Option<String>)> {
        let mut elements = vec![];
        node.children().into_iter().for_each(|node| {
            let Some((Some(expression), _)) = self.transform_any_jsx_child(
                &node,
                TransformAnyJsxChildOptions {
                    parent_id: transform_options.parent_id.clone(),
                },
            ) else {
                return;
            };
            elements.push(AnyJsArrayElement::AnyJsExpression(expression));
        });
        match elements.len() {
            1 => {
                if let AnyJsArrayElement::AnyJsExpression(expr) =
                    elements.into_iter().next().unwrap()
                {
                    Some((Some(expr), None))
                } else {
                    None
                }
            }
            _ => Some((
                Some(AnyJsExpression::JsArrayExpression(make_array(elements))),
                None,
            )),
        }
    }
    pub fn transform_jsx_self_closing_element(
        &self,
        node: &JsxSelfClosingElement,
    ) -> Option<(Option<AnyJsExpression>, Option<String>)> {
        todo!()
    }
}
