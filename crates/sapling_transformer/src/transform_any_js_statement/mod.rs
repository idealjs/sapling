use biome_js_factory::make::{js_class_member_list, js_function_declaration};
use biome_js_semantic::BindingExtensions;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsDecorator, AnyJsPropertyModifier, AnyJsStatement, JsClassDeclaration,
    JsFunctionDeclaration, JsReturnStatement, TsExternalModuleDeclaration, TsGlobalDeclaration,
    TsImportEqualsDeclaration, TsInterfaceDeclaration, TsModuleDeclaration, TsTypeAliasDeclaration,
};

use crate::{SaplingTransformer, get_js_module_source_from_binding, make_js_return_statement};

impl SaplingTransformer<'_> {
    // main entry
    pub fn transform_any_js_statement(&mut self, node: &AnyJsStatement) -> Option<AnyJsStatement> {
        match node {
            AnyJsStatement::JsClassDeclaration(inner) => self.transform_js_class_declaration(inner),
            AnyJsStatement::JsFunctionDeclaration(inner) => {
                self.transform_js_function_declaration(inner)
            }
            AnyJsStatement::JsReturnStatement(inner) => self.transform_js_return_statement(inner),
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
            _ => {
                unreachable!()
            }
        }
    }
    pub fn transform_js_function_declaration(
        &mut self,
        node: &JsFunctionDeclaration,
    ) -> Option<AnyJsStatement> {
        let new_func = js_function_declaration(
            node.function_token().ok()?,
            node.id().ok()?,
            node.parameters().ok()?,
            node.body().ok()?.clone(),
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
    pub fn transform_js_class_declaration(
        &mut self,
        node: &JsClassDeclaration,
    ) -> Option<AnyJsStatement> {
        self.decorated_members.clear();
        let mut new_members = vec![];

        for member in node.members() {
            match member {
                AnyJsClassMember::JsPropertyClassMember(member) => {
                    for modifer in member.modifiers() {
                        match modifer {
                            AnyJsPropertyModifier::JsDecorator(modifer) => {
                                let expr = modifer.expression().ok()?;
                                let binding = match expr {
                                    AnyJsDecorator::JsIdentifierExpression(expr) => {
                                        expr.name().ok()?.binding(&self.semantic_model)
                                    }
                                    AnyJsDecorator::JsStaticMemberExpression(expr) => expr
                                        .object()
                                        .ok()?
                                        .as_js_identifier_expression()?
                                        .name()
                                        .ok()?
                                        .binding(&self.semantic_model),
                                    _ => None,
                                }?;
                                let js_module_source = get_js_module_source_from_binding(
                                    &self.semantic_model,
                                    &binding,
                                )?;
                                if js_module_source == "@idealjs/sapling" {
                                    self.decorated_members.insert(
                                        member
                                            .name()
                                            .ok()?
                                            .as_js_literal_member_name()?
                                            .name()
                                            .ok()?
                                            .text()
                                            .into(),
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        for member in node.members() {
            match member {
                AnyJsClassMember::JsMethodClassMember(member) => {
                    let body = member.body().ok()?;
                    new_members.push(AnyJsClassMember::JsMethodClassMember(
                        member.clone().with_body(body.clone()),
                    ));
                }
                _ => {
                    new_members.push(member);
                }
            }
        }
        Some(AnyJsStatement::JsClassDeclaration(
            node.clone().with_members(js_class_member_list(new_members)),
        ))
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
