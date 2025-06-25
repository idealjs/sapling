use indextree::{Arena, NodeId};
use oxc_allocator::Allocator;
use oxc_ast::AstKind;
use oxc_ast_visit::Visit;
use oxc_parser::Parser;
use oxc_span::SourceType;
use sapling_macros::tree_builder;

#[tree_builder]
// #[derive(TreeBuilder)]
struct TestTreeBuilder<'a> {}

// impl<'a> TreeBuilder<'a> for TestTreeBuilder<'a> {
//     fn arena(&self) -> &Arena<AstKind<'a>> {
//         &self.arena
//     }
//     fn arena_mut(&mut self) -> &mut Arena<AstKind<'a>> {
//         &mut self.arena
//     }
//     fn node_stack(&self) -> &Vec<NodeId> {
//         &self.node_stack
//     }
//     fn node_stack_mut(&mut self) -> &mut Vec<NodeId> {
//         &mut self.node_stack
//     }
// }

// impl<'a> Visit<'a> for TestTreeBuilder<'a> {
//     fn enter_node(&mut self, kind: AstKind<'a>) {
//         <Self as TreeBuilder>::enter_node(self, kind);
//     }
//     fn leave_node(&mut self, kind: AstKind<'a>) {
//         <Self as TreeBuilder>::leave_node(self, kind);
//     }
// }

#[cfg(test)]
mod tests {
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

        // Create test tree builder
        let mut tree_builder = TestTreeBuilder {
            arena: Arena::new(),
            node_stack: vec![],
        };

        // This will trigger enter_node through the visit trait
        tree_builder.visit_program(&program);
    }
}
