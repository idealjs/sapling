use indextree::Arena;
use indextree::NodeId;
use oxc_allocator::Allocator;
use oxc_ast::ast::Program;
use oxc_ast_visit::VisitMut;
use oxc_ast_visit::walk_mut;
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_semantic::{Scoping, SemanticBuilder};
use oxc_span::SourceType;
use sapling_macros::tree_builder_mut;
use sapling_shared::Template;
use sapling_shared::TreeBuilderMut;

use sapling_shared::import::register_import_method;

#[tree_builder_mut]
struct TestVisitor<'a> {
    templates: &'a mut Vec<Template<'a>>,
}

impl<'a> TreeBuilderMut<'a> for TestVisitor<'a> {
    fn arena(&self) -> &Arena<oxc_ast::AstType> {
        &self.arena
    }

    fn arena_mut(&mut self) -> &mut Arena<oxc_ast::AstType> {
        &mut self.arena
    }

    fn node_stack(&self) -> &Vec<NodeId> {
        &self.node_stack
    }

    fn node_stack_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.node_stack
    }

    fn scoping_mut(&mut self) -> &mut Scoping {
        &mut self.scoping
    }

    fn allocator_mut(&mut self) -> &'a Allocator {
        self.allocator
    }
    fn templates_mut(&mut self) -> &mut Vec<crate::Template<'a>> {
        self.templates
    }
    fn templates_take(&mut self) -> Vec<Template<'a>> {
        std::mem::take(self.templates)
    }
}

impl<'a> VisitMut<'a> for TestVisitor<'a> {
    fn visit_program(&mut self, it: &mut Program<'a>) {
        // Test first import
        let _ = register_import_method(self, it, "createElement", "sapling");

        // Test same import should return the same expression
        let _ = register_import_method(self, it, "createElement", "sapling");

        // Test different import should return different expression
        let _ = register_import_method(self, it, "Fragment", "sapling");

        walk_mut::walk_program(self, it);
    }
}

#[test]
fn test_register_import() {
    let source = "// Initial empty program";
    let allocator = Allocator::default();
    let source_type = SourceType::default().with_typescript(true).with_jsx(true);

    let ret = Parser::new(&allocator, source, source_type).parse();
    let mut program = ret.program;

    let semantic_ret = SemanticBuilder::new().build(&program);
    let mut scoping = semantic_ret.semantic.into_scoping();

    let mut visitor = TestVisitor {
        arena: Arena::new(),
        node_stack: vec![],
        allocator: &allocator,
        scoping: &mut scoping,
        templates: &mut vec![],
    };
    visitor.visit_program(&mut program);

    // Generate and verify the output code
    let result = Codegen::new().build(&program);
    insta::assert_snapshot!(result.code);
}
