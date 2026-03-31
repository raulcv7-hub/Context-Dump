use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

/// Verifies that the --help flag returns the correct application identity string.
#[test]
fn test_cli_help_output() {
    let mut cmd = Command::cargo_bin("context").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Native Context Engine for LLMs"));
}

/// Verifies that the --version flag is functional.
#[test]
fn test_cli_version_output() {
    let mut cmd = Command::cargo_bin("context").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("context"));
}
