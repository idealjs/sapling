use core::panic;
use oxc_allocator::Box;
use oxc_allocator::FromIn;
use oxc_allocator::Vec;
use oxc_ast::ast::{
    BindingIdentifier, Expression, IdentifierName, IdentifierReference, ImportDeclaration,
    ImportDeclarationSpecifier, ImportOrExportKind, ImportSpecifier, ModuleExportName, Program,
    Statement,
};
use oxc_semantic::Scoping;
use oxc_span::{Atom, Span};
use oxc_syntax::node::NodeId;
use oxc_syntax::reference::{Reference, ReferenceFlags, ReferenceId};
use oxc_syntax::symbol::{SymbolFlags, SymbolId};
use std::cell::Cell;

use crate::TreeBuilderMut;

/// Generate a unique identifier name for imported variables.
/// The generated name follows the pattern '_$name' to:
/// 1. Avoid naming conflicts with user variables
/// 2. Match Babel's naming convention for transformed imports
/// 3. Clearly identify imported variables in the generated code
fn generate_import_string(name: &str) -> String {
    format!("_${}", name)
}

/// Create a reference for an imported symbol.
fn create_import_reference(
    scoping: &mut Scoping,
    symbol_id: SymbolId,
    node_id: NodeId,
) -> ReferenceId {
    let reference = Reference::new_with_symbol_id(node_id, symbol_id, ReferenceFlags::read());
    scoping.create_reference(reference)
}

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

    let import_base_name = generate_import_string(name);
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
        create_import_reference(scoping, symbol_id, node_id)
    } else {
        // Create new import declaration node
        let node_id = NodeId::new(program.body.len() as u32);

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

        // Create reference and binding for the import
        let mut scoping = visitor.scoping_mut();
        let reference_id = create_import_reference(&mut scoping, symbol_id, node_id);
        scoping.add_binding(root_scope, &import_lookup_key, symbol_id);

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
