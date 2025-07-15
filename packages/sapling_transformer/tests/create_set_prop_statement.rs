// biome相关依赖
use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::JsFileSource;
use biome_rowan::AstNode;
use biome_rowan::BatchMutationExt;
use sapling_transformer::{Config, SaplingTransformer, TransformResult};
use std::collections::HashMap;

#[test]
fn test_create_set_prop_statement() {
    let input_code = r#"<div id="foo" title={"bar"} foo:some={0}/>;"#;
    let parsed_root = parse(input_code, JsFileSource::tsx(), JsParserOptions::default());
    let js_tree = parsed_root.tree();
    let semantic_model = semantic_model(&js_tree, SemanticModelOptions::default());

    let js_module = js_tree.as_js_module().expect("tree not exist").clone();

    let transformer = SaplingTransformer {
        mutation: js_module.clone().begin(),
        js_module,
        pre_process_errors: Vec::new(),
        semantic_model,
        scope_generated_identifiers: HashMap::new(),
        config: Config::default(),
        transform_result: TransformResult::default(),
    };

    // 获取第一个JSX属性
    let node_path = transformer.js_module.syntax();
    let jsx_element = node_path
        .descendants()
        .find_map(biome_js_syntax::JsxSelfClosingElement::cast)
        .expect("No JSX element found");
    let mut attributes = jsx_element.attributes().into_iter();

    let id_attr = attributes.next().expect("No id attribute found");
    let title_attr = attributes.next().expect("No title attribute found");
    let foo_attr = attributes.next().expect("No foo attribute found");

    let id = "_el$";

    let stmt1 = transformer.create_set_prop_statement(id, id_attr);
    let stmt2 = transformer.create_set_prop_statement(id, title_attr);
    let stmt3 = transformer.create_set_prop_statement(id, foo_attr);

    let (stmt1, stmt2, stmt3) = match (stmt1, stmt2, stmt3) {
        (Some(s1), Some(s2), Some(s3)) => (s1, s2, s3),
        _ => panic!("stmt1 or stmt2 is None"),
    };

    insta::assert_snapshot!(format!(
        "{}\n{}\n{}",
        stmt1.to_string(),
        stmt2.to_string(),
        stmt3.to_string()
    ));
}
