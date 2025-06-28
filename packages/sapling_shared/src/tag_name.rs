use oxc_ast::ast::{
    JSXElement, JSXElementName,
    JSXMemberExpressionObject,
};

fn jsx_member_expression_object_to_string(obj: &JSXMemberExpressionObject) -> String {
    match obj {
        JSXMemberExpressionObject::IdentifierReference(ident) => ident.name.to_string(),
        JSXMemberExpressionObject::MemberExpression(expr) => {
            format!(
                "{}.{}",
                jsx_member_expression_object_to_string(&expr.object),
                expr.property.name.to_string()
            )
        }
        JSXMemberExpressionObject::ThisExpression(_) => "this".to_string(),
    }
}

pub fn jsx_element_name_to_string(node: &JSXElementName) -> String {
    match node {
        JSXElementName::MemberExpression(expr) => {
            format!(
                "{}.{}",
                jsx_member_expression_object_to_string(&expr.object),
                expr.property.name.to_string()
            )
        }
        JSXElementName::Identifier(ident) => ident.name.to_string(),
        JSXElementName::NamespacedName(ns) => {
            format!("{}:{}", ns.namespace.name, ns.name.name)
        }
        JSXElementName::IdentifierReference(ident) => ident.name.to_string(),
        JSXElementName::ThisExpression(_) => "this".to_string(),
    }
}

pub fn get_tag_name(tag: &JSXElement) -> String {
    jsx_element_name_to_string(&tag.opening_element.name)
}
