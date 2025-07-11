use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::{JsFileSource, JsSyntaxKind};
use biome_rowan::AstNode;

#[test]
fn test_scope_text_range_for_two_vars() {
    let code = r#"
        function foo() {
            let a = 1;
            let b = 2;
        }
    "#;
    let parsed = parse(code, JsFileSource::js_script(), JsParserOptions::default());
    let tree = parsed.tree();
    let model = semantic_model(&tree, SemanticModelOptions::default());

    // 1. 找到 a、b 两个变量的 node
    let mut a_node = None;
    let mut b_node = None;
    for node in tree.syntax().descendants() {
        if node.kind() == JsSyntaxKind::JS_IDENTIFIER_BINDING {
            let text = node.text_trimmed();
            if text == "a" {
                a_node = Some(node.clone());
            }
            if text == "b" {
                b_node = Some(node.clone());
            }
        }
    }
    let a_node = a_node.expect("a node not found");
    let b_node = b_node.expect("b node not found");
    println!("a_node: {:?}, b_node: {:?}", a_node, b_node);
    // 2. 使用 model 获取 scope
    let a_scope = model.scope(&a_node);
    let b_scope = model.scope(&b_node);

    // 3. 判断 text range 是否相同
    let a_range = a_scope.range();
    let b_range = b_scope.range();
    println!("a_scope range: {:?}, b_scope range: {:?}", a_range, b_range);
    assert_eq!(
        a_range, b_range,
        "a 和 b 应该在同一个作用域，text range 应该相同"
    );
}
