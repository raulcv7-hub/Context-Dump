use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ContentType {
    Text(String),
    Binary,
    Error(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct FileContext {
    pub path: PathBuf,
    pub relative_path: PathBuf,
    pub content: ContentType,
    pub language: String,
    pub token_count: usize,
}

impl FileContext {
    /// Creates a new FileContext instance.
    pub fn new(
        path: PathBuf,
        relative_path: PathBuf,
        content: ContentType,
        language: String,
        token_count: usize,
    ) -> Self {
        Self {
            path,
            relative_path,
            content,
            language,
            token_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_context_creation() {
        let ctx = FileContext::new(
            PathBuf::from("a"),
            PathBuf::from("a"),
            ContentType::Binary,
            "rs".into(),
            0,
        );
        assert_eq!(ctx.language, "rs");
    }
}
