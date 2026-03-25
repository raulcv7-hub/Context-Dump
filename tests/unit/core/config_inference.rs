use context::core::config::{ContextConfig, OutputFormat};
use std::path::PathBuf;

/// Verifies that the engine infers XML format for unknown or missing extensions.
#[test]
fn test_inference_default_fallback() {
    let config = ContextConfig::build_validated(
        PathBuf::from("."), None, OutputFormat::Xml, false, 
        None, false, false, false, false, true, 
        vec![], vec![], vec![], vec![]
    );
    assert_eq!(config.output_format, OutputFormat::Xml);
}

/// Verifies that .md files trigger the Markdown output format automatically.
#[test]
fn test_inference_markdown_detection() {
    let config = ContextConfig::build_validated(
        PathBuf::from("."), 
        Some(PathBuf::from("output.md")), 
        OutputFormat::Xml, // Initial default
        false, // Not explicit
        None, false, false, false, false, true, 
        vec![], vec![], vec![], vec![]
    );
    assert_eq!(config.output_format, OutputFormat::Markdown);
}