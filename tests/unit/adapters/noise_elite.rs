use context::adapters::fs_scanner::FsScanner;
use context::core::config::ContextConfig;
use context::ports::scanner::ProjectScanner;
use std::fs::{self, File};
use tempfile::tempdir;

/// Verifies that cloud and container artifacts are automatically suppressed.
#[test]
fn test_noise_elite_suppression() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create cloud/devops noise
    let aws_dir = root.join(".aws");
    fs::create_dir(&aws_dir).unwrap();
    File::create(aws_dir.join("credentials")).unwrap();

    let terraform_dir = root.join(".terraform");
    fs::create_dir(&terraform_dir).unwrap();

    File::create(root.join("valid.go")).unwrap();

    let config = ContextConfig {
        root_path: root.to_path_buf(),
        smart_ignore: true,
        ..ContextConfig::default()
    };

    let scanner = FsScanner::new();
    let results = scanner.scan(&config).unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].relative_path.to_string_lossy(), "valid.go");
}
