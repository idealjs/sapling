use oxc_allocator::{Allocator, CloneIn, Vec as AstVec};
use oxc_ast::AstType::Argument;
use oxc_ast::ast::{
    ArrayExpression, ArrayExpressionElement, BinaryExpression, BinaryOperator, CallExpression,
    ComputedMemberExpression, Expression, IdentifierReference, NumericLiteral, Program,
    StringLiteral,
};
use oxc_span::{Atom, Span};
use oxc_syntax::number::NumberBase;
use oxc_traverse::TraverseCtx;
use std::cell::Cell;

#[derive(Debug)]
pub struct TemplateItem<'a> {
    pub id: &'a Expression<'a>,
    pub template: &'a Expression<'a>,
}

#[derive(Debug)]
pub struct TemplateResult<'a> {
    pub template: Option<Vec<String>>,
    pub template_values: Option<Vec<Expression<'a>>>,
    pub wont_escape: bool,
}

pub fn create_template<'a>(
    ctx: &mut TraverseCtx<'a>,
    allocator: &'a Allocator,
    path: &mut Program<'a>,
    result: &TemplateResult<'a>,
) {
    // // If no template exists, return first expression
    // if result.template.is_none() {
    //     return CloneIn::clone_in(&result.template_values.as_ref().unwrap()[0], allocator);
    // }

    // let template = match &result.template {
    //     None => return CloneIn::clone_in(&result.template_values.as_ref().unwrap()[0], allocator),
    //     Some(tmpl) => {
    //         if tmpl.len() == 1 {
    //             // Single string template
    //             Expression::StringLiteral(ctx.alloc(StringLiteral {
    //                 span: Span::default(),
    //                 value: Atom::from(&*tmpl[0]),
    //                 raw: None,
    //                 lone_surrogates: false,
    //             }))
    //         } else {
    //             // Array of strings template
    //             let mut elements = AstVec::new_in(allocator);
    //             for t in tmpl {
    //                 let str_lit = Expression::StringLiteral(ctx.alloc(StringLiteral {
    //                     span: Span::default(),
    //                     value: Atom::from(t.as_str()),
    //                     raw: None,
    //                     lone_surrogates: false,
    //                 }));
    //                 let element = ArrayExpressionElement::Expression(str_lit);
    //                 elements.push(element);
    //             }

    //             Expression::ArrayExpression(ctx.alloc(ArrayExpression {
    //                 span: Span::default(),
    //                 elements,
    //             }))
    //         }
    //     }
    // };

    // // Generate unique identifier
    // let id = Expression::Identifier(ctx.alloc(IdentifierReference {
    //     span: Span::default(),
    //     name: Atom::from("tmpl$"), // TODO: Generate unique name
    //     reference_id: Cell::new(None),
    // }));

    // // Handle wont_escape case
    // if result.wont_escape {
    //     if result.template.as_ref().map_or(false, |t| t.len() == 1) {
    //         return id;
    //     }

    //     // Check for hydration key optimization case
    //     if result.template.as_ref().map_or(false, |t| t.len() == 2) {
    //         if let Some(values) = &result.template_values {
    //             if let Some(first_value) = values.first() {
    //                 if let Expression::CallExpression(call) = first_value {
    //                     if let Expression::Identifier(callee) = &call.callee {
    //                         if callee.name.as_str() == "_$ssrHydrationKey" {
    //                             // Create the optimized binary expression for hydration key
    //                             let member_expr0 = Expression::ComputedMemberExpression(ctx.alloc(
    //                                 ComputedMemberExpression {
    //                                     span: Span::default(),
    //                                     object: CloneIn::clone_in(&id, allocator),
    //                                     expression: Expression::NumericLiteral(ctx.alloc(
    //                                         NumericLiteral {
    //                                             span: Span::default(),
    //                                             value: 0.0,
    //                                             raw: None,
    //                                             base: NumberBase::Decimal,
    //                                         },
    //                                     )),
    //                                     optional: false,
    //                                 },
    //                             ));

    //                             let member_expr1 = Expression::ComputedMemberExpression(ctx.alloc(
    //                                 ComputedMemberExpression {
    //                                     span: Span::default(),
    //                                     object: CloneIn::clone_in(&id, allocator),
    //                                     expression: Expression::NumericLiteral(ctx.alloc(
    //                                         NumericLiteral {
    //                                             span: Span::default(),
    //                                             value: 1.0,
    //                                             raw: None,
    //                                             base: NumberBase::Decimal,
    //                                         },
    //                                     )),
    //                                     optional: false,
    //                                 },
    //                             ));

    //                             let inner_binary =
    //                                 Expression::BinaryExpression(ctx.alloc(BinaryExpression {
    //                                     span: Span::default(),
    //                                     operator: BinaryOperator::Addition,
    //                                     left: member_expr0,
    //                                     right: CloneIn::clone_in(first_value, allocator),
    //                                 }));

    //                             return Expression::BinaryExpression(ctx.alloc(BinaryExpression {
    //                                 span: Span::default(),
    //                                 operator: BinaryOperator::Addition,
    //                                 left: inner_binary,
    //                                 right: member_expr1,
    //                             }));
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // // Return SSR call expression
    // Expression::CallExpression(ctx.alloc(CallExpression {
    //     span: Span::default(),
    //     callee: Expression::Identifier(ctx.alloc(IdentifierReference {
    //         span: Span::default(),
    //         name: Atom::from("_$ssr"),
    //         reference_id: Cell::new(None),
    //     })),
    //     arguments: {
    //         let mut args = AstVec::new_in(allocator);
    //         let first_expr = CloneIn::clone_in(&id, allocator);
    //         args.push(ctx.alloc(Argument {
    //             span: Span::default(),
    //             kind: ArgumentKind::Expression(first_expr),
    //         }));
    //         if result.template.as_ref().map_or(false, |t| t.len() > 1) {
    //             if let Some(values) = &result.template_values {
    //                 for value in values {
    //                     let expr = CloneIn::clone_in(value, allocator);
    //                     args.push(ctx.alloc(Argument {
    //                         span: Span::default(),
    //                         kind: ArgumentKind::Expression(expr),
    //                     }));
    //                 }
    //             }
    //         }
    //         args
    //     },
    //     optional: false,
    //     type_arguments: None,
    //     pure: false,
    // }))
}
