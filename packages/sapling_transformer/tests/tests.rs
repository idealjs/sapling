#[cfg(test)]
mod tests {
    use biome_analyze::{AnalysisFilter, ControlFlow, Never, RuleFilter};

    use biome_formatter::{FormatError, IndentStyle, PrintError, Printed};
    use biome_js_formatter::context::JsFormatOptions;
    use biome_js_formatter::format_node;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_semantic::{SemanticModelOptions, semantic_model};
    use biome_js_syntax::{JsFileSource, JsModule};
    use biome_rowan::{BatchMutation, BatchMutationExt};
    use biome_test_utils::register_leak_checker;
    use camino::Utf8Path;
    use sapling_transformer::{SaplingTransformer, write_transformation_snapshot};
    use std::ops::Deref;
    use std::{fs::read_to_string, slice};

    mod attribute_expressions {
        #[test]
        pub fn index_tsx() {
            let test_file = "tests/specs/attribute_expressions/index.tsx";
            crate::tests::run_test(test_file);
        }
    }

    mod components {
        #[test]
        pub fn index_tsx() {
            let test_file = "tests/specs/components/index.tsx";
            crate::tests::run_test(test_file);
        }
    }

    mod conditional_expressions {
        #[test]
        pub fn index_tsx() {
            let test_file = "tests/specs/conditional_expressions/index.tsx";
            crate::tests::run_test(test_file);
        }
    }

    mod fragments {
        #[test]
        pub fn index_tsx() {
            let test_file = "tests/specs/fragments/index.tsx";
            crate::tests::run_test(test_file);
        }
    }

    mod insert_children {
        #[test]
        pub fn index_tsx() {
            let test_file = "tests/specs/insert_children/index.tsx";
            crate::tests::run_test(test_file);
        }
    }

    mod jsx_template {
        #[test]
        pub fn index_tsx() {
            let test_file = "tests/specs/jsx_template/index.tsx";
            crate::tests::run_test(test_file);
        }
    }

    mod simple_elements {
        #[test]
        pub fn index_tsx() {
            let test_file = "tests/specs/simple_elements/index.tsx";
            crate::tests::run_test(test_file);
        }
    }

    mod text_interpolation {
        #[test]
        pub fn index_tsx() {
            let test_file = "tests/specs/text_interpolation/index.tsx";
            crate::tests::run_test(test_file);
        }
    }

    fn run_test(input: &'static str) -> Option<()> {
        register_leak_checker();

        let input_file = Utf8Path::new(input);
        let file_name = input_file.file_name().unwrap();

        let mut snapshot = String::new();

        let input_code = read_to_string(input_file)
            .unwrap_or_else(|err| panic!("failed to read {input_file:?}: {err:?}"));

        let parsed_root = parse(
            input_code.as_str(),
            JsFileSource::tsx(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed_root.tree(), SemanticModelOptions::default());

        let js_tree = parsed_root.try_tree()?;
        let js_module = js_tree.as_js_module()?;

        let mut transformer = SaplingTransformer {
            mutation: js_module.clone().begin(),
            js_module: js_module.clone(),
            pre_process_errors: Vec::new(),
        };

        transformer.transform();

        let node = transformer.mutation.commit();

        let source_type = input_file.try_into().ok()?;
        let formatted = format_node(
            JsFormatOptions::new(source_type).with_indent_style(IndentStyle::Space),
            &node,
        )
        .ok()?
        .print()
        .ok()?
        .as_code()
        .to_string();

        write_transformation_snapshot(
            &mut snapshot,
            input_code.as_str(),
            formatted.as_str(),
            source_type.file_extension(),
        );

        insta::with_settings!({
            prepend_module_to_snapshot => false,
            snapshot_path => {
                let path = input_file.parent().unwrap();
                let path_str = path.as_str();
                if let Some(idx) = path_str.find("specs/") {
                    Utf8Path::new(&path_str[idx..])
                } else {
                    path
                }
            },
        },
        {
            insta::assert_snapshot!(file_name, snapshot, file_name);
        });

        None
    }
}
