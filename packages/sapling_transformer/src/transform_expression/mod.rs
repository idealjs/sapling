use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsArrayExpression, JsArrowFunctionExpression,
    JsAssignmentExpression, JsAwaitExpression, JsBinaryExpression, JsBogusExpression,
    JsCallExpression, JsClassExpression, JsComputedMemberExpression, JsConditionalExpression,
    JsFunctionExpression, JsIdentifierExpression, JsImportCallExpression, JsImportMetaExpression,
    JsInExpression, JsInstanceofExpression, JsLogicalExpression, JsMetavariable, JsNewExpression,
    JsNewTargetExpression, JsObjectExpression, JsParenthesizedExpression, JsPostUpdateExpression,
    JsPreUpdateExpression, JsSequenceExpression, JsStaticMemberExpression, JsSuperExpression,
    JsTemplateExpression, JsThisExpression, JsUnaryExpression, JsYieldExpression, TsAsExpression,
    TsInstantiationExpression, TsNonNullAssertionExpression, TsSatisfiesExpression,
    TsTypeAssertionExpression,
};

use crate::SaplingTransformer;

impl SaplingTransformer {
    // main entry
    pub fn transform_any_js_expression(
        &mut self,
        node: &AnyJsExpression,
    ) -> Option<AnyJsExpression> {
        match node {
            AnyJsExpression::AnyJsLiteralExpression(node) => {
                self.transform_any_js_literal_expression(node)
            }
            AnyJsExpression::JsArrayExpression(node) => self.transform_js_array_expression(node),
            AnyJsExpression::JsArrowFunctionExpression(node) => {
                self.transform_js_arrow_function_expression(node)
            }
            AnyJsExpression::JsAssignmentExpression(node) => {
                self.transform_js_assignment_expression(node)
            }
            AnyJsExpression::JsAwaitExpression(node) => self.transform_js_await_expression(node),
            AnyJsExpression::JsBinaryExpression(node) => self.transform_js_binary_expression(node),
            AnyJsExpression::JsBogusExpression(node) => self.transform_js_bogus_expression(node),
            AnyJsExpression::JsCallExpression(node) => self.transform_js_call_expression(node),
            AnyJsExpression::JsClassExpression(node) => self.transform_js_class_expression(node),
            AnyJsExpression::JsComputedMemberExpression(node) => {
                self.transform_js_computed_member_expression(node)
            }
            AnyJsExpression::JsConditionalExpression(node) => {
                self.transform_js_conditional_expression(node)
            }
            AnyJsExpression::JsFunctionExpression(node) => {
                self.transform_js_function_expression(node)
            }
            AnyJsExpression::JsIdentifierExpression(node) => {
                self.transform_js_identifier_expression(node)
            }
            AnyJsExpression::JsImportCallExpression(node) => {
                self.transform_js_import_call_expression(node)
            }
            AnyJsExpression::JsImportMetaExpression(node) => {
                self.transform_js_import_meta_expression(node)
            }
            AnyJsExpression::JsInExpression(node) => self.transform_js_in_expression(node),
            AnyJsExpression::JsInstanceofExpression(node) => {
                self.transform_js_instanceof_expression(node)
            }
            AnyJsExpression::JsLogicalExpression(node) => {
                self.transform_js_logical_expression(node)
            }
            AnyJsExpression::JsMetavariable(node) => {
                self.transform_js_metavariable_to_js_expression(node)
            }
            AnyJsExpression::JsNewExpression(node) => self.transform_js_new_expression(node),
            AnyJsExpression::JsNewTargetExpression(node) => {
                self.transform_js_new_target_expression(node)
            }
            AnyJsExpression::JsObjectExpression(node) => self.transform_js_object_expression(node),
            AnyJsExpression::JsParenthesizedExpression(node) => {
                self.transform_js_parenthesized_expression(node)
            }
            AnyJsExpression::JsPostUpdateExpression(node) => {
                self.transform_js_post_update_expression(node)
            }
            AnyJsExpression::JsPreUpdateExpression(node) => {
                self.transform_js_pre_update_expression(node)
            }
            AnyJsExpression::JsSequenceExpression(node) => {
                self.transform_js_sequence_expression(node)
            }
            AnyJsExpression::JsStaticMemberExpression(node) => {
                self.transform_js_static_member_expression(node)
            }
            AnyJsExpression::JsSuperExpression(node) => self.transform_js_super_expression(node),
            AnyJsExpression::JsTemplateExpression(node) => {
                self.transform_js_template_expression(node)
            }
            AnyJsExpression::JsThisExpression(node) => self.transform_js_this_expression(node),
            AnyJsExpression::JsUnaryExpression(node) => self.transform_js_unary_expression(node),
            AnyJsExpression::JsYieldExpression(node) => self.transform_js_yield_expression(node),
            AnyJsExpression::JsxTagExpression(node) => Some(AnyJsExpression::JsxTagExpression(
                self.transform_jsx_tag_expression(node)?,
            )),
            AnyJsExpression::TsAsExpression(node) => self.transform_ts_as_expression(node),
            AnyJsExpression::TsInstantiationExpression(node) => {
                self.transform_ts_instantiation_expression(node)
            }
            AnyJsExpression::TsNonNullAssertionExpression(node) => {
                self.transform_ts_non_null_assertion_expression(node)
            }
            AnyJsExpression::TsSatisfiesExpression(node) => {
                self.transform_ts_satisfies_expression(node)
            }
            AnyJsExpression::TsTypeAssertionExpression(node) => {
                self.transform_ts_type_assertion_expression(node)
            }
        }
    }
    pub fn transform_any_js_literal_expression(
        &self,
        node: &AnyJsLiteralExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_array_expression(
        &self,
        node: &JsArrayExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_arrow_function_expression(
        &self,
        node: &JsArrowFunctionExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_assignment_expression(
        &self,
        node: &JsAssignmentExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_await_expression(
        &self,
        node: &JsAwaitExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_binary_expression(
        &self,
        node: &JsBinaryExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_bogus_expression(
        &self,
        node: &JsBogusExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_call_expression(&self, node: &JsCallExpression) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_class_expression(
        &self,
        node: &JsClassExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_computed_member_expression(
        &self,
        node: &JsComputedMemberExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_conditional_expression(
        &self,
        node: &JsConditionalExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_function_expression(
        &self,
        node: &JsFunctionExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_identifier_expression(
        &self,
        node: &JsIdentifierExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_import_call_expression(
        &self,
        node: &JsImportCallExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_import_meta_expression(
        &self,
        node: &JsImportMetaExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_in_expression(&self, node: &JsInExpression) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_instanceof_expression(
        &self,
        node: &JsInstanceofExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_logical_expression(
        &self,
        node: &JsLogicalExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_metavariable_to_js_expression(
        &self,
        node: &JsMetavariable,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_new_expression(&self, node: &JsNewExpression) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_new_target_expression(
        &self,
        node: &JsNewTargetExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_object_expression(
        &self,
        node: &JsObjectExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_parenthesized_expression(
        &self,
        node: &JsParenthesizedExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_post_update_expression(
        &self,
        node: &JsPostUpdateExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_pre_update_expression(
        &self,
        node: &JsPreUpdateExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_sequence_expression(
        &self,
        node: &JsSequenceExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_static_member_expression(
        &self,
        node: &JsStaticMemberExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_super_expression(
        &self,
        node: &JsSuperExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_template_expression(
        &self,
        node: &JsTemplateExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_this_expression(&self, node: &JsThisExpression) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_unary_expression(
        &self,
        node: &JsUnaryExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_yield_expression(
        &self,
        node: &JsYieldExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_ts_as_expression(&self, node: &TsAsExpression) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_ts_instantiation_expression(
        &self,
        node: &TsInstantiationExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_ts_non_null_assertion_expression(
        &self,
        node: &TsNonNullAssertionExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_ts_satisfies_expression(
        &self,
        node: &TsSatisfiesExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_ts_type_assertion_expression(
        &self,
        node: &TsTypeAssertionExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
}
