use sapling_transformer::compatible::create_js_tag_statement::generate_create_js_tag_statement;

#[test]
fn test_generate_create_js_tag_statement() {
   let stmt1 = generate_create_js_tag_statement("_el1$", "div");
   let stmt2 = generate_create_js_tag_statement("_el2$", "div");
   insta::assert_snapshot!(format!("{}\n{}", stmt1.to_string(), stmt2.to_string()));
}
