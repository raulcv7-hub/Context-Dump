use crate::adapters::parsers::FileParser;
use anyhow::{anyhow, Result};
use std::path::Path;

pub struct NativePdfParser;

impl NativePdfParser {
    /// Factory for the native PDF parser.
    pub fn new() -> Self {
        Self
    }
}

impl Default for NativePdfParser {
    /// Default implementation for NativePdfParser.
    fn default() -> Self {
        Self::new()
    }
}

impl FileParser for NativePdfParser {
    /// Extracts text from a PDF file using the native pdf-extract crate.
    fn parse(&self, path: &Path) -> Result<String> {
        match pdf_extract::extract_text(path) {
            Ok(content) => {
                if content.trim().is_empty() {
                    return Err(anyhow!("PDF appears to be empty or image-based"));
                }
                Ok(content)
            }
            Err(e) => Err(anyhow!("Failed to extract PDF text: {}", e)),
        }
    }
}
