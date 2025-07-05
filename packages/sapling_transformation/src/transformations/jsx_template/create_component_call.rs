use biome_js_syntax::*;
use biome_js_factory::make::*;
use crate::jsx_template::{handle_jsx_attributes};
use crate::jsx_template::HelperUsageTracker;
use crate::transformations::jsx_template::handle_component_props::handle_component_props;

pub fn create_component_call(jsx_element: &JsxElement, tracker: &mut HelperUsageTracker) -> Option<AnyJsExpression> {
    let opening_element = jsx_element.opening_element().ok()?;
    let element_name = opening_element.name().ok()?;
    let jsx_name = element_name.as_jsx_name()?;
    let tag_token = jsx_name.value_token().ok()?;
    let component_name = tag_token.text_trimmed();

    // 生成组件标识符
    let component_ident = js_identifier_expression(
        js_reference_identifier(
            JsSyntaxToken::new_detached(T![ident], &component_name, Vec::new(), Vec::new())
        )
    );

    // 处理 props
    let props_expr = handle_component_props(&opening_element, jsx_element, tracker)?;

    // 生成 _$createComponent(Component, props) 调用
    let create_component_token = JsSyntaxToken::new_detached(T![ident], "_$createComponent", Vec::new(), Vec::new());
    
    let call = js_call_expression(
        js_identifier_expression(js_reference_identifier(create_component_token)).into(),
        js_call_arguments(
            token(T!['(']),
            js_call_argument_list(
                vec![
                    AnyJsCallArgument::AnyJsExpression(component_ident.into()),
                    AnyJsCallArgument::AnyJsExpression(props_expr),
                ],
                vec![token(T![,])],
            ),
            token(T![')']),
        ),
    ).build();

    Some(AnyJsExpression::JsCallExpression(call))
}