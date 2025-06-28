use indextree::{Arena, NodeId};
use oxc_ast::{AstKind, AstType};
use oxc_ast_visit::{Visit, VisitMut, walk, walk_mut};

use crate::{Config, Template, TreeBuilder, TreeBuilderMut, processor::pre_process_ast};
use sapling_macros::{tree_builder, tree_builder_mut};

#[tree_builder_mut]
pub struct SaplingVisitorMut<'a> {
    pub phanton_data: std::marker::PhantomData<&'a ()>,
    pub templates: &'a mut Vec<Template<'a>>,
    pub config: Config<'a>,
}

impl<'a> TreeBuilderMut<'a> for SaplingVisitorMut<'a> {
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
    fn scoping_mut(&mut self) -> &mut oxc_semantic::Scoping {
        self.scoping
    }
    fn allocator_mut(&mut self) -> &'a oxc_allocator::Allocator {
        self.allocator
    }
    fn templates_mut(&mut self) -> &mut Vec<crate::Template<'a>> {
        self.templates
    }
    fn templates_take(&mut self) -> Vec<crate::Template<'a>> {
        std::mem::take(self.templates)
    }
    fn config(&self) -> &Config {
        &self.config
    }
    fn config_mut(&mut self) -> &mut Config<'a> {
        &mut self.config
    }
}


impl<'a> VisitMut<'a> for SaplingVisitorMut<'a> {
    fn enter_node(&mut self, kind: AstType) {
        <Self as TreeBuilderMut>::enter_node(self, kind);
    }
    fn leave_node(&mut self, kind: AstType) {
        <Self as TreeBuilderMut>::leave_node(self, kind);
    }
    fn visit_jsx_element(&mut self, it: &mut oxc_ast::ast::JSXElement<'a>) {
        walk_mut::walk_jsx_element(self, it);
    }
    fn visit_jsx_fragment(&mut self, it: &mut oxc_ast::ast::JSXFragment<'a>) {
        walk_mut::walk_jsx_fragment(self, it);
    }
    fn visit_program(&mut self, it: &mut oxc_ast::ast::Program<'a>) {
        pre_process_ast(self, it, &Config::default());

        walk_mut::walk_program(self, it);
    }
}
