use std::collections::{HashMap, HashSet};

use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::{JsFileSource, JsLanguage};
use biome_rowan::BatchMutation;

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

pub use helpers::*;
pub use scope::*;
pub use transform_any_js_expression::*;
pub use transform_any_js_function_body::*;
pub use transform_any_js_module_item::*;
pub use transform_any_js_object_member::*;
pub use transform_any_js_statement::*;
pub use transform_any_jsx_child::*;
pub use transform_js_function_body::*;
pub use transform_js_variable_declarator::*;
pub use transform_jsx_tag_expression_to_statements::*;
pub use transformer::*;
pub use write_transformation_snapshot::*;
pub type JsBatchMutation = BatchMutation<JsLanguage>;

use biome_formatter::IndentStyle;
use biome_js_formatter::context::JsFormatOptions;
use biome_js_formatter::format_node;
use biome_rowan::BatchMutationExt;

pub fn transfrom(input_code: String) -> Option<String> {
    let parsed_root = parse(
        input_code.as_str(),
        JsFileSource::tsx(),
        JsParserOptions::default(),
    );
    let js_tree = parsed_root.try_tree()?;

    let semantic_model = semantic_model(&js_tree, SemanticModelOptions::default());

    let js_module = js_tree.as_js_module()?.clone();

    let mut transformer = SaplingTransformer {
        mutation: js_module.clone().begin(),
        js_module,
        semantic_model,
        scope_generated_identifiers: HashMap::new(),
        config: Config {
            ..Default::default()
        },
        decorated_members: HashSet::new(),
    };
    transformer.transform();
    let node = transformer.mutation.commit();
    let formatted = format_node(
        JsFormatOptions::new(JsFileSource::default()).with_indent_style(IndentStyle::Space),
        &node,
    )
    .ok()?
    .print()
    .ok()?
    .as_code()
    .to_string();
    Some(formatted)
}
