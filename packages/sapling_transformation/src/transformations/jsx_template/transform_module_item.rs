use biome_js_syntax::*;
use crate::jsx_template::transform_export;
use crate::jsx_template::transform_any_js_statement_with_tracker;

use crate::jsx_template::HelperUsageTracker;
pub fn transform_module_item_with_tracker(item: &AnyJsModuleItem, tracker: &mut HelperUsageTracker) -> AnyJsModuleItem {
    match item {
        AnyJsModuleItem::AnyJsStatement(stmt) => {
            AnyJsModuleItem::AnyJsStatement(transform_any_js_statement_with_tracker(stmt, tracker))
        },
        AnyJsModuleItem::JsExport(export) => {
            AnyJsModuleItem::JsExport(transform_export(export))
        },
        // 对于其他类型（如导入语句），直接克隆
        _ => item.clone(),
    }
}