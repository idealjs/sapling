pub mod bit_mask;
pub mod helpers;
pub mod scope;
pub mod transform_any_js_expression;
pub mod transform_any_js_function_body;
pub mod transform_any_js_module_item;
pub mod transform_any_js_object_member;
pub mod transform_any_js_statement;
pub mod transform_any_jsx_child;
pub mod transform_js_function_body;
pub mod transform_js_variable_declarator;
pub mod transform_jsx_tag_expression_to_statements;
pub mod transformer;
pub mod transfrom_jsx_tag_expression;
pub mod write_transformation_snapshot;

pub use bit_mask::*;
pub use helpers::*;
pub use transform_any_jsx_child::*;
pub use transform_jsx_tag_expression_to_statements::*;

use crate::transformer::{Config, SaplingTransformer};
use crate::transfrom_jsx_tag_expression::TransformAnyJsxTagExpressionOptions;
use biome_formatter::IndentStyle;
use biome_js_formatter::context::JsFormatOptions;
use biome_js_formatter::format_node;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{BindingExtensions, SemanticModelOptions, semantic_model};
use biome_js_syntax::JsFileSource;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsDecorator, AnyJsExpression, AnyJsPropertyModifier, JsClassDeclaration,
    JsSyntaxKind, JsxTagExpression,
};
use biome_rowan::AstNode;
use biome_rowan::BatchMutationExt;
use biome_rowan::SyntaxNodeCast;
use biome_rowan::WalkEvent;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn transform(input_code: String) -> Option<String> {
    let parsed_root = parse(
        input_code.as_str(),
        JsFileSource::tsx(),
        JsParserOptions::default(),
    );
    let js_tree = parsed_root.try_tree()?;

    let semantic_model = semantic_model(&js_tree, SemanticModelOptions::default());

    let mut decorated_members: HashSet<String> = HashSet::new();
    let mut bit_map = BitMask::new();
    let mut string_tree = StringTree::default();

    let mut transformer = SaplingTransformer {
        semantic_model: semantic_model.clone(),
        scope_generated_identifiers: HashMap::new(),
        config: Config::default(),
        decorated_members: &mut decorated_members,
        bit_map: &mut bit_map,
        string_tree: &mut string_tree,
    };

    let mut mutation = js_tree.clone().begin();

    js_tree
        .into_syntax()
        .preorder()
        .try_for_each(|event| match event {
            WalkEvent::Enter(syntax_node) => match syntax_node.kind() {
                JsSyntaxKind::JS_CLASS_DECLARATION => {
                    let node = syntax_node.cast::<JsClassDeclaration>()?;
                    for member in node.members() {
                        match member {
                            AnyJsClassMember::JsPropertyClassMember(member) => {
                                for modifer in member.modifiers() {
                                    match modifer {
                                        AnyJsPropertyModifier::JsDecorator(modifer) => {
                                            let expr = modifer.expression().ok()?;
                                            let binding = match expr {
                                                AnyJsDecorator::JsIdentifierExpression(expr) => {
                                                    expr.name().ok()?.binding(&semantic_model)
                                                }
                                                AnyJsDecorator::JsStaticMemberExpression(expr) => {
                                                    expr.object()
                                                        .ok()?
                                                        .as_js_identifier_expression()?
                                                        .name()
                                                        .ok()?
                                                        .binding(&semantic_model)
                                                }
                                                _ => None,
                                            }?;
                                            let js_module_source =
                                                get_js_module_source_from_binding(
                                                    &semantic_model,
                                                    &binding,
                                                )?;
                                            if js_module_source == "@idealjs/sapling" {
                                                let member_key: String = member
                                                    .name()
                                                    .ok()?
                                                    .as_js_literal_member_name()?
                                                    .name()
                                                    .ok()?
                                                    .text()
                                                    .into();
                                                transformer
                                                    .decorated_members
                                                    .insert(member_key.clone());
                                                transformer.bit_map.add_key(member_key);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    Some(())
                }
                JsSyntaxKind::JSX_TAG_EXPRESSION => {
                    let node = syntax_node.cast::<JsxTagExpression>()?;
                    let next_node = transformer.transform_jsx_tag_expression(
                        &node,
                        TransformAnyJsxTagExpressionOptions { parent_id: None },
                    )?;
                    mutation.replace_node(AnyJsExpression::JsxTagExpression(node), next_node);
                    Some(())
                }
                JsSyntaxKind::JS_EXPRESSION_STATEMENT => Some(()),
                JsSyntaxKind::JSX_EXPRESSION_CHILD => Some(()),
                JsSyntaxKind::JSX_EXPRESSION_ATTRIBUTE_VALUE => Some(()),
                _ => Some(()),
            },
            WalkEvent::Leave(syntax_node) => match syntax_node.kind() {
                JsSyntaxKind::JS_CLASS_DECLARATION => {
                    transformer.decorated_members.clear();
                    transformer.string_tree.clear();
                    Some(())
                }
                _ => Some(()),
            },
        });

    let syntax_node = mutation.commit();
    let formatted = format_node(
        JsFormatOptions::new(JsFileSource::default()).with_indent_style(IndentStyle::Space),
        &syntax_node,
    )
    .ok()?
    .print()
    .ok()?
    .as_code()
    .to_string();
    Some(formatted)
}
