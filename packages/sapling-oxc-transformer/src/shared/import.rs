use oxc_allocator::Allocator;
use oxc_allocator::{Box, CloneIn};
use oxc_ast::ast::Expression;
use oxc_span::{Atom, Span};
use oxc_syntax::{reference::ReferenceFlags, symbol::SymbolFlags};
use oxc_traverse::TraverseCtx;

/// Register an import method and return an Expression referencing it.
/// Similar to @babel/helper-module-imports addNamed functionality.
///
/// # Arguments
/// * `ctx` - Traverse context for scope management
/// * `name` - Name of import to register
/// * `module_name` - Module to import from

pub fn register_import_method<'a>(
    ctx: &mut TraverseCtx<'a>,
    allocator: &'a Allocator,
    name: &str,
    module_name: &str,
) -> Expression<'a> {
    // Get or create imports map in program scope
    let root_scope = ctx.scoping().root_scope_id();
    let import_lookup_key = format!("{module_name}:{name}");
    let import_base_name = format!("_${name}");

    // Get existing binding or create new one
    let (import_identifier, symbol_id) =
        if let Some(existing_id) = ctx.scoping().find_binding(root_scope, &import_lookup_key) {
            // For existing imports, use same naming pattern as new imports
            let unique_atom = Atom::from(CloneIn::clone_in(&import_base_name.as_str(), allocator));
            (unique_atom, existing_id)
        } else {
            // Generate new unique import identifier name
            let unique_atom = Atom::from(ctx.generate_uid_name(&import_base_name));

            // Create new import binding in root scope
            let binding = ctx.generate_binding(
                unique_atom.clone(),
                root_scope,
                SymbolFlags::Value | SymbolFlags::Import,
            );

            // Add binding to imports tracking
            ctx.scoping_mut()
                .add_binding(root_scope, &import_lookup_key, binding.symbol_id);

            (unique_atom, binding.symbol_id)
        };

    // Create bound identifier reference
    let span = Span::default(); // Use default span for generated nodes
    ctx.create_bound_ident_expr(span, import_identifier, symbol_id, ReferenceFlags::Value)
}
