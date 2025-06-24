use oxc_ast::ast::*;

pub fn check_length<'a>(children: &'a [JSXChild<'a>]) -> bool {
    let mut count = 0;
    for child in children {
        if match child {
            JSXChild::ExpressionContainer(expr) => {
                !matches!(expr.expression, JSXExpression::EmptyExpression(_))
            }
            JSXChild::Text(text) => {
                let raw = &text.value;

                let is_pure_whitespace = raw.chars().all(|c| c.is_whitespace());
                let is_pure_spaces = raw.chars().all(|c| c == ' ');

                !is_pure_whitespace || is_pure_spaces
            }
            _ => true,
        } {
            count += 1;
        }

        if count > 1 {
            return true;
        }
    }
    false
}
