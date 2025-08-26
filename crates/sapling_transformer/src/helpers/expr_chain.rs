use std::collections::HashSet;

use biome_js_semantic::{Binding, BindingExtensions, SemanticModel};
use biome_js_syntax::{AnyJsExpression, JsSyntaxKind, JsVariableDeclarator};
use biome_rowan::SyntaxNodeCast;

pub fn get_expr_chain_from_any_js_expression(
    semantic_model: &SemanticModel,
    decorated_members: &HashSet<String>,
    node: &AnyJsExpression,
) -> Option<Vec<String>> {
    match node {
        AnyJsExpression::JsComputedMemberExpression(expr) => {
            let object = expr.object().ok()?;
            let member = expr.member().ok()?;
            println!("test test object {:?}", object);
            println!("test test object {:?}", member);
            let mut chain =
                get_expr_chain_from_any_js_expression(semantic_model, decorated_members, &object)?;
            let member_chain =
                get_expr_chain_from_any_js_expression(semantic_model, decorated_members, &member)?;

            if let Some(first) = member_chain.first() {
                if decorated_members.contains(first) {
                    chain.extend(member_chain);
                }
            }
            Some(chain)
        }
        AnyJsExpression::JsStaticMemberExpression(expr) => {
            let object = expr.object().ok()?;
            let member = expr.member().ok()?;
            let mut chain =
                get_expr_chain_from_any_js_expression(semantic_model, decorated_members, &object)?;
            chain.push(member.value_token().ok()?.text_trimmed().to_string());
            Some(chain)
        }
        AnyJsExpression::JsIdentifierExpression(expr) => {
            let name = expr.name().ok()?;
            let binding = name.binding(semantic_model)?;
            let mut result =
                match get_expr_chain_from_binding(semantic_model, decorated_members, &binding) {
                    Some(value) => value,
                    None => vec![],
                };
            result.push(name.value_token().ok()?.text_trimmed().to_string());
            Some(result)
        }
        _ => Some(vec![]),
    }
}

pub fn get_expr_chain_from_binding(
    semantic_model: &SemanticModel,
    decorated_members: &HashSet<String>,
    binding: &Binding,
) -> Option<Vec<String>> {
    let node = binding.syntax();
    for ancestor in node.ancestors() {
        if ancestor.kind() == JsSyntaxKind::JS_VARIABLE_DECLARATOR {
            let declarator = ancestor.cast::<JsVariableDeclarator>()?;
            let initializer = declarator.initializer()?.expression().ok()?;
            // initializer 已经是 AnyJsExpression 类型
            return get_expr_chain_from_any_js_expression(
                semantic_model,
                decorated_members,
                &initializer,
            );
        }
    }
    Some(vec![])
}
