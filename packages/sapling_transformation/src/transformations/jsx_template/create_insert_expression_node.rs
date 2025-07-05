use biome_js_syntax::*;
use biome_js_factory::make::*;

use crate::jsx_template::HelperUsageTracker;
pub fn create_insert_expression_node_with_tracker(expr_child: &JsxExpressionChild, tracker: &mut HelperUsageTracker) -> Option<AnyJsStatement> {
    let el_var_token = JsSyntaxToken::new_detached(T![ident], "_el$", Vec::new(), Vec::new());
    let insert_node_token =
        JsSyntaxToken::new_detached(T![ident], "_$insertNode", Vec::new(), Vec::new());
    let create_text_token =
        JsSyntaxToken::new_detached(T![ident], "_$createTextNode", Vec::new(), Vec::new());

    tracker.insert_node = true;
    tracker.create_text_node = true;

    // 获取表达式内容
    let expression = expr_child.expression()?;

    // 直接将表达式节点作为参数传递给 _$createTextNode
    let create_text_call = js_call_expression(
        js_identifier_expression(js_reference_identifier(create_text_token)).into(),
        js_call_arguments(
            token(T!['(']),
            js_call_argument_list(
                vec![AnyJsCallArgument::AnyJsExpression(expression.clone())],
                vec![],
            ),
            token(T![')']),
        ),
    )
    .build();

    let insert_call = js_call_expression(
        js_identifier_expression(js_reference_identifier(insert_node_token)).into(),
        js_call_arguments(
            token(T!['(']),
            js_call_argument_list(
                vec![
                    AnyJsCallArgument::AnyJsExpression(
                        js_identifier_expression(js_reference_identifier(el_var_token))
                            .into(),
                    ),
                    AnyJsCallArgument::AnyJsExpression(
                        AnyJsExpression::JsCallExpression(create_text_call),
                    ),
                ],
                vec![token(T![,])],
            ),
            token(T![')']),
        ),
    )
    .build();

    let stmt = js_expression_statement(AnyJsExpression::JsCallExpression(insert_call))
        .with_semicolon_token(token(T![;]))
        .build();

    Some(stmt.into())
}