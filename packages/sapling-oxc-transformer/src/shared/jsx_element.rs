// use oxc_ast::{ast::*, AstPath};
// use oxc_span::{Atom, Span};

// #[derive(Debug, Clone)]
// pub enum JsxElementName {
//     Identifier(Ident),
//     MemberExpression(JsxMemberExpression),
//     NamespacedName(JsxNamespacedName),
// }

// #[derive(Debug, Clone)]
// pub struct JsxMemberExpression {
//     pub object: Box<JsxElementName>,
//     pub property: Ident,
// }

// #[derive(Debug, Clone)]
// pub struct JsxNamespacedName {
//     pub namespace: Ident,
//     pub name: Ident,
// }

// pub fn jsx_element_name_to_string(node: &JsxElementName) -> String {
//     match node {
//         JsxElementName::MemberExpression(expr) => {
//             format!("{}.{}", 
//                 jsx_element_name_to_string(&expr.object),
//                 expr.property.name.to_string()
//             )
//         },
//         JsxElementName::Identifier(ident) => ident.name.to_string(),
//         JsxElementName::NamespacedName(ns) => {
//             format!("{}:{}", ns.namespace.name, ns.name.name)
//         }
//     }
// }

// pub fn tag_name_to_identifier(name: &str) -> Expression {
//     let parts: Vec<&str> = name.split('.').collect();
//     if parts.len() == 1 {
//         Expression::Identifier(Ident {
//             span: Span::default(),
//             name: Atom::from(parts[0]),
//             optional: false,
//         })
//     } else {
//         let mut base = Expression::Identifier(Ident {
//             span: Span::default(),
//             name: Atom::from(parts[0]),
//             optional: false,
//         });
//         for part in parts.iter().skip(1) {
//             base = Expression::MemberExpression(Box::new(MemberExpression {
//                 span: Span::default(),
//                 object: Box::new(base),
//                 property: MemberProperty::Ident(Ident {
//                     span: Span::default(),
//                     name: Atom::from(*part),
//                     optional: false,
//                 }),
//                 computed: false,
//             }));
//         }
//         base
//     }
// }

// pub fn get_tag_name(tag: &JSXElement) -> String {
//     jsx_element_name_to_string(&tag.opening_element.name)
// }

// pub fn is_component(tag_name: &str) -> bool {
//     let first_char = tag_name.chars().next();
//     first_char.map_or(false, |c| {
//         c.is_uppercase() || tag_name.contains('.') || !c.is_ascii_alphabetic()
//     })
// }

// pub fn convert_jsx_identifier(node: &JsxElementName) -> Expression {
//     match node {
//         JsxElementName::Identifier(ident) if is_valid_identifier(&ident.name) => {
//             Expression::Identifier(ident.clone())
//         },
//         JsxElementName::Identifier(ident) => {
//             Expression::StringLiteral(Box::new(StringLiteral {
//                 span: Span::default(),
//                 value: ident.name.clone(),
//                 raw: None,
//             }))
//         },
//         JsxElementName::MemberExpression(member) => {
//             Expression::MemberExpression(Box::new(MemberExpression {
//                 span: Span::default(),
//                 object: Box::new(convert_jsx_identifier(&member.object)),
//                 property: MemberProperty::Ident(member.property.clone()),
//                 computed: false,
//             }))
//         },
//         JsxElementName::NamespacedName(ns) => {
//             Expression::StringLiteral(Box::new(StringLiteral {
//                 span: Span::default(),
//                 value: Atom::from(format!("{}:{}", ns.namespace.name, ns.name.name)),
//                 raw: None,
//             }))
//         }
//     }
// }

// fn is_valid_identifier(name: &Atom) -> bool {
//     // Simple validation - can be expanded based on JavaScript identifier rules
//     let name = name.as_str();
//     !name.is_empty() && 
//     name.chars().next().unwrap().is_ascii_alphabetic() &&
//     name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
// }
