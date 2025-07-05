use biome_rowan::AstNodeList;
use biome_js_syntax::*;
use crate::JsxElementInfo;
use crate::jsx_template::{collect_jsx_from_statement, collect_jsx_from_expression};

pub fn collect_jsx_elements(item: &AnyJsModuleItem, jsx_elements: &mut Vec<JsxElementInfo>) {
    match item {
        AnyJsModuleItem::AnyJsStatement(stmt) => {
            collect_jsx_from_statement(stmt, jsx_elements, false);

            // 递归处理 function 声明体内的语句
            if let AnyJsStatement::JsFunctionDeclaration(func_decl) = stmt {
                if let Ok(body) = func_decl.body() {
                    let stmts = body.statements();
                    for inner_stmt in stmts.iter() {
                        collect_jsx_from_statement(&inner_stmt, jsx_elements, true);
                    }
                }
            }
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
                                    collect_jsx_from_statement(&stmt, jsx_elements, true);
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