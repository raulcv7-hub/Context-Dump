use context::adapters::output::xml::XmlWriter;
use context::adapters::output::markdown::MarkdownWriter;
use context::core::config::ContextConfig;
use context::core::content::{ContentType, FileContext};
use context::ports::writer::ContextWriter;
use std::path::PathBuf;

/// Verifies that the XML output contains the expected CDATA sections for text content.
#[test]
fn test_xml_writer_cdata_integrity() {
    let config = ContextConfig::default();
    let files = vec![FileContext::new(
        PathBuf::from("a.rs"), PathBuf::from("a.rs"),
        ContentType::Text("code".into()), "rs".into(), 1
    )];

    let mut buf = Vec::new();
    XmlWriter::new().write(&files, &config, &mut buf).unwrap();
    let out = String::from_utf8(buf).unwrap();
    assert!(out.contains("<![CDATA[code]]>"));
}

/// Verifies that the Markdown writer uses proper code block fences.
#[test]
fn test_markdown_writer_fences() {
    let config = ContextConfig::default();
    let files = vec![FileContext::new(
        PathBuf::from("t.py"), PathBuf::from("t.py"),
        ContentType::Text("pass".into()), "py".into(), 1
    )];

    let mut buf = Vec::new();
    MarkdownWriter::new().write(&files, &config, &mut buf).unwrap();
    let out = String::from_utf8(buf).unwrap();
    assert!(out.contains("```py\npass```"));
}