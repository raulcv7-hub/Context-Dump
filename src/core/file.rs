use std::path::PathBuf;

/// Domain model representing a discovered file with its metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileNode {
    /// Absolute path to the file on the local filesystem.
    pub path: PathBuf,
    /// Path relative to the defined project root.
    pub relative_path: PathBuf,
    /// Whether the file is hidden (starts with a dot).
    pub is_hidden: bool,
    /// Whether the file is excluded by smart ignore heuristics.
    pub is_ignored: bool,
    /// Estimated token count for LLM context budget management.
    pub token_estimate: usize,
}

impl FileNode {
    /// Constructs a new FileNode with the provided metadata.
    pub fn new(
        path: PathBuf,
        relative_path: PathBuf,
        is_hidden: bool,
        is_ignored: bool,
        token_estimate: usize,
    ) -> Self {
        Self {
            path,
            relative_path,
            is_hidden,
            is_ignored,
            token_estimate,
        }
    }
}