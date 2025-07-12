// biome相关依赖
use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::JsFileSource;
use biome_rowan::AstNode;
use biome_rowan::BatchMutationExt;
use sapling_transformer::{Config, SaplingTransformer, TraverseResult};
use std::collections::HashMap;

#[test]
fn test_create_js_statement() {
    let input_code = "let a;";
    let parsed_root = parse(input_code, JsFileSource::tsx(), JsParserOptions::default());
    let js_tree = parsed_root.tree();
    let semantic_model = semantic_model(&js_tree, SemanticModelOptions::default());

    let js_module = js_tree.as_js_module().expect("tree not exist").clone();

    let mut transformer = SaplingTransformer {
        mutation: js_module.clone().begin(),
        js_module,
        pre_process_errors: Vec::new(),
        semantic_model,
        scope_generated_identifiers: HashMap::new(),
        config: Config::default(),
        traverse_result: TraverseResult::default(),
    };

    let node_path = transformer.js_module.syntax();
    let scope = transformer.semantic_model.scope(node_path);
    let stmt1 = transformer.create_js_tag_statement(&scope, "div");
    let stmt2 = transformer.create_js_tag_statement(&scope, "div");
    insta::assert_snapshot!(format!("{}\n{}", stmt1.to_string(), stmt2.to_string()));
}
