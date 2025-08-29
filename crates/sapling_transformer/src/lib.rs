pub mod helpers;
pub mod scope;
pub mod transform_any_js_expression;
pub mod transform_any_js_object_member;
pub mod transform_any_js_statement;
pub mod transform_any_jsx_child;
pub mod transform_jsx_tag_expression_to_statements;
pub mod transformer;
pub mod transfrom_jsx_tag_expression;
pub mod write_transformation_snapshot;

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
use biome_js_syntax::{AnyJsDecorator, JsClassDeclaration, JsSyntaxKind, JsxTagExpression};
use biome_rowan::AstNode;
use biome_rowan::SyntaxNodeCast;
use biome_rowan::SyntaxRewriter;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

impl biome_rowan::SyntaxRewriter for SaplingTransformer<'_> {
    type Language = biome_js_syntax::JsLanguage;

    fn visit_node(
        &mut self,
        node: biome_rowan::SyntaxNode<Self::Language>,
    ) -> biome_rowan::VisitNodeSignal<Self::Language> {
        use biome_rowan::VisitNodeSignal;

        match node.kind() {
            JsSyntaxKind::JS_CLASS_DECLARATION => {
                if let Some(class_node) = node.clone().cast::<JsClassDeclaration>() {
                    let _ = class_node
                        .members()
                        .into_iter()
                        .try_for_each(|member| -> Option<()> {
                            let member = match member.as_js_property_class_member() {
                                Some(v) => v,
                                None => return Some(()),
                            };

                            let is_reactive = member.modifiers().into_iter().any(|modifier| {
                                let modifier = match modifier.as_js_decorator() {
                                    Some(v) => v,
                                    None => return false,
                                };
                                let expr = match modifier.expression().ok() {
                                    Some(v) => v,
                                    None => return false,
                                };

                                let binding = match expr {
                                    AnyJsDecorator::JsIdentifierExpression(expr) => {
                                        match expr.name().ok() {
                                            Some(name_node) => {
                                                name_node.binding(&self.semantic_model)
                                            }
                                            None => return false,
                                        }
                                    }
                                    AnyJsDecorator::JsStaticMemberExpression(expr) => {
                                        match expr.object().ok() {
                                            Some(obj) => {
                                                if let Some(ident_expr) =
                                                    obj.as_js_identifier_expression()
                                                {
                                                    match ident_expr.name().ok() {
                                                        Some(name_node) => {
                                                            name_node.binding(&self.semantic_model)
                                                        }
                                                        None => return false,
                                                    }
                                                } else {
                                                    None
                                                }
                                            }
                                            None => return false,
                                        }
                                    }
                                    _ => None,
                                };

                                let binding = if let Some(binding) = binding {
                                    binding
                                } else {
                                    return false;
                                };

                                if get_js_module_source_from_binding(&self.semantic_model, &binding)
                                    .as_deref()
                                    == Some("@idealjs/sapling")
                                {
                                    return true;
                                }
                                return false;
                            });

                            let member_key = member
                                .name()
                                .ok()?
                                .as_js_literal_member_name()?
                                .name()
                                .ok()?
                                .text()
                                .to_string();
                            if is_reactive {
                                self.decorated_members.insert(member_key);
                            }
                            Some(())
                        });
                }
                VisitNodeSignal::Traverse(node)
            }
            JsSyntaxKind::JSX_TAG_EXPRESSION => {
                if let Some(jsx_node) = node.clone().cast::<JsxTagExpression>() {
                    // build transformer for this replacement

                    if let Some(new_expr) = self.transform_jsx_tag_expression(
                        &jsx_node,
                        TransformAnyJsxTagExpressionOptions { parent_id: None },
                    ) {
                        let new_syntax = new_expr.syntax().clone();
                        return VisitNodeSignal::Traverse(new_syntax);
                    }
                }
                VisitNodeSignal::Traverse(node)
            }
            _ => VisitNodeSignal::Traverse(node),
        }
    }
}

#[wasm_bindgen]
pub fn transform(input_code: String) -> Option<String> {
    // parse
    let parsed_root = parse(
        input_code.as_str(),
        JsFileSource::tsx(),
        JsParserOptions::default(),
    );
    let js_tree = parsed_root.try_tree()?;

    let semantic_model = semantic_model(&js_tree, SemanticModelOptions::default());

    // First pass: collect decorated members per class and build the bit map.
    // We keep a map from class text range -> set of decorated member names.
    let mut decorated_members: HashSet<String> = HashSet::new();
    let mut string_tree = StringTree::default();

    // run rewriter
    let root = js_tree.into_syntax();
    let mut transformer = SaplingTransformer {
        semantic_model: semantic_model,
        scope_generated_identifiers: HashMap::new(),
        config: Config::default(),
        decorated_members: &mut decorated_members,
        string_tree: &mut string_tree,
    };

    let syntax_node = transformer.transform(root);

    // format and return
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
