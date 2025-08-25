use crate::SaplingTransformer;

use biome_js_semantic::Scope;

use std::collections::HashSet;

impl SaplingTransformer<'_> {
    pub fn generate_unique_identifier(&mut self, scope: &Scope, base: &str) -> String {
        let mut name = base.to_string();
        let mut counter = 0;
        let range = scope.range();
        let used = self
            .scope_generated_identifiers
            .entry(range)
            .or_insert_with(HashSet::new);
        while scope.get_binding(&name).is_some() || used.contains(&name) {
            counter += 1;
            name = format!("{}{}", base, counter);
        }
        used.insert(name.clone());
        name
    }
}
