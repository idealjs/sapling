//! Functions for handling JSX element tag names

/// Convert JSX element name to string representation
pub fn jsx_element_name_to_string(element_name: &str) -> String {
    // Remove namespace prefixes if present
    if let Some(idx) = element_name.rfind(':') {
        element_name[idx + 1..].to_string()
    } else {
        element_name.to_string()
    }
}

/// Convert tag name to valid identifier
pub fn tag_name_to_identifier(tag_name: &str) -> String {
    tag_name
        .replace('-', "_")
        .replace(':', "__")
        .replace('.', "_dot_")
}

/// Get normalized tag name from element
pub fn get_tag_name(element_name: &str) -> String {
    let name = jsx_element_name_to_string(element_name);
    name.to_lowercase()
}
