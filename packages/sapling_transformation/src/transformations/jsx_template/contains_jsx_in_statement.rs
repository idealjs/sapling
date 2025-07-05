use biome_js_syntax::*;
use crate::JsxElementInfo;
use crate::transformations::jsx_template::contains_jsx_in_expression;

pub fn contains_jsx_in_statement(stmt: &AnyJsStatement) -> bool {
    match stmt {
        AnyJsStatement::JsExpressionStatement(expr_stmt) => {
            if let Ok(expr) = expr_stmt.expression() {
                contains_jsx_in_expression(&expr)
            } else {
                false
            }
        },
        AnyJsStatement::JsReturnStatement(return_stmt) => {
            if let Some(expr) = return_stmt.argument() {
                contains_jsx_in_expression(&expr)
            } else {
                false
            }
        },
        _ => false,
    }
}