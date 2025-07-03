use std::sync::LazyLock;

use biome_analyze::MetadataRegistry;
use biome_js_syntax::JsLanguage;
use biome_rowan::BatchMutation;

pub mod declare_transformation;
pub mod registry;
pub mod transformations;
pub mod helpers;

pub use registry::*;
pub use transformations::*;

pub type JsBatchMutation = BatchMutation<JsLanguage>;

pub static METADATA: LazyLock<MetadataRegistry> = LazyLock::new(|| {
    let mut metadata = MetadataRegistry::default();
    visit_transformation_registry(&mut metadata);
    metadata
});
