use biome_rowan::AstNodeList;
use crate::transformations::jsx_template::contains_jsx_in_statement;
use crate::transformations::jsx_template::contains_jsx_in_expression;
use biome_js_syntax::*;
use crate::JsxElementInfo;

/// 检查模块项是否包含 JSX 元素
pub fn contains_jsx(item: &AnyJsModuleItem) -> bool {
    match item {
        AnyJsModuleItem::AnyJsStatement(stmt) => contains_jsx_in_statement(stmt),
        AnyJsModuleItem::JsExport(export) => {
            // 简化版本：仅检查导出的表达式
            if let Ok(clause) = export.export_clause() {
                if let AnyJsExportClause::JsExportDefaultDeclarationClause(decl) = clause {
                    if let Ok(declaration) = decl.declaration() {
                        if let AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(expr_decl) = declaration {
                            if let Ok(body) = expr_decl.body() {
                                let list = body.statements();
                                for stmt in list.iter() {
                                    match stmt {
                                        AnyJsStatement::JsExpressionStatement(expr_stmt) => {
                                            if let Ok(expr) = expr_stmt.expression() {
                                                return contains_jsx_in_expression(&expr);
                                            }
                                        }
                                        AnyJsStatement::JsReturnStatement(ret_stmt) => {
                                            if let Some(expr) = ret_stmt.argument() {
                                                return contains_jsx_in_expression(&expr);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                return false;
                            }
                        }
                    }
                }
            }
            false
        },
        _ => false,
    }
}