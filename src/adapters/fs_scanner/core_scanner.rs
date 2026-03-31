use anyhow::Result;

use crate::adapters::fs_scanner::walker::WalkerEngine;
use crate::core::config::ContextConfig;
use crate::core::file::FileNode;
use crate::ports::scanner::ProjectScanner;

/// Facade for the filesystem scanning engine.
pub struct FsScanner;

impl FsScanner {
    /// Creates a new instance of the FsScanner.
    pub fn new() -> Self {
        Self
    }
}

impl Default for FsScanner {
    /// Provides the default implementation for FsScanner.
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectScanner for FsScanner {
    /// Initiates the scanning process utilizing the segmented WalkerEngine.
    fn scan(&self, config: &ContextConfig) -> Result<Vec<FileNode>> {
        let engine = WalkerEngine::new(config);
        engine.walk()
    }
}