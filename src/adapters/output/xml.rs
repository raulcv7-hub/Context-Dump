use anyhow::Result;
use chrono::Local;
use quick_xml::events::{BytesCData, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::ffi::OsStr;
use std::io::Write;

use crate::core::config::ContextConfig;
use crate::core::content::{ContentMinifier, ContentType, FileContext};
use crate::core::tree::TreeRenderer;
use crate::ports::writer::ContextWriter;

#[derive(Default)]
pub struct XmlWriter;

impl XmlWriter {
    pub fn new() -> Self {
        Self
    }
}

impl ContextWriter for XmlWriter {
    /// Genera un reporte XML detallado, informando errores específicos por archivo.
    fn write<W: Write>(
        &self,
        files: &[FileContext],
        config: &ContextConfig,
        writer: W,
    ) -> Result<()> {
        let mut xml = Writer::new_with_indent(writer, b' ', 4);
        xml.write_event(Event::Start(BytesStart::new("context")))?;

        // Sección de Metadatos
        xml.write_event(Event::Start(BytesStart::new("metadata")))?;
        xml.create_element("project_root")
            .write_text_content(BytesText::new(&config.root_path.to_string_lossy()))?;
        xml.create_element("scan_time")
            .write_text_content(BytesText::new(&Local::now().to_rfc3339()))?;

        let total_tokens: usize = files.iter().map(|f: &FileContext| f.token_count).sum();
        xml.write_event(Event::Start(BytesStart::new("stats")))?;
        xml.create_element("total_files")
            .write_text_content(BytesText::new(&files.len().to_string()))?;
        xml.create_element("total_tokens")
            .write_text_content(BytesText::new(&total_tokens.to_string()))?;
        xml.write_event(Event::End(BytesEnd::new("stats")))?;

        let root_name = config
            .root_path
            .file_name()
            .map(|n: &OsStr| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| ".".to_string());

        let tree_paths: Vec<_> = files
            .iter()
            .map(|f: &FileContext| &f.relative_path)
            .collect();
        let tree_view = TreeRenderer::build(&tree_paths).render(&root_name);
        xml.create_element("directory_structure")
            .write_text_content(BytesText::new(&tree_view))?;
        xml.write_event(Event::End(BytesEnd::new("metadata")))?;

        // Sección de Archivos
        xml.write_event(Event::Start(BytesStart::new("files")))?;
        for file in files {
            let mut elem = BytesStart::new("file");
            elem.push_attribute(("path", file.relative_path.to_string_lossy().as_ref()));
            elem.push_attribute(("language", file.language.as_str()));
            xml.write_event(Event::Start(elem))?;

            match &file.content {
                ContentType::Text(text) => {
                    let body = if config.minify {
                        ContentMinifier::minify(text, &file.language)
                    } else {
                        text.clone()
                    };
                    xml.write_event(Event::CData(BytesCData::new(&body)))?;
                }
                ContentType::Error(e) => {
                    // INFO: Ahora reportamos el error real en lugar de un genérico [SKIPPED]
                    let error_msg = format!("[ERROR: {}]", e);
                    xml.write_event(Event::CData(BytesCData::new(&error_msg)))?;
                }
                ContentType::Binary => {
                    xml.write_event(Event::CData(BytesCData::new("[BINARY CONTENT SKIPPED]")))?;
                }
            }
            xml.write_event(Event::End(BytesEnd::new("file")))?;
        }

        xml.write_event(Event::End(BytesEnd::new("files")))?;
        xml.write_event(Event::End(BytesEnd::new("context")))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_structure_integrity() {
        let config = ContextConfig::default();
        let mut buf = Vec::new();
        XmlWriter::new().write(&[], &config, &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("<context>"));
    }
}
