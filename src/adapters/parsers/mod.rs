pub mod docling;
pub mod text_parser;

pub use docling::DoclingParser;
pub use text_parser::PlainTextParser;

use anyhow::Result;
use std::path::Path;

/// Trait universal para extracción de contenido.
pub trait FileParser: Send + Sync {
    fn parse(&self, path: &Path) -> Result<String>;
}
