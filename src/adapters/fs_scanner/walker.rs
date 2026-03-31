use anyhow::Result;
use ignore::gitignore::GitignoreBuilder;
use ignore::WalkBuilder;
use std::sync::{Arc, Mutex};
use tracing::warn;

use crate::adapters::fs_scanner::filters::PathFilter;
use crate::adapters::fs_scanner::noise::NoiseDetector;
use crate::core::config::ContextConfig;
use crate::core::content::TokenCounter;
use crate::core::file::FileNode;

/// Engine responsible for traversing the filesystem and categorizing entries.
pub struct WalkerEngine<'a> {
    config: &'a ContextConfig,
}

impl<'a> WalkerEngine<'a> {
    /// Initializes a new WalkerEngine with the given configuration.
    pub fn new(config: &'a ContextConfig) -> Self {
        Self { config }
    }

    /// Executes the traversal, capturing valid files and ignored directories.
    pub fn walk(&self) -> Result<Vec<FileNode>> {
        let root = &self.config.root_path;
        let gitignore = self.build_gitignore(root);

        let dropped_dirs = Arc::new(Mutex::new(Vec::new()));
        let dropped_clone = dropped_dirs.clone();

        let mut builder = WalkBuilder::new(root);
        builder
            .standard_filters(false)
            .hidden(false)
            .follow_links(true)
            .filter_entry(move |entry| {
                let is_noise = NoiseDetector::is_noise(entry);
                if is_noise && entry.file_type().map_or(false, |ft| ft.is_dir()) {
                    if let Ok(mut dirs) = dropped_clone.lock() {
                        dirs.push(entry.path().to_path_buf());
                    }
                }
                !is_noise
            });

        if let Some(depth) = self.config.max_depth {
            builder.max_depth(Some(depth));
        }

        let mut files = self.collect_valid_files(builder, root, &gitignore);
        self.inject_ignored_directories(&mut files, dropped_dirs, root);

        files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
        Ok(files)
    }

    /// Compiles the gitignore rules for the specific project root.
    fn build_gitignore(&self, root: &std::path::Path) -> ignore::gitignore::Gitignore {
        let path = root.join(".gitignore");
        if path.exists() {
            let mut gi_builder = GitignoreBuilder::new(root);
            let _ = gi_builder.add(&path);
            gi_builder
                .build()
                .unwrap_or_else(|_| GitignoreBuilder::new(root).build().unwrap())
        } else {
            GitignoreBuilder::new(root).build().unwrap()
        }
    }

    /// Processes the WalkBuilder iterator into a collection of valid FileNodes.
    fn collect_valid_files(
        &self,
        builder: WalkBuilder,
        root: &std::path::Path,
        gitignore: &ignore::gitignore::Gitignore,
    ) -> Vec<FileNode> {
        let mut files = Vec::new();
        for result in builder.build() {
            let entry = match result {
                Ok(e) => e,
                Err(err) => {
                    warn!("Scanner entry error: {}", err);
                    continue;
                }
            };

            if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                let path = entry.path();
                let file_name = entry.file_name().to_string_lossy();
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();
                let metadata = entry.metadata().ok();

                let is_hidden = file_name.starts_with('.');
                let is_sensitive = NoiseDetector::is_sensitive(&file_name, &ext);
                let is_ignored = is_sensitive || !PathFilter::matches(path, self.config);

                let is_git_ignored = gitignore
                    .matched_path_or_any_parents(path, false)
                    .is_ignore();

                let token_estimate = metadata
                    .map(|m| TokenCounter::estimate_from_size(m.len()))
                    .unwrap_or(0);

                let path_buf = path.to_path_buf();
                let relative = path_buf
                    .strip_prefix(root)
                    .unwrap_or(&path_buf)
                    .to_path_buf();

                files.push(FileNode::new(
                    path_buf,
                    relative,
                    false,
                    is_hidden,
                    is_ignored,
                    is_sensitive,
                    is_git_ignored,
                    token_estimate,
                ));
            }
        }
        files
    }

    /// Appends the intercepted noise directories as collapsed nodes in the file tree.
    fn inject_ignored_directories(
        &self,
        files: &mut Vec<FileNode>,
        dropped_dirs: Arc<Mutex<Vec<std::path::PathBuf>>>,
        root: &std::path::Path,
    ) {
        if let Ok(dirs) = dropped_dirs.lock() {
            for dir_path in dirs.iter() {
                let file_name = dir_path.file_name().unwrap_or_default().to_string_lossy();
                let relative = dir_path.strip_prefix(root).unwrap_or(dir_path).to_path_buf();
                
                if relative.as_os_str().is_empty() {
                    continue;
                }

                files.push(FileNode::new(
                    dir_path.clone(),
                    relative,
                    true,
                    file_name.starts_with('.'),
                    true,
                    NoiseDetector::is_sensitive(&file_name, ""),
                    false,
                    0,
                ));
            }
        }
    }
}