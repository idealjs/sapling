use std::{collections::HashSet, fs::read_to_string};

use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::{JsFileSource, JsSyntaxKind, JsxExpressionChild};
use biome_rowan::{AstNode, SyntaxNodeCast, WalkEvent};
use camino::Utf8Path;
use sapling_transformer::{
    get_expr_chain_from_any_js_expression,
    write_transformation_snapshot::write_transformation_snapshot,
};

fn run_test(input: &'static str) -> Option<()> {
    let input_file = Utf8Path::new(input);
    let file_name = input_file.file_name().unwrap();

    let mut snapshot = String::new();

    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {input_file:?}: {err:?}"));

    let parsed_root = parse(
        input_code.as_str(),
        JsFileSource::tsx(),
        JsParserOptions::default(),
    );

    let js_tree = parsed_root.try_tree()?;
    let semantic_model = semantic_model(&js_tree, SemanticModelOptions::default());

    // 收集所有表达式链
    let mut all_chains = Vec::new();
    js_tree.into_syntax().preorder().try_for_each(|event| {
        if let WalkEvent::Enter(syntax_node) = event {
            match syntax_node.kind() {
                JsSyntaxKind::JSX_EXPRESSION_CHILD => {
                    let node = syntax_node.cast::<JsxExpressionChild>()?;

                    let expression = node.expression()?;
                    if let Some(chain) = get_expr_chain_from_any_js_expression(
                        &semantic_model,
                        &HashSet::new(),
                        &expression,
                    ) {
                        all_chains.push(chain);
                    }
                    ()
                }
                _ => (),
            }
        }
        Some(())
    });

    // 格式化输出到 snapshot
    let formatted = all_chains
        .into_iter()
        .map(|chain| {
            chain
                .iter()
                .map(|x| x.clone().unwrap_or("None".to_string()))
                .collect::<Vec<_>>()
                .join(",")
        })
        .collect::<Vec<_>>()
        .join("\n");

    write_transformation_snapshot(
        &mut snapshot,
        input_code.clone().as_str(),
        formatted.as_str(),
        input_file.extension()?,
    );

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => {
            let path = input_file.parent().unwrap();
            let path_str = path.as_str();
            if let Some(idx) = path_str.find("specs") {
                Utf8Path::new(&path_str[idx..])
            } else {
                path
            }
        },
    },
    {
        insta::assert_snapshot!(file_name.replace(".tsx", ".tsx.chain"), snapshot, &file_name.replace(".tsx", ".tsx.chain"));
    });
    None
}

#[test]
pub fn counter_tsx() {
    let test_file = "tests/specs/counter.tsx";
    run_test(test_file);
}
