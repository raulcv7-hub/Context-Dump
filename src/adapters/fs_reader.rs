use crate::adapters::parsers::{DoclingParser, FileParser, PlainTextParser};
use crate::core::config::ContextConfig;
use crate::core::content::{ContentMinifier, ContentType, FileContext};
use crate::core::file::FileNode;
use crate::ports::reader::FileReader;
use std::path::Path;

pub struct FsReader {
    docling: DoclingParser,
    text_engine: PlainTextParser,
}

impl FsReader {
    pub fn new() -> Self {
        Self {
            docling: DoclingParser::new(),
            text_engine: PlainTextParser::new(),
        }
    }

    fn get_extension(&self, path: &Path) -> String {
        path.extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_else(|| "text".to_string())
    }

    /// Filtro de seguridad: Solo estos formatos tocan el motor de IA.
    fn is_complex_binary(&self, ext: &str) -> bool {
        matches!(
            ext,
            "pdf" | "docx" | "xlsx" | "xls" | "pptx" | "png" | "jpg" | "jpeg" | "tiff"
        )
    }
}

impl FileReader for FsReader {
    fn read_file(&self, node: &FileNode, config: &ContextConfig) -> FileContext {
        let ext = self.get_extension(&node.path);

        if self.is_complex_binary(&ext) {
            match self.docling.parse_elite(&node.path) {
                Ok(res) => {
                    let text = if config.minify {
                        ContentMinifier::minify(&res.content, "md")
                    } else {
                        res.content
                    };
                    FileContext::new(
                        node.path.clone(),
                        node.relative_path.clone(),
                        ContentType::Text(text),
                        ext,
                        res.token_count,
                    )
                }
                Err(e) => FileContext::new(
                    node.path.clone(),
                    node.relative_path.clone(),
                    ContentType::Error(format!("IA Engine Fail: {}", e)),
                    ext,
                    0,
                ),
            }
        } else {
            // Motores de texto rápido para md, rs, Makefile, etc.
            match self.text_engine.parse(&node.path) {
                Ok(txt) => {
                    let text = if config.minify {
                        ContentMinifier::minify(&txt, &ext)
                    } else {
                        txt
                    };
                    let tokens = text.len() / 4;
                    FileContext::new(
                        node.path.clone(),
                        node.relative_path.clone(),
                        ContentType::Text(text),
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
}

impl Default for FsReader {
    fn default() -> Self {
        Self::new()
    }
}
