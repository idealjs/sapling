use biome_js_semantic::SemanticModel;
use biome_js_syntax::TextRange;

use std::collections::{HashMap, HashSet};

use crate::{BitMask, StringTree};

#[derive(Debug, Clone)]
pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}

pub struct SaplingTransformer<'a> {
    pub semantic_model: SemanticModel,
    pub scope_generated_identifiers: HashMap<TextRange, HashSet<String>>,
    pub config: Config,
    pub decorated_members: &'a mut HashSet<String>,
    pub bit_map: &'a mut BitMask,
    pub string_tree: &'a mut StringTree,
}

impl<'a> SaplingTransformer<'a> {}
