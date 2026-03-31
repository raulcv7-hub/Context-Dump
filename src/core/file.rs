use std::path::PathBuf;

/// Domain model representing a discovered file or directory with its metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileNode {
    pub path: PathBuf,
    pub relative_path: PathBuf,
    pub is_dir: bool,
    pub is_hidden: bool,
    pub is_ignored: bool,
    pub is_sensitive: bool,
    pub is_git_ignored: bool,
    pub token_estimate: usize,
}

impl FileNode {
    /// Constructs a new FileNode instance representing a filesystem entry.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        path: PathBuf,
        relative_path: PathBuf,
        is_dir: bool,
        is_hidden: bool,
        is_ignored: bool,
        is_sensitive: bool,
        is_git_ignored: bool,
        token_estimate: usize,
    ) -> Self {
        Self {
            path,
            relative_path,
            is_dir,
            is_hidden,
            is_ignored,
            is_sensitive,
            is_git_ignored,
            token_estimate,
        }
    }
}