use biome_js_syntax::JsLanguage;
use biome_rowan::BatchMutation;

pub mod compatible;
pub mod create_statement;
pub mod scope;
pub mod transformer;
pub mod write_transformation_snapshot;

pub use compatible::*;
pub use create_statement::*;
pub use scope::*;
pub use transformer::*;
pub use write_transformation_snapshot::*;

pub type JsBatchMutation = BatchMutation<JsLanguage>;
