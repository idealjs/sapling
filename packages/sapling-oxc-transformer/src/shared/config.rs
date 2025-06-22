use oxc_allocator::Allocator;
use oxc_ast::ast::{Program, Expression, Statement, Directive};
use oxc_span::{Atom, Span};

#[derive(Debug, Clone)]
pub struct Config<'a> {
    pub module_name: Atom<'a>,
    pub renderers: Option<Vec<Config<'a>>>,
    pub name: Atom<'a>,
    pub static_marker: Atom<'a>,
    pub memo_wrapper: Atom<'a>,
}

pub trait HasConfig<'a> {
    fn get_config(&self) -> Option<&Config<'a>>;
}

pub trait HasMetadata<'a> {
    fn metadata(&self) -> Option<&'a ProgramContext>;
}

impl<'a> HasConfig<'a> for Program<'a> {
    fn get_config(&self) -> Option<&Config<'a>> {
        // TODO: Properly implement config access through proper channels
        // For now use a placeholder implementation
        None
    }
}

impl<'a> HasMetadata<'a> for Directive<'a> {
    fn metadata(&self) -> Option<&'a ProgramContext> {
        None // TODO: Implement proper metadata access
    }
}

#[derive(Debug)]
pub struct StatementMetadata(ProgramContext);

pub fn get_config<'a, T: HasConfig<'a>>(ctx: &'a T) -> Option<&'a Config<'a>> {
    ctx.get_config()
}

pub fn get_renderer_config<'a, 'b>(ctx: &'a (impl HasConfig<'a> + 'a), renderer: &'b str) -> Option<&'a Config<'a>> {
    let config = get_config(ctx)?;
    config.renderers
        .as_ref()
        .and_then(|renderers| renderers.iter().find(|r| r.name == renderer))
        .or(Some(config))
}

#[derive(Debug)]
pub struct ProgramContext {
    pub config: Option<Config<'static>>,
}

pub struct ConfigBuilder<'a> {
    module_name: Option<Atom<'a>>,
    renderers: Option<Vec<Config<'a>>>,
    name: Option<Atom<'a>>,
    static_marker: Option<Atom<'a>>,
    memo_wrapper: Option<Atom<'a>>,
}

impl<'a> ConfigBuilder<'a> {
    pub fn new() -> Self {
        Self {
            module_name: None,
            renderers: None,
            name: None,
            static_marker: None,
            memo_wrapper: None,
        }
    }

    pub fn build(self) -> Option<Config<'a>> {
        Some(Config {
            module_name: self.module_name?,
            renderers: self.renderers,
            name: self.name?,
            static_marker: self.static_marker?,
            memo_wrapper: self.memo_wrapper?,
        })
    }
}
