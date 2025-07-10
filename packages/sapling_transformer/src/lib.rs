use biome_js_syntax::JsLanguage;
use biome_rowan::BatchMutation;

pub mod compatible;
pub mod transformer;
pub mod write_transformation_snapshot;

pub use compatible::*;
pub use transformer::*;
pub use write_transformation_snapshot::*;

pub type JsBatchMutation = BatchMutation<JsLanguage>;
