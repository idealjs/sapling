use biome_js_syntax::*;
use biome_js_factory::make::*;
use crate::JsxElementInfo;
use crate::TransformState;
use crate::jsx_template::{generate_solid_imports, transform_module_item};

pub fn transform_module(module: &JsModule, state: &TransformState) -> Option<JsModule> {
    // 获取原模块的所有项
    let original_items = module.items();
    let mut new_items = Vec::<AnyJsModuleItem>::new();

    // 1. 添加必要的导入语句
    if state.needs_imports {
        let import_statements = generate_solid_imports();
        for import_stmt in import_statements {
            new_items.push(AnyJsModuleItem::AnyJsStatement(import_stmt));
        }
    }

    // 2. 转换包含 JSX 的模块项
    for item in original_items {
        let transformed_item = transform_module_item(&item);
        new_items.push(transformed_item);
    }

    // 3. 重建模块 AST
    let new_module = js_module(
        js_directive_list(vec![]),
        js_module_item_list(new_items),
        module.eof_token().unwrap(),
    )
    .build();

    Some(new_module)
}