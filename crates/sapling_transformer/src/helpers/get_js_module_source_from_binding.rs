use biome_js_semantic::{Binding, BindingExtensions, SemanticModel};
use biome_js_syntax::{JsImport, JsSyntaxKind, JsVariableDeclarator};
use biome_rowan::SyntaxNodeCast;

pub fn get_js_module_source_from_binding(
    semantic_model: &SemanticModel,
    binding: &Binding,
) -> Option<String> {
    let node = binding.syntax();
    for ancestor in node.ancestors() {
        if ancestor.kind() == JsSyntaxKind::JS_IMPORT {
            let node = ancestor.cast::<JsImport>()?.import_clause().ok()?;
            let js_module_source = node.source().ok()?;
            let inner_text = js_module_source.inner_string_text().ok()?;
            let name = inner_text.text();
            return Some(name.into());
        }
        if ancestor.kind() == JsSyntaxKind::JS_VARIABLE_DECLARATOR {
            let binding = ancestor
                .cast::<JsVariableDeclarator>()?
                .initializer()?
                .expression()
                .ok()?
                .as_js_identifier_expression()?
                .name()
                .ok()?
                .binding(semantic_model)?;
            return get_js_module_source_from_binding(semantic_model, &binding);
        }
    }
    None
}
