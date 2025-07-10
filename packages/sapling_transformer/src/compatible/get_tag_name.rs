use crate::compatible::jsx_element_name_to_string;
use biome_js_syntax::{JsLanguage, JsxElement};
use biome_rowan::SyntaxNode;
use biome_rowan::SyntaxNodeCast;

pub fn get_tag_name(node_path: &SyntaxNode<JsLanguage>) -> Option<String> {
    let name = node_path
        .clone()
        .cast::<JsxElement>()?
        .opening_element()
        .ok()?
        .name()
        .ok()?;

    jsx_element_name_to_string(&name)
}
