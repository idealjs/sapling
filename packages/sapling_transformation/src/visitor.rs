use biome_js_syntax::{AnyJsExpression, JsLanguage, JsModule, JsSyntaxKind, JsxElement};
use biome_rowan::{AstNode, BatchMutation, BatchMutationExt, SyntaxNode, SyntaxNodeCast};

use crate::{
    CreateTemplate, DomTemplate, SsrTemplate, TemplateInput, UniversalTemplate, is_component,
    is_valid_html_nesting::is_valid_html_nesting,
};

pub struct SaplingVisitor {
    mutation: BatchMutation<JsLanguage>,
    js_module: JsModule,
    pre_process_errors: Vec<String>,
}

pub struct TransformNodePathInfo {
    pub top_level: Option<bool>,
    pub last_element: Option<bool>,
}

impl SaplingVisitor {
    pub fn traverse<L: biome_rowan::Language>(mut self) {
        self.mutation = self.js_module.clone().begin();
        self.pre_process();
        let descendants = self.js_module.syntax().descendants();
        descendants.for_each(|node_path| match node_path.kind() {
            JsSyntaxKind::JSX_ELEMENT => {
                self.transform_jsx(node_path);
            }
            JsSyntaxKind::JSX_FRAGMENT => {
                self.transform_jsx(node_path);
            }
            _ => {}
        });

        self.post_process();
    }
}

impl SaplingVisitor {
    pub fn pre_process(&mut self) {
        let descendants = self.js_module.syntax().descendants();
        descendants.for_each(|node| match node.kind() {
            JsSyntaxKind::JSX_ELEMENT => {
                let Some(node) = node.cast::<JsxElement>() else {
                    return;
                };
                let Some(parent) = node
                    .syntax()
                    .parent()
                    .and_then(|parent| parent.cast::<JsxElement>())
                else {
                    return;
                };

                let (Ok(el_tag), Ok(parent_tag)) = (
                    node.opening_element()
                        .and_then(|v| v.name())
                        .and_then(|v| v.name_value_token())
                        .and_then(|v| Ok(v.text().to_string())),
                    parent
                        .opening_element()
                        .and_then(|v| v.name())
                        .and_then(|v| v.name_value_token())
                        .and_then(|v| Ok(v.text().to_string())),
                ) else {
                    return;
                };

                let (el_tag, parent_tag) = (el_tag.as_str(), parent_tag.as_str());

                if is_component(el_tag) || is_component(parent_tag) {
                    if is_valid_html_nesting(parent_tag, el_tag) {
                        self.pre_process_errors.push(format!(
                            "Invalid JSX: <{}> cannot be child of <{}>",
                            el_tag, parent_tag
                        ));
                    }
                }
            }
            _ => {}
        });
    }

    pub fn post_process(&mut self) {}
    pub fn transform_jsx(&mut self, node_path: SyntaxNode<JsLanguage>) {
        let info = if node_path.kind() == JsSyntaxKind::JSX_FRAGMENT {
            TransformNodePathInfo {
                top_level: None,
                last_element: None,
            }
        } else {
            TransformNodePathInfo {
                top_level: Some(true),
                last_element: Some(true),
            }
        };
        let update_template = self.get_update_template(&node_path);
        let result = self.transform_node_path(&node_path, info);
        let mut create_template = self.get_create_template(&result);
        let template = create_template.create_template(&result, Some(false));
        let new_node = update_template(template);
        self.mutation
            .replace_element(node_path.into(), new_node.into());
    }

    pub fn get_update_template(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
    ) -> impl Fn(AnyJsExpression) -> AnyJsExpression + 'static {
        let value: bool;

        let node_path_ref = node_path.clone();
        let update_template = |node: AnyJsExpression| -> AnyJsExpression {
            // 这里将来可以安全使用 node_path_ref
            todo!()
        };
        update_template
    }

    pub fn get_create_template(&self, result: &TemplateInput) -> Box<dyn CreateTemplate> {
        if result.tag_name.is_some() && result.renderer == "dom" {
            return Box::new(DomTemplate {});
        }

        if result.renderer == "ssr" {
            return Box::new(SsrTemplate {});
        }
        Box::new(UniversalTemplate {})
    }

    pub fn transform_node_path(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
        info: TransformNodePathInfo,
    ) -> TemplateInput {
        return TemplateInput {
            id: todo!(),
            declarations: todo!(),
            exprs: todo!(),
            dynamics: todo!(),
            post_exprs: todo!(),
            tag_name: todo!(),
            template: todo!(),
            dynamic: todo!(),
            renderer: todo!(),
        };
    }
}
