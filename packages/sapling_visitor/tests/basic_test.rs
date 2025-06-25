use std::{fs, path::Path};

use indextree::Arena;
use oxc_allocator::Allocator;
use oxc_ast_visit::VisitMut;
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_span::SourceType;
use sapling_visitor::SaplingVisitor; // Import the trait for visit_program

#[test]
fn test_uppercase_function_transform() {
    let path = Path::new("tests/fixtures/Test.tsx");
    let source_text = fs::read_to_string(path).unwrap();

    let allocator = Allocator::default();
    let source_type = SourceType::default().with_typescript(true).with_jsx(true);

    let ret = Parser::new(&allocator, &source_text, source_type).parse();
    let mut program = ret.program;

    let mut visitor = SaplingVisitor {
        allocator: &allocator,
        arena: Arena::new(),
        node_stack: vec![],
    };

    visitor.visit_program(&mut program);

    let result = Codegen::new().build(&program);

    insta::assert_snapshot!(result.code);
}
