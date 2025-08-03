use crate::{SaplingTransformer, TransformAnyJsxChildOptions};
use biome_js_factory::make::{
    ident, js_identifier_expression, js_reference_identifier, js_return_statement, token,
};
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsStatement, AnyJsxTag, JsxElement, JsxFragment,
    JsxSelfClosingElement, JsxTagExpression, T,
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
    ) -> Option<AnyJsExpression> {
        let tag = node.tag().ok()?;
        match tag {
            AnyJsxTag::JsxElement(node) => self.transform_jsx_element_to_iife(&node),
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
    pub fn transform_jsx_element_to_iife(&mut self, node: &JsxElement) -> Option<AnyJsExpression> {
        let (mut statements, id) = self.transform_jsx_element_to_statements(node)?;

        let return_stmt = AnyJsStatement::JsReturnStatement(
            js_return_statement(token(T![return]))
                .with_argument(js_identifier_expression(js_reference_identifier(ident(&id))).into())
                .with_semicolon_token(token(T![;]))
                .build(),
        );
        statements.push(return_stmt);

        let iife = make_iife(vec![], statements);

        Some(AnyJsExpression::JsCallExpression(iife))
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
        _node: &JsxSelfClosingElement,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
}
