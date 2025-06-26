//! Functions for handling events in JSX

/// Detect if event handler is resolvable
pub fn detect_resolvable_event_handler() -> Result<(), &'static str> {
    todo!("Implement detection of resolvable event handlers")
}

/// Convert event name to proper format
pub fn to_event_name() -> Result<(), &'static str> {
    todo!("Implement conversion of event names to proper format")
}

/// Convert attribute name to event handler format
pub fn to_attribute_name() -> Result<(), &'static str> {
    todo!("Implement conversion of attribute names to event handler format")
}

/// Convert property name to camelCase
///
/// Converts a property name from kebab-case to camelCase
///
/// # Examples
///
/// ```
/// use sapling_shared::event_handler::to_property_name;
/// let result = to_property_name("background-color");
/// assert_eq!(result.unwrap(), "backgroundColor");
/// ```
pub fn to_property_name(name: &str) -> Result<String, &'static str> {
    let mut result = String::with_capacity(name.len());
    let mut capitalize_next = false;

    for c in name.chars() {
        if c == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c.to_ascii_lowercase());
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_property_name() {
        let test_cases = vec![
            ("background-color", "backgroundColor"),
            ("font-size", "fontSize"),
            ("simple", "simple"),
            ("with-multiple-dashes", "withMultipleDashes"),
            ("already-camelCase", "alreadyCamelcase"),
            ("UPPER-CASE", "upperCase"),
            ("", ""),
        ];

        for (input, expected) in test_cases {
            assert_eq!(to_property_name(input).unwrap(), expected);
        }
    }
}
