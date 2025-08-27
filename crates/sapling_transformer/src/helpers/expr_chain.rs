use std::collections::HashSet;

use biome_js_semantic::{Binding, BindingExtensions, SemanticModel};
use biome_js_syntax::{AnyJsExpression, JsSyntaxKind, JsVariableDeclarator};
use biome_rowan::SyntaxNodeCast;

pub fn get_expr_chain_from_any_js_expression(
    semantic_model: &SemanticModel,
    decorated_members: &HashSet<String>,
    node: &AnyJsExpression,
) -> Option<Vec<Option<String>>> {
    match node {
        AnyJsExpression::JsComputedMemberExpression(expr) => {
            let object = expr.object().ok()?;
            let mut chain =
                get_expr_chain_from_any_js_expression(semantic_model, decorated_members, &object)?;
            chain.push(None);
            Some(chain)
        }
        AnyJsExpression::JsStaticMemberExpression(expr) => {
            let object = expr.object().ok()?;
            let member = expr.member().ok()?;
            let mut chain =
                get_expr_chain_from_any_js_expression(semantic_model, decorated_members, &object)?;
            chain.push(
                member
                    .value_token()
                    .ok()
                    .map(|t| t.text_trimmed().to_string()),
            );
            Some(chain)
        }
        AnyJsExpression::JsIdentifierExpression(expr) => {
            let name = expr.name().ok()?;
            let binding = name.binding(semantic_model)?;
            get_expr_chain_from_binding(semantic_model, decorated_members, &binding)
        }
        _ => Some(vec![]),
    }
}

/**
 *   const { b: { bb: xx } } = obj  => ["b", "bb"]
 *   const { b } = obj              => ["b"]
 */
fn collect_property_chain_from_identifier(
    node: &biome_rowan::SyntaxNode<biome_js_syntax::JsLanguage>,
) -> Vec<Option<String>> {
    use biome_js_syntax::JsSyntaxKind;
    let mut chain = Vec::new();
    let mut current = node.parent();
    while let Some(parent) = current {
        match parent.kind() {
            JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY => {
                for child in parent.children() {
                    if child.kind() == JsSyntaxKind::JS_LITERAL_MEMBER_NAME {
                        chain.push(Some(child.text_trimmed().to_string()));
                    }
                }
            }
            JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => {
                for child in parent.children() {
                    if child.kind() == JsSyntaxKind::JS_IDENTIFIER_BINDING {
                        chain.push(Some(child.text_trimmed().to_string()));
                    }
                }
            }
            _ => {}
        }
        current = parent.parent();
    }
    chain.reverse();
    chain
}

pub fn get_expr_chain_from_binding(
    semantic_model: &SemanticModel,
    decorated_members: &HashSet<String>,
    binding: &Binding,
) -> Option<Vec<Option<String>>> {
    let node = binding.syntax();
    let chain = collect_property_chain_from_identifier(node);
    for ancestor in node.ancestors() {
        if ancestor.kind() == JsSyntaxKind::JS_VARIABLE_DECLARATOR {
            let declarator = ancestor.cast::<JsVariableDeclarator>()?;
            let initializer = declarator.initializer()?.expression().ok()?;
            let result = if let Some(mut init_chain) = get_expr_chain_from_any_js_expression(
                semantic_model,
                decorated_members,
                &initializer,
            ) {
                init_chain.extend(chain);
                Some(init_chain)
            } else {
                Some(chain)
            };
            return result;
        }
    }
    Some(vec![])
}
