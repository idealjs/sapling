use std::vec;

use crate::{SaplingTransformer, jsx_element_name_to_string};
use biome_js_factory::make::{
    ident, js_call_argument_list, js_call_arguments, js_call_expression, js_expression_statement,
    js_identifier_expression, js_reference_identifier, js_return_statement, js_string_literal,
    js_string_literal_expression, token,
};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsStatement, AnyJsxChild, AnyJsxTag, JsMetavariable,
    JsxElement, JsxExpressionChild, JsxFragment, JsxSelfClosingElement, JsxSpreadChild,
    JsxTagExpression, JsxText, T,
};
use biome_rowan::AstNode;
use sapling_transformation::helpers::jsx_template::make_js_call_arguments;

pub struct TransformJsxElementToStatementsOptions {
    pub need_return: bool,
}

pub struct TransformAnyJsxChildToStatementsOptions {
    pub parent_id: String,
}

impl SaplingTransformer {
    pub fn transform_jsx_tag_expression_to_statements(
        &mut self,
        node: &JsxTagExpression,
    ) -> Option<Vec<AnyJsStatement>> {
        let tag = node.tag().ok()?;
        match tag {
            AnyJsxTag::JsxElement(node) => {
                let (statements, _id) = self.transform_jsx_element_to_statements(
                    &node,
                    TransformJsxElementToStatementsOptions { need_return: true },
                )?;
                Some(statements)
            }
            AnyJsxTag::JsxFragment(node) => self.transform_jsx_fragment_to_statements(&node),
            AnyJsxTag::JsxSelfClosingElement(node) => {
                self.transform_jsx_self_closing_element_to_statements(&node)
            }
        }
    }
    pub fn transform_jsx_element_to_statements(
        &mut self,
        node: &JsxElement,
        transform_options: TransformJsxElementToStatementsOptions,
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
                    parent_id: id.clone(),
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
                    AnyJsCallArgument::AnyJsExpression(AnyJsExpression::AnyJsLiteralExpression(
                        js_string_literal_expression(js_string_literal(id.as_str())).into(),
                    ));
                let arg2 =
                    AnyJsCallArgument::AnyJsExpression(AnyJsExpression::AnyJsLiteralExpression(
                        js_string_literal_expression(js_string_literal(child_id.as_str())).into(),
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

        transform_options.need_return.then(|| {
            let return_stmt = AnyJsStatement::JsReturnStatement(
                js_return_statement(token(T![return]))
                    .with_argument(
                        js_identifier_expression(js_reference_identifier(ident(&id))).into(),
                    )
                    .with_semicolon_token(token(T![;]))
                    .build(),
            );
            statments.push(return_stmt);
        });

        Some((statments, id))
    }
    pub fn transform_jsx_fragment_to_statements(
        &self,
        node: &JsxFragment,
    ) -> Option<Vec<AnyJsStatement>> {
        None
    }
    pub fn transform_jsx_self_closing_element_to_statements(
        &self,
        node: &JsxSelfClosingElement,
    ) -> Option<Vec<AnyJsStatement>> {
        None
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
                let (statements, id) = self.transform_jsx_element_to_statements(
                    node,
                    TransformJsxElementToStatementsOptions { need_return: false },
                )?;
                Some((statements, Some(id)))
            }
            AnyJsxChild::JsxExpressionChild(node) => {
                let statements = self.transform_jsx_expression_child_to_statements(node)?;
                Some((statements, None))
            }
            AnyJsxChild::JsxFragment(node) => {
                let statements = self.transform_jsx_fragment_to_statements(node)?;
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
                let statements =
                    self.transform_jsx_text_to_statements(&transform_options.parent_id, node)?;
                Some((statements, None))
            }
        }
    }
    pub fn transform_js_metavariable_to_statements(
        &self,
        node: &JsMetavariable,
    ) -> Option<Vec<AnyJsStatement>> {
        None
    }
    pub fn transform_jsx_element_to_to_statements(
        &self,
        node: &JsxElement,
    ) -> Option<Vec<AnyJsStatement>> {
        None
    }
    pub fn transform_jsx_expression_child_to_statements(
        &self,
        node: &JsxExpressionChild,
    ) -> Option<Vec<AnyJsStatement>> {
        None
    }

    pub fn transform_jsx_spread_child_to_statements(
        &self,
        node: &JsxSpreadChild,
    ) -> Option<Vec<AnyJsStatement>> {
        None
    }
    pub fn transform_jsx_text_to_statements(
        &self,
        parent_id: &str,
        node: &JsxText,
    ) -> Option<Vec<AnyJsStatement>> {
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
                        js_identifier_expression(js_reference_identifier(ident(parent_id))),
                    )),
                    AnyJsCallArgument::AnyJsExpression(inner_call_expression.into()),
                ],
                vec![token(T!(,))],
            ),
            token(T![')']),
        );
        Some(vec![AnyJsStatement::JsExpressionStatement(
            js_expression_statement(AnyJsExpression::JsCallExpression(
                js_call_expression(callee, arguments).build(),
            ))
            .build(),
        )])
    }
}
