use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, Program};

#[derive(Debug)]
pub struct TemplateItem<'a> {
    pub id: &'a Expression<'a>,
    pub template: &'a Expression<'a>,
}

pub fn create_template<'a>(
    _allocator: &'a Allocator,
    _path: &mut Program<'a>,
    _result: &TemplateItem<'a>,
) -> Expression<'a> {
    todo!()
}
