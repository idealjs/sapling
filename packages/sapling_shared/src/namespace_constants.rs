//! Constants for XML namespaces and reserved names

/// Reserved XML namespaces that should be handled specially
pub const RESERVED_NAMESPACES: [&str; 5] = [
    "xmlns",       // XML namespace
    "xml",         // XML prefix
    "svg",         // SVG namespace
    "math",        // MathML namespace
    "xlink",       // XLink namespace
];

/// SVG namespace URI
pub const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

/// MathML namespace URI
pub const MATHML_NAMESPACE: &str = "http://www.w3.org/1998/Math/MathML";

/// XLink namespace URI
pub const XLINK_NAMESPACE: &str = "http://www.w3.org/1999/xlink";

/// XML namespace URI
pub const XML_NAMESPACE: &str = "http://www.w3.org/XML/1998/namespace";

/// XMLNS namespace URI
pub const XMLNS_NAMESPACE: &str = "http://www.w3.org/2000/xmlns/";
