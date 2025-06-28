use indextree::{Arena, NodeId};
use oxc_allocator::Allocator;
use oxc_allocator::FromIn;
use oxc_ast::ast::Statement;
use oxc_ast::{
    AstKind,
    ast::{Argument, Expression, IdentifierReference, Program, StringLiteral},
};
use oxc_ast_visit::{VisitMut, walk_mut::walk_program};
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_semantic::{Scoping, Semantic, SemanticBuilder};
use oxc_span::{Atom, SourceType, Span};
use sapling_macros::tree_builder_mut;
use sapling_shared::Config;
use sapling_shared::append_templates_ssr;
use sapling_shared::{
    Template, TreeBuilderMut, create_template_ssr, ssr::template::CreateTemplateInput,
};

#[tree_builder_mut(sapling_shared::TreeBuilderMut<'a>)]
struct TestVisitor<'a> {
    pub config: Config<'a>,
    pub templates: &'a mut Vec<Template<'a>>,
}

impl<'a> VisitMut<'a> for TestVisitor<'a> {
    fn visit_program(&mut self, it: &mut Program<'a>) {
        let expr = create_template_ssr(
            self,
            it,
            &mut CreateTemplateInput {
                template: None,
                exprs: oxc_allocator::Vec::from_array_in(
                    [Expression::Identifier(oxc_allocator::Box::new_in(
                        IdentifierReference {
                            span: Span::default(),
                            name: Atom::from_in("template", self.allocator),
                            reference_id: std::cell::Cell::new(None),
                        },
                        self.allocator,
                    ))],
                    self.allocator,
                ),
                template_values: oxc_allocator::Vec::new_in(self.allocator),
                wont_escape: false,
            },
            "sapling",
        );
        append_templates_ssr(self, it);

        let expr = create_template_ssr(
            self,
            it,
            &mut CreateTemplateInput {
                template: None,
                exprs: oxc_allocator::Vec::from_array_in(
                    [Expression::Identifier(oxc_allocator::Box::new_in(
                        IdentifierReference {
                            span: Span::default(),
                            name: Atom::from_in("template", self.allocator),
                            reference_id: std::cell::Cell::new(None),
                        },
                        self.allocator,
                    ))],
                    self.allocator,
                ),
                template_values: oxc_allocator::Vec::from_array_in(
                    [
                        Expression::StringLiteral(oxc_allocator::Box::new_in(
                            StringLiteral {
                                span: Span::default(),
                                value: Atom::from_in("Hello", self.allocator),
                                raw: None,
                                lone_surrogates: false,
                            },
                            self.allocator,
                        )),
                        Expression::NumericLiteral(oxc_allocator::Box::new_in(
                            oxc_ast::ast::NumericLiteral {
                                span: Span::default(),
                                value: 42.0,
                                raw: None,
                                base: oxc_ast::ast::NumberBase::Decimal,
                            },
                            self.allocator,
                        )),
                        Expression::Identifier(oxc_allocator::Box::new_in(
                            IdentifierReference {
                                span: Span::default(),
                                name: Atom::from_in("world", self.allocator),
                                reference_id: std::cell::Cell::new(None),
                            },
                            self.allocator,
                        )),
                    ],
                    self.allocator,
                ),
                wont_escape: false,
            },
            "sapling",
        );
        it.body
            .push(Statement::ExpressionStatement(oxc_allocator::Box::new_in(
                oxc_ast::ast::ExpressionStatement {
                    span: oxc_span::Span::default(),
                    expression: expr,
                },
                self.allocator,
            )));

        // Test case with template
        let expr = create_template_ssr(
            self,
            it,
            &mut CreateTemplateInput {
                template: Some(sapling_shared::ssr::template::TemplateValue::Single(
                    Atom::from_in("<div>Hello ${}!</div>", self.allocator),
                )),
                exprs: oxc_allocator::Vec::from_array_in(
                    [Expression::Identifier(oxc_allocator::Box::new_in(
                        IdentifierReference {
                            span: Span::default(),
                            name: Atom::from_in("template", self.allocator),
                            reference_id: std::cell::Cell::new(None),
                        },
                        self.allocator,
                    ))],
                    self.allocator,
                ),
                template_values: oxc_allocator::Vec::from_array_in(
                    [Expression::StringLiteral(oxc_allocator::Box::new_in(
                        StringLiteral {
                            span: Span::default(),
                            value: Atom::from_in("world", self.allocator),
                            raw: None,
                            lone_surrogates: false,
                        },
                        self.allocator,
                    ))],
                    self.allocator,
                ),
                wont_escape: false,
            },
            "sapling",
        );
        it.body
            .push(Statement::ExpressionStatement(oxc_allocator::Box::new_in(
                oxc_ast::ast::ExpressionStatement {
                    span: oxc_span::Span::default(),
                    expression: expr,
                },
                self.allocator,
            )));

        // Test case with multiple template parts
        let expr = create_template_ssr(
            self,
            it,
            &mut CreateTemplateInput {
                template: Some(sapling_shared::ssr::template::TemplateValue::Multiple(
                    oxc_allocator::Vec::from_array_in(
                        [
                            Atom::from_in("<div>Hello ", self.allocator),
                            Atom::from_in(", welcome to ", self.allocator),
                            Atom::from_in("!</div>", self.allocator),
                        ],
                        self.allocator,
                    ),
                )),
                exprs: oxc_allocator::Vec::from_array_in(
                    [Expression::Identifier(oxc_allocator::Box::new_in(
                        IdentifierReference {
                            span: Span::default(),
                            name: Atom::from_in("template", self.allocator),
                            reference_id: std::cell::Cell::new(None),
                        },
                        self.allocator,
                    ))],
                    self.allocator,
                ),
                template_values: oxc_allocator::Vec::from_array_in(
                    [
                        Expression::StringLiteral(oxc_allocator::Box::new_in(
                            StringLiteral {
                                span: Span::default(),
                                value: Atom::from_in("user", self.allocator),
                                raw: None,
                                lone_surrogates: false,
                            },
                            self.allocator,
                        )),
                        Expression::StringLiteral(oxc_allocator::Box::new_in(
                            StringLiteral {
                                span: Span::default(),
                                value: Atom::from_in("Sapling", self.allocator),
                                raw: None,
                                lone_surrogates: false,
                            },
                            self.allocator,
                        )),
                    ],
                    self.allocator,
                ),
                wont_escape: false,
            },
            "sapling",
        );
        it.body
            .push(Statement::ExpressionStatement(oxc_allocator::Box::new_in(
                oxc_ast::ast::ExpressionStatement {
                    span: oxc_span::Span::default(),
                    expression: expr,
                },
                self.allocator,
            )));
        append_templates_ssr(self, it);
    }
}

#[test]
fn test_create_template_ssr() {
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
