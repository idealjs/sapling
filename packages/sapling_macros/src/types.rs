use oxc_ast::AstKind;
use indextree::{Arena, NodeId};

pub struct NodeWithChildren<'a> {
    pub data: AstKind<'a>,
    pub first_child: Option<NodeId>,
    pub next_sibling: Option<NodeId>,
}

pub struct TreeBuilder<'a> {
    pub arena: Arena<AstKind<'a>>,
    pub current_parent: Option<NodeId>,
}

impl<'a> TreeBuilder<'a> {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            current_parent: None,
        }
    }
}
