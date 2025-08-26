use biome_js_semantic::SemanticModel;
use biome_js_syntax::TextRange;

use std::collections::{HashMap, HashSet};

use crate::BitMask;

#[derive(Debug, Clone)]
pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}
/// KeyMask 用于管理字符串 key 与对应的二进制 mask

pub struct SaplingTransformer<'a> {
    pub semantic_model: SemanticModel,
    pub scope_generated_identifiers: HashMap<TextRange, HashSet<String>>,
    pub config: Config,
    pub decorated_members: &'a mut HashSet<String>, // 记录被 State/Derive 装饰的成员变量名
    pub bit_map: &'a mut BitMask,
}

impl<'a> SaplingTransformer<'a> {}
