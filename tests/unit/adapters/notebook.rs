use context::adapters::parsers::{FileParser, NotebookParser};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

/// Verifies that the NotebookParser correctly processes standard Jupyter JSON structures.
/// Uses double-hash raw strings to avoid prefix ambiguities with markdown headers.
#[test]
fn test_notebook_extraction_logic() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test.ipynb");

    let json = r##"{
        "cells": [
            {"cell_type": "code", "source": ["print(1)"]},
            {"cell_type": "markdown", "source": "# Hello"}
        ]
    }"##;

    File::create(&path)
        .unwrap()
        .write_all(json.as_bytes())
        .unwrap();

    let parser = NotebookParser::new();
    let result = parser.parse(&path).expect("Failed to parse valid notebook");

    assert!(result.contains("print(1)"));
    assert!(result.contains("# Hello"));
    assert!(result.contains("markdown"));
}

/// Verifies that malformed notebook files return a descriptive error.
#[test]
fn test_notebook_invalid_json_error() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("bad.ipynb");
    File::create(&path)
        .unwrap()
        .write_all(b"not a json object")
        .unwrap();

    let parser = NotebookParser::new();
    let result = parser.parse(&path);
    assert!(result.is_err());
}
