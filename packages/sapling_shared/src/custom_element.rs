//! Functions for handling custom elements in JSX

use std::cell::Cell;

use crate::{TreeBuilderMut, register_import_method};
use oxc_allocator::{Box, CloneIn, Vec};
use oxc_ast::ast::{
    AssignmentExpression, AssignmentOperator, CallExpression, Expression, IdentifierName,
    IdentifierReference, MemberExpression, Program, StaticMemberExpression, VariableDeclarator,
};
use oxc_semantic::{NodeId, Reference, ReferenceFlags, SymbolFlags};
use oxc_span::{Atom, Span};

pub struct ToCustomElementInput<'a> {
    pub template: Atom<'a>,
    pub template_with_closing_tags: Atom<'a>,
    pub declarations: Vec<'a, VariableDeclarator<'a>>,
    pub exprs: Vec<'a, Expression<'a>>,
    pub dynamics: Vec<'a, Expression<'a>>,
    pub post_exprs: Vec<'a, Expression<'a>>,
    pub is_svg: bool,
    pub has_custom_element: bool,
    pub is_import_node: bool,
    pub tag_name: Option<Atom<'a>>,
    pub renderer: Atom<'a>,
    pub skip_template: bool,
}

/// Add context support to custom element
pub fn context_to_custom_element<'a>(
    visitor: &mut impl TreeBuilderMut<'a>,
    program: &mut Program<'a>,
    exprs: &mut Vec<'a, Expression<'a>>,
) {
    let allocator = visitor.allocator_mut();

    let root_scope = if let Some(root_scope) = program.scope_id.get() {
        root_scope
    } else {
        panic!("Root scope not found in program");
    };

    let scoping = visitor.scoping_mut();

    // Create new import declaration node
    let node_id = NodeId::new(program.body.len() as u32);

    // Create symbol for the import
    let symbol_id = scoping.create_symbol(
        Span::default(),
        "",
        SymbolFlags::Import | SymbolFlags::Value,
        root_scope,
        node_id,
    );
    let reference = Reference::new_with_symbol_id(node_id, symbol_id, ReferenceFlags::read());
    let id = IdentifierReference {
        span: Span::default(),
        name: Atom::from("el$"),
        reference_id: Cell::new(Some((scoping.create_reference(reference)))),
    };

    let member = MemberExpression::StaticMemberExpression(Box::new_in(
        StaticMemberExpression {
            span: Span::default(),
            object: Expression::Identifier(Box::new_in(id, allocator)),
            property: IdentifierName {
                span: Span::default(),
                name: Atom::from("_$owner"),
            },
            optional: false,
        },
        allocator,
    ));

    let config = visitor.config();
    let module_name = config.module_name.as_str().clone_in(allocator);

    exprs.push(Expression::AssignmentExpression(Box::new_in(
        AssignmentExpression {
            span: Span::default(),
            left: member.into(),
            right: Expression::CallExpression(Box::new_in(
                CallExpression {
                    span: Span::default(),
                    callee: register_import_method(visitor, program, "getOwner", module_name),
                    type_arguments: None,
                    arguments: Vec::new_in(allocator),
                    optional: false,
                    pure: false,
                },
                allocator,
            )),
            operator: AssignmentOperator::Assign,
        },
        allocator,
    )));
}

/// Convert component identifier to expression
pub fn convert_component_identifier() -> Result<(), &'static str> {
    todo!("Implement conversion of component identifiers to AST expressions")
}

/// Check if element is a component
pub fn is_component() -> Result<bool, &'static str> {
    todo!("Implement checking if element is a component")
}
