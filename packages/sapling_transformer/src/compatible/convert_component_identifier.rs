use biome_js_factory::make::jsx_name;
use biome_js_factory::make::{
    js_reference_identifier, js_static_member_expression, js_string_literal,
    js_string_literal_expression, js_this_expression,
};
use biome_js_syntax::AnyJsLiteralExpression;
use biome_js_syntax::JsSyntaxKind;
use biome_js_syntax::{
    AnyJsExpression, AnyJsxElementName, AnyJsxObjectName, JsName, JsSyntaxToken,
};

fn is_valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => (),
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn convert_name_token(name_token: &JsSyntaxToken) -> Option<AnyJsExpression> {
    let name = name_token.text();
    if name == "this" {
        let this_token = JsSyntaxToken::new_detached(
            JsSyntaxKind::THIS_KW,
            "this",
            std::iter::empty(),
            std::iter::empty(),
        );
        Some(js_this_expression(this_token).into())
    } else if is_valid_identifier(&name) {
        let ident = js_reference_identifier(name_token.clone());
        let ident_expr = biome_js_factory::make::js_identifier_expression(ident);
        Some(ident_expr.into())
    } else {
        let lit_token = js_string_literal(name);
        let lit_expr = js_string_literal_expression(lit_token);
        Some(AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(lit_expr),
        ))
    }
}

pub fn convert_component_identifier(node: &AnyJsxElementName) -> Option<AnyJsExpression> {
    match node {
        AnyJsxElementName::JsxName(v) => {
            let name_token = v.value_token().ok()?;
            convert_name_token(&name_token)
        }
        AnyJsxElementName::JsxReferenceIdentifier(v) => {
            let name_token = v.value_token().ok()?;
            convert_name_token(&name_token)
        }
        AnyJsxElementName::JsxMemberName(member) => {
            let object = member.object().ok()?;
            let property = member.member().ok()?;
            let object_element_name = match object {
                AnyJsxObjectName::JsxNamespaceName(v) => AnyJsxElementName::JsxNamespaceName(v),
                AnyJsxObjectName::JsxMemberName(v) => AnyJsxElementName::JsxMemberName(v),
                AnyJsxObjectName::JsxReferenceIdentifier(v) => {
                    AnyJsxElementName::JsxReferenceIdentifier(v)
                }
            };
            let obj_expr = convert_component_identifier(&object_element_name)?;
            let prop_expr = convert_component_identifier(&AnyJsxElementName::JsxName(jsx_name(
                property.value_token().ok()?,
            )))?;
            let computed = matches!(prop_expr, AnyJsExpression::AnyJsLiteralExpression(_));
            if computed {
                let l_brack_token = JsSyntaxToken::new_detached(
                    JsSyntaxKind::L_BRACK,
                    "[",
                    std::iter::empty(),
                    std::iter::empty(),
                );
                let r_brack_token = JsSyntaxToken::new_detached(
                    JsSyntaxKind::R_BRACK,
                    "]",
                    std::iter::empty(),
                    std::iter::empty(),
                );
                let member = biome_js_factory::make::js_computed_member_expression(
                    obj_expr,
                    l_brack_token,
                    prop_expr,
                    r_brack_token,
                )
                .build();
                Some(member.into())
            } else {
                let obj = match obj_expr {
                    biome_js_syntax::AnyJsExpression::JsIdentifierExpression(expr) => {
                        biome_js_syntax::AnyJsExpression::JsIdentifierExpression(expr.clone())
                    }
                    biome_js_syntax::AnyJsExpression::JsCallExpression(expr) => {
                        biome_js_syntax::AnyJsExpression::JsCallExpression(expr.clone())
                    }
                    _ => return None,
                };
                let prop = prop_expr.as_js_identifier_expression()?;
                use biome_rowan::AstNode;
                let prop_name =
                    biome_js_syntax::AnyJsName::JsName(JsName::cast(prop.clone().into_syntax())?);
                let dot_token = JsSyntaxToken::new_detached(
                    JsSyntaxKind::DOT,
                    ".",
                    std::iter::empty(),
                    std::iter::empty(),
                );
                let member =
                    js_static_member_expression(obj.clone(), dot_token.clone(), prop_name.clone());
                Some(member.into())
            }
        }
        AnyJsxElementName::JsMetavariable(v) => {
            let name_token = v.value_token().ok()?;
            let ident = js_reference_identifier(name_token.clone());
            let ident_expr = biome_js_factory::make::js_identifier_expression(ident);
            Some(ident_expr.into())
        }
        AnyJsxElementName::JsxNamespaceName(ns) => {
            let namespace_token = ns.namespace().ok()?.value_token().ok()?;
            let namespace = namespace_token.text();
            let name_token = ns.name().ok()?.value_token().ok()?;
            let name = name_token.text();
            let lit_token = js_string_literal(&format!("{}:{}", namespace, name));
            let lit_expr = js_string_literal_expression(lit_token);
            Some(AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(lit_expr),
            ))
        }
    }
}

pub fn get_name_from_any_js_expression(expression: &AnyJsExpression) -> Option<String> {
    match expression {
        AnyJsExpression::AnyJsLiteralExpression(expression) => {
            Some(expression.value_token().ok()?.text().to_string())
        }
        AnyJsExpression::JsComputedMemberExpression(expression) => Some(expression.to_string()),
        AnyJsExpression::JsStaticMemberExpression(expression) => Some(expression.to_string()),
        _ => None,
    }
}
