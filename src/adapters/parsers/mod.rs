pub mod pdf_parser;
pub mod text_parser;
pub mod office_parser;
pub mod notebook_parser;

pub use pdf_parser::NativePdfParser;
pub use text_parser::PlainTextParser;
pub use office_parser::NativeOfficeParser;
pub use notebook_parser::NotebookParser;

use anyhow::Result;
use std::path::Path;

/// Universal interface for all native file extractors.
pub trait FileParser: Send + Sync {
    /// Parses a specific file format into a plain string.
    fn parse(&self, path: &Path) -> Result<String>;
}