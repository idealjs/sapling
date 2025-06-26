use indextree::{Arena, NodeId};
use oxc_ast::{AstKind, AstType};
use oxc_ast_visit::{Visit, VisitMut};

pub trait TreeBuilder<'a>: Visit<'a> {
    fn arena(&self) -> &Arena<AstKind<'a>>;
    fn arena_mut(&mut self) -> &mut Arena<AstKind<'a>>;
    fn node_stack(&self) -> &Vec<NodeId>;
    fn node_stack_mut(&mut self) -> &mut Vec<NodeId>;
    fn current_parent(&self) -> Option<&NodeId> {
        self.node_stack().last()
    }
    fn push_parent(&mut self, node_id: NodeId) {
        self.node_stack_mut().push(node_id);
    }
    fn pop_parent(&mut self) -> Option<NodeId> {
        self.node_stack_mut().pop()
    }
    fn enter_node(&mut self, kind: AstKind<'a>) {
        let node_id = self.arena_mut().new_node(kind);
        if let Some(parent) = self.current_parent() {
            parent.append(node_id, self.arena_mut());
        }
        self.push_parent(node_id);
    }
    fn leave_node(&mut self, _: AstKind<'a>) {
        self.pop_parent();
    }
}

pub trait TreeBuilderMut<'a>: VisitMut<'a> {
    fn arena(&self) -> &Arena<AstType>;
    fn arena_mut(&mut self) -> &mut Arena<AstType>;
    fn node_stack(&self) -> &Vec<NodeId>;
    fn node_stack_mut(&mut self) -> &mut Vec<NodeId>;
    fn current_parent(&self) -> Option<&NodeId> {
        self.node_stack().last()
    }
    fn push_parent(&mut self, node_id: NodeId) {
        self.node_stack_mut().push(node_id);
    }
    fn pop_parent(&mut self) -> Option<NodeId> {
        self.node_stack_mut().pop()
    }
    fn enter_node(&mut self, kind: AstType) {
        let node_id = self.arena_mut().new_node(kind);
        if let Some(parent) = self.current_parent() {
            parent.append(node_id, self.arena_mut());
        }
        self.push_parent(node_id);
    }
    fn leave_node(&mut self, _: AstType) {
        self.pop_parent();
    }
}
