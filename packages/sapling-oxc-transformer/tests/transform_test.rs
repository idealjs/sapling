#![allow(clippy::print_stdout)]

use std::fs;
use std::path::Path;

use oxc_allocator::Allocator;
use oxc_ast::ast::*;
use oxc_codegen::Codegen;
use oxc_span::Atom;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use oxc_transformer::{TransformOptions, Transformer};
use oxc_traverse::Traverse;
use oxc_traverse::{TraverseCtx, traverse_mut};

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

// 自定义transformer结构体
struct MyTransform<'a> {
    allocator: &'a Allocator,
}

impl<'a> MyTransform<'a> {
    /// Rename the function
    // This function's name describes what it does, not just `transform_function`
    fn rename_function(&mut self, func: &mut Function<'a>) {
        if let Some(id) = &mut func.id {
            // Create the uppercase string
            let upper = id.name.to_uppercase();
            // Allocate the uppercase string slice on the heap
            let upper_string = self.allocator.alloc_str(&upper);
            // Create an Atom from the allocated string
            let atom = Atom::from(upper_string);
            id.name = atom.clone();
        }
    }
}

impl<'a> Traverse<'a> for MyTransform<'a> {
    fn enter_statement(&mut self, node: &mut Statement<'a>, ctx: &mut TraverseCtx<'a>) {
        if let Statement::FunctionDeclaration(decl) = node {
            self.rename_function(decl);
        }
    }
}

#[test]
fn test_uppercase_function_transform() {
    let source = r#"
        function hello() {
            console.log('Hello');
        }
    "#;

    let allocator = Allocator::default();
    let source_type = SourceType::default().with_module(true);

    // 解析源代码
    let ret = Parser::new(&allocator, source, source_type).parse();
    let mut program = ret.program;

    // 执行语义分析
    let semantic_ret = SemanticBuilder::new().build(&program);
    let scoping = semantic_ret.semantic.into_scoping();

    // 创建自定义transformer并应用转换
    let mut transformer = MyTransform { allocator: &allocator };

    traverse_mut(&mut transformer, &allocator, &mut program, scoping);

    // 生成代码
    let result = Codegen::new().build(&program);

    // 验证转换结果
    assert!(result.code.contains("function HELLO"));

    // 或使用快照测试
    insta::assert_snapshot!(result.code);
}
