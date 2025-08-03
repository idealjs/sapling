use std::vec;

use crate::{
    SaplingTransformer, TransformAnyJsxTextOptions, jsx_element_name_to_string,
    transfrom_jsx_tag_expression::TransformAnyJsxFragmentOptions,
};
use biome_js_factory::make::{
    ident, js_call_expression, js_expression_statement, js_identifier_expression,
    js_reference_identifier, js_return_statement, js_string_literal, js_string_literal_expression,
    token,
};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsStatement, AnyJsxChild, AnyJsxTag, JsMetavariable,
    JsxElement, JsxExpressionChild, JsxFragment, JsxSelfClosingElement, JsxSpreadChild,
    JsxTagExpression, JsxText, T,
};
use biome_rowan::AstNode;
use sapling_transformation::helpers::jsx_template::{make_iife, make_js_call_arguments};

pub struct TransformJsxElementToStatementsOptions {
    pub need_return: bool,
}

pub struct TransformAnyJsxChildToStatementsOptions {
    pub parent_id: Option<String>,
}

pub struct TransformJsxFragmentToStatementsOptions {
    pub parent_id: Option<String>,
}

pub struct TransformJsxTextToStatementsOptions {
    pub parent_id: String,
}

// todo none used
impl SaplingTransformer {
    pub fn transform_jsx_element_to_statements(
        &mut self,
        node: &JsxElement,
    ) -> Option<(Vec<AnyJsStatement>, String)> {
        let mut statments: Vec<AnyJsStatement> = vec![];
        let tag_name = jsx_element_name_to_string(&node.opening_element().ok()?.name().ok()?)?;
        let scope = self.semantic_model.scope(node.syntax());
        let id = self.generate_unique_identifier(&scope, "_el$");
        let js_tag_statement = self.create_js_tag_statement(id.as_str(), tag_name.as_str());
        statments.push(js_tag_statement);

        let attributes = node.opening_element().ok()?.attributes();
        attributes.into_iter().for_each(|attribute| {
            let set_prop_statement = self.create_set_prop_statement(id.as_str(), attribute);
            match set_prop_statement {
                Some(set_prop_statement) => {
                    statments.push(set_prop_statement);
                }
                None => {
                    return;
                }
            }
        });

        // Handle children
        let children = node.children();
        children.into_iter().for_each(|node| {
            let Some((statements, child_id)) = self.transform_any_jsx_child_to_statements(
                &node,
                TransformAnyJsxChildToStatementsOptions {
                    parent_id: Some(id.clone()),
                },
            ) else {
                return;
            };
            statments.extend(statements);
            if let Some(child_id) = child_id {
                // _$insertNode(id, child_id);
                let callee =
                    js_identifier_expression(js_reference_identifier(ident("_$insertNode")));
                let arg1 =
                    AnyJsCallArgument::AnyJsExpression(AnyJsExpression::JsIdentifierExpression(
                        js_identifier_expression(js_reference_identifier(ident(id.as_str()))),
                    ));
                let arg2 =
                    AnyJsCallArgument::AnyJsExpression(AnyJsExpression::JsIdentifierExpression(
                        js_identifier_expression(js_reference_identifier(ident(child_id.as_str()))),
                    ));
                let call_expr = js_call_expression(
                    callee.into(),
                    make_js_call_arguments(vec![arg1, arg2], vec![token(T!(,))]),
                )
                .build();
                let insert_node_statement =
                    js_expression_statement(AnyJsExpression::JsCallExpression(call_expr));
                statments.push(AnyJsStatement::JsExpressionStatement(
                    insert_node_statement.build(),
                ));
            }
        });

        Some((statments, id))
    }
    pub fn transform_jsx_fragment_to_insert_statement(
        &mut self,
        node: &JsxFragment,
        transform_options: TransformJsxFragmentToStatementsOptions,
    ) -> Option<Vec<AnyJsStatement>> {
        let (expression, _) = self.transform_jsx_fragment(
            node,
            TransformAnyJsxFragmentOptions {
                parent_id: transform_options.parent_id.clone(),
            },
        )?;

        let callee = js_identifier_expression(js_reference_identifier(ident("_$insert")));

        let arg1 = AnyJsCallArgument::AnyJsExpression(AnyJsExpression::JsIdentifierExpression(
            js_identifier_expression(js_reference_identifier(ident(
                transform_options.parent_id?.as_str(),
            ))),
        ));

        let call_expr = js_call_expression(
            callee.into(),
            make_js_call_arguments(
                vec![arg1, AnyJsCallArgument::AnyJsExpression(expression?)],
                vec![token(T!(,))],
            ),
        )
        .build();
        Some(vec![AnyJsStatement::JsExpressionStatement(
            js_expression_statement(call_expr.into()).build(),
        )])
    }
    pub fn transform_jsx_self_closing_element_to_statements(
        &self,
        node: &JsxSelfClosingElement,
    ) -> Option<Vec<AnyJsStatement>> {
        todo!()
    }
    pub fn transform_any_jsx_child_to_statements(
        &mut self,
        node: &AnyJsxChild,
        transform_options: TransformAnyJsxChildToStatementsOptions,
    ) -> Option<(Vec<AnyJsStatement>, Option<String>)> {
        match node {
            AnyJsxChild::JsMetavariable(node) => {
                let statements = self.transform_js_metavariable_to_statements(node)?;
                Some((statements, None))
            }
            AnyJsxChild::JsxElement(node) => {
                let (statements, id) = self.transform_jsx_element_to_statements(node)?;
                Some((statements, Some(id)))
            }
            AnyJsxChild::JsxExpressionChild(node) => {
                let statements = self.transform_jsx_expression_child_to_statements(node)?;
                Some((statements, None))
            }
            AnyJsxChild::JsxFragment(node) => {
                let statements = self.transform_jsx_fragment_to_insert_statement(
                    node,
                    TransformJsxFragmentToStatementsOptions {
                        parent_id: transform_options.parent_id.clone(),
                    },
                )?;
                Some((statements, None))
            }
            AnyJsxChild::JsxSelfClosingElement(node) => {
                let statements = self.transform_jsx_self_closing_element_to_statements(node)?;
                Some((statements, None))
            }
            AnyJsxChild::JsxSpreadChild(node) => {
                let statements = self.transform_jsx_spread_child_to_statements(node)?;
                Some((statements, None))
            }
            AnyJsxChild::JsxText(node) => {
                let statements = self.transform_jsx_text_to_statements(
                    node,
                    TransformAnyJsxChildToStatementsOptions {
                        parent_id: transform_options.parent_id.clone(),
                    },
                )?;
                Some((statements, None))
            }
        }
    }
    pub fn transform_js_metavariable_to_statements(
        &self,
        node: &JsMetavariable,
    ) -> Option<Vec<AnyJsStatement>> {
        todo!()
    }
    pub fn transform_jsx_expression_child_to_statements(
        &self,
        node: &JsxExpressionChild,
    ) -> Option<Vec<AnyJsStatement>> {
        todo!()
    }

    pub fn transform_jsx_spread_child_to_statements(
        &self,
        node: &JsxSpreadChild,
    ) -> Option<Vec<AnyJsStatement>> {
        todo!()
    }
    pub fn transform_jsx_text_to_statements(
        &self,
        node: &JsxText,
        transform_options: TransformAnyJsxChildToStatementsOptions,
    ) -> Option<Vec<AnyJsStatement>> {
        let (expr, _) = self.transform_jsx_text(
            node,
            TransformAnyJsxTextOptions {
                parent_id: transform_options.parent_id,
            },
        )?;

        Some(vec![AnyJsStatement::JsExpressionStatement(
            js_expression_statement(expr?).build(),
        )])
    }
}
