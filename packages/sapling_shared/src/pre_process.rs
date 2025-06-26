use indextree::Node;
use oxc_ast::AstKind;
use oxc_ast::ast::{JSXElement, JSXElementName, Program};
use oxc_ast_visit::Visit;
use sapling_macros::tree_builder;

use crate::TreeBuilder;
use crate::component::is_component;
use crate::config::Config;
use crate::html_nesting::is_valid_html_nesting;

#[tree_builder]
pub struct JSXValidator<'a>;

impl<'a> TreeBuilder<'a> for JSXValidator<'a> {
    fn arena(&self) -> &indextree::Arena<AstKind<'a>> {
        &self.arena
    }

    fn arena_mut(&mut self) -> &mut indextree::Arena<AstKind<'a>> {
        &mut self.arena
    }

    fn node_stack(&self) -> &Vec<indextree::NodeId> {
        &self.node_stack
    }

    fn node_stack_mut(&mut self) -> &mut Vec<indextree::NodeId> {
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

pub fn pre_process<'a>(program: &'a Program<'a>, opts: &'a Config<'a>) -> Config<'a> {
    let merged = Config {
        module_name: opts.module_name.clone(),
        generate: opts.generate.clone(),
        hydratable: opts.hydratable,
        delegate_events: opts.delegate_events,
        delegated_events: opts.delegated_events.clone(),
        built_ins: opts.built_ins.clone(),
        require_import_source: opts.require_import_source,
        wrap_conditionals: opts.wrap_conditionals,
        omit_nested_closing_tags: opts.omit_nested_closing_tags,
        omit_last_closing_tag: opts.omit_last_closing_tag,
        omit_quotes: opts.omit_quotes,
        context_to_custom_elements: opts.context_to_custom_elements,
        static_marker: opts.static_marker,
        effect_wrapper: opts.effect_wrapper,
        memo_wrapper: opts.memo_wrapper,
        validate: opts.validate,
        ..Default::default() // Merge with default values
    };
    let lib = merged.require_import_source;
    if lib.is_some() {
        let comments = &program.comments;

        let mut should_process = false;
        for comment in comments {
            let content = &program.source_text
                [comment.content_span().start as usize..comment.content_span().end as usize];

            if let Some(idx) = content.find("@jsxImportSource") {
                // 使用 as_ref() 来获取 Option<&String>
                if let Some(module_name) = merged.require_import_source {
                    if content[idx..].contains(module_name) {
                        should_process = true;
                        break;
                    }
                }
            }
        }

        if !should_process {
            return merged;
        }
    }

    // Validate JSX if needed
    if merged.validate {
        let mut validator = JSXValidator {
            arena: indextree::Arena::new(),
            node_stack: Vec::new(),
        };
        validator.visit_program(program);
    }

    merged
}
