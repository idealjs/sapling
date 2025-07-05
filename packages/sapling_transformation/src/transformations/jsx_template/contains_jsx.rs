use biome_rowan::AstNodeList;
use crate::transformations::jsx_template::contains_jsx_in_statement;
use biome_js_syntax::*;

/// 检查模块项是否包含 JSX 元素
pub fn contains_jsx(item: &AnyJsModuleItem) -> bool {
    match item {
        AnyJsModuleItem::AnyJsStatement(stmt) => {
            if contains_jsx_in_statement(stmt) {
                return true;
            }
            // 递归处理 function 声明体内的语句
            if let AnyJsStatement::JsFunctionDeclaration(func_decl) = stmt {
                if let Ok(body) = func_decl.body() {
                    let stmts = body.statements();
                    for inner_stmt in stmts.iter() {
                        if contains_jsx_in_statement(&inner_stmt) {
                            return true;
                        }
                    }
                }
            }
            false
        },
        AnyJsModuleItem::JsExport(export) => {
            // 简化版本：仅检查导出的表达式
            if let Ok(clause) = export.export_clause() {
                if let AnyJsExportClause::JsExportDefaultDeclarationClause(decl) = clause {
                    if let Ok(declaration) = decl.declaration() {
                        if let AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(expr_decl) = declaration {
                            if let Ok(body) = expr_decl.body() {
                                let list = body.statements();
                                for stmt in list.iter() {
                                    if contains_jsx_in_statement(&stmt) {
                                        return true;
                                    }
                                }
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