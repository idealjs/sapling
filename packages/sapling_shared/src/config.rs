#[derive(Debug)]
pub struct Config<'a> {
    pub module_name: String,
    pub generate: String,
    pub hydratable: bool,
    pub delegate_events: bool,
    pub delegated_events: Vec<&'a str>,
    pub built_ins: Vec<&'a str>,
    pub require_import_source: Option<&'a str>,
    pub wrap_conditionals: bool,
    pub omit_nested_closing_tags: bool,
    pub omit_last_closing_tag: bool,
    pub omit_quotes: bool,
    pub context_to_custom_elements: bool,
    pub static_marker: &'a str,
    pub effect_wrapper: &'a str,
    pub memo_wrapper: &'a str,
    pub validate: bool,
}

impl<'a> Config<'a> {
    pub fn new() -> Self {
        Self {
            module_name: "dom".to_string(),
            generate: "dom".to_string(),
            hydratable: false,
            delegate_events: true,
            delegated_events: Vec::new(),
            built_ins: Vec::new(),
            require_import_source: None,
            wrap_conditionals: true,
            omit_nested_closing_tags: false,
            omit_last_closing_tag: true,
            omit_quotes: true,
            context_to_custom_elements: false,
            static_marker: "@once",
            effect_wrapper: "effect",
            memo_wrapper: "memo",
            validate: true,
        }
    }
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Self::new()
    }
}
