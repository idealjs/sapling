pub fn is_component(tag_name: &str) -> bool {
    if let Some(first) = tag_name.chars().next() {
        first.to_lowercase().to_string() != first.to_string()
            || tag_name.contains('.')
            || !first.is_ascii_alphabetic()
    } else {
        false
    }
}
