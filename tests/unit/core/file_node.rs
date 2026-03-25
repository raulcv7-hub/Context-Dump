use context::core::file::FileNode;
use std::path::PathBuf;

/// Verifies that the FileNode constructor correctly maps all metadata fields.
#[test]
fn test_file_node_integrity() {
    let path = PathBuf::from("/tmp/test.rs");
    let rel = PathBuf::from("test.rs");
    
    let node = FileNode::new(
        path.clone(),
        rel.clone(),
        true,  // is_hidden
        false, // is_ignored
        150    // token_estimate
    );

    assert_eq!(node.path, path);
    assert_eq!(node.relative_path, rel);
    assert!(node.is_hidden);
    assert!(!node.is_ignored);
    assert_eq!(node.token_estimate, 150);
}