use biome_js_factory::make::{
    ident, js_arrow_function_expression, js_call_expression, js_directive_list,
    js_identifier_expression, js_parameter_list, js_reference_identifier, js_statement_list, token,
};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsxChild, JsMetavariable, JsxExpressionChild,
    JsxSpreadChild, JsxText, T,
};
use sapling_transformation::helpers::jsx_template::{
    make_js_arrow_function_expression, make_js_call_arguments, make_js_function_body,
    make_js_parameters, make_js_return_statement,
};

#[derive(Debug)]
pub struct TransformAnyJsxTextOptions {
    pub parent_id: Option<String>,
}

pub struct TransformAnyJsxChildOptions {
    pub parent_id: Option<String>,
}

pub struct TransformJsxExpressionChildOptions {
    pub parent_id: Option<String>,
}

use crate::{
    SaplingTransformer, generate_create_text_node_expr, generate_insert_node_expr,
    transfrom_jsx_tag_expression::TransformAnyJsxFragmentOptions,
};

impl SaplingTransformer {
    // main entry
    pub fn transform_any_jsx_child(
        &mut self,
        node: &AnyJsxChild,
        transform_options: TransformAnyJsxChildOptions,
    ) -> Option<AnyJsExpression> {
        match node {
            AnyJsxChild::JsMetavariable(node) => self.transform_js_metavariable(node),
            AnyJsxChild::JsxElement(node) => self.transform_jsx_element_to_create_jsx_tag_element(node),
            AnyJsxChild::JsxExpressionChild(node) => self.transform_jsx_expression_child(
                node,
                TransformJsxExpressionChildOptions {
                    parent_id: transform_options.parent_id,
                },
            ),
            AnyJsxChild::JsxFragment(node) => self.transform_jsx_fragment(
                node,
                TransformAnyJsxFragmentOptions {
                    parent_id: transform_options.parent_id,
                },
            ),
            AnyJsxChild::JsxSelfClosingElement(node) => {
                self.transform_jsx_self_closing_element_to_create_jsx_tag_element(node)
            }
            AnyJsxChild::JsxSpreadChild(node) => self.transform_jsx_spread_child(node),
            AnyJsxChild::JsxText(node) => self.transform_jsx_text(
                node,
                TransformAnyJsxTextOptions {
                    parent_id: transform_options.parent_id,
                },
            ),
        }
    }

    pub fn transform_jsx_text(
        &self,
        node: &JsxText,
        transform_options: TransformAnyJsxTextOptions,
    ) -> Option<AnyJsExpression> {
        // _$insertNode(_el$, _$createTextNode(`template`));
        let binding = node.to_string();
        let node_value = binding.as_str();
        // due to new line between JSX_CHILD_LIST
        // if node is new line return None
        if node_value.trim().is_empty() {
            return None;
        }
        let inner_call_expression = generate_create_text_node_expr(node_value);
        let parent_id = transform_options.parent_id?;
        Some(AnyJsExpression::from(generate_insert_node_expr(
            parent_id.as_str(),
            &inner_call_expression.to_string(),
        )))
    }

    pub fn transform_js_metavariable(&self, _node: &JsMetavariable) -> Option<AnyJsExpression> {
        todo!()
    }

    pub fn transform_jsx_expression_child(
        &self,
        node: &JsxExpressionChild,
        transform_options: TransformJsxExpressionChildOptions,
    ) -> Option<AnyJsExpression> {
        let expression = node.expression()?;
        let parent_id = transform_options.parent_id?;
        let params = make_js_parameters(js_parameter_list(vec![], vec![]));
        let body = make_js_function_body(
            js_directive_list(vec![]),
            js_statement_list(vec![make_js_return_statement(expression).into()]),
        );
        let expr = AnyJsExpression::JsArrowFunctionExpression(make_js_arrow_function_expression(
            params, body,
        ));
        let call_expr = sapling_transformation::helpers::jsx_template::make_insert(
            parent_id.as_str(),
            expr,
        );
        Some(AnyJsExpression::JsCallExpression(call_expr))
    }

    pub fn transform_jsx_spread_child(&self, _node: &JsxSpreadChild) -> Option<AnyJsExpression> {
        todo!()
    }
}
