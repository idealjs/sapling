use biome_rowan::AstNodeList;
use biome_js_syntax::*;
use crate::JsxElementInfo;
use crate::jsx_template::{collect_jsx_from_statement, collect_jsx_from_expression};

pub fn collect_jsx_elements(item: &AnyJsModuleItem, jsx_elements: &mut Vec<JsxElementInfo>) {
    match item {
        AnyJsModuleItem::AnyJsStatement(stmt) => {
            collect_jsx_from_statement(stmt, jsx_elements, false);
        },
        AnyJsModuleItem::JsExport(export) => {
            // 简化版本：仅处理导出的表达式
            if let Ok(clause) = export.export_clause() {
                if let AnyJsExportClause::JsExportDefaultDeclarationClause(decl) = clause {
                    if let Ok(declaration) = decl.declaration() {
                        if let AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(expr_decl) = declaration {
                            if let Ok(body) = expr_decl.body() {
                                let list = body.statements();
                                for stmt in list.iter() {
                                    if let AnyJsStatement::JsExpressionStatement(ref expr_stmt) = stmt {
                                        if let Ok(expr) = expr_stmt.expression() {
                                            collect_jsx_from_expression(&expr, jsx_elements, false);
                                            return;
                                        }
                                    }
                                    if let AnyJsStatement::JsReturnStatement(ref ret_stmt) = stmt {
                                        if let Some(expr) = ret_stmt.argument() {
                                            collect_jsx_from_expression(&expr, jsx_elements, false);
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        _ => {},
    }
}