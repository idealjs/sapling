use std::{fs, path::Path};

use oxc_allocator::{Allocator, Box as AstBox, Vec as AstVec};
use oxc_ast::ast::{
    Expression, IdentifierReference, Program, Statement, StringLiteral, VariableDeclaration,
    VariableDeclarationKind,
};
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::{Atom, SourceType, Span};
use oxc_syntax::symbol::SymbolId;
use oxc_traverse::{Traverse, TraverseCtx, traverse_mut};
use std::cell::Cell;

use sapling_oxc_transformer::ssr::template::{TemplateItem, append_templates};

pub struct Transformer<'a> {
    allocator: &'a Allocator,
    templates: Vec<TemplateItem<'a>>,
}

impl<'a> Transformer<'a> {
    pub fn new(allocator: &'a Allocator) -> Self {
        Self {
            allocator,
            templates: Vec::new(),
        }
    }
}

impl<'a> Transformer<'a> {
    pub fn add_template(&mut self, template: TemplateItem<'a>) {
        self.templates.push(template);
    }
}

impl<'a> Traverse<'a> for Transformer<'a> {
    fn enter_program(&mut self, node: &mut Program<'a>, ctx: &mut TraverseCtx<'a>) {
        // Add a test template
        let id_ref = ctx.alloc(IdentifierReference {
            span: Span::default(),
            name: Atom::from("testTemplate"),
            reference_id: Cell::new(None),
        });
        let id_expr: Expression<'a> = Expression::Identifier(id_ref);

        let str_lit = ctx.alloc(StringLiteral {
            span: Span::default(),
            value: Atom::from("Test template content"),
            raw: Some(Atom::from("\"Test template content\"")),
            lone_surrogates: false,
        });
        let template_expr: Expression<'a> = Expression::StringLiteral(str_lit);
        
        let template_item: TemplateItem<'a> = TemplateItem {
            id:  self.allocator.alloc(id_expr),
            template: self.allocator.alloc(template_expr),
        };
        self.templates.push(template_item);

        let _ = append_templates(ctx, self.allocator, node, &self.templates);
    }
}

#[test]
fn test_append_templates_transform() {
    let path = Path::new("tests/fixtures/Test.tsx");
    let source_text = fs::read_to_string(path).unwrap();

    let allocator = Allocator::default();
    let source_type = SourceType::default().with_typescript(true).with_jsx(true);

    let ret = Parser::new(&allocator, &source_text, source_type).parse();
    let mut program = ret.program;

    let semantic_ret = SemanticBuilder::new().build(&program);
    let scoping = semantic_ret.semantic.into_scoping();

    let mut transformer = Transformer::new(&allocator);

    traverse_mut(&mut transformer, &allocator, &mut program, scoping);

    let result = Codegen::new().build(&program);

    insta::assert_snapshot!(result.code);
}
