use biome_js_syntax::*;
use biome_js_factory::make::*;
use crate::TransformState;
use crate::jsx_template::generate_solid_imports;

use super::transform_module_item_with_tracker;
use crate::jsx_template::HelperUsageTracker;
pub fn transform_module(module: &JsModule, _state: &TransformState) -> Option<JsModule> {
    // 获取原模块的所有项
    let original_items = module.items();
    let mut new_items = Vec::<AnyJsModuleItem>::new();

    // 新增：统计 helper 使用
    let mut tracker = HelperUsageTracker::default();

    // 2. 转换包含 JSX 的模块项
    for item in original_items {
        let transformed_item = transform_module_item_with_tracker(&item, &mut tracker);
        new_items.push(transformed_item);
    }

    // 1. 添加必要的导入语句（动态）
    let import_statements = generate_solid_imports(
        tracker.create_text_node,
        tracker.insert_node,
        tracker.create_element,
    );
    for import_stmt in import_statements {
        new_items.insert(0, AnyJsModuleItem::JsImport(import_stmt));
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