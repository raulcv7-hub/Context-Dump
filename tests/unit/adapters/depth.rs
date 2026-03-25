use context::adapters::fs_scanner::FsScanner;
use context::core::config::ContextConfig;
use context::ports::scanner::ProjectScanner;
use std::fs::{self, File};
use tempfile::tempdir;

/// Verifies that the scanner respects the maximum depth constraint.
#[test]
fn test_scanner_max_depth_constraint() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    File::create(root.join("level0.txt")).unwrap();
    let dir1 = root.join("dir1");
    fs::create_dir(&dir1).unwrap();
    File::create(dir1.join("level1.txt")).unwrap();

    let config = ContextConfig {
        root_path: root.to_path_buf(),
        max_depth: Some(2), // Root is level 0, dir1 is 1, level1.txt is 2
        ..ContextConfig::default()
    };

    let scanner = FsScanner::new();
    let results = scanner.scan(&config).unwrap();
    let paths: Vec<String> = results.iter()
        .map(|n| n.relative_path.to_string_lossy().to_string())
        .collect();

    assert!(paths.iter().any(|p| p.contains("level0.txt")));
    assert!(paths.iter().any(|p| p.contains("level1.txt")));
}

/// Verifies that the hidden files flag correctly marks dotfiles.
#[test]
fn test_scanner_hidden_files_toggle() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    File::create(root.join(".env")).unwrap();

    let config = ContextConfig {
        root_path: root.to_path_buf(),
        include_hidden: true,
        ..ContextConfig::default()
    };

    let results = FsScanner::new().scan(&config).unwrap();
    let hidden_file = results.iter().find(|n| n.relative_path.to_string_lossy() == ".env").unwrap();
    
    assert!(hidden_file.is_hidden);
}