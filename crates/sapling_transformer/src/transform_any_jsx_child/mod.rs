use biome_js_factory::make::js_expression_statement;
use biome_js_syntax::{AnyJsExpression, AnyJsxChild, JsxExpressionChild, JsxSpreadChild, JsxText};

#[derive(Debug)]
pub struct TransformAnyJsxTextOptions {
    pub parent_id: Option<String>,
}

pub struct TransformAnyJsxChildOptions {
    pub parent_id: Option<String>,
}

pub struct TransformJsxExpressionChildOptions {
    pub parent_id: Option<String>,
}

use crate::{
    SaplingTransformer, make_arrow_function_from_statement, make_create_jsx_tag_element,
    make_create_text_node, make_effect, make_insert, make_insert_node,
    transfrom_jsx_tag_expression::TransformAnyJsxFragmentOptions,
};

impl SaplingTransformer<'_> {
    pub fn transform_any_jsx_child(
        &mut self,
        node: &AnyJsxChild,
        transform_options: TransformAnyJsxChildOptions,
    ) -> Option<AnyJsExpression> {
        match node {
            AnyJsxChild::JsxElement(node) => {
                let (statements, id) = self.transform_jsx_element(node)?;
                let call = make_create_jsx_tag_element(&vec![], &statements, id);

                Some(AnyJsExpression::JsCallExpression(call))
            }
            AnyJsxChild::JsxExpressionChild(node) => self.transform_jsx_expression_child(
                node,
                TransformJsxExpressionChildOptions {
                    parent_id: transform_options.parent_id,
                },
            ),
            AnyJsxChild::JsxFragment(node) => self.transform_jsx_fragment(
                node,
                TransformAnyJsxFragmentOptions {
                    parent_id: transform_options.parent_id,
                },
            ),
            AnyJsxChild::JsxSelfClosingElement(node) => {
                let (statements, id) =
                    self.transform_jsx_self_closing_element_to_statements(node)?;
                let call = make_create_jsx_tag_element(&vec![], &statements, id);

                Some(AnyJsExpression::JsCallExpression(call))
            }
            AnyJsxChild::JsxSpreadChild(node) => self.transform_jsx_spread_child(node),
            AnyJsxChild::JsxText(node) => self.transform_jsx_text(
                node,
                TransformAnyJsxTextOptions {
                    parent_id: transform_options.parent_id,
                },
            ),
            _ => {
                unreachable!()
            }
        }
    }

    pub fn transform_jsx_text(
        &self,
        node: &JsxText,
        transform_options: TransformAnyJsxTextOptions,
    ) -> Option<AnyJsExpression> {
        let binding = node.to_string();
        let node_value = binding.as_str().trim();

        if node_value.is_empty() {
            return None;
        }
        let inner_call_expression = make_create_text_node(node_value);
        let parent_id = transform_options.parent_id?;
        Some(AnyJsExpression::from(make_insert_node(
            parent_id.as_str(),
            &inner_call_expression.to_string(),
        )))
    }

    pub fn transform_jsx_expression_child(
        &mut self,
        node: &JsxExpressionChild,
        transform_options: TransformJsxExpressionChildOptions,
    ) -> Option<AnyJsExpression> {
        let expression = node.expression()?;
        let parent_id = transform_options.parent_id?;

        let call_expr = match expression {
            AnyJsExpression::JsxTagExpression(_) => {
                make_insert(parent_id.as_str(), expression.clone())
            }
            _ => make_effect(
                AnyJsExpression::JsArrowFunctionExpression(make_arrow_function_from_statement(
                    biome_js_syntax::AnyJsStatement::JsExpressionStatement(
                        js_expression_statement(AnyJsExpression::JsCallExpression(make_insert(
                            parent_id.as_str(),
                            expression.clone(),
                        )))
                        .build(),
                    ),
                )),
                vec![expression.clone()],
            ),
        };

        Some(AnyJsExpression::JsCallExpression(call_expr))
    }

    pub fn transform_jsx_spread_child(&self, _node: &JsxSpreadChild) -> Option<AnyJsExpression> {
        todo!()
    }
}
