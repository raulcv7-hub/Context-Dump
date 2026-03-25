use context::adapters::fs_scanner::FsScanner;
use context::core::config::ContextConfig;
use context::ports::scanner::ProjectScanner;
use std::fs::{self, File};
use tempfile::tempdir;

/// Verifies that standard noise (VCS and binaries) is automatically flagged.
#[test]
fn test_noise_detector_basic_flagging() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let git_dir = root.join(".git");
    fs::create_dir(&git_dir).unwrap();
    File::create(git_dir.join("config")).unwrap();
    File::create(root.join("main.rs")).unwrap();

    let config = ContextConfig {
        root_path: root.to_path_buf(),
        smart_ignore: true,
        ..ContextConfig::default()
    };

    let scanner = FsScanner::new();
    let results = scanner.scan(&config).unwrap();

    let main_node = results.iter().find(|n| n.relative_path.to_string_lossy() == "main.rs").unwrap();
    assert!(!main_node.is_ignored);

    // .git should be filtered out entirely by the WalkBuilder if it's noise
    let git_found = results.iter().any(|n| n.relative_path.to_string_lossy().contains(".git"));
    assert!(!git_found);
}