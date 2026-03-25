use context::adapters::fs_scanner::FsScanner;
use context::core::config::ContextConfig;
use context::ports::scanner::ProjectScanner;
use crate::common;

#[test]
fn test_scanner_finds_standard_files() {
    let dir = common::create_mock_project().unwrap();
    let config = ContextConfig {
        root_path: dir.path().to_path_buf(),
        ..ContextConfig::default()
    };

    let scanner = FsScanner::new();
    let results = scanner.scan(&config).unwrap();

    let file_names: Vec<String> = results
        .iter()
        .map(|n| n.relative_path.to_string_lossy().into_owned())
        .collect();

    assert!(file_names.contains(&"main.rs".to_string()));
}