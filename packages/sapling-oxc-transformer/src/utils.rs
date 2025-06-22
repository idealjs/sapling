use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

lazy_static! {
    pub static ref RESERVED_NAMESPACES: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("class");
        set.insert("on");
        set.insert("oncapture");
        set.insert("style");
        set.insert("use");
        set.insert("prop");
        set.insert("attr");
        set.insert("bool");
        set
    };
    pub static ref NON_SPREAD_NAMESPACES: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("class");
        set.insert("style");
        set.insert("use");
        set.insert("prop");
        set.insert("attr");
        set.insert("bool");
        set
    };
}

pub fn is_component(tag_name: &str) -> bool {
    let first_char = tag_name.chars().next();
    first_char.map_or(false, |c| {
        c.is_uppercase() || tag_name.contains('.') || !c.is_ascii_alphabetic()
    })
}

pub fn trim_whitespace(text: &str) -> String {
    let text = text.replace('\r', "");
    if text.contains('\n') {
        text.split('\n')
            .enumerate()
            .map(|(i, t)| if i > 0 { t.trim_start() } else { t })
            .filter(|s| !s.trim().is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    }
}

pub fn to_event_name(name: &str) -> String {
    name[2..].to_lowercase()
}

pub fn to_attribute_name(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    for c in name.chars() {
        if c.is_uppercase() {
            result.push('-');
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

pub fn to_property_name(name: &str) -> String {
    name.to_lowercase()
        .split('-')
        .enumerate()
        .map(|(i, part)| {
            if i == 0 {
                part.to_string()
            } else {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first_char) => first_char.to_uppercase().chain(chars).collect::<String>(),
                }
            }
        })
        .collect()
}

pub fn wrapped_by_text(list: &[Option<TextNode>], start_index: usize) -> bool {
    let mut index = start_index;
    let mut wrapped = false;

    while index > 0 {
        index -= 1;
        if let Some(Some(node)) = list.get(index) {
            if node.text.is_some() {
                wrapped = true;
                break;
            }
            if node.id.is_some() {
                return false;
            }
        }
    }

    if !wrapped {
        return false;
    }

    index = start_index;
    while index < list.len() {
        if let Some(Some(node)) = list.get(index) {
            if node.text.is_some() {
                return true;
            }
            if node.id.is_some() {
                return false;
            }
        }
        index += 1;
    }
    false
}

pub fn escape_html(s: &str, attr: bool) -> String {
    if s.is_empty() {
        return String::new();
    }

    let delim = if attr { '"' } else { '<' };
    let esc_delim = if attr { "&quot;" } else { "&lt;" };

    if !s.contains(delim) && !s.contains('&') {
        return s.to_string();
    }

    let mut result = String::with_capacity(s.len());
    let mut last_pos = 0;
    let mut chars = s.char_indices();

    while let Some((i, c)) = chars.next() {
        match c {
            '&' => {
                result.push_str(&s[last_pos..i]);
                result.push_str("&amp;");
                last_pos = i + 1;
            }
            c if c == delim => {
                result.push_str(&s[last_pos..i]);
                result.push_str(esc_delim);
                last_pos = i + 1;
            }
            _ => {}
        }
    }

    if last_pos < s.len() {
        result.push_str(&s[last_pos..]);
    }

    result
}

pub fn can_native_spread(key: &str, check_namespaces: bool) -> bool {
    if check_namespaces && key.contains(':') {
        let namespace = key.split(':').next().unwrap();
        if NON_SPREAD_NAMESPACES.contains(namespace) {
            return false;
        }
    }
    key != "ref"
}

const CHARS: &str = "etaoinshrdlucwmfygpbTAOISWCBvkxjqzPHFMDRELNGUKVYJQZX_$";
const BASE: u32 = CHARS.len() as u32;

pub fn get_numbered_id(mut num: u32) -> String {
    let mut result = String::new();
    let chars: Vec<char> = CHARS.chars().collect();

    loop {
        let digit = (num % BASE) as usize;
        num = num / BASE;
        result.insert(0, chars[digit]);
        if num == 0 {
            break;
        }
    }

    result
}

pub fn filter_children(children: &[JsxChild]) -> Vec<&JsxChild> {
    children
        .iter()
        .filter(|child| match child {
            JsxChild::ExpressionContainer(expr) => !matches!(expr, JsxExpression::Empty),
            JsxChild::Text(text) => !text.value.trim().is_empty(),
            _ => true,
        })
        .collect()
}

pub fn check_length(children: &[JsxChild]) -> bool {
    let mut count = 0;
    for child in children {
        match child {
            JsxChild::ExpressionContainer(expr) => {
                if !matches!(expr, JsxExpression::Empty) {
                    count += 1;
                }
            }
            JsxChild::Text(text) => {
                if !text.value.trim().is_empty() {
                    count += 1;
                }
            }
            _ => count += 1,
        }
        if count > 1 {
            return true;
        }
    }
    false
}

lazy_static! {
    pub static ref TEMPLATE_ESCAPES: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        m.insert('{', "\\{");
        m.insert('}', "\\}");
        m.insert('`', "\\`");
        m.insert('\\', "\\\\");
        m.insert('\n', "\\n");
        m.insert('\t', "\\t");
        m.insert('\u{0008}', "\\b");
        m.insert('\u{000C}', "\\f");
        m.insert('\u{000B}', "\\v");
        m.insert('\r', "\\r");
        m
    };
}

pub fn escape_string_for_template(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        if let Some(escaped) = TEMPLATE_ESCAPES.get(&c) {
            result.push_str(escaped);
        } else {
            result.push(c);
        }
    }
    result
}

#[derive(Debug, Clone, PartialEq)]
pub enum JsxChild {
    Element(JsxElement),
    ExpressionContainer(JsxExpression),
    Text(JsxText),
    Fragment(JsxFragment),
}

#[derive(Debug, Clone, PartialEq)]
pub struct JsxElement {
    pub tag: String,
    pub attributes: Vec<JsxAttribute>,
    pub children: Vec<JsxChild>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JsxExpression {
    Empty,
    Expression(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct JsxText {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JsxFragment {
    pub children: Vec<JsxChild>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JsxAttribute {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TextNode {
    pub text: Option<String>,
    pub id: Option<String>,
}
