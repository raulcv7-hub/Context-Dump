use anyhow::Result;
use ignore::WalkBuilder;
use tracing::warn;

use crate::adapters::fs_scanner::filters::PathFilter;
use crate::adapters::fs_scanner::noise::NoiseDetector;
use crate::core::config::ContextConfig;
use crate::core::file::FileNode;
use crate::core::content::TokenCounter;
use crate::ports::scanner::ProjectScanner;

/// Filesystem scanner implementation using the ignore crate.
pub struct FsScanner;

impl FsScanner {
    /// Creates a new FsScanner instance.
    pub fn new() -> Self {
        Self
    }
}

impl Default for FsScanner {
    /// Standard implementation for FsScanner.
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectScanner for FsScanner {
    /// Scans the directory tree and populates node metadata with token estimates.
    fn scan(&self, config: &ContextConfig) -> Result<Vec<FileNode>> {
        let root = &config.root_path;
        let mut builder = WalkBuilder::new(root);
        builder.standard_filters(false).hidden(false)
            .filter_entry(|entry| !NoiseDetector::is_noise(entry));

        if let Some(depth) = config.max_depth {
            builder.max_depth(Some(depth));
        }

        let mut files = Vec::new();
        for result in builder.build() {
            let entry = match result {
                Ok(e) => e,
                Err(err) => { warn!("Scanner entry error: {}", err); continue; }
            };

            if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                let path = entry.path();
                let file_name = entry.file_name().to_string_lossy();
                let metadata = entry.metadata().ok();

                let is_hidden = file_name.starts_with('.');
                let is_ignored = !PathFilter::matches(path, config);
                let token_estimate = metadata.map(|m| TokenCounter::estimate_from_size(m.len()))
                    .unwrap_or(0);

                let path_buf = path.to_path_buf();
                let relative = path_buf.strip_prefix(root).unwrap_or(&path_buf).to_path_buf();

                files.push(FileNode::new(path_buf, relative, is_hidden, is_ignored, token_estimate));
            }
        }

        files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
        Ok(files)
    }
}