pub fn is_component(tag_name: &str) -> bool {
    let first_char = tag_name.chars().next();
    first_char.map_or(false, |c| {
        c.is_uppercase() || tag_name.contains('.') || !c.is_ascii_alphabetic()
    })
}
