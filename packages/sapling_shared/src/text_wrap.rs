//! Functions for checking text node wrapping

/// Check if a node is wrapped by text nodes
pub fn wrapped_by_text(nodes: &[&str], index: usize) -> bool {
    if index >= nodes.len() {
        return false;
    }

    let has_prev_text = if index > 0 {
        is_text_node(&nodes[index - 1])
    } else {
        false
    };

    let has_next_text = if index < nodes.len() - 1 {
        is_text_node(&nodes[index + 1]) 
    } else {
        false
    };

    has_prev_text || has_next_text
}

/// Check if a node is a text node
fn is_text_node(node: &str) -> bool {
    node.trim().len() > 0 && !node.contains('<')
}
