pub fn is_custom_component(tag_name: &str) -> bool {
    tag_name.chars().next().map(|c| c.is_ascii_uppercase()).unwrap_or(false)
}