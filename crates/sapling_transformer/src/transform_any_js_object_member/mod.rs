use biome_js_factory::make::{
    js_getter_object_member, js_method_object_member, js_property_object_member,
    js_setter_object_member,
};
use biome_js_syntax::AnyJsObjectMember;

use crate::SaplingTransformer;

impl SaplingTransformer<'_> {
    pub fn transform_any_js_object_member(
        &mut self,
        node: &AnyJsObjectMember,
    ) -> Option<AnyJsObjectMember> {
        match node {
            AnyJsObjectMember::JsBogusMember(node) => self.transform_js_bogus_member(node),
            AnyJsObjectMember::JsGetterObjectMember(node) => {
                self.transform_js_getter_object_member(node)
            }
            AnyJsObjectMember::JsMethodObjectMember(node) => {
                self.transform_js_method_object_member(node)
            }
            AnyJsObjectMember::JsPropertyObjectMember(node) => {
                self.transform_js_property_object_member(node)
            }
            AnyJsObjectMember::JsSetterObjectMember(node) => {
                self.transform_js_setter_object_member(node)
            }
            AnyJsObjectMember::JsShorthandPropertyObjectMember(node) => {
                self.transform_js_shorthand_property_object_member(node)
            }
            AnyJsObjectMember::JsSpread(node) => self.transform_js_spread(node),
        }
    }

    fn transform_js_bogus_member(
        &mut self,
        _node: &biome_js_syntax::JsBogusMember,
    ) -> Option<AnyJsObjectMember> {
        todo!()
    }
    fn transform_js_getter_object_member(
        &mut self,
        node: &biome_js_syntax::JsGetterObjectMember,
    ) -> Option<AnyJsObjectMember> {
        let body = node.body().ok()?;
        let new_body = self.transform_js_function_body(&body);
        Some(AnyJsObjectMember::JsGetterObjectMember(
            js_getter_object_member(
                node.get_token().ok()?,
                node.name().ok()?,
                node.l_paren_token().ok()?,
                node.r_paren_token().ok()?,
                new_body?,
            )
            .build(),
        ))
    }
    fn transform_js_method_object_member(
        &mut self,
        node: &biome_js_syntax::JsMethodObjectMember,
    ) -> Option<AnyJsObjectMember> {
        let body = node.body().ok()?;
        let new_body = self.transform_js_function_body(&body);
        Some(AnyJsObjectMember::JsMethodObjectMember(
            js_method_object_member(node.name().ok()?, node.parameters().ok()?, new_body?).build(),
        ))
    }
    fn transform_js_property_object_member(
        &mut self,
        node: &biome_js_syntax::JsPropertyObjectMember,
    ) -> Option<AnyJsObjectMember> {
        let value = node.value().ok()?;
        let new_expr = self.transform_any_js_expression(&value);
        Some(AnyJsObjectMember::JsPropertyObjectMember(
            js_property_object_member(node.name().ok()?, node.colon_token().ok()?, new_expr?),
        ))
    }
    fn transform_js_setter_object_member(
        &mut self,
        node: &biome_js_syntax::JsSetterObjectMember,
    ) -> Option<AnyJsObjectMember> {
        let body = node.body().ok()?;
        let new_body = self.transform_js_function_body(&body);
        Some(AnyJsObjectMember::JsSetterObjectMember(
            js_setter_object_member(
                node.set_token().ok()?,
                node.name().ok()?,
                node.l_paren_token().ok()?,
                node.parameter().ok()?,
                node.r_paren_token().ok()?,
                new_body?,
            )
            .build(),
        ))
    }
    fn transform_js_shorthand_property_object_member(
        &mut self,
        node: &biome_js_syntax::JsShorthandPropertyObjectMember,
    ) -> Option<AnyJsObjectMember> {
        Some(AnyJsObjectMember::JsShorthandPropertyObjectMember(
            node.clone(),
        ))
    }
    fn transform_js_spread(
        &mut self,
        node: &biome_js_syntax::JsSpread,
    ) -> Option<AnyJsObjectMember> {
        Some(AnyJsObjectMember::JsSpread(node.clone()))
    }
}
