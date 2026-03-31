use context::core::tree::TreeRenderer;
use std::path::PathBuf;

/// Verifies the ASCII tree generation for directory structures.
#[test]
fn test_tree_ascii_rendering() {
    let paths = vec![PathBuf::from("main.rs"), PathBuf::from("Cargo.toml")];

    let renderer = TreeRenderer::build(&paths);
    let output = renderer.render("project");

    assert!(output.contains("project"));
    assert!(output.contains("main.rs"));
    assert!(output.contains("Cargo.toml"));
}
