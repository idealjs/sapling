use biome_js_syntax::*;
use biome_js_factory::make::*;
use biome_rowan::AstNode;
use crate::jsx_template::{handle_jsx_attributes};
use crate::transformations::jsx_template::is_custom_component::is_custom_component;
use crate::jsx_template::create_component_call::create_component_call;
use crate::jsx_template::create_native_element::create_native_element;
use crate::HelperUsageTracker;

pub fn create_solidjs_call_with_tracker(jsx_element: &JsxElement, tracker: &mut HelperUsageTracker) -> Option<AnyJsExpression> {
    // 获取元素名称
    let opening_element = jsx_element.opening_element().ok()?;
    let element_name = opening_element.name().ok()?;
    let jsx_name = element_name.as_jsx_name()?;
    let tag_token = jsx_name.value_token().ok()?;
    let tag_name = tag_token.text_trimmed();

    // 判断是否自定义组件
    if is_custom_component(&tag_name) {
        tracker.create_component = true;
        // 对于组件，生成 _$createComponent 调用
        return create_component_call(jsx_element, tracker);
    }

    // 原生标签走模板生成逻辑
    create_native_element(jsx_element, &tag_name, tracker)
}