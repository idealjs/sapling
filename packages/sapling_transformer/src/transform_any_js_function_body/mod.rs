use biome_js_syntax::AnyJsFunctionBody;

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
