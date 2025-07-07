// AST 通用表达类型引入
use biome_js_syntax::{AnyJsExpression, JsSyntaxToken, JsVariableDeclarator};
/// create_template 直接返回 AST 节点表达
pub trait CreateTemplate {
    fn create_template(&mut self, input: &TemplateInput, wrap: Option<bool>) -> AnyJsExpression;
}

#[derive(Debug, Clone)]
pub struct TemplateDynamic {
    pub elem: AnyJsExpression,
    pub key: String,
    pub value: AnyJsExpression,
}

#[derive(Debug, Clone)]
pub struct TemplateInput {
    pub id: Option<JsSyntaxToken>,
    pub declarations: Option<JsVariableDeclarator>,
    pub exprs: Vec<AnyJsExpression>,
    pub dynamics: Vec<TemplateDynamic>,
    pub post_exprs: Vec<AnyJsExpression>,
    pub tag_name: Option<String>,
    pub template: Option<String>,
    pub dynamic: Option<bool>,
    pub renderer: String,
}

pub struct UniversalTemplate;

impl CreateTemplate for UniversalTemplate {
    fn create_template(&mut self, _input: &TemplateInput, wrap: Option<bool>) -> AnyJsExpression {
        todo!()
    }
}

pub struct DomTemplate;

impl CreateTemplate for DomTemplate {
    fn create_template(&mut self, _input: &TemplateInput, wrap: Option<bool>) -> AnyJsExpression {
        todo!()
    }
}

pub struct SsrTemplate;

impl CreateTemplate for SsrTemplate {
    fn create_template(&mut self, _input: &TemplateInput, wrap: Option<bool>) -> AnyJsExpression {
        todo!()
    }
}
