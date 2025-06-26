//! Functions for checking array lengths and child node counts

/// Check length of children array
pub fn check_length(children: &[&str]) -> usize {
    children
        .iter()
        .filter(|&child| {
            let trimmed = child.trim();
            !trimmed.is_empty() && !trimmed.starts_with("<!--")
        })
        .count()
}
