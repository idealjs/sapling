use biome_js_syntax::*;
use biome_js_factory::make::*;
use super::transform_expression_with_tracker;

pub fn transform_arrow_function(arrow_fn: &JsArrowFunctionExpression) -> Option<AnyJsExpression> {
    if let Ok(body) = arrow_fn.body() {
        if let AnyJsFunctionBody::AnyJsExpression(expr) = body {
            // 递归 transform 箭头函数体
            let mut tracker = crate::jsx_template::HelperUsageTracker::default();
            if let Some(transformed_expr) = transform_expression_with_tracker(&expr, &mut tracker) {
                // 修复：如果 transform 后是数组表达式（Fragment），直接返回数组表达式
                if let biome_js_syntax::AnyJsExpression::JsArrayExpression(_) = &transformed_expr {
                    return Some(transformed_expr);
                }
                if let Ok(params) = arrow_fn.parameters() {
                    let new_arrow_fn = js_arrow_function_expression(
                        params,
                        arrow_fn.fat_arrow_token().ok()?,
                        AnyJsFunctionBody::AnyJsExpression(transformed_expr),
                    )
                    .build();
                    return Some(AnyJsExpression::JsArrowFunctionExpression(new_arrow_fn));
                }
            }
        } else if let AnyJsFunctionBody::JsFunctionBody(block) = body {
            // 递归 transform 语句块中的 return 语句
            use biome_rowan::AstNodeList;
            let mut changed = false;
            let mut new_stmts = Vec::new();
            for stmt in block.statements() {
                if let AnyJsStatement::JsReturnStatement(ret) = &stmt {
                    if let Some(expr) = ret.argument() {
                        let mut tracker = crate::jsx_template::HelperUsageTracker::default();
                        if let Some(transformed_expr) = transform_expression_with_tracker(&expr, &mut tracker) {
                            let new_ret = js_return_statement(ret.return_token().expect("Missing return"))
                                .with_argument(transformed_expr)
                                .with_semicolon_token(ret.semicolon_token().expect("Missing semicolon"))
                                .build();
                            new_stmts.push(AnyJsStatement::JsReturnStatement(new_ret));
                            changed = true;
                            continue;
                        }
                    }
                }
                new_stmts.push(stmt.clone());
            }
            if changed {
                let new_body = js_function_body(
                    block.l_curly_token().expect("Missing {"),
                    block.directives(),
                    js_statement_list(new_stmts),
                    block.r_curly_token().expect("Missing }"),
                );
                if let Ok(params) = arrow_fn.parameters() {
                    let new_arrow_fn = js_arrow_function_expression(
                        params,
                        arrow_fn.fat_arrow_token().ok()?,
                        AnyJsFunctionBody::JsFunctionBody(new_body),
                    )
                    .build();
                    return Some(AnyJsExpression::JsArrowFunctionExpression(new_arrow_fn));
                }
            }
        }
    }
    None
}