use crate::adapters::parsers::{
    ArchiveParser, FileParser, NativeOfficeParser, NativePdfParser, NotebookParser, PlainTextParser,
};
use crate::core::config::ContextConfig;
use crate::core::content::{CodeAnalyzer, ContentType, FileContext, PiiMasker, TokenCounter};
use crate::core::file::FileNode;
use crate::ports::reader::FileReader;
use std::path::Path;

pub struct FsReader {
    pdf_engine: NativePdfParser,
    text_engine: PlainTextParser,
    office_engine: NativeOfficeParser,
    notebook_engine: NotebookParser,
    archive_engine: ArchiveParser,
}

impl FsReader {
    pub fn new() -> Self {
        Self {
            pdf_engine: NativePdfParser::new(),
            text_engine: PlainTextParser::new(),
            office_engine: NativeOfficeParser::new(),
            notebook_engine: NotebookParser::new(),
            archive_engine: ArchiveParser::new(),
        }
    }

    fn get_extension(&self, path: &Path) -> String {
        path.extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_else(|| "text".to_string())
    }
}

impl FileReader for FsReader {
    fn read_file(&self, node: &FileNode, config: &ContextConfig) -> FileContext {
        let ext = self.get_extension(&node.path);

        if node.token_estimate > config.max_tokens_per_file {
            return FileContext::new(
                node.path.clone(),
                node.relative_path.clone(),
                ContentType::Omitted,
                ext,
                0,
                false,
            );
        }

        let result = match ext.as_str() {
            "pdf" => self.pdf_engine.parse(&node.path),
            "docx" | "xlsx" | "xls" => self.office_engine.parse(&node.path),
            "ipynb" => self.notebook_engine.parse(&node.path),
            "zip" | "gz" | "tgz" => self.archive_engine.parse(&node.path),
            _ => self.text_engine.parse(&node.path),
        };

        match result {
            Ok(content) => {
                let sanitized = PiiMasker::mask(&content);
                let tokens = TokenCounter::count(&sanitized);
                let is_suspicious = CodeAnalyzer::is_suspicious(&sanitized);

                FileContext::new(
                    node.path.clone(),
                    node.relative_path.clone(),
                    ContentType::Text(sanitized),
                    ext,
                    tokens,
                    is_suspicious,
                )
            }
            Err(e) => FileContext::new(
                node.path.clone(),
                node.relative_path.clone(),
                ContentType::Error(e.to_string()),
                ext,
                0,
                false,
            ),
        }
    }
}

impl Default for FsReader {
    fn default() -> Self {
        Self::new()
    }
}
