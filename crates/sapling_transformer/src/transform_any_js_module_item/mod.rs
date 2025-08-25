use biome_js_syntax::{AnyJsModuleItem, JsExport, JsImport};

use crate::SaplingTransformer;

impl SaplingTransformer {
    pub fn transform_any_js_module_item(
        &mut self,
        node: &AnyJsModuleItem,
    ) -> Option<AnyJsModuleItem> {
        match node {
            AnyJsModuleItem::AnyJsStatement(node) => Some(AnyJsModuleItem::AnyJsStatement(
                self.transform_any_js_statement(&node)?,
            )),
            AnyJsModuleItem::JsExport(node) => self.transform_js_export(&node),
            AnyJsModuleItem::JsImport(node) => self.transform_js_import(&node),
        }
    }

    pub fn transform_js_import(&mut self, node: &JsImport) -> Option<AnyJsModuleItem> {
        Some(AnyJsModuleItem::JsImport(node.clone()))
    }

    pub fn transform_js_export(&mut self, node: &JsExport) -> Option<AnyJsModuleItem> {
        Some(AnyJsModuleItem::JsExport(node.clone()))
    }
}
