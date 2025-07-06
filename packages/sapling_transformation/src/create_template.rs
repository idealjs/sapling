// AST 通用表达类型引入
use biome_js_syntax::{AnyJsExpression, JsSyntaxToken, JsVariableDeclarator};
/// create_template 直接返回 AST 节点表达
pub trait CreateTemplate {
    fn create_template(
        &mut self,
        input: TemplateInput,
        wrap: Option<bool>,
    ) -> Result<AnyJsExpression, String>;
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
}

pub struct UniversalTemplate;

impl CreateTemplate for UniversalTemplate {
    fn create_template(
        &mut self,
        _input: TemplateInput,
        wrap: Option<bool>,
    ) -> Result<AnyJsExpression, String> {
        // TODO: 构造 universal 模式下的 AST 节点
        Err("universal AST 构造未实现".to_string())
    }
}

pub struct DomTemplate;

impl CreateTemplate for DomTemplate {
    fn create_template(
        &mut self,
        _input: TemplateInput,
        wrap: Option<bool>,
    ) -> Result<AnyJsExpression, String> {
        // TODO: 构造 dom 模式下的 AST 节点
        Err("dom AST 构造未实现".to_string())
    }
}

pub struct SsrTemplate;

impl CreateTemplate for SsrTemplate {
    fn create_template(
        &mut self,
        _input: TemplateInput,
        wrap: Option<bool>,
    ) -> Result<AnyJsExpression, String> {
        // TODO: 构造 ssr/dom' 模式下的 AST 节点
        Err("ssr AST 构造未实现".to_string())
    }
}
