// 生成单条语句的箭头函数表达式
use biome_js_factory::make::{
    js_arrow_function_expression, js_directive_list, js_function_body, js_parameter_list,
    js_parameters, js_statement_list, token,
};
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsFunctionBody, AnyJsStatement, JsArrowFunctionExpression, T,
};

pub fn make_arrow_function_from_statement(statement: AnyJsStatement) -> JsArrowFunctionExpression {
    let statements = js_statement_list(vec![statement]);
    let function_body = js_function_body(
        token(T!['{']),
        js_directive_list(vec![]),
        statements,
        token(T!['}']),
    );
    let params = js_parameters(
        token(T!['(']),
        js_parameter_list(vec![], vec![]),
        token(T![')']),
    );
    js_arrow_function_expression(
        AnyJsArrowFunctionParameters::JsParameters(params),
        token(T![=>]),
        AnyJsFunctionBody::JsFunctionBody(function_body),
    )
    .build()
}
