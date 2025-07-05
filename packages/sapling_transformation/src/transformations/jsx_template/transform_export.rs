use biome_js_syntax::*;

pub fn transform_export(export: &JsExport) -> JsExport {
    // 简化版本：对于导出语句暂时返回原样
    // 后续可以根据需要实现更复杂的导出转换逻辑
    export.clone()
}