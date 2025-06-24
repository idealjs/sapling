use indextree::Arena;
use indextree::Node;
use oxc_ast::AstKind;
use oxc_ast::ast::*;

use crate::is_component;

pub fn get_static_expression<'a>(node: Node<AstKind<'a>>, arena: Arena<AstKind<'a>>) -> bool {
    let node_data = node.get();
    let parent = Node::parent(&node);
    if let Some(parent_node_id) = parent {
        if let Some(parent_node) = arena.get(parent_node_id) {
            if let (
                AstKind::JSXExpressionContainer(container),
                AstKind::JSXElement(parent_element),
            ) = (node_data, parent_node.get())
            {
                // Check if parent is not a component
                if let JSXElementName::Identifier(ident) = &parent_element.opening_element.name {
                    if !is_component(&ident.name) {
                        // Check not sequence expression
                        if let JSXExpression::SequenceExpression(_) = container.expression {
                            return false;
                        }

                        // For now we consider literals as static
                        return match container.expression {
                            JSXExpression::StringLiteral(_) | JSXExpression::NullLiteral(_) => true,
                            JSXExpression::NumericLiteral(_) => true,
                            JSXExpression::BooleanLiteral(_) => true,
                            JSXExpression::BigIntLiteral(_) => true,
                            _ => return false,
                        };
                    }
                }
            }
        }
    }

    false
}
