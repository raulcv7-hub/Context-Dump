use context::adapters::fs_scanner::FsScanner;
use context::core::config::ContextConfig;
use context::ports::scanner::ProjectScanner;
use std::fs::{self, File};
use tempfile::tempdir;

/// Verifies that path inclusion acts as a whitelist for specific subdirectories.
#[test]
fn test_filter_path_inclusion_logic() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let api_dir = root.join("api");
    fs::create_dir(&api_dir).unwrap();
    File::create(api_dir.join("service.rs")).unwrap();
    File::create(root.join("main.rs")).unwrap();

    let config = ContextConfig {
        root_path: root.to_path_buf(),
        include_paths: vec!["api".to_string()],
        ..ContextConfig::default()
    };

    let results = FsScanner::new().scan(&config).unwrap();
    
    let service = results.iter().find(|n| n.relative_path.to_string_lossy().contains("service.rs")).unwrap();
    let main = results.iter().find(|n| n.relative_path.to_string_lossy() == "main.rs").unwrap();

    assert!(!service.is_ignored);
    assert!(main.is_ignored);
}