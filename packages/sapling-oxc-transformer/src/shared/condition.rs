// use oxc_ast::ast::*;
// use oxc_span::Span;
// use crate::shared::dynamic::{is_dynamic, DynamicFlags};

// pub fn transform_condition(path: &AstPath, inline: bool, deep: bool) -> Expression {
//     let expr = path.node();
//     let config = crate::shared::config::get_config(path).unwrap();
//     let memo = crate::shared::import::register_import_method(path, config.memo_wrapper.as_str(), None);

//     if let Expression::ConditionalExpression(conditional) = expr {
//         let span = conditional.span;
//         if is_dynamic(&path.get("consequent"), DynamicFlags {
//             check_tags: true,
//             ..Default::default()
//         }) {
//             let test = &conditional.test;
//             let cond = if !matches!(test, Expression::BinaryExpression(_)) {
//                 Expression::UnaryExpression(UnaryExpression {
//                     span,
//                     operator: UnaryOperator::Not,
//                     argument: Box::new(Expression::UnaryExpression(UnaryExpression {
//                         span,
//                         operator: UnaryOperator::Not,
//                         argument: Box::new(test.clone()),
//                         prefix: true
//                     })),
//                     prefix: true
//                 })
//             } else {
//                 test.clone()
//             };

//             if inline {
//                 return Expression::CallExpression(Box::new(CallExpression {
//                     span,
//                     callee: Box::new(Expression::Identifier(Identifier {
//                         span,
//                         name: memo.name,
//                         optional: false,
//                     })),
//                     arguments: vec![Expression::ArrowFunction(Box::new(ArrowFunction {
//                         span,
//                         params: vec![],
//                         body: Box::new(cond),
//                         is_async: false,
//                         is_generator: false,
//                         return_type: None,
//                         type_parameters: None,
//                     }))],
//                     optional: false,
//                     type_arguments: None,
//                 }));
//             }
//         }
//     }

//     Expression::ArrowFunction(Box::new(ArrowFunction {
//         span: Span::default(),
//         params: vec![],
//         body: Box::new(expr.clone()),
//         is_async: false,
//         is_generator: false,
//         return_type: None,
//         type_parameters: None,
//     }))
// }
