use context::adapters::fs_reader::FsReader;
use context::adapters::fs_scanner::FsScanner;
use context::adapters::output::xml::XmlWriter;
use context::core::config::{ContextConfig, OutputFormat};
use context::ports::reader::FileReader;
use context::ports::scanner::ProjectScanner;
use context::ports::writer::ContextWriter;
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_full_engine_logic_flow() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir(root.join("src")).unwrap();
    File::create(root.join("src/main.rs"))
        .unwrap()
        .write_all(b"fn main() { println!(\"Hello\"); }")
        .unwrap();
    File::create(root.join("Cargo.toml"))
        .unwrap()
        .write_all(b"[package]\nname = \"test\"")
        .unwrap();

    let config = ContextConfig {
        root_path: root.to_path_buf(),
        output_format: OutputFormat::Xml,
        ..ContextConfig::default()
    };

    let scanner = FsScanner::new();
    let nodes = scanner.scan(&config).expect("Scanning should succeed");
    assert_eq!(nodes.len(), 2);

    let reader = FsReader::new();
    let contexts: Vec<_> = nodes
        .iter()
        .map(|node| reader.read_file(node, &config))
        .collect();

    let context_refs: Vec<_> = contexts.iter().collect();

    assert_eq!(contexts.len(), 2);

    let mut buffer = Vec::new();
    let writer = XmlWriter::new();
    writer
        .write(&context_refs, &config, &mut buffer)
        .expect("Writing should succeed");

    let output = String::from_utf8(buffer).unwrap();

    assert!(output.contains("<context>"));
    assert!(output.contains("main.rs"));
    assert!(output.contains("Cargo.toml"));
    assert!(output.contains("fn main()"));
}
