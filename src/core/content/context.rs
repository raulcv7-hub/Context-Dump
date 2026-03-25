use serde::Serialize;
use std::path::PathBuf;

/// Categorization of file content after extraction.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ContentType {
    /// Extracted UTF-8 string.
    Text(String),
    /// Marker for skipped binary data.
    Binary,
    /// Metadata regarding an extraction failure.
    Error(String),
}

/// Integrated representation of a file's data and system metadata.
#[derive(Debug, Clone, Serialize)]
pub struct FileContext {
    pub path: PathBuf,
    pub relative_path: PathBuf,
    pub content: ContentType,
    pub language: String,
    pub token_count: usize,
}

impl FileContext {
    /// Constructor for a new FileContext.
    pub fn new(
        p: PathBuf,
        rp: PathBuf,
        c: ContentType,
        l: String,
        tc: usize,
    ) -> Self {
        Self { path: p, relative_path: rp, content: c, language: l, token_count: tc }
    }
}