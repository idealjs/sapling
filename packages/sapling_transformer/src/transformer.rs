use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsStatement, JsLanguage, JsModule, JsSyntaxKind};
use biome_js_syntax::{JsStatementList, TextRange};
use biome_rowan::AstNode;
use biome_rowan::{BatchMutation, SyntaxNode, SyntaxNodeCast};

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Config {
    pub module_name: String,
    pub generate: String,
    pub hydratable: bool,
    pub delegate_events: bool,
    pub delegated_events: Vec<String>,
    pub built_ins: Vec<String>,
    pub require_import_source: bool,
    pub wrap_conditionals: bool,
    pub omit_nested_closing_tags: bool,
    pub omit_last_closing_tag: bool,
    pub omit_quotes: bool,
    pub context_to_custom_elements: bool,
    pub static_marker: String,
    pub effect_wrapper: String,
    pub memo_wrapper: String,
    pub validate: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            module_name: "dom".to_string(),
            generate: "dom".to_string(),
            hydratable: false,
            delegate_events: true,
            delegated_events: vec![],
            built_ins: vec![],
            require_import_source: false,
            wrap_conditionals: true,
            omit_nested_closing_tags: false,
            omit_last_closing_tag: true,
            omit_quotes: true,
            context_to_custom_elements: false,
            static_marker: "@once".to_string(),
            effect_wrapper: "effect".to_string(),
            memo_wrapper: "memo".to_string(),
            validate: true,
        }
    }
}

pub struct SaplingTransformer {
    pub mutation: BatchMutation<JsLanguage>,
    pub js_module: JsModule,
    pub pre_process_errors: Vec<String>,
    pub semantic_model: SemanticModel,
    pub scope_generated_identifiers: HashMap<TextRange, HashSet<String>>,
    pub config: Config,
    pub transform_result: TransformResult,
}

// impl Default for SaplingTransformer {
//     fn default() -> Self {
//         Self {
//             mutation: Default::default(),
//             js_module: Default::default(),
//             pre_process_errors: Default::default(),
//             semantic_model: Default::default(),
//             scope_generated_identifiers: Default::default(),
//             config: Default::default(),
//             traverse_result: Default::default(),
//         }
//     }
// }

#[derive(Debug, Clone, Default)]

pub struct TransformResult {
    pub statments: Vec<AnyJsStatement>,
}

impl SaplingTransformer {
    pub fn transform(&mut self) {
        let syntax_node = self.js_module.syntax();

        self.traverse_syntax_node(syntax_node.clone());
    }

    pub fn traverse_syntax_node(&mut self, syntax_node: SyntaxNode<JsLanguage>) -> Option<()> {
        if matches!(syntax_node.kind(), JsSyntaxKind::JS_STATEMENT_LIST) {
            let node = syntax_node.cast::<JsStatementList>()?;
            node.into_iter().for_each(|statement| {
                let Some(new_statement) = self.transform_any_js_statement(&statement) else {
                    return;
                };
                self.mutation.replace_node(statement, new_statement);
            });
            None
        } else {
            syntax_node.children().for_each(|syntax_node| {
                self.traverse_syntax_node(syntax_node);
            });
            None
        }
    }
}
