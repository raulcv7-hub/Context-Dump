use context::adapters::fs_scanner::FsScanner;
use context::core::config::ContextConfig;
use context::ports::scanner::ProjectScanner;
use std::collections::HashSet;
use std::fs::File;
use tempfile::tempdir;

/// Validates that extension whitelisting correctly flags non-matching files as ignored.
#[test]
fn test_extension_filtering() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    File::create(root.join("logic.rs")).unwrap();
    File::create(root.join("styles.css")).unwrap();

    let mut include_exts = HashSet::new();
    include_exts.insert("rs".to_string());

    let config = ContextConfig {
        root_path: root.to_path_buf(),
        include_extensions: include_exts,
        ..ContextConfig::default()
    };

    let results = FsScanner::new().scan(&config).unwrap();

    let rs_file = results
        .iter()
        .find(|n| n.relative_path.to_string_lossy() == "logic.rs")
        .unwrap();
    let css_file = results
        .iter()
        .find(|n| n.relative_path.to_string_lossy() == "styles.css")
        .unwrap();

    assert!(!rs_file.is_ignored);
    assert!(css_file.is_ignored);
}

/// Validates that path exclusion strings correctly flag matches as ignored.
#[test]
fn test_path_exclusion() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    File::create(root.join("public.rs")).unwrap();
    File::create(root.join("secret.rs")).unwrap();

    let config = ContextConfig {
        root_path: root.to_path_buf(),
        exclude_paths: vec!["secret".to_string()],
        ..ContextConfig::default()
    };

    let results = FsScanner::new().scan(&config).unwrap();
    let secret_file = results
        .iter()
        .find(|n| n.relative_path.to_string_lossy().contains("secret"))
        .unwrap();

    assert!(secret_file.is_ignored);
}
