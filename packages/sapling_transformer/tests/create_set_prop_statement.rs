use biome_js_factory::make::ident;
use biome_js_factory::make::jsx_attribute;
use biome_js_factory::make::jsx_attribute_initializer_clause;
use biome_js_factory::make::jsx_expression_attribute_value;
use biome_js_factory::make::jsx_name;
use biome_js_factory::make::jsx_namespace_name;
use biome_js_factory::make::jsx_string;
use biome_js_factory::make::jsx_string_literal;
use biome_js_factory::make::token;
use biome_js_factory::make::{
    js_number_literal, js_number_literal_expression, js_string_literal_expression,
};
use biome_js_syntax::AnyJsExpression;
use biome_js_syntax::AnyJsLiteralExpression;
use biome_js_syntax::AnyJsxAttribute;
use biome_js_syntax::T;
use sapling_transformer::compatible::set_prop_statement::generate_set_prop_statement;

#[test]
fn test_create_set_prop_statement() {
    // id="foo"
    let id_attr = AnyJsxAttribute::from(
        jsx_attribute(jsx_name(ident("id")).into())
            .with_initializer(jsx_attribute_initializer_clause(
                token(T![=]),
                jsx_string(jsx_string_literal("foo")).into(),
            ))
            .build(),
    );

    // title={"bar"}
    let title_attr = AnyJsxAttribute::from(
        jsx_attribute(jsx_name(ident("title")).into())
            .with_initializer(jsx_attribute_initializer_clause(
                token(T![=]),
                jsx_expression_attribute_value(
                    token(T!['{']),
                    AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(
                            js_string_literal_expression(jsx_string_literal("bar")),
                        ),
                    ),
                    token(T!['}']),
                )
                .into(),
            ))
            .build(),
    );

    // foo:some={0}
    let foo_attr = AnyJsxAttribute::from(
        jsx_attribute(
            jsx_namespace_name(
                jsx_name(ident("foo")),
                token(T![:]),
                jsx_name(ident("some")),
            )
            .into(),
        )
        .with_initializer(jsx_attribute_initializer_clause(
            token(T![=]),
            jsx_expression_attribute_value(
                token(T!['{']),
                AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsNumberLiteralExpression(
                        js_number_literal_expression(js_number_literal("0")),
                    ),
                ),
                token(T!['}']),
            )
            .into(),
        ))
        .build(),
    );

    let stmt1 = generate_set_prop_statement("_el$", id_attr).expect("stmt1 is None");
    let stmt2 = generate_set_prop_statement("_el$", title_attr).expect("stmt2 is None");
    let stmt3 = generate_set_prop_statement("_el$", foo_attr).expect("stmt3 is None");

    insta::assert_snapshot!(format!(
        "{}\n{}\n{}",
        stmt1.to_string(),
        stmt2.to_string(),
        stmt3.to_string()
    ));
}
