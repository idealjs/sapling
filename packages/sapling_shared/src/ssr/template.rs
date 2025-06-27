use crate::{Template, TreeBuilderMut, create_import_reference, register_import_method};
use oxc_allocator::{Allocator, Box, CloneIn, FromIn, IntoIn, Vec};
use oxc_ast::AstKind;
use oxc_ast::ast::{
    Argument, ArrayExpression, ArrayExpressionElement, BinaryExpression, BinaryOperator,
    CallExpression, ComputedMemberExpression, Expression, IdentifierReference, MemberExpression,
    NumberBase, NumericLiteral, Program, ReturnStatement, Statement, StringLiteral,
};
use oxc_semantic::{NodeId, Scoping, SymbolFlags};
use oxc_span::{Atom, Span};

fn expression_to_argument<'a>(expr: Expression<'a>) -> Argument<'a> {
    match expr {
        Expression::BooleanLiteral(lit) => Argument::BooleanLiteral(lit),
        Expression::NullLiteral(lit) => Argument::NullLiteral(lit),
        Expression::NumericLiteral(lit) => Argument::NumericLiteral(lit),
        Expression::BigIntLiteral(lit) => Argument::BigIntLiteral(lit),
        Expression::RegExpLiteral(lit) => Argument::RegExpLiteral(lit),
        Expression::StringLiteral(lit) => Argument::StringLiteral(lit),
        Expression::TemplateLiteral(lit) => Argument::TemplateLiteral(lit),
        Expression::Identifier(id) => Argument::Identifier(id),
        Expression::MetaProperty(prop) => Argument::MetaProperty(prop),
        Expression::Super(sup) => Argument::Super(sup),
        Expression::ArrayExpression(arr) => Argument::ArrayExpression(arr),
        Expression::ArrowFunctionExpression(arrow) => Argument::ArrowFunctionExpression(arrow),
        Expression::AssignmentExpression(assign) => Argument::AssignmentExpression(assign),
        Expression::AwaitExpression(await_expr) => Argument::AwaitExpression(await_expr),
        Expression::BinaryExpression(binary) => Argument::BinaryExpression(binary),
        Expression::CallExpression(call) => Argument::CallExpression(call),
        Expression::ChainExpression(chain) => Argument::ChainExpression(chain),
        Expression::ClassExpression(class) => Argument::ClassExpression(class),
        Expression::ConditionalExpression(cond) => Argument::ConditionalExpression(cond),
        Expression::FunctionExpression(func) => Argument::FunctionExpression(func),
        Expression::ImportExpression(import) => Argument::ImportExpression(import),
        Expression::LogicalExpression(logic) => Argument::LogicalExpression(logic),
        Expression::NewExpression(new_expr) => Argument::NewExpression(new_expr),
        Expression::ObjectExpression(obj) => Argument::ObjectExpression(obj),
        Expression::ParenthesizedExpression(paren) => Argument::ParenthesizedExpression(paren),
        Expression::SequenceExpression(seq) => Argument::SequenceExpression(seq),
        Expression::TaggedTemplateExpression(tagged) => Argument::TaggedTemplateExpression(tagged),
        Expression::ThisExpression(this) => Argument::ThisExpression(this),
        Expression::UnaryExpression(unary) => Argument::UnaryExpression(unary),
        Expression::UpdateExpression(update) => Argument::UpdateExpression(update),
        Expression::YieldExpression(yield_expr) => Argument::YieldExpression(yield_expr),
        Expression::PrivateInExpression(private_in) => Argument::PrivateInExpression(private_in),
        Expression::JSXElement(jsx) => Argument::JSXElement(jsx),
        Expression::JSXFragment(frag) => Argument::JSXFragment(frag),
        Expression::TSAsExpression(as_expr) => Argument::TSAsExpression(as_expr),
        Expression::TSSatisfiesExpression(satisfies) => Argument::TSSatisfiesExpression(satisfies),
        Expression::TSTypeAssertion(type_assert) => Argument::TSTypeAssertion(type_assert),
        Expression::TSNonNullExpression(non_null) => Argument::TSNonNullExpression(non_null),
        Expression::TSInstantiationExpression(inst) => Argument::TSInstantiationExpression(inst),
        Expression::V8IntrinsicExpression(v8) => Argument::V8IntrinsicExpression(v8),
        Expression::ComputedMemberExpression(computed) => {
            Argument::ComputedMemberExpression(computed)
        }
        Expression::StaticMemberExpression(static_member) => {
            Argument::StaticMemberExpression(static_member)
        }
        Expression::PrivateFieldExpression(private) => Argument::PrivateFieldExpression(private),
    }
}

pub enum TemplateValue<'a> {
    Single(String),
    Multiple(Vec<'a, String>),
}

pub struct CreateTemplateInput<'a> {
    pub template: Option<TemplateValue<'a>>,
    pub exprs: Vec<'a, Expression<'a>>,
    pub template_values: Vec<'a, Expression<'a>>,
    pub wont_escape: bool,
}

/// Create SSR template for handling template strings and caching
pub fn create_template<'a>(
    visitor: &'a mut impl TreeBuilderMut<'a>,
    program: &mut Program<'a>,
    input: &mut CreateTemplateInput<'a>,
    module_name: &str,
) -> Expression<'a> {
    // 如果没有模板，返回第一个表达式
    if input.template.is_none() {
        // 直接获取 Vec 中的第一个元素的所有权
        return input.exprs.remove(0);
    }
    let root_scope = if let Some(root_scope) = program.scope_id.get() {
        root_scope
    } else {
        panic!("Root scope not found in program");
    };

    let allocator = visitor.allocator_mut();

    // 创建模板表达式
    let template = match &input.template {
        Some(TemplateValue::Single(s)) => Expression::StringLiteral(Box::new_in(
            StringLiteral {
                span: Span::default(),
                value: Atom::from_in(s, allocator),
                raw: None,
                lone_surrogates: false,
            },
            allocator,
        )),
        Some(TemplateValue::Multiple(arr)) if arr.len() == 1 => {
            Expression::StringLiteral(Box::new_in(
                StringLiteral {
                    span: Span::default(),
                    value: Atom::from_in(&arr[0], allocator),
                    raw: None,
                    lone_surrogates: false,
                },
                allocator,
            ))
        }
        Some(TemplateValue::Multiple(arr)) => {
            let elements = arr.iter().map(|s| {
                ArrayExpressionElement::StringLiteral(Box::new_in(
                    StringLiteral {
                        span: Span::default(),
                        value: Atom::from_in(s, allocator),
                        raw: None,
                        lone_surrogates: false,
                    },
                    allocator,
                ))
            });

            Expression::ArrayExpression(Box::new_in(
                ArrayExpression {
                    span: Span::default(),
                    elements: Vec::from_iter_in(elements, allocator),
                },
                allocator,
            ))
        }
        None => unreachable!(),
    };

    let templates = visitor.templates_mut();

    let found = templates
        .iter()
        .find(|tmp| match (&tmp.template, &template) {
            (Expression::ArrayExpression(t1), Expression::ArrayExpression(t2)) => {
                t1.elements.iter().enumerate().all(|(index, value)| {
                    if let Some(t2_element) = t2.elements.get(index) {
                        match (value, t2_element) {
                            (
                                ArrayExpressionElement::StringLiteral(s1),
                                ArrayExpressionElement::StringLiteral(s2),
                            ) => s1.value == s2.value,
                            (
                                ArrayExpressionElement::Identifier(id1),
                                ArrayExpressionElement::Identifier(id2),
                            ) => id1.name == id2.name,
                            _ => false,
                        }
                    } else {
                        false
                    }
                })
            }
            (Expression::StringLiteral(s1), Expression::StringLiteral(s2)) => s1.value == s2.value,
            _ => false,
        });

    let id = if let Some(found) = found {
        found.id
    } else {
        let mut scoping = visitor.scoping_mut();

        // Create new import declaration node
        let node_id = NodeId::new(program.body.len() as u32);

        // Create symbol for the import
        let symbol_id = scoping.create_symbol(
            Span::default(),
            "",
            SymbolFlags::Import | SymbolFlags::Value,
            root_scope,
            node_id,
        );

        let reference_id = create_import_reference(&mut scoping, symbol_id, node_id);
        let templates = visitor.templates_mut();
        templates.push(Template {
            id: reference_id,
            template: template.clone_in(allocator),
            template_with_closing_tags: template,
            renderer: "ssr",
        });
        reference_id
    };

    if input.wont_escape {
        match &input.template {
            Some(TemplateValue::Single(templ)) => {
                return Expression::Identifier(Box::new_in(
                    IdentifierReference {
                        span: Span::default(),
                        name: Atom::from_in("template", allocator),
                        reference_id: std::cell::Cell::new(Some(id)),
                    },
                    allocator,
                ));
            }
            Some(TemplateValue::Multiple(arr)) if arr.len() == 1 => {
                return Expression::Identifier(Box::new_in(
                    IdentifierReference {
                        span: Span::default(),
                        name: Atom::from_in("template", allocator),
                        reference_id: std::cell::Cell::new(Some(id)),
                    },
                    allocator,
                ));
            }
            Some(TemplateValue::Multiple(arr)) if arr.len() == 2 => {
                if let Some(arg) = input.template_values.get(0) {
                    if let Expression::CallExpression(call_expr) = arg {
                        if let Expression::Identifier(callee) = &call_expr.callee {
                            if callee.name.as_str() == "_$ssrHydrationKey" {
                                // remove unnecessary ssr call when only hydration key is used

                                let member_expr0 =
                                    Expression::ComputedMemberExpression(Box::new_in(
                                        ComputedMemberExpression {
                                            span: Span::default(),
                                            object: Expression::Identifier(Box::new_in(
                                                IdentifierReference {
                                                    span: Span::default(),
                                                    name: Atom::from_in("template", allocator),
                                                    reference_id: std::cell::Cell::new(Some(id)),
                                                },
                                                allocator,
                                            )),
                                            expression: Expression::NumericLiteral(Box::new_in(
                                                NumericLiteral {
                                                    span: Span::default(),
                                                    value: 0.0,
                                                    raw: None,
                                                    base: NumberBase::Decimal,
                                                },
                                                allocator,
                                            )),
                                            optional: false,
                                        },
                                        allocator,
                                    ));

                                let member_expr1 =
                                    Expression::ComputedMemberExpression(Box::new_in(
                                        ComputedMemberExpression {
                                            span: Span::default(),
                                            object: Expression::Identifier(Box::new_in(
                                                IdentifierReference {
                                                    span: Span::default(),
                                                    name: Atom::from_in("template", allocator),
                                                    reference_id: std::cell::Cell::new(Some(id)),
                                                },
                                                allocator,
                                            )),
                                            expression: Expression::NumericLiteral(Box::new_in(
                                                NumericLiteral {
                                                    span: Span::default(),
                                                    value: 1.0,
                                                    raw: None,
                                                    base: NumberBase::Decimal,
                                                },
                                                allocator,
                                            )),
                                            optional: false,
                                        },
                                        allocator,
                                    ));

                                let binary_expr1 = Expression::BinaryExpression(Box::new_in(
                                    BinaryExpression {
                                        span: Span::default(),
                                        operator: BinaryOperator::Addition,
                                        left: member_expr0,
                                        right: arg.clone_in(allocator),
                                    },
                                    allocator,
                                ));

                                return Expression::BinaryExpression(Box::new_in(
                                    BinaryExpression {
                                        span: Span::default(),
                                        operator: BinaryOperator::Addition,
                                        left: binary_expr1,
                                        right: member_expr1,
                                    },
                                    allocator,
                                ));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let allocator = visitor.allocator_mut();

    let arguments: Vec<'_, Argument<'_>> = match &input.template {
        Some(TemplateValue::Multiple(arr)) if arr.len() > 1 => {
            let mut exprs = Vec::from_array_in(
                [Argument::Identifier(Box::new_in(
                    IdentifierReference {
                        span: Span::default(),
                        name: Atom::from_in("template", allocator),
                        reference_id: std::cell::Cell::new(Some(id)),
                    },
                    allocator,
                ))],
                allocator,
            );
            exprs.extend(
                input
                    .template_values
                    .iter()
                    .map(|expr| expression_to_argument(expr.clone_in(allocator))),
            );
            exprs
        }
        _ => Vec::from_array_in(
            [Argument::Identifier(Box::new_in(
                IdentifierReference {
                    span: Span::default(),
                    name: Atom::from_in("template", allocator),
                    reference_id: std::cell::Cell::new(Some(id)),
                },
                allocator,
            ))],
            allocator,
        ),
    };

    Expression::CallExpression(Box::new_in(
        CallExpression {
            span: Span::default(),
            callee: register_import_method(visitor, program, "ssr", module_name),
            arguments,
            optional: false,
            type_arguments: None,
            pure: false,
        },
        allocator,
    ))
}

/// Append template declarations to the program body
pub fn append_templates() {
    todo!("Implement append_templates");
}
