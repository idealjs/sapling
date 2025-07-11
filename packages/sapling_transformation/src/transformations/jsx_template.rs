mod collect_jsx_elements;
mod collect_jsx_from_expression;
mod collect_jsx_from_statement;
mod contains_jsx;
mod contains_jsx_in_expression;
mod contains_jsx_in_statement;
mod create_child_insert_statement;
mod create_children_expression;
mod create_component_call;
mod create_expression_insert_statement;
mod create_insert_expression_node;
mod create_insert_text_node;
mod create_native_element;
mod create_solidjs_call_self_closing;
mod create_solidjs_call_with_tracker;
mod create_text_insert_statement;
mod generate_solid_imports;
mod handle_component_props;
mod handle_jsx_attributes;
mod handle_jsx_self_closing_attributes;
mod is_custom_component;
mod transform_any_js_statement;
mod transform_arrow_function;
mod transform_export;
mod transform_expression;
mod transform_module;
mod transform_module_item;

pub use collect_jsx_elements::collect_jsx_elements;
pub use collect_jsx_from_expression::collect_jsx_from_expression;
pub use collect_jsx_from_statement::collect_jsx_from_statement;
pub use contains_jsx::contains_jsx;
pub use contains_jsx_in_expression::contains_jsx_in_expression;
pub use contains_jsx_in_statement::contains_jsx_in_statement;
pub use create_child_insert_statement::create_child_insert_statement;
pub use create_children_expression::create_children_expression;
pub use create_component_call::create_component_call;
pub use create_expression_insert_statement::create_expression_insert_statement;
pub use create_insert_expression_node::create_insert_expression_node_with_tracker;
pub use create_insert_text_node::create_insert_text_node_with_tracker;
pub use create_native_element::create_native_element;
pub use create_solidjs_call_self_closing::create_solidjs_call_with_tracker_self_closing;
pub use create_text_insert_statement::create_text_insert_statement;
pub use generate_solid_imports::generate_solid_imports;
pub use handle_component_props::handle_component_props;
pub use handle_jsx_attributes::handle_jsx_attributes;
pub use is_custom_component::is_custom_component;
pub use transform_any_js_statement::transform_any_js_statement_with_tracker;
pub use transform_arrow_function::transform_arrow_function;
pub use transform_export::transform_export;
pub use transform_expression::transform_expression_with_tracker;
pub use transform_module::transform_module;
pub use transform_module_item::transform_module_item_with_tracker;

// 用于统计 runtime helper 使用情况
#[derive(Default, Debug)]
pub struct HelperUsageTracker {
    pub create_text_node: bool,
    pub insert_node: bool,
    pub create_element: bool,
    pub insert: bool,
    pub use_ref: bool,
    pub create_component: bool,
    pub merge_props: bool,
    pub memo: bool,
    pub for_component: bool,
}

#[derive(Debug, Clone)]
pub struct JsxElementInfo {
    /// JSX 元素的标签名称
    pub tag_name: String,
    /// 元素在模块中的位置索引
    pub position: usize,
    /// 是否为自闭合标签
    pub is_self_closing: bool,
    /// 是否在函数内部
    pub in_function: bool,
}
