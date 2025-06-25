use std::{fs, path::Path};

use indextree::{Arena, NodeId};
use oxc_allocator::Allocator;
use oxc_ast::{
    AstKind,
    ast::{ExpressionStatement, Program, Statement},
};
use oxc_ast_visit::Visit;
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::{SourceType, Span};
use oxc_traverse::{Traverse, TraverseCtx, traverse_mut};

use sapling_macros::tree_builder;
use sapling_shared::{TreeBuilder, import::register_import_method};

#[tree_builder]
struct TestVisitor<'a> {}

impl<'a> TreeBuilder<'a> for TestVisitor<'a> {
    fn arena(&self) -> &Arena<AstKind<'a>> {
        &self.arena
    }
    fn arena_mut(&mut self) -> &mut Arena<AstKind<'a>> {
        &mut self.arena
    }
    fn node_stack(&self) -> &Vec<NodeId> {
        &self.node_stack
    }
    fn node_stack_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.node_stack
    }
}

impl<'a> Visit<'a> for TestVisitor<'a> {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        <Self as TreeBuilder>::enter_node(self, kind);
    }
    fn leave_node(&mut self, kind: AstKind<'a>) {
        <Self as TreeBuilder>::leave_node(self, kind);
    }
}

#[test]
fn test_register_import() {
    let path = Path::new("tests/fixtures/Test.tsx");
    let source_text = fs::read_to_string(path).unwrap();

    let allocator = Allocator::default();
    let source_type = SourceType::default().with_typescript(true).with_jsx(true);

    let ret = Parser::new(&allocator, &source_text, source_type).parse();
    let mut program = ret.program;

    let mut visitor = TestVisitor {
        arena: Arena::new(),
        node_stack: vec![],
    };

    visitor.visit_program(&mut program);

    insta::assert_snapshot!(format!("{:?}", visitor.arena));
}
