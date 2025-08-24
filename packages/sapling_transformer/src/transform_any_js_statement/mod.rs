use biome_js_factory::make::{
    js_class_member_list, js_function_declaration, js_variable_declaration,
    js_variable_declarator_list, js_variable_statement, token,
};
use biome_js_syntax::{
    AnyJsClassMember, AnyJsStatement, JsBlockStatement, JsBogusStatement, JsBreakStatement,
    JsClassDeclaration, JsContinueStatement, JsDebuggerStatement, JsDoWhileStatement,
    JsEmptyStatement, JsExpressionStatement, JsForInStatement, JsForOfStatement, JsForStatement,
    JsFunctionDeclaration, JsIfStatement, JsLabeledStatement, JsMetavariable, JsReturnStatement,
    JsSwitchStatement, JsThrowStatement, JsTryFinallyStatement, JsTryStatement,
    JsVariableDeclarator, JsVariableStatement, JsWhileStatement, JsWithStatement, T,
    TsDeclareFunctionDeclaration, TsDeclareStatement, TsEnumDeclaration,
    TsExternalModuleDeclaration, TsGlobalDeclaration, TsImportEqualsDeclaration,
    TsInterfaceDeclaration, TsModuleDeclaration, TsTypeAliasDeclaration,
};

use crate::{SaplingTransformer, make_js_return_statement};

impl SaplingTransformer {
    // main entry
    pub fn transform_any_js_statement(&mut self, node: &AnyJsStatement) -> Option<AnyJsStatement> {
        match node {
            AnyJsStatement::JsBlockStatement(inner) => self.transform_js_block_statement(inner),
            AnyJsStatement::JsBogusStatement(inner) => self.transform_js_bogus_statement(inner),
            AnyJsStatement::JsBreakStatement(inner) => self.transform_js_break_statement(inner),
            AnyJsStatement::JsClassDeclaration(inner) => self.transform_js_class_declaration(inner),
            AnyJsStatement::JsContinueStatement(inner) => {
                self.transform_js_continue_statement(inner)
            }
            AnyJsStatement::JsDebuggerStatement(inner) => {
                self.transform_js_debugger_statement(inner)
            }
            AnyJsStatement::JsDoWhileStatement(inner) => {
                self.transform_js_do_while_statement(inner)
            }
            AnyJsStatement::JsEmptyStatement(inner) => self.transform_js_empty_statement(inner),
            AnyJsStatement::JsExpressionStatement(inner) => {
                self.transform_js_expression_statement(inner)
            }
            AnyJsStatement::JsForInStatement(inner) => self.transform_js_for_in_statement(inner),
            AnyJsStatement::JsForOfStatement(inner) => self.transform_js_for_of_statement(inner),
            AnyJsStatement::JsForStatement(inner) => self.transform_js_for_statement(inner),
            AnyJsStatement::JsFunctionDeclaration(inner) => {
                self.transform_js_function_declaration(inner)
            }
            AnyJsStatement::JsIfStatement(inner) => self.transform_js_if_statement(inner),
            AnyJsStatement::JsLabeledStatement(inner) => self.transform_js_labeled_statement(inner),
            AnyJsStatement::JsMetavariable(inner) => {
                self.transform_js_metavariable_to_js_statement(inner)
            }
            AnyJsStatement::JsReturnStatement(inner) => self.transform_js_return_statement(inner),
            AnyJsStatement::JsSwitchStatement(inner) => self.transform_js_switch_statement(inner),
            AnyJsStatement::JsThrowStatement(inner) => self.transform_js_throw_statement(inner),
            AnyJsStatement::JsTryFinallyStatement(inner) => {
                self.transform_js_try_finally_statement(inner)
            }
            AnyJsStatement::JsTryStatement(inner) => self.transform_js_try_statement(inner),
            AnyJsStatement::JsVariableStatement(inner) => Some(
                AnyJsStatement::JsVariableStatement(self.transform_js_variable_statement(inner)?),
            ),
            AnyJsStatement::JsWhileStatement(inner) => self.transform_js_while_statement(inner),
            AnyJsStatement::JsWithStatement(inner) => self.transform_js_with_statement(inner),
            AnyJsStatement::TsDeclareFunctionDeclaration(inner) => {
                self.transform_ts_declare_function_declaration(inner)
            }
            AnyJsStatement::TsDeclareStatement(inner) => self.transform_ts_declare_statement(inner),
            AnyJsStatement::TsEnumDeclaration(inner) => self.transform_ts_enum_declaration(inner),
            AnyJsStatement::TsExternalModuleDeclaration(inner) => {
                self.transform_ts_external_module_declaration(inner)
            }
            AnyJsStatement::TsGlobalDeclaration(inner) => {
                self.transform_ts_global_declaration(inner)
            }
            AnyJsStatement::TsImportEqualsDeclaration(inner) => {
                self.transform_ts_import_equals_declaration(inner)
            }
            AnyJsStatement::TsInterfaceDeclaration(inner) => {
                self.transform_ts_interface_declaration(inner)
            }
            AnyJsStatement::TsModuleDeclaration(inner) => {
                self.transform_ts_module_declaration(inner)
            }
            AnyJsStatement::TsTypeAliasDeclaration(inner) => {
                self.transform_ts_type_alias_declaration(inner)
            }
        }
    }
    pub fn transform_js_function_declaration(
        &mut self,
        node: &JsFunctionDeclaration,
    ) -> Option<AnyJsStatement> {
        let new_body = self.transform_js_function_body(&node.body().ok()?)?;

        let new_func = js_function_declaration(
            node.function_token().ok()?,
            node.id().ok()?,
            node.parameters().ok()?,
            new_body,
        )
        .build();

        Some(AnyJsStatement::JsFunctionDeclaration(new_func))
    }
    pub fn transform_js_return_statement(
        &mut self,
        node: &JsReturnStatement,
    ) -> Option<AnyJsStatement> {
        let argument = node.argument()?;
        let new_expression = self.transform_any_js_expression(&argument)?;
        Some(AnyJsStatement::JsReturnStatement(make_js_return_statement(
            new_expression,
        )))
    }
    pub fn transform_js_variable_statement(
        &mut self,
        node: &JsVariableStatement,
    ) -> Option<JsVariableStatement> {
        let declaractors = node.declaration().ok()?.declarators();
        let new_declarators: Vec<JsVariableDeclarator> = declaractors
            .into_iter()
            .filter_map(|node| self.transform_js_variable_declarator(node.ok()?))
            .collect();
        let declarators_len = new_declarators.len();
        let separators = if declarators_len == 0 {
            Vec::new()
        } else {
            vec![token(T!(,)); declarators_len - 1]
        };
        let variable_kind = node.declaration().ok()?.kind().ok()?;

        Some(
            js_variable_statement(
                js_variable_declaration(
                    variable_kind,
                    js_variable_declarator_list(new_declarators, separators),
                )
                .build(),
            )
            .build(),
        )
    }
    // === 未实现函数，已统一移动到 impl 末尾，参数名加下划线 ===
    pub fn transform_js_block_statement(&self, _node: &JsBlockStatement) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_bogus_statement(&self, _node: &JsBogusStatement) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_break_statement(&self, _node: &JsBreakStatement) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_class_declaration(
        &mut self,
        node: &JsClassDeclaration,
    ) -> Option<AnyJsStatement> {
        let mut new_members = vec![];
        for member in node.members() {
            let method = match member.as_js_method_class_member() {
                Some(m) => m,
                None => {
                    new_members.push(member);
                    continue;
                }
            };

            match method.body().ok() {
                Some(body) => {
                    let new_body = self.transform_js_function_body(&body);
                    new_members.push(AnyJsClassMember::JsMethodClassMember(
                        method.clone().with_body(new_body?),
                    ));
                }
                None => {
                    new_members.push(member);
                    continue;
                }
            };
        }
        Some(AnyJsStatement::JsClassDeclaration(
            node.clone().with_members(js_class_member_list(new_members)),
        ))
    }
    pub fn transform_js_continue_statement(
        &self,
        _node: &JsContinueStatement,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_debugger_statement(
        &self,
        _node: &JsDebuggerStatement,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_do_while_statement(
        &self,
        _node: &JsDoWhileStatement,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_empty_statement(&self, _node: &JsEmptyStatement) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_expression_statement(
        &self,
        _node: &JsExpressionStatement,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_for_in_statement(
        &self,
        _node: &JsForInStatement,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_for_of_statement(
        &self,
        _node: &JsForOfStatement,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_for_statement(&self, _node: &JsForStatement) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_if_statement(&self, _node: &JsIfStatement) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_labeled_statement(
        &self,
        _node: &JsLabeledStatement,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_metavariable_to_js_statement(
        &self,
        _node: &JsMetavariable,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_switch_statement(
        &self,
        _node: &JsSwitchStatement,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_throw_statement(&self, _node: &JsThrowStatement) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_try_finally_statement(
        &self,
        _node: &JsTryFinallyStatement,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_try_statement(&self, _node: &JsTryStatement) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_while_statement(&self, _node: &JsWhileStatement) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_js_with_statement(&self, _node: &JsWithStatement) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_ts_declare_function_declaration(
        &self,
        _node: &TsDeclareFunctionDeclaration,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_ts_declare_statement(
        &self,
        _node: &TsDeclareStatement,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_ts_enum_declaration(
        &self,
        _node: &TsEnumDeclaration,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_ts_external_module_declaration(
        &self,
        _node: &TsExternalModuleDeclaration,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_ts_global_declaration(
        &self,
        _node: &TsGlobalDeclaration,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_ts_import_equals_declaration(
        &self,
        _node: &TsImportEqualsDeclaration,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_ts_interface_declaration(
        &self,
        _node: &TsInterfaceDeclaration,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_ts_module_declaration(
        &self,
        _node: &TsModuleDeclaration,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
    pub fn transform_ts_type_alias_declaration(
        &self,
        _node: &TsTypeAliasDeclaration,
    ) -> Option<AnyJsStatement> {
        todo!()
    }
}
