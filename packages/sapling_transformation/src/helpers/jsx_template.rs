use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument,
    AnyJsModuleItem, JsAssignmentExpression, JsCallArguments, JsDirective, JsIdentifierBinding,
    JsIdentifierExpression,
};
use biome_rowan::{AstNode, BatchMutation, BatchMutationExt, SyntaxNode};
use std::{any::Any, vec};

use biome_js_factory::make::{
    js_arrow_function_expression, js_assignment_expression, js_call_argument_list,
    js_call_arguments, js_call_expression, js_computed_member_assignment, js_directive_list,
    js_expression_statement, js_function_body, js_function_expression, js_identifier_assignment,
    js_identifier_binding, js_identifier_expression, js_initializer_clause, js_name,
    js_parameter_list, js_parameters, js_parenthesized_expression, js_reference_identifier,
    js_return_statement, js_statement_list, js_static_member_assignment,
    js_static_member_expression, js_variable_declaration, js_variable_declarator,
    js_variable_declarator_list, js_variable_statement, token,
};
use biome_js_syntax::{
    AnyJsExpression, AnyJsParameter, AnyJsStatement, AnyJsxChild, JsArrowFunctionExpression,
    JsCallExpression, JsDirectiveList, JsExpressionStatement, JsFunctionBody, JsFunctionExpression,
    JsLanguage, JsParameterList, JsParameters, JsReturnStatement, JsStatementList, JsSyntaxToken,
    JsxElement, JsxTagExpression, T,
};

use biome_rowan::SyntaxToken;

pub fn make_js_call_expression(
    arrow_function_expression: JsArrowFunctionExpression,
) -> JsCallExpression {
    let callee = js_parenthesized_expression(
        token(T!['(']),
        AnyJsExpression::JsArrowFunctionExpression(arrow_function_expression),
        token(T![')']),
    );

    let arguments = make_js_call_arguments(vec![], vec![]);

    let expression = js_call_expression(
        AnyJsExpression::JsParenthesizedExpression(callee),
        arguments,
    )
    .build();

    expression
}

pub fn make_js_call_arguments(
    arguments: Vec<AnyJsCallArgument>,
    separators: Vec<JsSyntaxToken>,
) -> JsCallArguments {
    js_call_arguments(
        token(T!['(']),
        js_call_argument_list(arguments, separators),
        token(T![')']),
    )
}

pub fn make_js_arrow_function_expression(
    params: JsParameters,
    body: JsFunctionBody,
) -> JsArrowFunctionExpression {
    use biome_js_syntax::{AnyJsArrowFunctionParameters, AnyJsFunctionBody, T};

    let params = AnyJsArrowFunctionParameters::JsParameters(params);
    let body: AnyJsFunctionBody = AnyJsFunctionBody::JsFunctionBody(body);
    js_arrow_function_expression(params, token(T![=>]), body).build()
}

pub fn make_js_parameters(
    js_parameter_list: JsParameterList, // parameters: Vec<AnyJsParameter>,
                                        // separators: Vec<JsSyntaxToken>,
) -> JsParameters {
    js_parameters(token(T!['(']), js_parameter_list, token(T![')']))
}

pub fn make_members() -> JsStatementList {
    js_statement_list(vec![])
}

pub fn make_js_return_statement(any_js_expression: AnyJsExpression) -> JsReturnStatement {
    js_return_statement(token(T![return]))
        .with_argument(any_js_expression)
        .with_semicolon_token(token(T![;]))
        .build()
}

pub fn make_js_function_body(
    directives: JsDirectiveList,
    statements: JsStatementList,
) -> JsFunctionBody {
    js_function_body(token(T!['{']), directives, statements, token(T!['}']))
}

pub fn collect_jsx_element_tags(node: &JsxElement) {
    let opening_element = node
        .opening_element()
        .expect("JsxElement should have an opening element");
    let name = opening_element
        .name()
        .expect("Opening element should have a name");
    let attr = opening_element.attributes();
    let jsx_name = name.as_jsx_name().expect("Name should be a JsxName");
    let value_token = jsx_name
        .value_token()
        .expect("JsxName should have a value token");
    let name = value_token.text();
    let child_list = node.children();
    child_list.into_iter().for_each(|child| match child {
        AnyJsxChild::JsxElement(jsx_element) => {
            collect_jsx_element_tags(&jsx_element);
        }
        _ => {}
    });
}

pub fn collect_jsx_tag_expression(node: &JsxTagExpression) {
    let tag = node.tag().expect("JsxTagExpression should have a tag");
    let jsx_element = tag.as_jsx_element().expect("Tag should be a JsxElement");
    collect_jsx_element_tags(jsx_element);
}

pub struct StatementItemConfig {
    pub el_var: String,                        // "_el$"
    pub tmpl_fn: String,                       // "_tmpl$"
    pub event_bindings: Vec<(String, String)>, // [("$$click", "increment")]
    pub inserts: Vec<(String, String)>,        // [("_el$", "count")]
    pub return_var: String,                    // "_el$"
}

pub fn make_statement_items(config: &StatementItemConfig) -> Vec<AnyJsStatement> {
    let mut stmts = Vec::new();

    // 1. var _el$ = _tmpl$();
    let el_var_token = SyntaxToken::new_detached(T![ident], &config.el_var, Vec::new(), Vec::new());
    let tmpl_fn_token =
        SyntaxToken::new_detached(T![ident], &config.tmpl_fn, Vec::new(), Vec::new());
    let el_var = js_reference_identifier(el_var_token.clone());
    let tmpl_fn = js_reference_identifier(tmpl_fn_token.clone());
    let declarator = js_variable_declarator(AnyJsBindingPattern::AnyJsBinding(
        AnyJsBinding::JsIdentifierBinding(js_identifier_binding(el_var_token.clone())),
    ))
    .with_initializer(
        js_initializer_clause(
            token(T![=]),
            js_call_expression(
                js_identifier_expression(tmpl_fn.clone()).into(),
                make_js_call_arguments(vec![], vec![]),
            )
            .build()
            .into(),
        )
        .into(),
    )
    .build();
    let var_decl = js_variable_statement(
        js_variable_declaration(
            token(T![var]),
            js_variable_declarator_list(vec![declarator], vec![]),
        )
        .build(),
    )
    .build();
    stmts.push(var_decl.into());

    // 2. _el$.$$click = increment;
    for (event, handler) in &config.event_bindings {
        let event_token = SyntaxToken::new_detached(T![ident], event, Vec::new(), Vec::new());
        let handler_token = SyntaxToken::new_detached(T![ident], handler, Vec::new(), Vec::new());
        let member = js_static_member_assignment(
            js_identifier_expression(el_var.clone()).into(),
            token(T![.]),
            js_name(event_token).into(),
        );
        let assign = js_expression_statement(
            make_js_assignment_expression(
                member.into(),
                js_identifier_expression(js_reference_identifier(handler_token)).into(),
            )
            .into(),
        )
        .build();
        stmts.push(assign.into());
    }

    // 3. _$insert(_el$, count);
    for (el, value) in &config.inserts {
        let el_token = SyntaxToken::new_detached(T![ident], el, Vec::new(), Vec::new());
        let value_token = SyntaxToken::new_detached(T![ident], value, Vec::new(), Vec::new());
        let insert_token = SyntaxToken::new_detached(T![ident], "_$insert", Vec::new(), Vec::new());
        let call = js_expression_statement(AnyJsExpression::JsCallExpression(
            js_call_expression(
                js_identifier_expression(js_reference_identifier(insert_token)).into(),
                make_js_call_arguments(
                    vec![
                        AnyJsCallArgument::AnyJsExpression(
                            js_identifier_expression(js_reference_identifier(el_token.clone()))
                                .into(),
                        ),
                        AnyJsCallArgument::AnyJsExpression(
                            js_identifier_expression(js_reference_identifier(value_token.clone()))
                                .into(),
                        ),
                    ],
                    vec![],
                ),
            )
            .build(),
        ))
        .build();
        stmts.push(call.into());
    }

    // 4. return _el$;
    let ret = make_js_return_statement(js_identifier_expression(el_var).into()).into();
    stmts.push(ret);

    stmts
}

pub fn make_js_assignment_expression(
    left: AnyJsAssignment,
    right: AnyJsExpression,
) -> JsAssignmentExpression {
    js_assignment_expression(
        AnyJsAssignmentPattern::AnyJsAssignment(left),
        token(T![=]),
        right,
    )
}

pub fn replace<P, T>(
    target: &P,
    get_parent: impl Fn(&P) -> T,
    get_new_parent: impl Fn(&T) -> T,
) -> BatchMutation<JsLanguage>
where
    P: AstNode<Language = JsLanguage>,
    T: AstNode<Language = JsLanguage>,
{
    let mut mutation = target.clone().begin();
    let parent = get_parent(&target);
    let new_node = get_new_parent(&parent);

    mutation.replace_node(parent, new_node);

    mutation
}
