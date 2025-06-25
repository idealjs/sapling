pub mod children;
pub mod component;
pub mod condition;
pub mod config;
pub mod dynamic;
pub mod id_gen;
pub mod import;
pub mod jsx_element;
pub mod length_checker;
pub mod native_spread;
pub mod string_utils;
pub mod tag_name;
pub mod text_wrap;
pub mod utils;
pub mod validate;

pub use children::*;
pub use component::*;
pub use condition::*;
pub use config::*;
pub use dynamic::*;
pub use id_gen::*;
pub use import::*;
pub use jsx_element::*;
pub use length_checker::*;
pub use native_spread::*;
pub use string_utils::*;
pub use tag_name::*;
pub use text_wrap::*;
pub use utils::*;

use lazy_static::lazy_static;
use std::collections::HashSet;
use indextree::{Arena, NodeId};
use oxc_allocator::Allocator;
use oxc_ast::{AstKind, AstType};
use oxc_ast::ast::Statement;
use oxc_ast_visit::{Visit, VisitMut};
use oxc_parser::Parser;
use oxc_span::SourceType;
use oxc_traverse::{Traverse, TraverseCtx};

lazy_static! {
    pub static ref RESERVED_NAMESPACES: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("class");
        set.insert("on");
        set.insert("oncapture");
        set.insert("style");
        set.insert("use");
        set.insert("prop");
        set.insert("attr");
        set.insert("bool");
        set
    };
}

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
