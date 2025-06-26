use indextree::{Arena, NodeId};
use oxc_allocator::Allocator;
use oxc_ast::AstType;
use oxc_ast_visit::VisitMut;
use oxc_ast_visit::walk_mut::{walk_jsx_element, walk_jsx_fragment, walk_program, walk_statement};

use sapling_macros::tree_builder_mut;
use sapling_shared::pre_process::pre_process;
use sapling_shared::{Config, TreeBuilderMut};

#[tree_builder_mut]
pub struct SaplingVisitor<'a> {
    pub allocator: &'a Allocator,
}

impl<'a> TreeBuilderMut<'a> for SaplingVisitor<'a> {
    fn arena(&self) -> &Arena<AstType> {
        &self.arena
    }
    fn arena_mut(&mut self) -> &mut Arena<AstType> {
        &mut self.arena
    }
    fn node_stack(&self) -> &Vec<NodeId> {
        &self.node_stack
    }
    fn node_stack_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.node_stack
    }
}

impl<'a> VisitMut<'a> for SaplingVisitor<'a> {
    fn enter_node(&mut self, kind: AstType) {
        <Self as TreeBuilderMut>::enter_node(self, kind);
    }
    fn leave_node(&mut self, kind: AstType) {
        <Self as TreeBuilderMut>::leave_node(self, kind);
        match kind {
            AstType::Program => {}
            _ => {}
        }
    }
    fn visit_jsx_element(&mut self, it: &mut oxc_ast::ast::JSXElement<'a>) {
        walk_jsx_element(self, it);
    }
    fn visit_jsx_fragment(&mut self, it: &mut oxc_ast::ast::JSXFragment<'a>) {
        walk_jsx_fragment(self, it);
    }
    fn visit_program(&mut self, it: &mut oxc_ast::ast::Program<'a>) {
        pre_process(it, &Config::default());
        walk_program(self, it);
    }
}
