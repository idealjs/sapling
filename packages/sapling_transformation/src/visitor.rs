use biome_js_factory::make::{js_function_body, js_parameter_list, js_parameters};
use biome_js_syntax::{
    AnyJsExpression, AnyJsxTag, JsLanguage, JsModule, JsParenthesizedExpression, JsSyntaxKind,
    JsxElement,
};
use biome_rowan::{
    AstNode, BatchMutation, BatchMutationExt, SyntaxNode, SyntaxNodeCast, SyntaxNodeChildren,
    SyntaxNodeOptionExt,
};

use crate::{
    CreateTemplate, DomTemplate, SsrTemplate, TemplateInput, UniversalTemplate,
    helpers::jsx_template::{
        make_js_arrow_function_expression, make_js_call_expression, make_js_function_body,
        make_js_parameters,
    },
    is_component,
    is_valid_html_nesting::is_valid_html_nesting,
};

pub struct SaplingVisitor {
    pub mutation: BatchMutation<JsLanguage>,
    pub js_module: JsModule,
    pub pre_process_errors: Vec<String>,
}

pub struct TransformNodePathInfo {
    pub top_level: Option<bool>,
    pub last_element: Option<bool>,
}

impl SaplingVisitor {
    pub fn traverse(&mut self) {
        self.mutation = self.js_module.clone().begin();
        self.pre_process();
        let descendants = self.js_module.syntax().descendants();

        descendants
            .filter(|node| matches!(node.kind(), JsSyntaxKind::JSX_TAG_EXPRESSION))
            .filter(|node| {
                let next = !node
                    .parent()
                    .map_or(false, |p| AnyJsxTag::can_cast(p.kind()));
                println!(
                    "Node: {:?}, Parent: {:?}, Next: {}",
                    node.kind(),
                    node.parent().map(|p| p.kind()),
                    next
                );
                next
            })
            .for_each(|node| self.transform_jsx(node));

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
        // AnyJsxTag
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
        let Some(result) = self.transform_node_path(&node_path, info) else {
            return;
        };
        let mut create_template = self.get_create_template(&result);
        let template = create_template.create_template(&result, Some(false));
        let new_node = update_template(template);

        println!("Parent Kind: {:?}", node_path.parent().kind());
        println!("Node Kind: {:?}", node_path.kind());
        println!("Node Kind: {:?}", node_path);
        println!("New node: {:?}", new_node);
        self.mutation
            .replace_element(node_path.into(), new_node.into());
    }

    pub fn get_update_template(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
    ) -> impl Fn(AnyJsExpression) -> AnyJsExpression + 'static {
        let value: bool;

        let node_path_ref = node_path.clone();
        let update_template = |node: AnyJsExpression| -> AnyJsExpression { node };
        update_template
    }

    pub fn get_create_template(&self, result: &TemplateInput) -> Box<dyn CreateTemplate> {
        if result.tag_name.is_some() && result.renderer.as_deref() == Some("dom") {
            return Box::new(DomTemplate {});
        }

        if result.renderer.as_deref() == Some("ssr") {
            return Box::new(SsrTemplate {});
        }
        Box::new(UniversalTemplate {})
    }

    pub fn transform_node_path(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
        info: TransformNodePathInfo,
    ) -> Option<TemplateInput> {
        let result: Option<TemplateInput> = match node_path.kind() {
            JsSyntaxKind::JSX_ELEMENT => self.transform_element(node_path, &info),
            JsSyntaxKind::JSX_FRAGMENT => {
                let mut result = TemplateInput {
                    id: None,
                    declarations: vec![],
                    exprs: vec![],
                    dynamics: vec![],
                    post_exprs: vec![],
                    tag_name: None,
                    template: None,
                    dynamic: None,
                    renderer: None,
                    text: false,
                };
                self.transform_fragment_children(node_path, &mut result);
                Some(result)
            }
            JsSyntaxKind::JSX_TEXT => {
                //              const text =
                //   staticValue !== undefined
                //     ? info.doNotEscape
                //       ? staticValue.toString()
                //       : escapeHTML(staticValue.toString())
                //     : trimWhitespace(node.extra.raw);
                // if (!text.length) return null;
                let mut result = TemplateInput {
                    declarations: vec![],
                    exprs: vec![],
                    dynamics: vec![],
                    post_exprs: vec![],
                    text: true,
                    id: None,
                    tag_name: None,
                    template: None,
                    dynamic: None,
                    renderer: None,
                };
                // const results = {
                //   template: text,
                //   declarations: [],
                //   exprs: [],
                //   dynamics: [],
                //   postExprs: [],
                //   text: true
                // };
                // if (!info.skipId && config.generate !== "ssr")
                //   results.id = path.scope.generateUidIdentifier("el$");
                // return results;
                None
            }
            JsSyntaxKind::JSX_EXPRESSION_CHILD => None,
            JsSyntaxKind::JSX_SPREAD_CHILD => None,
            _ => {
                return None;
            }
        };

        result
    }

    pub fn transform_fragment_children(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
        result: &mut TemplateInput,
    ) {
    }

    pub fn transform_element(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
        info: &TransformNodePathInfo,
    ) -> Option<TemplateInput> {
        None
    }
    pub fn transform_element_dom(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
        info: &TransformNodePathInfo,
    ) {
    }
    pub fn transform_element_ssr(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
        info: &TransformNodePathInfo,
    ) {
    }
    pub fn transform_element_universal(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
        info: &TransformNodePathInfo,
    ) {
    }
    pub fn transform_condition(
        &self,
        node_path: &SyntaxNode<JsLanguage>,
        inline: bool,
        deep: bool,
    ) {
        node_path.children().for_each(|node| {});
    }
    pub fn transform_component_children(&self, children: SyntaxNodeChildren<JsLanguage>) {}
}
