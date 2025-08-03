use biome_js_factory::make::{
    js_arrow_function_expression, js_function_expression, js_object_expression,
    js_object_member_list, token,
};
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsArrayExpression, JsArrowFunctionExpression,
    JsAssignmentExpression, JsAwaitExpression, JsBinaryExpression, JsBogusExpression,
    JsCallExpression, JsClassExpression, JsComputedMemberExpression, JsConditionalExpression,
    JsFunctionExpression, JsIdentifierExpression, JsImportCallExpression, JsImportMetaExpression,
    JsInExpression, JsInstanceofExpression, JsLogicalExpression, JsMetavariable, JsNewExpression,
    JsNewTargetExpression, JsObjectExpression, JsParenthesizedExpression, JsPostUpdateExpression,
    JsPreUpdateExpression, JsSequenceExpression, JsStaticMemberExpression, JsSuperExpression,
    JsTemplateExpression, JsThisExpression, JsUnaryExpression, JsYieldExpression, T,
    TsAsExpression, TsInstantiationExpression, TsNonNullAssertionExpression, TsSatisfiesExpression,
    TsTypeAssertionExpression,
};
use biome_rowan::TriviaPieceKind;

use crate::{
    SaplingTransformer, transfrom_jsx_tag_expression::TransformAnyJsxTagExpressionOptions,
};

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
            AnyJsExpression::JsxTagExpression(node) => self.transform_jsx_tag_expression(
                node,
                TransformAnyJsxTagExpressionOptions { parent_id: None },
            ),
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
        Some(AnyJsExpression::AnyJsLiteralExpression(node.clone()))
    }

    pub fn transform_js_arrow_function_expression(
        &mut self,
        node: &JsArrowFunctionExpression,
    ) -> Option<AnyJsExpression> {
        let new_body = self.transform_any_js_function_body(&node.body().ok()?)?;
        Some(AnyJsExpression::JsArrowFunctionExpression(
            js_arrow_function_expression(
                node.parameters().ok()?,
                node.fat_arrow_token().ok()?,
                new_body,
            )
            .build(),
        ))
    }

    pub fn transform_js_function_expression(
        &mut self,
        node: &JsFunctionExpression,
    ) -> Option<AnyJsExpression> {
        let new_body = self.transform_js_function_body(&node.body().ok()?)?;
        Some(AnyJsExpression::JsFunctionExpression(
            js_function_expression(
                node.function_token().ok()?,
                node.parameters().ok()?,
                new_body,
            )
            .build(),
        ))
    }

    pub fn transform_js_parenthesized_expression(
        &mut self,
        node: &JsParenthesizedExpression,
    ) -> Option<AnyJsExpression> {
        let expression = node.expression().ok()?;
        self.transform_any_js_expression(&expression)
    }

    pub fn transform_js_array_expression(
        &self,
        _node: &JsArrayExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }

    pub fn transform_js_assignment_expression(
        &self,
        _node: &JsAssignmentExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_await_expression(
        &self,
        _node: &JsAwaitExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_binary_expression(
        &self,
        _node: &JsBinaryExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_bogus_expression(
        &self,
        _node: &JsBogusExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_call_expression(&self, node: &JsCallExpression) -> Option<AnyJsExpression> {
        Some(AnyJsExpression::JsCallExpression(node.clone()))
    }
    pub fn transform_js_class_expression(
        &self,
        _node: &JsClassExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_computed_member_expression(
        &self,
        _node: &JsComputedMemberExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_conditional_expression(
        &self,
        _node: &JsConditionalExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }

    pub fn transform_js_identifier_expression(
        &self,
        _node: &JsIdentifierExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_import_call_expression(
        &self,
        _node: &JsImportCallExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_import_meta_expression(
        &self,
        _node: &JsImportMetaExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_in_expression(&self, _node: &JsInExpression) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_instanceof_expression(
        &self,
        _node: &JsInstanceofExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_logical_expression(
        &self,
        _node: &JsLogicalExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_metavariable_to_js_expression(
        &self,
        _node: &JsMetavariable,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_new_expression(&self, _node: &JsNewExpression) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_new_target_expression(
        &self,
        _node: &JsNewTargetExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_object_expression(
        &mut self,
        node: &JsObjectExpression,
    ) -> Option<AnyJsExpression> {
        let members = node.members();
        let mut new_members = vec![];
        let mut separators = vec![];
        members.into_iter().for_each(|member| {
            let Ok(member) = member else {
                return;
            };
            let new_member = self.transform_any_js_object_member(&member);
            let Some(new_member) = new_member else {
                return;
            };
            new_members.push(new_member);
            separators
                .push(token(T!(,)).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]));
        });
        Some(AnyJsExpression::JsObjectExpression(js_object_expression(
            token(T!['{']),
            js_object_member_list(new_members, separators),
            token(T!['}']),
        )))
    }

    pub fn transform_js_post_update_expression(
        &self,
        _node: &JsPostUpdateExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_pre_update_expression(
        &self,
        _node: &JsPreUpdateExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_sequence_expression(
        &self,
        _node: &JsSequenceExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_static_member_expression(
        &self,
        _node: &JsStaticMemberExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_super_expression(
        &self,
        _node: &JsSuperExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_template_expression(
        &self,
        _node: &JsTemplateExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_this_expression(
        &self,
        _node: &JsThisExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_unary_expression(
        &self,
        _node: &JsUnaryExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_js_yield_expression(
        &self,
        _node: &JsYieldExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_ts_as_expression(&self, _node: &TsAsExpression) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_ts_instantiation_expression(
        &self,
        _node: &TsInstantiationExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_ts_non_null_assertion_expression(
        &self,
        _node: &TsNonNullAssertionExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_ts_satisfies_expression(
        &self,
        _node: &TsSatisfiesExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
    pub fn transform_ts_type_assertion_expression(
        &self,
        _node: &TsTypeAssertionExpression,
    ) -> Option<AnyJsExpression> {
        todo!()
    }
}
