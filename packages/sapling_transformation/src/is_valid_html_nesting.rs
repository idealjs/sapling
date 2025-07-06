// packages/sapling_transformation/src/is_valid_html_nesting.rs

use std::collections::{HashMap, HashSet};

fn get_only_valid_children() -> HashMap<&'static str, HashSet<&'static str>> {
    let mut m = HashMap::new();
    m.insert("head", HashSet::from([
        "base", "basefront", "bgsound", "link", "meta", "title", "noscript", "noframes", "style", "script", "template"
    ]));
    m.insert("optgroup", HashSet::from(["option"]));
    m.insert("select", HashSet::from(["optgroup", "option", "hr", "button"]));
    m.insert("math", HashSet::from(["mrow"]));
    m.insert("script", HashSet::new());
    m.insert("table", HashSet::from(["caption", "colgroup", "tbody", "tfoot", "thead"]));
    m.insert("tr", HashSet::from(["td", "th"]));
    m.insert("colgroup", HashSet::from(["col"]));
    m.insert("tbody", HashSet::from(["tr"]));
    m.insert("thead", HashSet::from(["tr"]));
    m.insert("tfoot", HashSet::from(["tr"]));
    m.insert("iframe", HashSet::new());
    m.insert("option", HashSet::new());
    m.insert("textarea", HashSet::new());
    m.insert("style", HashSet::new());
    m.insert("title", HashSet::new());
    m
}

fn get_only_valid_parents() -> HashMap<&'static str, HashSet<&'static str>> {
    let mut m = HashMap::new();
    m.insert("html", HashSet::new());
    m.insert("body", HashSet::from(["html"]));
    m.insert("head", HashSet::from(["html"]));
    m.insert("td", HashSet::from(["tr"]));
    m.insert("colgroup", HashSet::from(["table"]));
    m.insert("caption", HashSet::from(["table"]));
    m.insert("tbody", HashSet::from(["table"]));
    m.insert("tfoot", HashSet::from(["table"]));
    m.insert("col", HashSet::from(["colgroup"]));
    m.insert("th", HashSet::from(["tr"]));
    m.insert("thead", HashSet::from(["table"]));
    m.insert("tr", HashSet::from(["tbody", "thead", "tfoot"]));
    m.insert("dd", HashSet::from(["dl", "div"]));
    m.insert("dt", HashSet::from(["dl", "div"]));
    m.insert("figcaption", HashSet::from(["figure"]));
    m.insert("summary", HashSet::from(["details"]));
    m.insert("area", HashSet::from(["map"]));
    m
}

fn get_known_invalid_children() -> HashMap<&'static str, HashSet<&'static str>> {
    let mut m = HashMap::new();
    m.insert("p", HashSet::from([
        "address", "article", "aside", "blockquote", "center", "details", "dialog", "dir", "div", "dl", "fieldset",
        "figure", "footer", "form", "h1", "h2", "h3", "h4", "h5", "h6", "header", "hgroup", "hr", "li", "main", "nav",
        "menu", "ol", "p", "pre", "section", "table", "ul"
    ]));
    m.insert("svg", HashSet::from([
        "b", "blockquote", "br", "code", "dd", "div", "dl", "dt", "em", "embed", "h1", "h2", "h3", "h4", "h5", "h6",
        "hr", "i", "img", "li", "menu", "meta", "ol", "p", "pre", "ruby", "s", "small", "span", "strong", "sub", "sup",
        "table", "u", "ul", "var"
    ]));
    m
}

fn get_known_invalid_parents() -> HashMap<&'static str, HashSet<&'static str>> {
    let headings: HashSet<&'static str> = HashSet::from(["h1", "h2", "h3", "h4", "h5", "h6"]);
    let mut m = HashMap::new();
    m.insert("a", HashSet::from(["a"]));
    m.insert("button", HashSet::from(["button"]));
    m.insert("dd", HashSet::from(["dd", "dt"]));
    m.insert("dt", HashSet::from(["dd", "dt"]));
    m.insert("form", HashSet::from(["form"]));
    m.insert("li", HashSet::from(["li"]));
    m.insert("h1", headings.clone());
    m.insert("h2", headings.clone());
    m.insert("h3", headings.clone());
    m.insert("h4", headings.clone());
    m.insert("h5", headings.clone());
    m.insert("h6", headings);
    m
}

/// 判断给定的 parent-child 嵌套是否为有效 HTML
pub fn is_valid_html_nesting(parent: &str, child: &str) -> bool {
    let only_valid_children = get_only_valid_children();
    let only_valid_parents = get_only_valid_parents();
    let known_invalid_children = get_known_invalid_children();
    let known_invalid_parents = get_known_invalid_parents();

    if let Some(valid_children) = only_valid_children.get(parent) {
        return valid_children.contains(child);
    }
    if let Some(valid_parents) = only_valid_parents.get(child) {
        return valid_parents.contains(parent);
    }
    if let Some(invalid_children) = known_invalid_children.get(parent) {
        if invalid_children.contains(child) {
            return false;
        }
    }
    if let Some(invalid_parents) = known_invalid_parents.get(child) {
        if invalid_parents.contains(parent) {
            return false;
        }
    }
    true
}