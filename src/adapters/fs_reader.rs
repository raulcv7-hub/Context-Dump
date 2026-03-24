use crate::adapters::parsers::{FileParser, NativePdfParser, PlainTextParser, NativeOfficeParser};
use crate::core::config::ContextConfig;
use crate::core::content::{ContentType, FileContext};
use crate::core::file::FileNode;
use crate::ports::reader::FileReader;
use std::path::Path;

pub struct FsReader {
    pdf_engine: NativePdfParser,
    text_engine: PlainTextParser,
    office_engine: NativeOfficeParser,
}

impl FsReader {
    /**
     * Initializes the filesystem reader with all native engines.
     */
    pub fn new() -> Self {
        Self {
            pdf_engine: NativePdfParser::new(),
            text_engine: PlainTextParser::new(),
            office_engine: NativeOfficeParser::new(),
        }
    }

    /**
     * Extracts the file extension for routing logic.
     */
    fn get_extension(&self, path: &Path) -> String {
        path.extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_else(|| "text".to_string())
    }
}

impl FileReader for FsReader {
    /**
     * Dispatches the file to the correct native parser.
     */
    fn read_file(&self, node: &FileNode, _config: &ContextConfig) -> FileContext {
        let ext = self.get_extension(&node.path);

        let result = match ext.as_str() {
            "pdf" => self.pdf_engine.parse(&node.path),
            "docx" | "xlsx" | "xls" => self.office_engine.parse(&node.path),
            _ => self.text_engine.parse(&node.path),
        };

        match result {
            Ok(content) => {
                let tokens = content.len() / 4;
                FileContext::new(
                    node.path.clone(),
                    node.relative_path.clone(),
                    ContentType::Text(content),
                    ext,
                    tokens,
                )
            }
            Err(e) => FileContext::new(
                node.path.clone(),
                node.relative_path.clone(),
                ContentType::Error(e.to_string()),
                ext,
                0,
            ),
        }
    }
}

impl Default for FsReader {
    /**
     * Default constructor for FsReader.
     */
    fn default() -> Self {
        Self::new()
    }
}