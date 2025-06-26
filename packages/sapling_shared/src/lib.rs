pub mod children;
pub mod component;
pub mod component_transform;
pub mod condition;
pub mod config;
pub mod config_utils;
pub mod custom_element;
pub mod dom;
pub mod dom_element;
pub mod dynamic;
pub mod dynamic_wrapper;
pub mod element_attributes;
pub mod element_children;
pub mod element_utils;
pub mod escape_utils;
pub mod event_handler;
pub mod expression_detector;
pub mod fragment_transform;
pub mod html_constants;
pub mod html_nesting;
pub mod id_gen;
pub mod import;
pub mod jsx_utils;
pub mod jsx_validator;
pub mod length_checker;
pub mod namespace_constants;
pub mod native_spread;
pub mod node_transform;
pub mod processor;
pub mod spread_attributes;
pub mod ssr;
pub mod string_utils;
pub mod tag_name;
pub mod template_append;
pub mod text_wrap;
pub mod tree_builder;
pub mod utils;
pub mod validate;

// Re-exports of commonly used items
pub use children::*;
pub use component::*;
pub use condition::*;
pub use config::*;
pub use dom::constants::*;
pub use dom_element::*;
pub use dynamic::*;
pub use element_attributes::*;
pub use id_gen::*;
pub use import::*;
pub use jsx_utils::*;
pub use length_checker::*;
pub use native_spread::*;
pub use ssr::*;
pub use string_utils::*;
pub use tag_name::*;
pub use text_wrap::*;
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
