use biome_js_syntax::*;
pub fn handle_jsx_attributes(
    _opening_element: JsxOpeningElement,
) -> Option<Vec<AnyJsStatement>> {
    // TODO: 遍历 opening_element.attributes()
    // - 静态属性: 生成 _el$.setAttribute(name, value)
    // - 动态属性: 生成 _el$.[name] = value
    // - 事件: 生成 _el$.addEventListener(...)
    // 返回生成的 AnyJsStatement 列表
    None
}