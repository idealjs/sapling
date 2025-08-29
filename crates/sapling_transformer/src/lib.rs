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
    AnyJsClassMember, AnyJsDecorator, AnyJsPropertyModifier, JsClassDeclaration, JsSyntaxKind,
    JsxTagExpression,
};
use biome_rowan::AstNode;
use biome_rowan::SyntaxNodeCast;
use biome_rowan::SyntaxRewriter;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

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
    let decorated_members: HashSet<String> = HashSet::new();
    let string_tree = StringTree::default();

    // Second pass: use a SyntaxRewriter to replace JSX tag expressions.
    // The rewriter will lookup the decorated members for the class ancestor of the JSX node.
    struct SaplingRewriter {
        semantic_model: biome_js_semantic::SemanticModel,
        decorated_members: HashSet<String>,
        string_tree: StringTree,
    }

    impl biome_rowan::SyntaxRewriter for SaplingRewriter {
        type Language = biome_js_syntax::JsLanguage;

        fn visit_node(
            &mut self,
            node: biome_rowan::SyntaxNode<Self::Language>,
        ) -> biome_rowan::VisitNodeSignal<Self::Language> {
            use biome_rowan::VisitNodeSignal;

            match node.kind() {
                JsSyntaxKind::JS_CLASS_DECLARATION => {
                    if let Some(class_node) = node.clone().cast::<JsClassDeclaration>() {
                        for member in class_node.members() {
                            if let AnyJsClassMember::JsPropertyClassMember(member) = member {
                                for modifer in member.modifiers() {
                                    if let AnyJsPropertyModifier::JsDecorator(modifer) = modifer {
                                        if let Some(expr) = modifer.expression().ok() {
                                            // resolve binding stepwise to avoid returning references to temporaries
                                            let binding = match expr {
                                                AnyJsDecorator::JsIdentifierExpression(expr) => {
                                                    if let Some(name_node) = expr.name().ok() {
                                                        name_node.binding(&self.semantic_model)
                                                    } else {
                                                        None
                                                    }
                                                }
                                                AnyJsDecorator::JsStaticMemberExpression(expr) => {
                                                    if let Some(obj) = expr.object().ok() {
                                                        if let Some(ident_expr) =
                                                            obj.as_js_identifier_expression()
                                                        {
                                                            if let Some(name_node) =
                                                                ident_expr.name().ok()
                                                            {
                                                                name_node
                                                                    .binding(&self.semantic_model)
                                                            } else {
                                                                None
                                                            }
                                                        } else {
                                                            None
                                                        }
                                                    } else {
                                                        None
                                                    }
                                                }
                                                _ => None,
                                            };
                                            let binding = match binding {
                                                Some(b) => b,
                                                None => continue,
                                            };

                                            if let Some(js_module_source) =
                                                get_js_module_source_from_binding(
                                                    &self.semantic_model,
                                                    &binding,
                                                )
                                            {
                                                if js_module_source == "@idealjs/sapling" {
                                                    if let Some(name_node) = member.name().ok() {
                                                        if let Some(lit) =
                                                            name_node.as_js_literal_member_name()
                                                        {
                                                            if let Ok(id) = lit.name() {
                                                                let member_key: String =
                                                                    id.text().into();
                                                                self.decorated_members
                                                                    .insert(member_key.clone());
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    VisitNodeSignal::Traverse(node)
                }
                JsSyntaxKind::JSX_TAG_EXPRESSION => {
                    if let Some(jsx_node) = node.clone().cast::<JsxTagExpression>() {
                        // build transformer for this replacement
                        let mut transformer = SaplingTransformer {
                            semantic_model: self.semantic_model.clone(),
                            scope_generated_identifiers: HashMap::new(),
                            config: Config::default(),
                            decorated_members: &mut self.decorated_members,
                            string_tree: &mut self.string_tree,
                        };

                        if let Some(new_expr) = transformer.transform_jsx_tag_expression(
                            &jsx_node,
                            TransformAnyJsxTagExpressionOptions { parent_id: None },
                        ) {
                            let new_syntax = new_expr.syntax().clone();
                            return VisitNodeSignal::Replace(new_syntax);
                        }
                    }
                    VisitNodeSignal::Traverse(node)
                }
                _ => VisitNodeSignal::Traverse(node),
            }
        }
    }

    // run rewriter
    let root = js_tree.into_syntax();
    let mut rewriter = SaplingRewriter {
        semantic_model,
        decorated_members,
        string_tree,
    };
    let syntax_node = rewriter.transform(root);

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
