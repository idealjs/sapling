use core::panic;
use oxc_allocator::Box;
use oxc_allocator::FromIn;
use oxc_allocator::Vec;
use oxc_ast::ast::{
    BindingIdentifier, Expression, IdentifierName, IdentifierReference, ImportDeclaration,
    ImportDeclarationSpecifier, ImportOrExportKind, ImportSpecifier, ModuleExportName, Program,
    Statement,
};
use oxc_span::{Atom, Span};
use oxc_syntax::node::NodeId;
use oxc_syntax::reference::{Reference, ReferenceFlags};
use oxc_syntax::symbol::SymbolFlags;
use std::cell::Cell;

use crate::TreeBuilderMut;

pub fn register_import_method<'a, V>(
    visitor: &mut V,
    program: &mut Program<'a>,
    name: &str,
    module_name: &str,
) -> Expression<'a>
where
    V: TreeBuilderMut<'a>,
{
    // Create import lookup key using same format as JS version
    let root_scope = if let Some(root_scope) = program.scope_id.get() {
        root_scope
    } else {
        panic!("Root scope not found in program");
    };

    // Create unique identifier name for import
    let import_base_name = format!("_${name}");
    let allocator = visitor.allocator_mut();

    // Store lookup key for future reference if needed
    let import_lookup_key = format!("{}:{}", module_name, name);
    // Check if binding already exists (placeholder for future logic)
    let reference_id = if let Some(symbol_id) = visitor
        .scoping_mut()
        .find_binding(root_scope, &import_lookup_key)
    {
        let scoping = visitor.scoping_mut();
        let scope_id = scoping.symbol_scope_id(symbol_id);
        let node_id = scoping.get_node_id(scope_id);

        // Create reference - since this is an import, we're reading its value
        let reference = Reference::new_with_symbol_id(node_id, symbol_id, ReferenceFlags::read());

        // Register the reference in scoping
        let reference_id = visitor.scoping_mut().create_reference(reference);

        reference_id
    } else {
        // Create new import declaration node
        let node_id = NodeId::new(0);

        // Create symbol for the import
        let symbol_id = visitor.scoping_mut().create_symbol(
            Span::default(),
            name,
            SymbolFlags::Import | SymbolFlags::Value,
            root_scope,
            node_id,
        );

        // Create import specifier
        let specifier = ImportDeclarationSpecifier::ImportSpecifier(Box::new_in(
            ImportSpecifier {
                span: Span::default(),
                local: BindingIdentifier {
                    span: Span::default(),
                    name: Atom::from_in(import_base_name.as_str(), allocator),
                    symbol_id: Cell::new(Some(symbol_id)),
                },
                imported: ModuleExportName::IdentifierName(IdentifierName {
                    span: Span::default(),
                    name: Atom::from_in(name, allocator),
                }),
                import_kind: ImportOrExportKind::Value,
            },
            allocator,
        ));

        // Create and add import declaration
        let import_declaration = ImportDeclaration {
            span: Span::default(),
            specifiers: Some(Vec::from_array_in([specifier], allocator)),
            source: oxc_ast::ast::StringLiteral {
                span: Span::default(),
                value: Atom::from_in(module_name, allocator),
                raw: None,
                lone_surrogates: false,
            },
            with_clause: None,
            phase: None,
            import_kind: ImportOrExportKind::Value,
        };

        program.body.push(Statement::ImportDeclaration(Box::new_in(
            import_declaration,
            allocator,
        )));

        // Create reference - since this is an import, we're reading its value
        let reference = Reference::new_with_symbol_id(node_id, symbol_id, ReferenceFlags::read());

        // Register the reference in scoping
        let reference_id = visitor.scoping_mut().create_reference(reference);
        visitor
            .scoping_mut()
            .add_binding(root_scope, &import_lookup_key, symbol_id);

        reference_id
    };

    // // Create and return identifier expression with the reference
    Expression::Identifier(Box::new_in(
        IdentifierReference {
            span: Span::default(),
            name: Atom::from_in(import_base_name.as_str(), allocator),
            reference_id: Cell::new(Some(reference_id)),
        },
        allocator,
    ))
}
