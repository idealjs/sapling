use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;
use sapling_shared::{config::Config, processor::pre_process_ast};

#[test]
fn test_config_merging() {
    let source = "// Empty program";
    let allocator = Allocator::default();
    let source_type = SourceType::default().with_typescript(true).with_jsx(true);
    let ret = Parser::new(&allocator, source, source_type).parse();
    let program = ret.program;

    // Test with default config
    let empty_config = Config::default();
    let result = pre_process_ast(&program, &empty_config);
    assert_eq!(result.hydratable, false);
    assert_eq!(result.delegate_events, true);
    assert_eq!(result.validate, true);

    // Test with custom config
    let custom_config = Config {
        module_name: "test".to_string(),
        hydratable: false,
        delegate_events: false,
        validate: false,
        ..Default::default()
    };
    let result = pre_process_ast(&program, &custom_config);
    assert_eq!(result.hydratable, false);
    assert_eq!(result.delegate_events, false);
    assert_eq!(result.validate, false);
    assert_eq!(result.module_name, "test");
}

#[test]
fn test_jsx_import_source() {
    // Test with matching import source
    let source = r#"
        // @jsxImportSource sapling
        function App() {
            return <div>Hello</div>;
        }
    "#;
    let allocator = Allocator::default();
    let source_type = SourceType::default().with_typescript(true).with_jsx(true);
    let ret = Parser::new(&allocator, source, source_type).parse();
    let program = ret.program;

    let sapling_str = "sapling";
    let config = Config {
        require_import_source: Some(&sapling_str),
        ..Default::default()
    };
    let result = pre_process_ast(&program, &config);
    assert_eq!(result.require_import_source, Some(sapling_str));

    // Test without import source comment
    let source = r#"
        function App() {
            return <div>Hello</div>;
        }
    "#;
    let ret = Parser::new(&allocator, source, source_type).parse();
    let program = ret.program;
    let result = pre_process_ast(&program, &config);
    assert_eq!(result.require_import_source, Some(sapling_str));
}

#[test]
fn test_valid_jsx_nesting() {
    let source = r#"
        function App() {
            return (
                <div>
                    <p>Text</p>
                    <span><em>Emphasis</em></span>
                </div>
            );
        }
    "#;
    let allocator = Allocator::default();
    let source_type = SourceType::default().with_typescript(true).with_jsx(true);
    let ret = Parser::new(&allocator, source, source_type).parse();
    let program = ret.program;

    let config = Config {
        validate: true,
        ..Default::default()
    };

    // Should not panic for valid nesting
    pre_process_ast(&program, &config);
}

#[test]
#[should_panic(expected = "Invalid JSX: <div> cannot be child of <p>")]
fn test_invalid_jsx_nesting() {
    let source = r#"
        function App() {
            return (
                <p>
                    <div>Invalid nesting</div>
                </p>
            );
        }
    "#;
    let allocator = Allocator::default();
    let source_type = SourceType::default().with_typescript(true).with_jsx(true);
    let ret = Parser::new(&allocator, source, source_type).parse();
    let program = ret.program;

    let config = Config {
        validate: true,
        ..Default::default()
    };

    // Should panic for invalid nesting
    pre_process_ast(&program, &config);
}

#[test]
fn test_component_nesting() {
    let source = r#"
        function App() {
            return (
                <CustomComponent>
                    <div>
                        <p>Valid component nesting</p>
                    </div>
                </CustomComponent>
            );
        }
    "#;
    let allocator = Allocator::default();
    let source_type = SourceType::default().with_typescript(true).with_jsx(true);
    let ret = Parser::new(&allocator, source, source_type).parse();
    let program = ret.program;

    let config = Config {
        validate: true,
        ..Default::default()
    };

    // Should not panic as components can contain any elements
    pre_process_ast(&program, &config);
}

#[test]
fn test_mixed_jsx_nesting() {
    let source = r#"
        function App() {
            return (
                <div>
                    <CustomComponent>
                        <p>Valid nesting</p>
                    </CustomComponent>
                    <p><span>Also valid</span></p>
                </div>
            );
        }
    "#;
    let allocator = Allocator::default();
    let source_type = SourceType::default().with_typescript(true).with_jsx(true);
    let ret = Parser::new(&allocator, source, source_type).parse();
    let program = ret.program;

    let config = Config {
        validate: true,
        ..Default::default()
    };

    // Should not panic for valid mixed nesting
    pre_process_ast(&program, &config);
}
