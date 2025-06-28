use oxc_ast::AstType;
use oxc_ast_visit::{VisitMut, walk_mut};

use crate::{Config, Template, TreeBuilderMut, processor::pre_process_ast};
use sapling_macros::tree_builder_mut;

#[tree_builder_mut(crate::TreeBuilderMut<'a>)]
pub struct SaplingVisitorMut<'a> {
    pub phanton_data: std::marker::PhantomData<&'a ()>,
    pub templates: &'a mut Vec<Template<'a>>,
    pub config: Config<'a>,
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
