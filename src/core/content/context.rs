use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ContentType {
    Text(String),
    Binary,
    Omitted,
    Error(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct FileContext {
    pub path: PathBuf,
    pub relative_path: PathBuf,
    pub content: ContentType,
    pub language: String,
    pub token_count: usize,
    pub is_suspicious: bool,
}

impl FileContext {
    pub fn new(
        p: PathBuf,
        rp: PathBuf,
        c: ContentType,
        l: String,
        tc: usize,
        suspicious: bool,
    ) -> Self {
        Self {
            path: p,
            relative_path: rp,
            content: c,
            language: l,
            token_count: tc,
            is_suspicious: suspicious,
        }
    }
}
