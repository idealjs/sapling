use biome_js_factory::make::{
    js_named_import_specifier, js_named_import_specifiers, js_import_named_clause, js_import,
    js_literal_export_name, js_identifier_binding, js_module_source,
    js_named_import_specifier_list,
};
use biome_js_syntax::{AnyJsBinding, JsSyntaxToken, T};

pub fn generate_solid_imports(
    need_create_text_node: bool,
    need_insert_node: bool,
    need_create_element: bool,
    need_insert: bool,
    need_use: bool,
    need_create_component: bool,
    need_merge_props: bool,
    need_memo: bool,
    need_for: bool,
) -> Vec<biome_js_syntax::JsImport> {
    use biome_js_factory::make::token;

    let mut imports = Vec::new();
    // 每个 helper 单独生成 import 语句，保证格式与预期一致
    let mut add_import = |export: &str, local: &str| {
    let export_token = JsSyntaxToken::new_detached(T![ident], export, vec![], vec![]);
    let local_token = JsSyntaxToken::new_detached(T![ident], local, vec![], vec![]);
    let named_specifier = js_named_import_specifiers(
        token(T!['{']),
        js_named_import_specifier_list(
            vec![
                js_named_import_specifier(
                    js_literal_export_name(export_token),
                    token(T![as]),
                    AnyJsBinding::JsIdentifierBinding(js_identifier_binding(local_token)),
                ).build().into()
            ],
            vec![]
        ),
        token(T!['}']),
    );
    let import_clause = js_import_named_clause(
        named_specifier,
        token(T![from]),
        biome_js_syntax::AnyJsModuleSource::JsModuleSource(
            js_module_source(
                biome_js_factory::make::js_string_literal("solid-universal-module")
            )
        ),
    ).build();
    let import_stmt = js_import(
        token(T![import]),
        import_clause.into(),
    ).build();
    imports.push(import_stmt);
};
    if need_create_text_node {
        add_import("createTextNode", "_$createTextNode");
    }
    if need_insert_node {
        add_import("insertNode", "_$insertNode");
    }
    if need_create_element {
        add_import("createElement", "_$createElement");
    }
    if need_insert {
        add_import("insert", "_$insert");
    }
    if need_use {
        add_import("use", "_$use");
    }
    if need_create_component {
        add_import("createComponent", "_$createComponent");
    }
    if need_merge_props {
        add_import("mergeProps", "_$mergeProps");
    }
    if need_memo {
        add_import("memo", "_$memo");
    }
    if need_for {
        add_import("For", "_$For");
    }
    imports
}