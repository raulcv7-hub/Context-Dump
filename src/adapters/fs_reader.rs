use crate::adapters::parsers::{FileParser, NativePdfParser, PlainTextParser, NativeOfficeParser, NotebookParser};
use crate::core::config::ContextConfig;
use crate::core::content::{ContentType, FileContext, TokenCounter};
use crate::core::file::FileNode;
use crate::ports::reader::FileReader;
use std::path::Path;

/// Filesystem reader that manages specialized native parsers.
pub struct FsReader {
    pdf_engine: NativePdfParser,
    text_engine: PlainTextParser,
    office_engine: NativeOfficeParser,
    notebook_engine: NotebookParser,
}

impl FsReader {
    /// Initializes the reader with all supported native parsing engines.
    pub fn new() -> Self {
        Self {
            pdf_engine: NativePdfParser::new(),
            text_engine: PlainTextParser::new(),
            office_engine: NativeOfficeParser::new(),
            notebook_engine: NotebookParser::new(),
        }
    }

    fn get_extension(&self, path: &Path) -> String {
        path.extension().and_then(|e| e.to_str()).map(|s| s.to_lowercase())
            .unwrap_or_else(|| "text".to_string())
    }
}

impl FileReader for FsReader {
    /// Reads and tokenizes file content using the appropriate native engine.
    fn read_file(&self, node: &FileNode, _config: &ContextConfig) -> FileContext {
        let ext = self.get_extension(&node.path);

        let result = match ext.as_str() {
            "pdf" => self.pdf_engine.parse(&node.path),
            "docx" | "xlsx" | "xls" => self.office_engine.parse(&node.path),
            "ipynb" => self.notebook_engine.parse(&node.path),
            _ => self.text_engine.parse(&node.path),
        };

        match result {
            Ok(content) => {
                let tokens = TokenCounter::count(&content);
                FileContext::new(node.path.clone(), node.relative_path.clone(), 
                    ContentType::Text(content), ext, tokens)
            }
            Err(e) => FileContext::new(node.path.clone(), node.relative_path.clone(),
                ContentType::Error(e.to_string()), ext, 0),
        }
    }
}

impl Default for FsReader {
    /// Provides the default implementation for FsReader.
    fn default() -> Self {
        Self::new()
    }
}