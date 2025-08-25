use biome_js_factory::make::{js_array_element_list, js_array_expression, token};
use biome_js_syntax::{AnyJsArrayElement, JsArrayExpression, T};

pub fn make_array(elements: Vec<AnyJsArrayElement>) -> JsArrayExpression {
    js_array_expression(
        token(T!['[']),
        js_array_element_list(
            elements.clone(),
            vec![token(T![,]); elements.len().saturating_sub(1)],
        ),
        token(T![']']),
    )
}
