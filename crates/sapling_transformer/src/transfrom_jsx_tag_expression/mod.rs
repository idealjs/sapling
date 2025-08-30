use crate::{
    SaplingTransformer, TransformAnyJsxChildOptions, make_array, make_create_jsx_tag_element,
};

use biome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsxTag, JsxFragment, JsxTagExpression,
};

pub struct TransformAnyJsxFragmentOptions {
    pub parent_id: Option<String>,
}

pub struct TransformAnyJsxTagExpressionOptions {
    pub parent_id: Option<String>,
}

impl SaplingTransformer<'_> {
    // main entry
    pub fn transform_jsx_tag_expression(
        &mut self,
        node: &JsxTagExpression,
        transform_options: TransformAnyJsxTagExpressionOptions,
    ) -> Option<AnyJsExpression> {
        let tag = node.tag().ok()?;
        match tag {
            AnyJsxTag::JsxElement(node) => {
                let (statements, id) = self.transform_jsx_element(&node)?;
                let call = make_create_jsx_tag_element(&vec![], &statements, id);

                Some(AnyJsExpression::JsCallExpression(call))
            }
            AnyJsxTag::JsxFragment(node) => self.transform_jsx_fragment(
                &node,
                TransformAnyJsxFragmentOptions {
                    parent_id: transform_options.parent_id,
                },
            ),
            AnyJsxTag::JsxSelfClosingElement(node) => {
                let (statements, id) =
                    self.transform_jsx_self_closing_element_to_statements(&node)?;
                let call = make_create_jsx_tag_element(&vec![], &statements, id);

                Some(AnyJsExpression::JsCallExpression(call))
            }
        }
    }

    pub fn transform_jsx_fragment(
        &mut self,
        node: &JsxFragment,
        transform_options: TransformAnyJsxFragmentOptions,
    ) -> Option<AnyJsExpression> {
        let mut elements = vec![];
        node.children().into_iter().for_each(|node| {
            let Some(expression) = self.transform_any_jsx_child(
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
                if let Some(AnyJsArrayElement::AnyJsExpression(expr)) = elements.into_iter().next()
                {
                    Some(expr)
                } else {
                    None
                }
            }
            _ => Some(AnyJsExpression::JsArrayExpression(make_array(elements))),
        }
    }
}
