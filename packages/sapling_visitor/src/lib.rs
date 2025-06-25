use indextree::{Arena, NodeId};
use oxc_allocator::Allocator;
use oxc_ast::AstKind;
use oxc_ast::ast::Statement;
use oxc_ast_visit::Visit;
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

