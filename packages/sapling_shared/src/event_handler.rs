//! Functions for handling events in JSX

use oxc_ast::{AstBuilder, AstKind, ast::Expression};
use oxc_semantic::AstNode;
use oxc_syntax::{reference, scope::ScopeId};
use oxc_traverse::TraverseCtx;

use crate::TreeBuilderMut;

pub fn detect_resolvable_event_handler<'a>(
    visitor: &mut impl TreeBuilderMut<'a>,
    handler: &Expression<'a>,
) -> bool {
    let scoping = visitor.scoping_mut();

    match handler {
        Expression::Identifier(ident) => {
            let reference_id = ident.reference_id();
            let reference = scoping.get_reference(reference_id);
            let node_id = reference.node_id();
            let node = visitor.semantic_mut().nodes().get_node(node_id);
            match node.kind() {
                AstKind::VariableDeclarator(declarator) => {
                    match &declarator.init {
                        Some(init) => {
                            // Check if the initializer is a function expression or arrow function
                            match init {
                                Expression::FunctionExpression(_) => true,
                                Expression::ArrowFunctionExpression(_) => true,
                                _ => false,
                            }
                        }
                        None => false, // No initializer means it's not resolvable
                    }
                }
                AstKind::Function(_) => true,
                AstKind::ArrowFunctionExpression(_) => true,
                _ => false,
            }
        }
        Expression::FunctionExpression(_) | Expression::ArrowFunctionExpression(_) => true,
        _ => false,
    }
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
