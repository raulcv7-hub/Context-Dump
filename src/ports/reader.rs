use crate::core::config::ContextConfig;
use crate::core::content::FileContext;
use crate::core::file::FileNode;

/// Interface for reading and processing file content.
pub trait FileReader: Send + Sync {
    /// Reads a file node and converts it into a full FileContext.
    /// It now requires access to ContextConfig to handle minification and context-aware parsing.
    fn read_file(&self, node: &FileNode, config: &ContextConfig) -> FileContext;
}
