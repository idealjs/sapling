use std::collections::HashSet;

use crate::Template;
use crate::TreeBuilder;
use crate::TreeBuilderMut;
use crate::component::is_component;
use crate::config::Config;
use crate::html_nesting::is_valid_html_nesting;
use indextree::Node;
use oxc_allocator::Allocator;
use oxc_allocator::Vec;
use oxc_ast::AstKind;
use oxc_ast::ast::{
    JSXElement,
    JSXElementName, Program,
};
use oxc_ast_visit::Visit;
use sapling_macros::tree_builder;

// Mock structure for HTML validation results
#[derive(Debug)]
struct MarkupValidationResult {
    html: String,
    browser: String,
}

// Mock validation function
fn is_invalid_markup(html: &str) -> Option<MarkupValidationResult> {
    // TODO: Implement actual HTML validation
    // For now just return None to indicate valid markup
    None
}

#[tree_builder]
pub struct JSXValidator<'a>;

impl<'a> TreeBuilder<'a> for JSXValidator<'a> {
    fn arena(&self) -> &indextree::Arena<AstKind<'a>> {
        &self.arena
    }

    fn arena_mut(&mut self) -> &mut indextree::Arena<AstKind<'a>> {
        &mut self.arena
    }

    fn node_stack(&self) -> &std::vec::Vec<indextree::NodeId> {
        &self.node_stack
    }

    fn node_stack_mut(&mut self) -> &mut std::vec::Vec<indextree::NodeId> {
        &mut self.node_stack
    }
}

// JSX validation implementation
impl<'a> JSXValidator<'a> {
    fn validate_jsx_nesting(&self, element: &JSXElement<'a>) {
        let el_name = match &element.opening_element.name {
            JSXElementName::Identifier(id) => &id.name,
            _ => return,
        };

        if is_component(el_name) {
            return;
        }

        let current_node = self
            .arena()
            .get(
                *self
                    .node_stack
                    .last()
                    .expect("node stack should not be empty"),
            )
            .expect("current node should exist");

        let parent = match Node::parent(&current_node) {
            Some(parent_id) => self.arena().get(parent_id).unwrap(),
            None => return,
        };

        // Get parent element
        let parent_element = match parent.get() {
            AstKind::JSXElement(parent_el) => parent_el,
            _ => return,
        };

        // Get parent element name
        let parent_name = match &parent_element.opening_element.name {
            JSXElementName::Identifier(id) => &id.name,
            _ => return,
        };

        // Validate nesting
        if !is_component(parent_name) && !is_valid_html_nesting(parent_name, el_name) {
            panic!(
                "Invalid JSX: <{}> cannot be child of <{}>",
                el_name, parent_name
            );
        }
    }
}

impl<'a> Visit<'a> for JSXValidator<'a> {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        <Self as TreeBuilder>::enter_node(self, kind);
    }
    fn leave_node(&mut self, kind: AstKind<'a>) {
        <Self as TreeBuilder>::leave_node(self, kind);
    }
    fn visit_jsx_element(&mut self, it: &JSXElement<'a>) {
        let kind = AstKind::JSXElement(self.alloc(it));
        <Self as Visit<'a>>::enter_node(self, kind);

        // Validate JSX nesting rules
        self.validate_jsx_nesting(it);

        self.visit_span(&it.span);
        self.visit_jsx_opening_element(&it.opening_element);
        self.visit_jsx_children(&it.children);
        if let Some(closing_element) = &it.closing_element {
            self.visit_jsx_closing_element(closing_element);
        }
        <Self as Visit<'a>>::leave_node(self, kind);
    }
}

pub fn pre_process_ast<'a>(
    visitor: &mut impl TreeBuilderMut<'a>,
    program: &mut Program<'a>,
    opts: &Config,
) {
    let require_import_source = opts
        .require_import_source
        .or(visitor.config().require_import_source);

    let validate = opts.validate || visitor.config().validate;

    if require_import_source.is_some() {
        let comments = &program.comments;

        let mut should_process = false;
        for comment in comments {
            let content = &program.source_text
                [comment.content_span().start as usize..comment.content_span().end as usize];

            if let Some(idx) = content.find("@jsxImportSource") {
                // 使用 as_ref() 来获取 Option<&String>
                if let Some(module_name) = require_import_source {
                    if content[idx..].contains(module_name) {
                        should_process = true;
                        break;
                    }
                }
            }
        }

        if !should_process {
            return ();
        }
    }

    // Validate JSX if needed
    if validate {
        let mut validator = JSXValidator {
            arena: indextree::Arena::new(),
            node_stack: std::vec::Vec::new(),
            allocator: &oxc_allocator::Allocator::default(),
            scoping: &mut oxc_semantic::Scoping::default(),
        };
        validator.visit_program(program);
    }
    ()
}

pub fn post_process_ast<'a>(
    allocator: &'a Allocator,
    program: &Program,
    events: Option<&HashSet<String>>,
    templates: Option<&Vec<Template>>,
) -> Result<(), &'static str> {
    // Handle event delegation

    // if let Some(events) = events {
    //     let expression_statement = ExpressionStatement {
    //         span: Span::default(),
    //         expression: Expression::CallExpression(Box::new_in(
    //             CallExpression {
    //                 span: todo!(),
    //                 callee: register_import_method(
    //                     program,
    //                     "delegateEvents",
    //                     get_renderer_config(path, renderer),
    //                 ),
    //                 type_arguments: todo!(),
    //                 arguments: oxc_allocator::Vec::from_iter_in(
    //                     events.iter().map(|_| {
    //                         let argument = Argument::StringLiteral(Box::new_in(
    //                             StringLiteral {
    //                                 span: Span::default(),
    //                                 value: todo!(),
    //                                 raw: todo!(),
    //                                 lone_surrogates: false,
    //                             },
    //                             allocator,
    //                         ));
    //                         argument
    //                     }),
    //                     allocator,
    //                 ),
    //                 optional: todo!(),
    //                 pure: todo!(),
    //             },
    //             allocator,
    //         )),
    //     };

    //     // Create array of StringLiterals from events
    //     // (Unreachable and incomplete code removed to fix compile error)
    // }

    // // Handle templates
    // if let Some(templates) = &program.scope.data.templates {
    //     // Validate templates if configured
    //     if program.config.validate {
    //         for template in templates {
    //             if let Some(html) = &template.template_with_closing_tags {
    //                 if let Some(result) = is_invalid_markup(html) {
    //                     println!(
    //                         "\nThe HTML provided is malformed and will yield unexpected output when evaluated by a browser.\n"
    //                     );
    //                     println!("User HTML:\n{}", result.html);
    //                     println!("Browser HTML:\n{}", result.browser);
    //                     println!("Original HTML:\n{}", html);
    //                 }
    //             }
    //         }
    //     }

    //     // Process DOM and SSR templates separately
    //     let dom_templates: Vec<_> = templates.iter().filter(|t| t.renderer == "dom").collect();

    //     let ssr_templates: Vec<_> = templates.iter().filter(|t| t.renderer == "ssr").collect();

    //     if !dom_templates.is_empty() {
    //         dom::append_templates(program, dom_templates);
    //     }

    //     if !ssr_templates.is_empty() {
    //         ssr::append_templates(program, ssr_templates);
    //     }
    // }

    Ok(())
}

/// Process spreads in transformed AST
pub fn process_spreads() -> Result<(), &'static str> {
    todo!("Implement processing of spread attributes in transformed AST")
}

/// Validate templates in transformed AST
pub fn validate_templates() -> Result<(), &'static str> {
    todo!("Implement validation of templates in transformed AST")
}

/// Add event delegation to transformed AST
pub fn add_event_delegation() -> Result<(), &'static str> {
    todo!("Implement addition of event delegation to transformed AST")
}
