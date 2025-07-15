use biome_js_syntax::JsLanguage;
use biome_rowan::BatchMutation;

pub mod compatible;
pub mod create_statement;
pub mod scope;
pub mod transform_expression;
pub mod transform_js_variable_declarator;
pub mod transform_jsx_child;
pub mod transform_jsx_tag;
pub mod transform_statement;
pub mod transformer;
pub mod write_transformation_snapshot;

pub use compatible::*;
pub use create_statement::*;
pub use scope::*;
pub use transform_expression::*;
pub use transform_js_variable_declarator::*;
pub use transform_jsx_child::*;
pub use transform_jsx_tag::*;
pub use transform_statement::*;
pub use transformer::*;
pub use write_transformation_snapshot::*;

pub type JsBatchMutation = BatchMutation<JsLanguage>;
