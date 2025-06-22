pub mod children;
pub mod component;
pub mod condition;
pub mod config;
pub mod dynamic;
pub mod id_gen;
pub mod import;
pub mod jsx_element;
pub mod length_checker;
pub mod native_spread;
pub mod string_utils;
pub mod tag_name;
pub mod text_wrap;
pub mod utils;

pub use children::*;
pub use component::*;
pub use condition::*;
pub use config::*;
pub use dynamic::*;
pub use id_gen::*;
pub use import::*;
pub use jsx_element::*;
pub use length_checker::*;
pub use native_spread::*;
pub use string_utils::*;
pub use tag_name::*;
pub use text_wrap::*;
pub use utils::*;

use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    pub static ref RESERVED_NAMESPACES: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("class");
        set.insert("on");
        set.insert("oncapture");
        set.insert("style");
        set.insert("use");
        set.insert("prop");
        set.insert("attr");
        set.insert("bool");
        set
    };
}
