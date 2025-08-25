use biome_js_factory::make::{js_initializer_clause, js_variable_declarator, token};
use biome_js_syntax::{JsVariableDeclarator, T};

use crate::SaplingTransformer;

impl SaplingTransformer<'_> {
    // main entry
    pub fn transform_js_variable_declarator(
        &mut self,
        node: JsVariableDeclarator,
    ) -> Option<JsVariableDeclarator> {
        let id = node.id().ok()?;
        let initializer = node.initializer()?;
        let expression = initializer.expression().ok()?;
        let new_expression = self.transform_any_js_expression(&expression)?;

        Some(
            js_variable_declarator(id)
                .with_initializer(js_initializer_clause(token(T!(=)), new_expression))
                .build(),
        )
    }
}
