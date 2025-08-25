use biome_js_semantic::SemanticModel;
use biome_js_syntax::TextRange;

use std::collections::{HashMap, HashSet};

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
    pub decorated_members: &'a mut HashSet<String>, // 记录被 State/Derive 装饰的成员变量名
}

impl<'a> SaplingTransformer<'a> {}
