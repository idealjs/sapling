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

pub fn escape_string_for_template(s: &str) -> String {
    use lazy_static::lazy_static;
    use std::collections::HashMap;

    lazy_static! {
        static ref TEMPLATE_ESCAPES: HashMap<char, &'static str> = {
            let mut m = HashMap::new();
            m.insert('{', "\\{");
            m.insert('}', "\\}");
            m.insert('`', "\\`");
            m.insert('\\', "\\\\");
            m.insert('\n', "\\n");
            m.insert('\t', "\\t");
            m.insert('\u{0008}', "\\b"); // backspace
            m.insert('\u{000C}', "\\f"); // form feed
            m.insert('\u{000B}', "\\v"); // vertical tab
            m.insert('\r', "\\r");
            m
        };
    }

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
