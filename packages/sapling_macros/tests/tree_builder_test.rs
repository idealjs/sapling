use indextree::{Arena, NodeId};
use oxc_allocator::Allocator;
use oxc_ast::AstKind;
use oxc_ast::ast::*;
use oxc_ast_visit::Visit;
use oxc_ast_visit::VisitMut;
use oxc_parser::Parser;
use oxc_span::SourceType;
use sapling_macros::{TreeBuilder, tree_builder};

#[tree_builder]
// #[derive(TreeBuilder)]
struct TestTreeBuilder<'a> {}

trait TreeBuilder<'a>: Visit<'a> {
    type ArenaType; // 定义关联类型

    // 要求实现获取字段的方法
    fn arena(&self) -> &Self::ArenaType;
    fn arena_mut(&mut self) -> &mut Self::ArenaType;

    fn enter_node(&mut self, kind: AstKind<'a>) {
        println!("tree builder")
    }
}

impl<'a> TreeBuilder<'a> for TestTreeBuilder<'a> {
    type ArenaType = Arena<AstKind<'a>>;
    fn arena(&self) -> &Self::ArenaType {
        &self.arena
    }
    fn arena_mut(&mut self) -> &mut Self::ArenaType {
        &mut self.arena
    }
}

impl<'a> Visit<'a> for TestTreeBuilder<'a> {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        println!("Visit")
    }
    fn visit_program(&mut self, it: &Program<'a>) {}
}
