//! Constants for HTML elements categorization and behavior

/// List of HTML inline elements
pub static INLINE_ELEMENTS: [&str; 53] = [
    "a", "abbr", "acronym", "b", "bdi", "bdo", "big", "br", "button",
    "canvas", "cite", "code", "data", "datalist", "del", "dfn", "em",
    "embed", "i", "iframe", "img", "input", "ins", "kbd", "label", "map",
    "mark", "meter", "noscript", "object", "output", "picture", "progress",
    "q", "ruby", "s", "samp", "script", "select", "slot", "small", "span",
    "strong", "sub", "sup", "svg", "template", "textarea", "time", "u",
    "tt", "var", "video"
];

/// List of HTML block elements
pub static BLOCK_ELEMENTS: [&str; 34] = [
    "address", "article", "aside", "blockquote", "dd", "details", "dialog",
    "div", "dl", "dt", "fieldset", "figcaption", "figure", "footer", "form",
    "h1", "h2", "h3", "h4", "h5", "h6", "header", "hgroup", "hr", "li",
    "main", "menu", "nav", "ol", "p", "pre", "section", "table", "ul"
];

/// List of HTML elements that must be self-closed
pub static VOID_ELEMENTS: [&str; 16] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "keygen",
    "link", "menuitem", "meta", "param", "source", "track", "wbr"
];

/// Body element placeholder for innerHTML context
pub static BODY_ELEMENT: &str = "body";
