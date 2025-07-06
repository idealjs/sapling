use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule};
use biome_js_syntax::JsModule;
use biome_rowan::BatchMutationExt;

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
mod transform_arrow_function;
mod transform_export;
mod transform_expression;
mod transform_module;
mod transform_module_item;
mod transform_any_js_statement;

use crate::{JsBatchMutation, declare_transformation};

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
pub use transform_arrow_function::transform_arrow_function;
pub use transform_export::transform_export;
pub use transform_expression::transform_expression_with_tracker;
pub use transform_module::transform_module;
pub use transform_module_item::transform_module_item_with_tracker;
pub use transform_any_js_statement::transform_any_js_statement_with_tracker;

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

declare_transformation! {
    /// Transform JSX elements to SolidJS-style runtime calls
    pub(crate) JsxTemplate {
        version: "0.1.0",
        name: "jsx_template",
        language: "js",
    }
}

impl Rule for JsxTemplate {
    type Query = Ast<JsModule>;
    type State = TransformState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let module = ctx.query();

        // 检查模块中是否包含 JSX 元素
        let mut has_jsx = false;
        let mut jsx_elements = Vec::new();

        // 遍历模块查找 JSX
        let items = module.items();
        for item in items {
            if contains_jsx(&item) {
                has_jsx = true;
                collect_jsx_elements(&item, &mut jsx_elements);
            }
        }

        if has_jsx {
            Some(TransformState {
                _jsx_elements: jsx_elements,
                _needs_imports: has_jsx,
            })
        } else {
            None
        }
    }

    fn transform(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsBatchMutation> {
        let module = ctx.query();

        // 构建新的模块
        let new_module = transform_module(module, state)?;

        // 创建批量变更 - 替换整个模块
        let mut mutation = module.clone().begin();
        mutation.replace_node(module.clone(), new_module);
        Some(mutation)
    }
}

#[derive(Debug)]
pub struct TransformState {
    _jsx_elements: Vec<JsxElementInfo>,
    _needs_imports: bool,
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
