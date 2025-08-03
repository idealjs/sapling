use biome_js_factory::make::{
    ident, js_call_argument_list, js_call_arguments, js_call_expression, js_identifier_expression,
    js_reference_identifier, js_string_literal, js_string_literal_expression, token,
};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsxChild, JsMetavariable, JsxExpressionChild,
    JsxSpreadChild, JsxText, T,
};

#[derive(Debug)]
pub struct TransformAnyJsxTextOptions {
    pub parent_id: Option<String>,
}

pub struct TransformAnyJsxChildOptions {
    pub parent_id: Option<String>,
}

use crate::{SaplingTransformer, transfrom_jsx_tag_expression::TransformAnyJsxFragmentOptions};

impl SaplingTransformer {
    // main entry
    pub fn transform_any_jsx_child(
        &mut self,
        node: &AnyJsxChild,
        transform_options: TransformAnyJsxChildOptions,
    ) -> Option<AnyJsExpression> {
        match node {
            AnyJsxChild::JsMetavariable(node) => self.transform_js_metavariable(node),
            AnyJsxChild::JsxElement(node) => self.transform_jsx_element_to_iife(node),
            AnyJsxChild::JsxExpressionChild(node) => self.transform_jsx_expression_child(node),
            AnyJsxChild::JsxFragment(node) => self.transform_jsx_fragment(
                node,
                TransformAnyJsxFragmentOptions {
                    parent_id: transform_options.parent_id,
                },
            ),
            AnyJsxChild::JsxSelfClosingElement(node) => {
                self.transform_jsx_self_closing_element(node)
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
        let callee = AnyJsExpression::JsIdentifierExpression(js_identifier_expression(
            js_reference_identifier(ident("_$insertNode")),
        ));

        let string_literal = js_string_literal_expression(js_string_literal(node_value));
        let inner_callee = AnyJsExpression::JsIdentifierExpression(js_identifier_expression(
            js_reference_identifier(ident("_$createTextNode")),
        ));
        let inner_call_expression = js_call_expression(
            inner_callee,
            js_call_arguments(
                token(T!['(']),
                js_call_argument_list(
                    vec![AnyJsCallArgument::AnyJsExpression(
                        AnyJsExpression::AnyJsLiteralExpression(
                            biome_js_syntax::AnyJsLiteralExpression::JsStringLiteralExpression(
                                string_literal,
                            ),
                        ),
                    )],
                    vec![],
                ),
                token(T![')']),
            ),
        )
        .build();
        let arguments = js_call_arguments(
            token(T!['(']),
            js_call_argument_list(
                vec![
                    AnyJsCallArgument::AnyJsExpression(AnyJsExpression::JsIdentifierExpression(
                        js_identifier_expression(js_reference_identifier(ident(
                            transform_options.parent_id?.as_str(),
                        ))),
                    )),
                    AnyJsCallArgument::AnyJsExpression(inner_call_expression.into()),
                ],
                vec![token(T!(,))],
            ),
            token(T![')']),
        );
        Some(AnyJsExpression::JsCallExpression(
            js_call_expression(callee, arguments).build(),
        ))
    }

    pub fn transform_js_metavariable(&self, _node: &JsMetavariable) -> Option<AnyJsExpression> {
        todo!()
    }

    pub fn transform_jsx_expression_child(
        &self,
        _node: &JsxExpressionChild,
    ) -> Option<AnyJsExpression> {
        todo!()
    }

    pub fn transform_jsx_spread_child(&self, _node: &JsxSpreadChild) -> Option<AnyJsExpression> {
        todo!()
    }
}
