use biome_js_syntax::*;
use crate::jsx_template::{transform_statement, transform_export};

pub fn transform_module_item(item: &AnyJsModuleItem) -> AnyJsModuleItem {
    match item {
        AnyJsModuleItem::AnyJsStatement(stmt) => {
            AnyJsModuleItem::AnyJsStatement(transform_statement(stmt))
        },
        AnyJsModuleItem::JsExport(export) => {
            AnyJsModuleItem::JsExport(transform_export(export))
        },
        // 对于其他类型（如导入语句），直接克隆
        _ => item.clone(),
    }
}