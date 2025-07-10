use biome_js_syntax::{AnyJsxElementName, AnyJsxObjectName};

pub fn jsx_element_name_to_string(node: &AnyJsxElementName) -> Option<String> {
    match node {
        AnyJsxElementName::JsxMemberName(jsx_member_name) => {
            let object = {
                let object = jsx_member_name.object().ok()?;
                let object_element_name = match object {
                    AnyJsxObjectName::JsxNamespaceName(v) => AnyJsxElementName::JsxNamespaceName(v),
                    AnyJsxObjectName::JsxMemberName(v) => AnyJsxElementName::JsxMemberName(v),
                    AnyJsxObjectName::JsxReferenceIdentifier(v) => {
                        AnyJsxElementName::JsxReferenceIdentifier(v)
                    }
                };
                jsx_element_name_to_string(&object_element_name)?
            };
            let property = jsx_member_name
                .member()
                .ok()?
                .value_token()
                .ok()?
                .text()
                .to_string();
            Some(format!("{}.{}", object, property))
        }
        AnyJsxElementName::JsxName(v) => Some(v.value_token().ok()?.text().to_string()),
        AnyJsxElementName::JsxReferenceIdentifier(v) => {
            Some(v.value_token().ok()?.text().to_string())
        }
        AnyJsxElementName::JsMetavariable(v) => Some(v.value_token().ok()?.text().to_string()),
        AnyJsxElementName::JsxNamespaceName(ns) => {
            let namespace = ns.namespace().ok()?.value_token().ok()?.text().to_string();
            let name = ns.name().ok()?.value_token().ok()?.text().to_string();
            Some(format!("{}:{}", namespace, name))
        }
    }
}
