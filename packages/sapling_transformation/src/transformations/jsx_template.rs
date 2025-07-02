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
    JsStatementList, JsVariableStatement, JsxElement, T, TsEnumDeclaration,
};
use biome_rowan::{AstNode, BatchMutationExt, TriviaPieceKind};

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
    member_names: Vec<(String, Option<JsInitializerClause>)>,
}

impl Rule for JsxTemplate {
    type Query = Ast<JsxElement>;
    type State = JsxTemplateMembers;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        None
    }

    fn transform(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsBatchMutation> {
        None
    }
}
