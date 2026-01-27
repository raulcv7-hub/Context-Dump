pub mod bridge;
pub mod dto;
pub mod resolver;

use crate::adapters::parsers::docling::bridge::DoclingBridge;
pub use crate::adapters::parsers::docling::dto::DoclingResult;
use crate::adapters::parsers::FileParser;
use anyhow::Result;
use std::path::Path;

pub struct DoclingParser;

impl DoclingParser {
    pub fn new() -> Self {
        Self
    }

    /// Ingiere el archivo y devuelve el DTO con contenido y tokens.
    pub fn parse_elite(&self, path: &Path) -> Result<DoclingResult> {
        DoclingBridge::run(path)
    }
}

impl Default for DoclingParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FileParser for DoclingParser {
    /// Implementación del trait FileParser.
    fn parse(&self, path: &Path) -> Result<String> {
        self.parse_elite(path).map(|r| r.content)
    }
}
