use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use crate::common;

/// Verifies that the --extensions flag restricts output to specific file types.
#[test]
fn test_cli_extension_whitelist() {
    let dir = common::create_mock_project().unwrap();
    let mut cmd = Command::cargo_bin("context").unwrap();
    
    // We only want README.md, so we filter by 'md' extension
    cmd.current_dir(dir.path())
        .arg(".")
        .arg("--stdout")
        .arg("-e")
        .arg("md");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("README.md"))
        .stdout(predicate::str::contains("main.rs").not());
}

/// Verifies that the --exclude-path flag correctly omits directories.
#[test]
fn test_cli_path_exclusion() {
    let dir = common::create_mock_project().unwrap();
    let mut cmd = Command::cargo_bin("context").unwrap();
    
    cmd.current_dir(dir.path())
        .arg(".")
        .arg("--stdout")
        .arg("-X")
        .arg("src");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("README.md"))
        .stdout(predicate::str::contains("lib.rs").not());
}