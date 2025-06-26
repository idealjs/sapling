use indextree::Arena;
use indextree::Node;
use oxc_ast::AstKind;
use oxc_ast::ast::*;

use crate::is_component;
use crate::get_tag_name;

pub fn get_static_expression<'a>(node: &Node<AstKind<'a>>, arena: &Arena<AstKind<'a>>) -> bool {
    let node_data = node.get();
    let parent = Node::parent(&node);

    if let Some(parent_node_id) = parent {
        if let Some(parent_node) = arena.get(parent_node_id) {
            if let (
                AstKind::JSXExpressionContainer(container),
                AstKind::JSXElement(parent_element),
            ) = (node_data, parent_node.get())
            {
                if !is_component(get_tag_name(parent_element).as_str()) {
                    // Check not sequence expression
                    if let JSXExpression::SequenceExpression(_) = container.expression {
                        return false;
                    }

                    // For now we consider literals as static
                    return match container.expression {
                        JSXExpression::StringLiteral(_)
                        | JSXExpression::NullLiteral(_)
                        | JSXExpression::NumericLiteral(_)
                        | JSXExpression::BooleanLiteral(_)
                        | JSXExpression::BigIntLiteral(_) => true,
                        _ => false,
                    };
                }
            }
        }
    }

    false
}
