use std::sync::LazyLock;

use biome_analyze::MetadataRegistry;
use biome_js_syntax::JsLanguage;
use biome_rowan::BatchMutation;

pub mod declare_transformation;
pub mod helpers;
pub mod registry;
pub mod transformations;
pub mod visitor;
pub mod compatible;

pub use registry::*;
pub use transformations::*;
pub use visitor::*;
pub use compatible::*;

pub type JsBatchMutation = BatchMutation<JsLanguage>;

pub static METADATA: LazyLock<MetadataRegistry> = LazyLock::new(|| {
    let mut metadata = MetadataRegistry::default();
    visit_transformation_registry(&mut metadata);
    metadata
});
