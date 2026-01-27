use anyhow::Result;
use ignore::WalkBuilder;
use tracing::warn;

use crate::adapters::fs_scanner::filters::PathFilter;
use crate::adapters::fs_scanner::noise::NoiseDetector;
use crate::core::config::ContextConfig;
use crate::core::file::FileNode;
use crate::ports::scanner::ProjectScanner;

pub struct FsScanner;

impl FsScanner {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FsScanner {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectScanner for FsScanner {
    /// Escanea el sistema de archivos y calcula una estimación de tokens por archivo.
    fn scan(&self, config: &ContextConfig) -> Result<Vec<FileNode>> {
        let root = &config.root_path;
        let mut builder = WalkBuilder::new(root);
        builder
            .standard_filters(false)
            .hidden(false)
            .filter_entry(|entry| !NoiseDetector::is_noise(entry));

        if let Some(depth) = config.max_depth {
            builder.max_depth(Some(depth));
        }

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
                let metadata = entry.metadata().ok();

                let is_hidden = file_name.starts_with('.');
                let is_ignored = !PathFilter::matches(path, config);
                let token_estimate = metadata.map(|m| m.len() as usize / 4).unwrap_or(0);

                let path_buf = path.to_path_buf();
                let relative = path_buf
                    .strip_prefix(root)
                    .unwrap_or(&path_buf)
                    .to_path_buf();

                files.push(FileNode::new(
                    path_buf,
                    relative,
                    is_hidden,
                    is_ignored,
                    token_estimate,
                ));
            }
        }

        files.sort_by(|a: &FileNode, b: &FileNode| a.relative_path.cmp(&b.relative_path));
        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::ContextConfig;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_scanner_isolation_and_hidden_capture() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let hidden_name = ".hidden_test";
        File::create(root.join(hidden_name)).unwrap();
        File::create(root.join("visible.rs")).unwrap();

        // FIX: Uso de struct update syntax en lugar de mutación post-default
        let config = ContextConfig {
            root_path: root.to_path_buf(),
            ..ContextConfig::default()
        };

        let scanner = FsScanner::new();
        let results = scanner.scan(&config).unwrap();

        let hidden_exists = results
            .iter()
            .any(|n| n.is_hidden && n.relative_path.to_string_lossy() == hidden_name);
        assert!(hidden_exists, "Should capture the hidden file");
        assert!(results.len() >= 2);
    }

    #[test]
    fn test_scanner_noise_exclusion() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let git_dir = root.join(".git");
        std::fs::create_dir(&git_dir).unwrap();
        File::create(git_dir.join("config")).unwrap();
        File::create(root.join("legit.rs")).unwrap();

        // FIX: Uso de struct update syntax en lugar de mutación post-default
        let config = ContextConfig {
            root_path: root.to_path_buf(),
            ..ContextConfig::default()
        };

        let scanner = FsScanner::new();
        let results = scanner.scan(&config).unwrap();

        for node in &results {
            assert!(!node.relative_path.to_string_lossy().contains(".git"));
        }
        assert!(results
            .iter()
            .any(|n| n.relative_path.to_string_lossy() == "legit.rs"));
    }
}
