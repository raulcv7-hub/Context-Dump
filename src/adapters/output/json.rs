use anyhow::Result;
use chrono::Local;
use serde::Serialize;
use std::ffi::OsStr;
use std::io::Write;

use crate::core::config::ContextConfig;
use crate::core::content::{ContentMinifier, ContentType, FileContext};
use crate::core::tree::TreeRenderer;
use crate::ports::writer::ContextWriter;

#[derive(Serialize)]
struct JsonReport<'a> {
    metadata: JsonMetadata,
    files: &'a [FileContext],
}

#[derive(Serialize)]
struct JsonMetadata {
    project_root: String,
    scan_time: String,
    stats: JsonStats,
    directory_tree: String,
}

#[derive(Serialize)]
struct JsonStats {
    total_files: usize,
    total_tokens: usize,
}

#[derive(Default)]
pub struct JsonWriter;

impl JsonWriter {
    pub fn new() -> Self {
        Self
    }
}

impl ContextWriter for JsonWriter {
    fn write<W: Write>(
        &self,
        files: &[FileContext],
        config: &ContextConfig,
        writer: W,
    ) -> Result<()> {
        let total_tokens: usize = files.iter().map(|f: &FileContext| f.token_count).sum();
        let root_name = config
            .root_path
            .file_name()
            .map(|n: &OsStr| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| ".".to_string());

        let tree_paths: Vec<_> = files
            .iter()
            .map(|f: &FileContext| &f.relative_path)
            .collect();
        let tree_view = TreeRenderer::build(&tree_paths).render(&root_name);

        let processed_files: Vec<FileContext> = if config.minify {
            files
                .iter()
                .map(|f: &FileContext| {
                    let mut new_f = f.clone();
                    if let ContentType::Text(ref t) = f.content {
                        new_f.content = ContentType::Text(ContentMinifier::minify(t, &f.language));
                    }
                    new_f
                })
                .collect()
        } else {
            files.to_vec()
        };

        let report = JsonReport {
            metadata: JsonMetadata {
                project_root: config.root_path.to_string_lossy().to_string(),
                scan_time: Local::now().to_rfc3339(),
                stats: JsonStats {
                    total_files: files.len(),
                    total_tokens,
                },
                directory_tree: tree_view,
            },
            files: &processed_files,
        };

        serde_json::to_writer_pretty(writer, &report)?;
        Ok(())
    }
}
