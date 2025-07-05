use biome_js_syntax::*;
use crate::JsxElementInfo;
use crate::jsx_template::collect_jsx_from_expression;

pub fn collect_jsx_from_statement(stmt: &AnyJsStatement, jsx_elements: &mut Vec<JsxElementInfo>, in_function: bool) {
    match stmt {
        AnyJsStatement::JsExpressionStatement(expr_stmt) => {
            if let Ok(expr) = expr_stmt.expression() {
                collect_jsx_from_expression(&expr, jsx_elements, in_function);
            }
        },
        AnyJsStatement::JsReturnStatement(return_stmt) => {
            if let Some(expr) = return_stmt.argument() {
                collect_jsx_from_expression(&expr, jsx_elements, in_function);
            }
        },
        _ => {},
    }
}