use crate::common;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

/// Verifies that the --stdout flag correctly dumps XML content to the terminal.
#[test]
fn test_cli_stdout_xml_format() {
    let dir = common::create_mock_project().unwrap();
    let mut cmd = Command::cargo_bin("context").unwrap();

    cmd.current_dir(dir.path())
        .arg(".")
        .arg("--stdout")
        .arg("--format")
        .arg("xml");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("<context>"))
        .stdout(predicate::str::contains("<file path=\"main.rs\""));
}

/// Verifies that the --stdout flag correctly dumps Markdown content to the terminal.
#[test]
fn test_cli_stdout_markdown_format() {
    let dir = common::create_mock_project().unwrap();
    let mut cmd = Command::cargo_bin("context").unwrap();

    cmd.current_dir(dir.path())
        .arg(".")
        .arg("--stdout")
        .arg("--format")
        .arg("markdown");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("# Project Context Report"))
        .stdout(predicate::str::contains("### `main.rs`"))
        .stdout(predicate::str::contains("```rs"));
}
