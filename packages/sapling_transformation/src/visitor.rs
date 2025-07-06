use biome_js_syntax::{JsLanguage, JsModule, JsSyntaxKind, JsxElement};
use biome_rowan::{AstNode, BatchMutation, BatchMutationExt, SyntaxNodeCast};

use crate::is_valid_html_nesting::is_valid_html_nesting;

pub struct SaplingVisitor {
    mutation: BatchMutation<JsLanguage>,
    js_module: JsModule,
    pre_process_errors: Vec<String>,
}

impl SaplingVisitor {
    pub fn traverse<L: biome_rowan::Language>(mut self) {
        self.mutation = self.js_module.clone().begin();
        let descendants = self.js_module.syntax().descendants();
        descendants.for_each(|node| match node.kind() {
            JsSyntaxKind::JSX_ELEMENT => {
                let Some(node) = node.cast::<JsxElement>() else {
                    return;
                };
                let Some(parent) = node
                    .syntax()
                    .parent()
                    .and_then(|parent| parent.cast::<JsxElement>())
                else {
                    return;
                };

                let (Ok(el_tag), Ok(parent_tag)) = (
                    node.opening_element()
                        .and_then(|v| v.name())
                        .and_then(|v| v.name_value_token())
                        .and_then(|v| Ok(v.text().to_string())),
                    parent
                        .opening_element()
                        .and_then(|v| v.name())
                        .and_then(|v| v.name_value_token())
                        .and_then(|v| Ok(v.text().to_string())),
                ) else {
                    return;
                };

                let (el_tag, parent_tag) = (el_tag.as_str(), parent_tag.as_str());

                if is_component(el_tag) || is_component(parent_tag) {
                    if is_valid_html_nesting(parent_tag, el_tag) {
                        self.pre_process_errors.push(format!(
                            "Invalid JSX: <{}> cannot be child of <{}>",
                            el_tag, parent_tag
                        ));
                    }
                }
            }
            _ => {}
        });
    }
}

fn is_component(tag_name: &str) -> bool {
    if let Some(first) = tag_name.chars().next() {
        first.to_lowercase().to_string() != first.to_string()
            || tag_name.contains('.')
            || !first.is_ascii_alphabetic()
    } else {
        false
    }
}
