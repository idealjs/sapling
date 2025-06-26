pub mod children;
pub mod component;
pub mod condition;
pub mod config;
pub mod dom;
pub mod dynamic;
pub mod html_nesting;
pub mod id_gen;
pub mod import;
pub mod jsx_element;
pub mod length_checker;
pub mod native_spread;
pub mod pre_process;
pub mod ssr;
pub mod string_utils;
pub mod tag_name;
pub mod text_wrap;
pub mod tree_builder;
pub mod utils;
pub mod validate;

pub use children::*;
pub use component::*;
pub use condition::*;
pub use config::*;
pub use dynamic::*;
pub use id_gen::*;
pub use import::*;
pub use length_checker::*;
pub use native_spread::*;
pub use string_utils::*;
pub use tag_name::*;
pub use text_wrap::*;
pub use dom::constants::*;
pub use ssr::{append_templates::*, template::*};
pub use tree_builder::*;

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
