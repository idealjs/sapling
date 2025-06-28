use indextree::{Arena, NodeId};
use oxc_allocator::Allocator;
use oxc_ast::ast::Program;
use oxc_ast_visit::VisitMut;
use oxc_parser::Parser;
use oxc_semantic::{Scoping, SemanticBuilder};
use oxc_span::SourceType;
use sapling_macros::tree_builder_mut;
use sapling_shared::{
    Template, TreeBuilderMut, config::Config, processor::pre_process_ast, visitor,
};

#[tree_builder_mut(sapling_shared::TreeBuilderMut<'a>)]
struct TestVisitor<'a> {
    pub config: Config<'a>,
    pub templates: &'a mut Vec<Template<'a>>,
}

impl<'a> VisitMut<'a> for TestVisitor<'a> {
    fn visit_program(&mut self, it: &mut Program<'a>) {
        pre_process_ast(self, it, &Config::default());
    }
}

fn create_test_visitor<'a>(
    allocator: &'a Allocator,
    source: &'a str,
) -> (Scoping, oxc_ast::ast::Program<'a>) {
    let source_type = SourceType::default().with_typescript(true).with_jsx(true);

    let ret = Parser::new(allocator, source, source_type).parse();
    let program = ret.program;

    let semantic_ret = SemanticBuilder::new().build(&program);
    let scoping = semantic_ret.semantic.into_scoping();

    (scoping, program)
}

#[test]
fn test_config_merging() {
    let source = "// Empty program";
    let allocator = Allocator::default();
    let (mut scoping, mut program) = create_test_visitor(&allocator, source);
    let mut visitor = TestVisitor {
        arena: &mut Arena::new(),
        node_stack: &mut vec![],
        allocator: &allocator,
        scoping: &mut scoping,
        templates: &mut vec![],
        config: Config::default(),
    };
    visitor.visit_program(&mut program);
}

#[test]
fn test_jsx_import_source() {
    // Test with matching import source
    let source = r#"
        // @jsxImportSource sapling
        function App() {
            return <div>Hello</div>;
        }
    "#;
    let allocator = Allocator::default();
    let (mut scoping, mut program) = create_test_visitor(&allocator, source);
    let mut visitor = TestVisitor {
        arena: &mut Arena::new(),
        node_stack: &mut vec![],
        allocator: &allocator,
        scoping: &mut scoping,
        templates: &mut vec![],
        config: Config::default(),
    };
    visitor.visit_program(&mut program);

    // Test without import source comment
    let source = r#"
        function App() {
            return <div>Hello</div>;
        }
    "#;
    let allocator = Allocator::default();
    let (mut scoping, mut program) = create_test_visitor(&allocator, source);
    let mut visitor = TestVisitor {
        arena: &mut Arena::new(),
        node_stack: &mut vec![],
        allocator: &allocator,
        scoping: &mut scoping,
        templates: &mut vec![],
        config: Config::default(),
    };
    visitor.visit_program(&mut program);
}

#[test]
fn test_valid_jsx_nesting() {
    let source = r#"
        function App() {
            return (
                <div>
                    <p>Text</p>
                    <span><em>Emphasis</em></span>
                </div>
            );
        }
    "#;
    let allocator = Allocator::default();
    let (mut scoping, mut program) = create_test_visitor(&allocator, source);
    let mut visitor = TestVisitor {
        arena: &mut Arena::new(),
        node_stack: &mut vec![],
        allocator: &allocator,
        scoping: &mut scoping,
        templates: &mut vec![],
        config: Config::default(),
    };
    visitor.visit_program(&mut program);
}

#[test]
#[should_panic(expected = "Invalid JSX: <div> cannot be child of <p>")]
fn test_invalid_jsx_nesting() {
    let source = r#"
        function App() {
            return (
                <p>
                    <div>Invalid nesting</div>
                </p>
            );
        }
    "#;
    let allocator = Allocator::default();
    let (mut scoping, mut program) = create_test_visitor(&allocator, source);
    let mut visitor = TestVisitor {
        arena: &mut Arena::new(),
        node_stack: &mut vec![],
        allocator: &allocator,
        scoping: &mut scoping,
        templates: &mut vec![],
        config: Config::default(),
    };
    visitor.visit_program(&mut program);
}

#[test]
fn test_component_nesting() {
    let source = r#"
        function App() {
            return (
                <CustomComponent>
                    <div>
                        <p>Valid component nesting</p>
                    </div>
                </CustomComponent>
            );
        }
    "#;
    let allocator = Allocator::default();
    let (mut scoping, mut program) = create_test_visitor(&allocator, source);
    let mut visitor = TestVisitor {
        arena: &mut Arena::new(),
        node_stack: &mut vec![],
        allocator: &allocator,
        scoping: &mut scoping,
        templates: &mut vec![],
        config: Config::default(),
    };
    visitor.visit_program(&mut program);
}

#[test]
fn test_mixed_jsx_nesting() {
    let source = r#"
        function App() {
            return (
                <div>
                    <CustomComponent>
                        <p>Valid nesting</p>
                    </CustomComponent>
                    <p><span>Also valid</span></p>
                </div>
            );
        }
    "#;
    let allocator = Allocator::default();
    let (mut scoping, mut program) = create_test_visitor(&allocator, source);
    let mut visitor = TestVisitor {
        arena: &mut Arena::new(),
        node_stack: &mut vec![],
        allocator: &allocator,
        scoping: &mut scoping,
        templates: &mut vec![],
        config: Config::default(),
    };
    visitor.visit_program(&mut program);
}
