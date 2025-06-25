use oxc_ast::ast::{JSXElement, JSXElementName, Program};
use oxc_ast::{AstKind, CommentKind};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::walk_program;

use crate::component::is_component;
use crate::config::Config;

pub fn is_valid_html_nesting(_parent: &str, _child: &str) -> bool {
    // TODO: 实现 HTML 嵌套验证
    true
}

pub struct JSXValidator;

impl<'a> Visit<'a> for JSXValidator {
    fn visit_jsx_element(&mut self, it: &JSXElement<'a>) {}
    // pub fn jsx_element(&self, element: &JSXElement) -> Option<String> {
    //     let el_name = match &element.opening_element.name {
    //         JSXElementName::Identifier(id) => &id.name,
    //         _ => return None,
    //     };

    //     if is_component(el_name) {
    //         return None;
    //     }

    //     // 获取父元素 - 通过遍历而不是直接访问
    //     let parent_el = if let Some(parent) = element
    //         .opening_element
    //         .span
    //         .source_file()
    //         .find_parent(element.span)
    //     {
    //         match parent.kind() {
    //             AstKind::JSXElement(el) => el,
    //             _ => return None,
    //         }
    //     } else {
    //         return None;
    //     };

    //     let parent_name = match &parent_el.opening_element.name {
    //         JSXElementName::Identifier(id) => &id.name,
    //         _ => return None,
    //     };

    //     if !is_component(parent_name) && !is_valid_html_nesting(parent_name, el_name) {
    //         return Some(format!(
    //             "Invalid JSX: <{}> cannot be child of <{}>",
    //             el_name, parent_name
    //         ));
    //     }

    //     None
    // }
}

pub fn process<'a>(program: &'a Program<'a>, opts: &'a Config<'a>) -> Config<'a> {
    // Merge default config with provided opts
    // Create merged config from defaults and opts
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
        static_marker: opts.static_marker.clone(),
        effect_wrapper: opts.effect_wrapper.clone(),
        memo_wrapper: opts.memo_wrapper.clone(),
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
        let mut validator = JSXValidator {};
        validator.visit_program(program);
    }

    merged
}
