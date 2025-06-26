use std::{fs, path::Path};

use indextree::{Arena, NodeId};
use oxc_allocator::Allocator;
use oxc_ast::{
    AstKind,
    ast::{JSXExpression, JSXExpressionContainer},
};
use oxc_ast_visit::{Visit, walk};
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;

use sapling_macros::tree_builder;
use sapling_shared::{TreeBuilder, get_static_expression};

mod debug;
use debug::DebugArena;

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

        println!("Expression source text: {:?}", kind);
    }
    fn leave_node(&mut self, kind: AstKind<'a>) {
        <Self as TreeBuilder>::leave_node(self, kind);
    }

    fn visit_jsx_expression_container(&mut self, it: &JSXExpressionContainer<'a>) {
        let kind = AstKind::JSXExpressionContainer(self.alloc(it));
        <Self as Visit<'a>>::enter_node(self, kind);

        let current_node = self
            .arena()
            .get(*self.node_stack.last().unwrap())
            .expect("current node should exist");

        // 调用 get_static_expression
        let is_static = get_static_expression(current_node, self.arena());

        self.visit_span(&it.span);
        self.visit_jsx_expression(&it.expression);
        <Self as Visit<'a>>::leave_node(self, kind);

        match &it.expression {
            JSXExpression::StringLiteral(_)
            | JSXExpression::NumericLiteral(_)
            | JSXExpression::BooleanLiteral(_) => {
                assert!(is_static, "should be recognized as a static expression");
            }
            _ => {
                assert!(!is_static, "should be recognized as a dynamic expression");
            }
        }
    }
}

#[test]
fn test_get_static_expression() {
    let path = Path::new("tests/fixtures/Test.tsx");
    let source_text = fs::read_to_string(path).unwrap();

    let allocator = Allocator::default();
    let source_type = SourceType::default().with_typescript(true).with_jsx(true);

    let ret = Parser::new(&allocator, &source_text, source_type).parse();
    let mut program = ret.program;

    let semantic_ret = SemanticBuilder::new().build(&program);
    let scoping = semantic_ret.semantic.into_scoping();

    let mut visitor = TestVisitor {
        arena: Arena::new(),
        node_stack: vec![],
        allocator: &allocator,
        scoping: &scoping,
    };

    visitor.visit_program(&mut program);

    insta::assert_snapshot!(format!("{:?}", DebugArena::new(visitor.arena)));
}
