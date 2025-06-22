use oxc_allocator::Allocator;
use oxc_ast::ast::*;
use oxc_traverse::{Traverse, TraverseCtx};

pub mod shared;
pub mod dom;
pub mod ssr;

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
