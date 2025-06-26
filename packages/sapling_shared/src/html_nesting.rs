use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref HEADINGS: HashSet<&'static str> = {
        let mut set: HashSet<&'static str> = HashSet::new();
        set.extend(["h1", "h2", "h3", "h4", "h5", "h6"]);
        set
    };

    static ref ONLY_VALID_CHILDREN: HashMap<&'static str, HashSet<&'static str>> = {
        let mut map: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();
        
        // Head elements
        let mut set: HashSet<&'static str> = HashSet::new();
        set.extend([
            "base", "basefront", "bgsound", "link", "meta", "title",
            "noscript", "noframes", "style", "script", "template"
        ]);
        map.insert("head", set);

        // Form elements
        set = HashSet::new();
        set.insert("option");
        map.insert("optgroup", set);

        set = HashSet::new();
        set.extend(["optgroup", "option", "hr", "button"]);
        map.insert("select", set);

        // Math elements
        set = HashSet::new();
        set.insert("mrow");
        map.insert("math", set);

        // Script element cannot have any children
        set = HashSet::new();
        map.insert("script", set);

        // Table elements
        set = HashSet::new();
        set.extend(["caption", "colgroup", "tbody", "tfoot", "thead"]);
        map.insert("table", set);

        set = HashSet::<&'static str>::new();
        set.extend(["td", "th"]);
        map.insert("tr", set);

        set = HashSet::<&'static str>::new();
        set.insert("col");
        map.insert("colgroup", set);

        set = HashSet::<&'static str>::new();
        set.insert("tr");
        map.insert("tbody", set);

        set = HashSet::<&'static str>::new();
        set.insert("tr");
        map.insert("thead", set);

        set = HashSet::<&'static str>::new();
        set.insert("tr");
        map.insert("tfoot", set);

        // Elements that cannot have any children
        let empty_set: HashSet<&'static str> = HashSet::new();
        map.insert("iframe", empty_set.clone());
        map.insert("option", empty_set.clone());
        map.insert("textarea", empty_set.clone());
        map.insert("style", empty_set.clone());
        map.insert("title", empty_set);

        map
    };

    static ref ONLY_VALID_PARENTS: HashMap<&'static str, HashSet<&'static str>> = {
        let mut map: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();

        // Sections
        let empty_set: HashSet<&'static str> = HashSet::new();
        map.insert("html", empty_set);

        let mut set: HashSet<&'static str> = HashSet::new();
        set.insert("html");
        map.insert("body", set.clone());
        map.insert("head", set);

        // Table elements
        set = HashSet::new();
        set.insert("tr");
        map.insert("td", set.clone());
        map.insert("th", set);

        set = HashSet::new();
        set.insert("table");
        map.insert("colgroup", set.clone());
        map.insert("caption", set.clone());
        map.insert("tbody", set.clone());
        map.insert("tfoot", set.clone());
        map.insert("thead", set);

        set = HashSet::new();
        set.insert("colgroup");
        map.insert("col", set);

        set = HashSet::new();
        set.extend(["tbody", "thead", "tfoot"]);
        map.insert("tr", set);

        // Data list elements
        set = HashSet::new();
        set.extend(["dl", "div"]);
        map.insert("dd", set.clone());
        map.insert("dt", set);

        // Other elements
        set = HashSet::new();
        set.insert("figure");
        map.insert("figcaption", set);

        set = HashSet::new();
        set.insert("details");
        map.insert("summary", set);

        set = HashSet::new();
        set.insert("map");
        map.insert("area", set);

        map
    };

    static ref KNOWN_INVALID_CHILDREN: HashMap<&'static str, HashSet<&'static str>> = {
        let mut map: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();

        // Paragraph invalid children
        let mut set: HashSet<&'static str> = HashSet::new();
        set.extend([
            "address", "article", "aside", "blockquote", "center", "details",
            "dialog", "dir", "div", "dl", "fieldset", "figure", "footer", "form",
            "h1", "h2", "h3", "h4", "h5", "h6", "header", "hgroup", "hr", "li",
            "main", "nav", "menu", "ol", "p", "pre", "section", "table", "ul"
        ]);
        map.insert("p", set);

        // SVG invalid children
        set = HashSet::new();
        set.extend([
            "b", "blockquote", "br", "code", "dd", "div", "dl", "dt", "em",
            "embed", "h1", "h2", "h3", "h4", "h5", "h6", "hr", "i", "img",
            "li", "menu", "meta", "ol", "p", "pre", "ruby", "s", "small",
            "span", "strong", "sub", "sup", "table", "u", "ul", "var"
        ]);
        map.insert("svg", set);

        map
    };

    static ref KNOWN_INVALID_PARENTS: HashMap<&'static str, HashSet<&'static str>> = {
        let mut map: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();

        // Self-nesting restrictions
        let mut set: HashSet<&'static str> = HashSet::new();
        set.insert("a");
        map.insert("a", set);

        set = HashSet::new();
        set.insert("button");
        map.insert("button", set);

        set = HashSet::new();
        set.extend(["dd", "dt"]);
        map.insert("dd", set.clone());
        map.insert("dt", set);

        set = HashSet::new();
        set.insert("form");
        map.insert("form", set);

        set = HashSet::new();
        set.insert("li");
        map.insert("li", set);

        // Heading nesting restrictions
        let headings = HEADINGS.clone();
        map.insert("h1", headings.clone());
        map.insert("h2", headings.clone());
        map.insert("h3", headings.clone());
        map.insert("h4", headings.clone());
        map.insert("h5", headings.clone());
        map.insert("h6", headings);

        map
    };
}

/// Returns true if the given parent-child HTML element nesting is valid
///
/// # Arguments
///
/// * `parent` - The parent HTML element tag name
/// * `child` - The child HTML element tag name
///
/// # Examples
///
/// ```
/// use sapling_shared::html_nesting::is_valid_html_nesting;
///
/// assert!(is_valid_html_nesting("div", "p"));
/// assert!(!is_valid_html_nesting("p", "div")); 
/// ```
pub fn is_valid_html_nesting(parent: &str, child: &str) -> bool {
    // Check if there is a list of only valid children for the parent
    if let Some(valid_children) = ONLY_VALID_CHILDREN.get(parent) {
        return valid_children.contains(child);
    }

    // Check if there is a list of only valid parents for the child
    if let Some(valid_parents) = ONLY_VALID_PARENTS.get(child) {
        return valid_parents.contains(parent);
    }

    // Check if the child is in the list of known invalid children for the parent
    if let Some(invalid_children) = KNOWN_INVALID_CHILDREN.get(parent) {
        if invalid_children.contains(child) {
            return false;
        }
    }

    // Check if the parent is in the list of known invalid parents for the child
    if let Some(invalid_parents) = KNOWN_INVALID_PARENTS.get(child) {
        if invalid_parents.contains(parent) {
            return false;
        }
    }

    // If no restrictions are found, the nesting is valid
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_children() {
        assert!(is_valid_html_nesting("head", "title"));
        assert!(is_valid_html_nesting("select", "option"));
        assert!(is_valid_html_nesting("table", "tbody"));
        assert!(is_valid_html_nesting("tr", "td"));
    }

    #[test]
    fn test_valid_parents() {
        assert!(is_valid_html_nesting("html", "head"));
        assert!(is_valid_html_nesting("table", "thead"));
        assert!(is_valid_html_nesting("tbody", "tr"));
        assert!(is_valid_html_nesting("figure", "figcaption"));
    }

    #[test]
    fn test_invalid_children() {
        assert!(!is_valid_html_nesting("p", "div"));
        assert!(!is_valid_html_nesting("svg", "p"));
        assert!(!is_valid_html_nesting("p", "h1"));
    }

    #[test]
    fn test_invalid_parents() {
        assert!(!is_valid_html_nesting("a", "a"));
        assert!(!is_valid_html_nesting("button", "button"));
        assert!(!is_valid_html_nesting("form", "form"));
        assert!(!is_valid_html_nesting("h1", "h2"));
    }

    #[test]
    fn test_unrestricted_nesting() {
        assert!(is_valid_html_nesting("div", "span"));
        assert!(is_valid_html_nesting("span", "a"));
        assert!(is_valid_html_nesting("div", "p"));
    }
}
