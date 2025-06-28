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
use sapling_shared::Config;
use sapling_shared::Template;
use sapling_shared::TreeBuilderMut;

use sapling_shared::import::register_import_method;

#[tree_builder_mut(sapling_shared::TreeBuilderMut<'a>)]
struct TestVisitor<'a> {
    templates: &'a mut Vec<Template<'a>>,
    config: Config<'a>,
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
        arena: &mut Arena::new(),
        node_stack: &mut vec![],
        allocator: &allocator,
        scoping: &mut scoping,
        templates: &mut vec![],
        config: Config::default(),
    };
    visitor.visit_program(&mut program);

    // Generate and verify the output code
    let result = Codegen::new().build(&program);
    insta::assert_snapshot!(result.code);
}
