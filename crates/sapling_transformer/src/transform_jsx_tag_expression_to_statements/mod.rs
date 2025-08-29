use std::vec;

use crate::{
    SaplingTransformer, TransformAnyJsxTextOptions, jsx_element_name_to_string, make_insert_node,
    transfrom_jsx_tag_expression::TransformAnyJsxFragmentOptions,
};
use crate::{
    make_arrow_function_from_statement, make_create_element, make_effect, make_insert,
    make_set_prop,
};
use biome_js_factory::make::js_expression_statement;
use biome_js_syntax::{
    AnyJsExpression, AnyJsStatement, AnyJsxAttribute, AnyJsxAttributeValue, AnyJsxChild,
    JsxElement, JsxExpressionChild, JsxFragment, JsxSelfClosingElement, JsxText,
};
use biome_rowan::AstNode;

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
    pub parent_id: Option<String>,
}

pub struct TransformJsxExpressionChildToStatementsOptions {
    pub parent_id: Option<String>,
}

impl SaplingTransformer<'_> {
    pub fn transform_jsx_element_to_statements(
        &mut self,
        node: &JsxElement,
    ) -> Option<(Vec<AnyJsStatement>, String)> {
        let mut statments: Vec<AnyJsStatement> = vec![];
        let tag_name = jsx_element_name_to_string(&node.opening_element().ok()?.name().ok()?)?;
        let scope = self.semantic_model.scope(node.syntax());
        let id = self.generate_unique_identifier(&scope, "_el$");
        let js_tag_statement = make_create_element(id.as_str(), tag_name.as_str());
        statments.push(js_tag_statement);

        let attributes = node.opening_element().ok()?.attributes();
        attributes.into_iter().for_each(|attribute| {
            let set_prop_statement = make_set_prop(id.as_str(), &attribute);
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
                statments.push(AnyJsStatement::JsExpressionStatement(
                    js_expression_statement(
                        make_insert_node(id.as_str(), child_id.as_str()).into(),
                    )
                    .build(),
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
        let expression = self.transform_jsx_fragment(
            node,
            TransformAnyJsxFragmentOptions {
                parent_id: transform_options.parent_id.clone(),
            },
        )?;

        let parent_id = transform_options.parent_id?;
        let call_expr = make_insert(parent_id.as_str(), expression);
        Some(vec![AnyJsStatement::JsExpressionStatement(
            js_expression_statement(call_expr.into()).build(),
        )])
    }

    pub fn transform_any_jsx_child_to_statements(
        &mut self,
        node: &AnyJsxChild,
        transform_options: TransformAnyJsxChildToStatementsOptions,
    ) -> Option<(Vec<AnyJsStatement>, Option<String>)> {
        match node {
            AnyJsxChild::JsxElement(node) => {
                let (statements, id) = self.transform_jsx_element_to_statements(node)?;
                Some((statements, Some(id)))
            }
            AnyJsxChild::JsxExpressionChild(node) => {
                let statements = self.transform_jsx_expression_child_to_statements(
                    node,
                    TransformJsxExpressionChildToStatementsOptions {
                        parent_id: Some(transform_options.parent_id?.clone()),
                    },
                )?;
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
            AnyJsxChild::JsxSelfClosingElement(node) => self
                .transform_jsx_self_closing_element_to_statements(node)
                .map(|(stmts, id)| (stmts, Some(id))),

            AnyJsxChild::JsxText(node) => {
                let statements = self.transform_jsx_text_to_statements(
                    node,
                    TransformAnyJsxChildToStatementsOptions {
                        parent_id: transform_options.parent_id.clone(),
                    },
                )?;
                Some((statements, None))
            }
            _ => {
                unreachable!()
            }
        }
    }

    pub fn transform_jsx_expression_child_to_statements(
        &mut self,
        node: &JsxExpressionChild,
        transform_options: TransformJsxExpressionChildToStatementsOptions,
    ) -> Option<Vec<AnyJsStatement>> {
        let expr = self.transform_jsx_expression_child(
            node,
            crate::TransformJsxExpressionChildOptions {
                parent_id: transform_options.parent_id,
            },
        );

        Some(vec![AnyJsStatement::JsExpressionStatement(
            js_expression_statement(expr?.into()).build(),
        )])
    }

    pub fn transform_jsx_text_to_statements(
        &self,
        node: &JsxText,
        transform_options: TransformAnyJsxChildToStatementsOptions,
    ) -> Option<Vec<AnyJsStatement>> {
        let expr = self.transform_jsx_text(
            node,
            TransformAnyJsxTextOptions {
                parent_id: transform_options.parent_id,
            },
        )?;

        Some(vec![AnyJsStatement::JsExpressionStatement(
            js_expression_statement(expr).build(),
        )])
    }

    pub fn transform_jsx_self_closing_element_to_statements(
        &mut self,
        node: &JsxSelfClosingElement,
    ) -> Option<(Vec<AnyJsStatement>, String)> {
        let mut statments: Vec<AnyJsStatement> = vec![];
        let tag_name = jsx_element_name_to_string(&node.name().ok()?)?;
        let scope = self.semantic_model.scope(node.syntax());
        let id = self.generate_unique_identifier(&scope, "_el$");
        let js_tag_statement = make_create_element(id.as_str(), tag_name.as_str());
        statments.push(js_tag_statement);

        let attributes = node.attributes();
        attributes.into_iter().for_each(|attribute| {
            let set_prop_statement = make_set_prop(id.as_str(), &attribute);
            let is_call_expr = match &attribute {
                AnyJsxAttribute::JsxAttribute(attr) => {
                    let expr = attr
                        .initializer()
                        .and_then(|init| init.value().ok())
                        .and_then(|val| match val {
                            AnyJsxAttributeValue::JsxExpressionAttributeValue(expr_val) => {
                                expr_val.expression().ok()
                            }
                            _ => None,
                        });
                    matches!(expr, Some(AnyJsExpression::JsCallExpression(_)))
                }
                _ => false,
            };
            match set_prop_statement {
                Some(set_prop_statement) => {
                    if is_call_expr {
                        statments.push(AnyJsStatement::JsExpressionStatement(
                            js_expression_statement(AnyJsExpression::JsCallExpression(
                                make_effect(
                                    AnyJsExpression::JsArrowFunctionExpression(
                                        make_arrow_function_from_statement(set_prop_statement),
                                    ),
                                    Vec::new(),
                                ),
                            ))
                            .build(),
                        ));
                    } else {
                        statments.push(set_prop_statement);
                    }
                }
                None => {
                    return;
                }
            }
        });

        Some((statments, id))
    }
}
