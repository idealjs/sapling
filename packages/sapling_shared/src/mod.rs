//! Sapling JSX transformation modules

// Constants
pub mod html_constants;
pub mod namespace_constants;

// Core JSX handling
pub mod jsx_utils;
pub mod jsx_validator;
pub mod component_transform;
pub mod fragment_transform;
pub mod node_transform;

// Element handling
pub mod dom_element;
pub mod element_attributes;
pub mod element_children;
pub mod element_utils;
pub mod custom_element;

// Attribute handling
pub mod spread_attributes;
pub mod event_handler;

// Dynamic content
pub mod dynamic_wrapper;
pub mod expression_detector;

// Template handling
pub mod template_append;

// String utilities
pub mod string_utils;
pub mod escape_utils;
pub mod text_wrap;
pub mod tag_name;

// Processing stages
pub mod preprocessor;
pub mod post_processor;

// Children handling
pub mod children;
pub mod length_checker;

// Configuration and utilities
pub mod config_utils;
pub mod id_gen;
pub mod native_spread;

// Re-exports of commonly used items
pub use children::{filter_children, check_length};
pub use dom_element::transform_element;
pub use element_attributes::{transform_attributes, set_attr};
pub use jsx_utils::convert_jsx_identifier;
pub use string_utils::{escape_html, trim_whitespace, escape_string_for_template};
pub use tag_name::{jsx_element_name_to_string, tag_name_to_identifier};
