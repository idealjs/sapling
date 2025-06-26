//! Functions for handling JSX children elements

/// Filter JSX child nodes based on validity criteria
pub fn filter_children(children: &[&str]) -> Vec<String> {
    children
        .iter()
        .filter(|&child| {
            // Remove empty text nodes
            let trimmed = child.trim();
            !trimmed.is_empty() && !is_comment(trimmed)
        })
        .map(|&child| child.to_string())
        .collect()
}

/// Check if a string is a comment
fn is_comment(text: &str) -> bool {
    text.starts_with("<!--") && text.ends_with("-->")
}

/// Check length of children array
pub fn check_length(children: &[&str]) -> usize {
    filter_children(children).len()
}
