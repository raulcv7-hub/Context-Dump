use context::core::config::{ContextConfig, OutputFormat};
use std::path::PathBuf;

#[test]
fn test_config_inference_logic() {
    let config = ContextConfig::build_validated(
        PathBuf::from("."),
        Some(PathBuf::from("report.md")),
        OutputFormat::Xml,
        false,
        None,
        false,
        false,
        false,
        false,
        true,
        vec![],
        vec![],
        vec![],
        vec![],
    );

    assert_eq!(config.output_format, OutputFormat::Markdown);
}

#[test]
fn test_default_config_is_xml() {
    let config = ContextConfig::default();
    assert_eq!(config.output_format, OutputFormat::Xml);
}