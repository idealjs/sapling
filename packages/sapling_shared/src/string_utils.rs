//! String manipulation utilities for JSX transformation

/// Escape HTML special characters
pub fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Trim whitespace from text content
pub fn trim_whitespace(input: &str) -> String {
    input.trim().to_string()
}

/// Escape special characters in template strings
pub fn escape_string_for_template(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('`', "\\`")
        .replace('$', "\\$")
}
