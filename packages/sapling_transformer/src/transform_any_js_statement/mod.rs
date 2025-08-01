use biome_js_factory::make::{
    js_function_declaration, js_return_statement, js_variable_declaration,
    js_variable_declarator_list, js_variable_statement, token,
};
use biome_js_syntax::{
    AnyJsStatement, JsBlockStatement, JsBogusStatement, JsBreakStatement, JsClassDeclaration,
    JsContinueStatement, JsDebuggerStatement, JsDoWhileStatement, JsEmptyStatement,
    JsExpressionStatement, JsForInStatement, JsForOfStatement, JsForStatement,
    JsFunctionDeclaration, JsIfStatement, JsLabeledStatement, JsMetavariable, JsReturnStatement,
    JsSwitchStatement, JsThrowStatement, JsTryFinallyStatement, JsTryStatement,
    JsVariableDeclarator, JsVariableStatement, JsWhileStatement, JsWithStatement, T,
    TsDeclareFunctionDeclaration, TsDeclareStatement, TsEnumDeclaration,
    TsExternalModuleDeclaration, TsGlobalDeclaration, TsImportEqualsDeclaration,
    TsInterfaceDeclaration, TsModuleDeclaration, TsTypeAliasDeclaration,
};
use sapling_transformation::helpers::jsx_template::make_js_return_statement;

use crate::SaplingTransformer;

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
    pub fn transform_js_block_statement(&self, node: &JsBlockStatement) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_js_bogus_statement(&self, node: &JsBogusStatement) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_js_break_statement(&self, node: &JsBreakStatement) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_js_class_declaration(
        &self,
        node: &JsClassDeclaration,
    ) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_js_continue_statement(
        &self,
        node: &JsContinueStatement,
    ) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_js_debugger_statement(
        &self,
        node: &JsDebuggerStatement,
    ) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_js_do_while_statement(
        &self,
        node: &JsDoWhileStatement,
    ) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_js_empty_statement(&self, node: &JsEmptyStatement) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_js_expression_statement(
        &self,
        node: &JsExpressionStatement,
    ) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_js_for_in_statement(&self, node: &JsForInStatement) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_js_for_of_statement(&self, node: &JsForOfStatement) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_js_for_statement(&self, node: &JsForStatement) -> Option<AnyJsStatement> {
        None
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
    pub fn transform_js_if_statement(&self, node: &JsIfStatement) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_js_labeled_statement(
        &self,
        node: &JsLabeledStatement,
    ) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_js_metavariable_to_js_statement(
        &self,
        node: &JsMetavariable,
    ) -> Option<AnyJsStatement> {
        None
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
    pub fn transform_js_switch_statement(
        &self,
        node: &JsSwitchStatement,
    ) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_js_throw_statement(&self, node: &JsThrowStatement) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_js_try_finally_statement(
        &self,
        node: &JsTryFinallyStatement,
    ) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_js_try_statement(&self, node: &JsTryStatement) -> Option<AnyJsStatement> {
        None
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
    pub fn transform_js_while_statement(&self, node: &JsWhileStatement) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_js_with_statement(&self, node: &JsWithStatement) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_ts_declare_function_declaration(
        &self,
        node: &TsDeclareFunctionDeclaration,
    ) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_ts_declare_statement(
        &self,
        node: &TsDeclareStatement,
    ) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_ts_enum_declaration(
        &self,
        node: &TsEnumDeclaration,
    ) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_ts_external_module_declaration(
        &self,
        node: &TsExternalModuleDeclaration,
    ) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_ts_global_declaration(
        &self,
        node: &TsGlobalDeclaration,
    ) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_ts_import_equals_declaration(
        &self,
        node: &TsImportEqualsDeclaration,
    ) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_ts_interface_declaration(
        &self,
        node: &TsInterfaceDeclaration,
    ) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_ts_module_declaration(
        &self,
        node: &TsModuleDeclaration,
    ) -> Option<AnyJsStatement> {
        None
    }
    pub fn transform_ts_type_alias_declaration(
        &self,
        node: &TsTypeAliasDeclaration,
    ) -> Option<AnyJsStatement> {
        None
    }
}
