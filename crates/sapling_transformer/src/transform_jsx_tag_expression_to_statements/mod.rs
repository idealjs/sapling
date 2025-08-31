use std::vec;

use crate::{
    SaplingTransformer, TransformAnyJsxTextOptions, jsx_element_name_to_string, make_insert_node,
    transfrom_jsx_tag_expression::TransformAnyJsxFragmentOptions,
};
use crate::{
    make_array, make_arrow_function_from_statement, make_create_component, make_create_element,
    make_effect, make_insert, make_set_prop,
};
use biome_js_factory::make::{
    ident, js_expression_statement, js_string_literal, js_string_literal_expression, jsx_attribute,
    jsx_attribute_initializer_clause, jsx_expression_attribute_value, jsx_name, jsx_tag_expression,
    token,
};
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsStatement, AnyJsxAttribute, AnyJsxAttributeName,
    AnyJsxAttributeValue, AnyJsxChild, AnyJsxTag, JsxAttributeList, JsxChildList, JsxElement,
    JsxExpressionChild, JsxFragment, JsxSelfClosingElement, JsxTagExpression, JsxText, T,
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

pub struct TransformJsxAttributeListOptions {
    pub parent_id: Option<String>,
}

pub struct TransformJsxChildListOptions {
    pub parent_id: Option<String>,
    pub is_component: bool,
}

impl SaplingTransformer<'_> {
    pub fn transform_jsx_element(
        &mut self,
        node: &JsxElement,
    ) -> Option<(Vec<AnyJsStatement>, String)> {
        let tag_name = jsx_element_name_to_string(&node.opening_element().ok()?.name().ok()?)?;
        let mut statements: Vec<AnyJsStatement> = vec![];
        let scope = self.semantic_model.scope(node.syntax());
        let id = self.generate_unique_identifier(&scope, "_el$");
        let is_component = tag_name.chars().next()?.is_uppercase();

        if is_component {
            let js_tag_statement = make_create_component(id.as_str(), tag_name.as_str());
            statements.push(js_tag_statement);
        } else {
            let js_tag_statement = make_create_element(id.as_str(), tag_name.as_str());
            statements.push(js_tag_statement);
        }
        let attributes = node.opening_element().ok()?.attributes();
        let attr_stmts = self.transform_jsx_attribute_list(
            &attributes,
            TransformJsxAttributeListOptions {
                parent_id: Some(id.clone()),
            },
        )?;
        statements.extend(attr_stmts);

        let jsx_child_list = node.children();

        let child_stmts = self.transform_jsx_child_list(
            &jsx_child_list,
            TransformJsxChildListOptions {
                parent_id: Some(id.clone()),
                is_component,
            },
        )?;
        statements.extend(child_stmts);

        Some((statements, id))
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
                let (statements, id) = self.transform_jsx_element(node)?;
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
        let attr_stmts = self.transform_jsx_attribute_list(
            &attributes,
            TransformJsxAttributeListOptions {
                parent_id: Some(id.clone()),
            },
        )?;
        statments.extend(attr_stmts);
        Some((statments, id))
    }

    pub fn transform_jsx_attribute_list(
        &mut self,
        node: &JsxAttributeList,
        transform_options: TransformJsxAttributeListOptions,
    ) -> Option<Vec<AnyJsStatement>> {
        let mut statements = vec![];
        let parent_id = transform_options.parent_id?;

        for attribute in node.into_iter() {
            // 先从属性节点提取 prop_key 与 prop_value，如果不是 JsxAttribute 则跳过
            let set_prop_result = match &attribute {
                AnyJsxAttribute::JsxAttribute(attr) => {
                    // 提取属性名字符串（支持 namespace)
                    let name_node = match attr.name().ok() {
                        Some(n) => n,
                        None => continue,
                    };
                    let name_token = match name_node {
                        AnyJsxAttributeName::JsxName(name) => {
                            let tok = match name.value_token().ok() {
                                Some(t) => t,
                                None => continue,
                            };
                            // 直接转换为 String
                            tok.text_trimmed().to_string()
                        }
                        AnyJsxAttributeName::JsxNamespaceName(name) => {
                            let ns = match name.namespace().ok() {
                                Some(n) => n,
                                None => continue,
                            };
                            let ns_token = match ns.value_token().ok() {
                                Some(t) => t,
                                None => continue,
                            };
                            let ns = ns_token.text_trimmed();
                            let name_val = match name.name().ok() {
                                Some(nv) => nv,
                                None => continue,
                            };
                            let nm_token = match name_val.value_token().ok() {
                                Some(t) => t,
                                None => continue,
                            };
                            let nm = nm_token.text_trimmed();
                            format!("{ns}:{nm}")
                        }
                    };

                    // 提取属性值（attr 是 &JsxAttribute），并将 AnyJsxAttributeValue 转为 AnyJsExpression 再传给 make_set_prop
                    if let Some(value) = attr.initializer().and_then(|init| init.value().ok()) {
                        let expr_opt: Option<AnyJsExpression> = match value {
                            AnyJsxAttributeValue::JsxString(str_val) => {
                                if let Some(tok) = str_val.value_token().ok() {
                                    Some(AnyJsExpression::AnyJsLiteralExpression(
                                        js_string_literal_expression(tok).into(),
                                    ))
                                } else {
                                    None
                                }
                            }
                            AnyJsxAttributeValue::JsxExpressionAttributeValue(expr_val) => {
                                expr_val.expression().ok()
                            }
                            AnyJsxAttributeValue::AnyJsxTag(_) => {
                                // 保持原有 TODO 行为：无法转换则跳过
                                None
                            }
                        };

                        match expr_opt {
                            Some(expr) => {
                                make_set_prop(parent_id.as_str(), name_token.as_str(), expr)
                            }
                            None => None,
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            };

            let should_effect = match &attribute {
                AnyJsxAttribute::JsxAttribute(attr) => {
                    let value = attr.initializer().and_then(|attr| attr.value().ok());

                    match value {
                        Some(AnyJsxAttributeValue::JsxString(_)) => false,
                        Some(AnyJsxAttributeValue::AnyJsxTag(_)) => false,
                        Some(AnyJsxAttributeValue::JsxExpressionAttributeValue(value)) => {
                            match value.expression().ok() {
                                Some(AnyJsExpression::JsArrowFunctionExpression(_)) => false,
                                Some(AnyJsExpression::AnyJsLiteralExpression(_)) => false,
                                Some(AnyJsExpression::JsxTagExpression(_)) => false,
                                _ => true,
                            }
                        }
                        _ => true,
                    }
                }
                _ => true,
            };

            match set_prop_result {
                Some(set_prop_statement) => {
                    if should_effect {
                        // 从 attribute 中提取 listener 表达式（如果是表达式属性）
                        let mut listeners: Vec<AnyJsExpression> = vec![];
                        if let AnyJsxAttribute::JsxAttribute(attr) = &attribute {
                            if let Some(value) =
                                attr.initializer().and_then(|attr| attr.value().ok())
                            {
                                if let AnyJsxAttributeValue::JsxExpressionAttributeValue(expr_val) =
                                    value
                                {
                                    if let Some(expr) = expr_val.expression().ok() {
                                        listeners.push(expr.clone());
                                    }
                                }
                            }
                        }
                        let arrow_expr = AnyJsExpression::JsArrowFunctionExpression(
                            make_arrow_function_from_statement(set_prop_statement),
                        );
                        statements.push(AnyJsStatement::JsExpressionStatement(
                            js_expression_statement(AnyJsExpression::JsCallExpression(
                                make_effect(arrow_expr, listeners),
                            ))
                            .build(),
                        ));
                    } else {
                        statements.push(set_prop_statement)
                    }
                }
                None => {
                    continue;
                }
            }
        }

        Some(statements)
    }

    pub fn transform_jsx_child_list(
        &mut self,
        node: &JsxChildList,
        transform_options: TransformJsxChildListOptions,
    ) -> Option<Vec<AnyJsStatement>> {
        let mut statements = vec![];
        let parent_id = transform_options.parent_id?;
        let is_component = transform_options.is_component;
        if is_component {
            // For component parents, collect children as expressions and set as a "children" prop:
            // set_prop(_el$, "children", [ childExpr1, childExpr2 ])
            let mut elements: Vec<AnyJsArrayElement> = vec![];
            node.into_iter().for_each(|node| {
                let el = match node {
                    AnyJsxChild::JsMetavariable(node) => None,
                    AnyJsxChild::JsxElement(node) => Some(AnyJsExpression::JsxTagExpression(
                        jsx_tag_expression(AnyJsxTag::JsxElement(node)),
                    )),
                    AnyJsxChild::JsxExpressionChild(node) => node.expression(),
                    AnyJsxChild::JsxFragment(node) => None,
                    AnyJsxChild::JsxSelfClosingElement(node) => None,
                    AnyJsxChild::JsxSpreadChild(node) => None,
                    AnyJsxChild::JsxText(node) => None,
                };
                if let Some(el) = el {
                    elements.push(AnyJsArrayElement::AnyJsExpression(el));
                }
            });
            println!("test test {:?}", node);

            if let Some(set_stmt) = make_set_prop(
                parent_id.as_str(),
                "children",
                AnyJsExpression::JsArrayExpression(make_array(elements)),
            ) {
                statements.push(set_stmt);
            }
        } else {
            // Handle children
            node.into_iter().for_each(|node| {
                let Some((child_stmts, child_id)) = self.transform_any_jsx_child_to_statements(
                    &node,
                    TransformAnyJsxChildToStatementsOptions {
                        parent_id: Some(parent_id.clone()),
                    },
                ) else {
                    return;
                };
                statements.extend(child_stmts);
                if let Some(child_id) = child_id {
                    statements.push(AnyJsStatement::JsExpressionStatement(
                        js_expression_statement(
                            make_insert_node(parent_id.as_str(), child_id.as_str()).into(),
                        )
                        .build(),
                    ));
                }
            });
        }
        Some(statements)
    }
}
