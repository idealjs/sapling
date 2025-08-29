use biome_js_factory::make::{js_arrow_function_expression, js_function_expression};
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsArrowFunctionExpression, JsCallExpression,
    JsFunctionExpression, JsIdentifierExpression, JsParenthesizedExpression,
};

use crate::SaplingTransformer;

impl SaplingTransformer<'_> {
    // main entry
    pub fn transform_any_js_expression(
        &mut self,
        node: &AnyJsExpression,
    ) -> Option<AnyJsExpression> {
        match node {
            AnyJsExpression::AnyJsLiteralExpression(node) => {
                self.transform_any_js_literal_expression(node)
            }
            AnyJsExpression::JsArrowFunctionExpression(node) => {
                self.transform_js_arrow_function_expression(node)
            }
            AnyJsExpression::JsFunctionExpression(node) => {
                self.transform_js_function_expression(node)
            }
            AnyJsExpression::JsIdentifierExpression(node) => {
                self.transform_js_identifier_expression(node)
            }
            _ => {
                unreachable!()
            }
        }
    }
    pub fn transform_any_js_literal_expression(
        &self,
        node: &AnyJsLiteralExpression,
    ) -> Option<AnyJsExpression> {
        Some(AnyJsExpression::AnyJsLiteralExpression(node.clone()))
    }

    pub fn transform_js_arrow_function_expression(
        &mut self,
        node: &JsArrowFunctionExpression,
    ) -> Option<AnyJsExpression> {
        Some(AnyJsExpression::JsArrowFunctionExpression(
            js_arrow_function_expression(
                node.parameters().ok()?,
                node.fat_arrow_token().ok()?,
                node.body().ok().clone()?,
            )
            .build(),
        ))
    }

    pub fn transform_js_function_expression(
        &mut self,
        node: &JsFunctionExpression,
    ) -> Option<AnyJsExpression> {
        Some(AnyJsExpression::JsFunctionExpression(
            js_function_expression(
                node.function_token().ok()?,
                node.parameters().ok()?,
                node.body().ok().clone()?,
            )
            .build(),
        ))
    }

    pub fn transform_js_parenthesized_expression(
        &mut self,
        node: &JsParenthesizedExpression,
    ) -> Option<AnyJsExpression> {
        let expression = node.expression().ok()?;
        self.transform_any_js_expression(&expression)
    }

    pub fn transform_js_call_expression(&self, node: &JsCallExpression) -> Option<AnyJsExpression> {
        Some(AnyJsExpression::JsCallExpression(node.clone()))
    }

    pub fn transform_js_identifier_expression(
        &self,
        node: &JsIdentifierExpression,
    ) -> Option<AnyJsExpression> {
        Some(AnyJsExpression::JsIdentifierExpression(node.clone()))
    }
}
