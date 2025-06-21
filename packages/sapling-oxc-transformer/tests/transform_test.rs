#![allow(clippy::print_stdout)]

use std::fs;
use std::path::Path;

use oxc_allocator::Allocator;
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use oxc_transformer::{TransformOptions, Transformer};

#[test]
fn test_transform_tsx() {
    let path = Path::new("tests/Test.tsx");
    let source_text = fs::read_to_string(path).unwrap();
    let allocator = Allocator::default();
    let source_type = SourceType::default().with_typescript(true).with_jsx(true);

    // Parse the source code
    let ret = Parser::new(&allocator, &source_text, source_type).parse();

    let mut program = ret.program;

    // Perform semantic analysis
    let semantic_ret = SemanticBuilder::new()
        .with_excess_capacity(2.0)
        .build(&program);

    let scoping = semantic_ret.semantic.into_scoping();

    // Transform the AST with all transformations enabled
    let transform_options = TransformOptions::enable_all();
    Transformer::new(&allocator, path, &transform_options)
        .build_with_scoping(scoping, &mut program);

    // Generate code and create snapshot
    let result = Codegen::new().build(&program);
    insta::assert_snapshot!(result.code);
}
