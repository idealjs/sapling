use indextree::{Arena, NodeId};
use oxc_allocator::Allocator;
use oxc_ast::AstKind;
use oxc_ast::ast::{Atom, Statement};
use oxc_ast_visit::walk_mut::walk_statement;
use oxc_ast_visit::{Visit, VisitMut};
use oxc_parser::Parser;
use oxc_span::SourceType;
use oxc_traverse::{Traverse, TraverseCtx};
pub struct Transformer<'a> {
    allocator: &'a Allocator,
}

impl<'a> Transformer<'a> {
    pub fn new(allocator: &'a Allocator) -> Self {
        Self { allocator }
    }
}

impl<'a> Transformer<'a> {}

impl<'a> Traverse<'a> for Transformer<'a> {
    fn enter_statement(&mut self, node: &mut Statement<'a>, ctx: &mut TraverseCtx<'a>) {}
}

pub struct SaplingVisitor<'a> {
    pub allocator: &'a Allocator,
}

impl<'a> VisitMut<'a> for SaplingVisitor<'a> {
    fn visit_statement(&mut self, node: &mut Statement<'a>) {
        walk_statement(self, node);
        if let Statement::FunctionDeclaration(func) = node {
            if let Some(name) = func.id.as_mut() {
                let uppercase_name = name.name.to_uppercase();
                let allocated = self.allocator.alloc_str(&uppercase_name);
                name.name = Atom::from(allocated);
            }
        }
    }
}
