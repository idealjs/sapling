use oxc_allocator::{Allocator, Box, CloneIn, Vec as AstVec};
use oxc_ast::ast::{
    BindingIdentifier, BindingPattern, BindingPatternKind, Expression, Program, Statement,
    TemplateLiteral, VariableDeclaration, VariableDeclarationKind, VariableDeclarator,
};
use oxc_span::Span;
use oxc_syntax::symbol::SymbolId;
use oxc_traverse::TraverseCtx;
use std::cell::Cell;

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

pub fn append_templates<'a>(
    ctx: &mut TraverseCtx<'a>,
    allocator: &'a Allocator,
    path: &mut Program<'a>,
    templates: &[TemplateItem<'a>],
) -> Result<(), &'static str> {
    let mut declarators = AstVec::new_in(allocator);

    for template in templates {
        if let Expression::Identifier(id) = template.id {
            // Create binding pattern with identifier
            let binding = BindingPattern {
                kind: BindingPatternKind::BindingIdentifier(ctx.alloc(BindingIdentifier {
                    span: id.span,
                    name: id.name.clone(),
                    symbol_id: Cell::new(Some(SymbolId::new(0))),
                })),
                type_annotation: None,
                optional: false,
            };

            // Create declarator
            let declarator = VariableDeclarator {
                span: Span::default(),
                id: binding,
                init: Some(CloneIn::clone_in(template.template, allocator)),
                definite: false,
                kind: VariableDeclarationKind::Var,
            };

            declarators.push(declarator);
        } else {
            return Err("Only identifier expressions are supported as template IDs");
        }
    }

    if declarators.is_empty() {
        return Ok(());
    }

    // Create and box variable declaration
    let declaration = ctx.alloc(VariableDeclaration {
        span: Span::default(),
        kind: VariableDeclarationKind::Var,
        declarations: declarators,
        declare: false,
    });

    path.body
        .insert(0, Statement::VariableDeclaration(declaration));
    Ok(())
}
