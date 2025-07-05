#[cfg(test)]
mod tests {
    use biome_analyze::{AnalysisFilter, ControlFlow, Never, RuleFilter};
    use biome_formatter::IndentStyle;
    use biome_js_formatter::context::JsFormatOptions;
    use biome_js_formatter::format_node;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_syntax::JsFileSource;
    use biome_rowan::AstNode;
    use biome_test_utils::{
        assert_diagnostics_expectation_comment, create_analyzer_options, diagnostic_to_string,
        register_leak_checker, scripts_from_json, write_transformation_snapshot,
    };
    use camino::Utf8Path;
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

    fn run_test(input: &'static str) {
        register_leak_checker();

        let input_file = Utf8Path::new(input);
        let file_name = input_file.file_name().unwrap();

        let rule_filter = RuleFilter::Rule("transformations", "jsx_template");
        let filter = AnalysisFilter {
            enabled_rules: Some(slice::from_ref(&rule_filter)),
            ..AnalysisFilter::default()
        };

        let mut snapshot = String::new();
        let extension = input_file.extension().unwrap_or_default();

        let input_code = read_to_string(input_file)
            .unwrap_or_else(|err| panic!("failed to read {input_file:?}: {err:?}"));

        if let Some(scripts) = scripts_from_json(extension, &input_code) {
            for script in scripts {
                analyze_and_snap(
                    &mut snapshot,
                    &script,
                    JsFileSource::js_script(),
                    filter,
                    file_name,
                    input_file,
                    JsParserOptions::default(),
                );
            }
        } else {
            let Ok(source_type) = input_file.try_into() else {
                return;
            };
            analyze_and_snap(
                &mut snapshot,
                &input_code,
                source_type,
                filter,
                file_name,
                input_file,
                JsParserOptions::default(),
            );
        };

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
    }

    pub(crate) fn analyze_and_snap(
        snapshot: &mut String,
        input_code: &str,
        source_type: JsFileSource,
        filter: AnalysisFilter,
        file_name: &str,
        input_file: &Utf8Path,
        parser_options: JsParserOptions,
    ) {
        let parsed = parse(input_code, source_type, parser_options.clone());
        let root = parsed.tree();

        let mut diagnostics = Vec::new();
        let options = create_analyzer_options(input_file, &mut diagnostics);

        let mut transformations = vec![];
        let (_, errors) =
            sapling_transformer::transform(&root, filter, &options, source_type, |event| {
                for transformation in event.transformations() {
                    let node = transformation.mutation.commit();
                    let formatted = format_node(
                        JsFormatOptions::new(source_type).with_indent_style(IndentStyle::Space),
                        &node,
                    )
                    .unwrap();

                    transformations.push(formatted.print().unwrap().as_code().to_string());
                }
                ControlFlow::<Never>::Continue(())
            });

        for error in errors {
            diagnostics.push(diagnostic_to_string(file_name, input_code, error));
        }

        write_transformation_snapshot(
            snapshot,
            input_code,
            transformations.as_slice(),
            source_type.file_extension(),
        );

        assert_diagnostics_expectation_comment(input_file, root.syntax(), diagnostics.len());
    }
}
