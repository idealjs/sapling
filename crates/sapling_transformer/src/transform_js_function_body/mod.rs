use biome_js_factory::make::{js_directive_list, js_function_body, js_statement_list, token};
use biome_js_syntax::{AnyJsStatement, JsFunctionBody, T};

use crate::SaplingTransformer;

impl SaplingTransformer<'_> {
    pub fn transform_js_function_body(&mut self, node: &JsFunctionBody) -> Option<JsFunctionBody> {
        let new_statements: Vec<AnyJsStatement> = node
            .statements()
            .into_iter()
            .filter_map(|inner| self.transform_any_js_statement(&inner))
            .collect();

        Some(js_function_body(
            token(T!['{']),
            js_directive_list(vec![]),
            js_statement_list(new_statements),
            token(T!['}']),
        ))
    }
}
