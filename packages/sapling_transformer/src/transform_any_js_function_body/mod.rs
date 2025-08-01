use biome_js_factory::make::{js_directive_list, js_function_body, js_statement_list, token};
use biome_js_syntax::{AnyJsFunctionBody, AnyJsStatement, JsFunctionBody, T};

use crate::SaplingTransformer;

impl SaplingTransformer {
    pub fn transform_any_js_function_body(
        &mut self,
        node: &AnyJsFunctionBody,
    ) -> Option<AnyJsFunctionBody> {
        match node {
            AnyJsFunctionBody::JsFunctionBody(inner) => Some(AnyJsFunctionBody::JsFunctionBody(
                self.transform_js_function_body(inner)?,
            )),

            AnyJsFunctionBody::AnyJsExpression(inner) => Some(AnyJsFunctionBody::AnyJsExpression(
                self.transform_any_js_expression(inner)?,
            )),
        }
    }
}
