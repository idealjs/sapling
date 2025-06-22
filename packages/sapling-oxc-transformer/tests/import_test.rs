use oxc_allocator::Allocator;
use oxc_ast::ast::{ExpressionStatement, Program, Statement};
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::{SourceType, Span};
use oxc_traverse::{Traverse, TraverseCtx, traverse_mut};

use sapling_oxc_transformer::shared::import::register_import_method;

struct TestVisitor<'a> {
    allocator: &'a Allocator,
}

impl<'a> Traverse<'a> for TestVisitor<'a> {
    fn enter_program(&mut self, node: &mut Program<'a>, ctx: &mut TraverseCtx<'a>) {
        // Test first import
        let expr1 = register_import_method(ctx, self.allocator, "createElement", "sapling");
        let expr_stmt1 = ctx.alloc(ExpressionStatement {
            expression: expr1,
            span: Span::default(),
        });
        node.body.push(Statement::ExpressionStatement(expr_stmt1));

        // Test same import should return the same expression
        let expr2 = register_import_method(ctx, self.allocator, "createElement", "sapling");
        let expr_stmt2 = ctx.alloc(ExpressionStatement {
            expression: expr2,
            span: Span::default(),
        });
        node.body.push(Statement::ExpressionStatement(expr_stmt2));

        // Test different import should return different expression
        let expr3 = register_import_method(ctx, self.allocator, "Fragment", "sapling");
        let expr_stmt3 = ctx.alloc(ExpressionStatement {
            expression: expr3,
            span: Span::default(),
        });
        node.body.push(Statement::ExpressionStatement(expr_stmt3));
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
    let scoping = semantic_ret.semantic.into_scoping();

    let mut visitor = TestVisitor {
        allocator: &allocator,
    };
    traverse_mut(&mut visitor, &allocator, &mut program, scoping);

    // Generate and verify the output code
    let result = Codegen::new().build(&program);
    insta::assert_snapshot!(result.code);
}
