use indextree::{Arena, NodeId};
use oxc_allocator::Allocator;
use oxc_ast::AstKind;
use oxc_ast_visit::Visit;
use oxc_parser::Parser;
use oxc_span::SourceType;
use sapling_macros::tree_builder;

#[tree_builder]
struct TestTreeBuilder<'a> {}

impl<'a> Visit<'a> for TestTreeBuilder<'a> {
    fn enter_node(&mut self, kind: AstKind<'a>) {}
    fn leave_node(&mut self, kind: AstKind<'a>) {}
}

#[cfg(test)]
mod tests {
    use oxc_semantic::SemanticBuilder;

    use super::*;

    #[test]
    fn test_enter_node_execution() {
        // Initialize allocator and source type
        let allocator = Allocator::default();
        let source_type = SourceType::default();

        // Create a simple program to parse
        let source = "let x = 1;";

        // Parse the source code
        let ret = Parser::new(&allocator, source, source_type).parse();
        let program = ret.program;

        let semantic_ret = SemanticBuilder::new().build(&program);
        let scoping = semantic_ret.semantic.into_scoping();

        // Create test tree builder
        let mut tree_builder = TestTreeBuilder {
            arena: Arena::new(),
            node_stack: vec![],
            allocator: &allocator,
            scoping: &scoping,
        };

        // This will trigger enter_node through the visit trait
        tree_builder.visit_program(&program);
    }
}
