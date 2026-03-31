use context::adapters::fs_scanner::FsScanner;
use context::core::config::ContextConfig;
use context::ports::scanner::ProjectScanner;
use std::fs::{self, File};
use tempfile::tempdir;

#[cfg(unix)]
#[test]
/// Ensures that the scanner follows symbolic links to directories on Unix systems.
fn test_scanner_follows_symlinks() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let target_dir = tempdir().unwrap();
    File::create(target_dir.path().join("linked_file.txt")).unwrap();

    let link_path = root.join("my_link");
    std::os::unix::fs::symlink(target_dir.path(), &link_path).unwrap();

    let config = ContextConfig {
        root_path: root.to_path_buf(),
        ..ContextConfig::default()
    };

    let scanner = FsScanner::new();
    let results = scanner.scan(&config).unwrap();

    assert!(results.iter().any(|n| n
        .relative_path
        .to_string_lossy()
        .contains("linked_file.txt")));
}

#[test]
/// Ensures that directories are not incorrectly identified as noise based on their size.
fn test_scanner_ignores_directory_size_as_noise() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let sub = root.join("valid_dir");
    fs::create_dir(&sub).unwrap();
    File::create(sub.join("file.rs")).unwrap();

    let config = ContextConfig {
        root_path: root.to_path_buf(),
        ..ContextConfig::default()
    };

    let scanner = FsScanner::new();
    let results = scanner.scan(&config).unwrap();

    assert!(results
        .iter()
        .any(|n| n.relative_path.to_string_lossy().contains("valid_dir")));
}
