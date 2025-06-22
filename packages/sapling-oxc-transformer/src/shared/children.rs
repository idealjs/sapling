use oxc_ast::ast::*;

pub fn filter_children<'a>(children: &'a [JSXChild<'a>]) -> Vec<&'a JSXChild<'a>> {
    children
        .iter()
        .filter(|child| match child {
            JSXChild::ExpressionContainer(expr) => {
                !matches!(expr.expression, JSXExpression::EmptyExpression(_))
            }
            JSXChild::Text(text) => !text.value.trim().is_empty(),
            _ => true,
        })
        .collect()
}
