// use crate::{JsBatchMutation, declare_transformation};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule};
use biome_js_factory::make::{
    ident, js_assignment_expression, js_call_argument_list, js_call_arguments, js_call_expression,
    js_computed_member_assignment, js_decorator_list, js_directive_list, js_expression_statement,
    js_formal_parameter, js_function_body, js_function_expression, js_identifier_assignment,
    js_identifier_binding, js_identifier_expression, js_logical_expression, js_module_item_list,
    js_number_literal_expression, js_object_expression, js_object_member_list, js_parameter_list,
    js_parameters, js_parenthesized_expression, js_reference_identifier, js_statement_list,
    js_string_literal, js_string_literal_expression, js_variable_declaration,
    js_variable_declarator, js_variable_declarator_list, js_variable_statement, token,
};
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument,
    AnyJsExpression, AnyJsFormalParameter, AnyJsLiteralExpression, AnyJsModuleItem, AnyJsParameter,
    AnyJsStatement, JsAssignmentExpression, JsComputedMemberAssignment, JsExpressionStatement,
    JsFunctionExpression, JsInitializerClause, JsLogicalExpression, JsModuleItemList,
    JsReturnStatement, JsStatementList, JsSyntaxKind, JsVariableStatement, JsxElement,
    JsxTagExpression, T, TsEnumDeclaration,
};
use biome_rowan::{AstNode, BatchMutationExt, TriviaPieceKind};

use crate::helpers::jsx_template::{
    StatementItemConfig, collect_jsx_tag_expression, make_js_arrow_function_expression,
    make_js_call_expression, make_js_function_body, make_js_parameters, make_js_return_statement,
    make_statement_items,
};
use crate::{JsBatchMutation, declare_transformation};

declare_transformation! {
    /// Transform a TypeScript [TsEnumDeclaration]
    pub(crate) JsxTemplate {
        version: "0.1.0",
        name: "jsx_template",
        language: "js",
    }
}

#[derive(Debug)]
pub struct JsxTemplateMembers {
    name: String,
    attr_names: Vec<(String, Option<JsInitializerClause>)>,
}

impl Rule for JsxTemplate {
    type Query = Ast<JsxTagExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        // let child_list = node.children();
        // let attrs = node
        //     .opening_element()
        //     .expect("JsxElement should have an opening element")
        //     .attributes();

        println!("node run : {:?}", node);
        collect_jsx_tag_expression(node);
        // let mut member_names = vec![];
        // let id = node.id().ok()?;
        Some(())
    }

    fn transform(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsBatchMutation> {
        let node = ctx.query();
        let mut mutation = node.clone().begin();
        let parent = node.syntax().parent();
        println!("node transform : {:?}", node);

        if let Some(parent) = parent {
            match parent.kind() {
                JsSyntaxKind::JS_RETURN_STATEMENT => {
                    if let Some(prev_node) = JsReturnStatement::cast(parent) {
                        let next_node =
                            make_js_return_statement(AnyJsExpression::JsCallExpression(
                                make_js_call_expression(make_js_arrow_function_expression(
                                    make_js_parameters(js_parameter_list(vec![], vec![])),
                                    make_js_function_body(
                                        js_directive_list(vec![]),
                                        js_statement_list(make_statement_items(
                                            &StatementItemConfig {
                                                el_var: "_el$".to_string(),
                                                tmpl_fn: "_tmpl$".to_string(),
                                                event_bindings: vec![(
                                                    "$$click".to_string(),
                                                    "increment".to_string(),
                                                )],
                                                inserts: vec![(
                                                    "_el$".to_string(),
                                                    "count".to_string(),
                                                )],
                                                return_var: "_el$".to_string(),
                                            },
                                        )),
                                    ),
                                )),
                            ));

                        mutation.replace_node(prev_node, next_node);
                    }
                }
                _ => {}
            }
        }
        // None
        Some(mutation)
    }
}
