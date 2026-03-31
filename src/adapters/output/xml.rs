use anyhow::Result;
use chrono::Local;
use quick_xml::events::{BytesCData, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::ffi::OsStr;
use std::io::Write;

use crate::core::config::ContextConfig;
use crate::core::content::{ContentType, FileContext};
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
    fn write<W: Write>(
        &self,
        files: &[&FileContext],
        config: &ContextConfig,
        writer: W,
    ) -> Result<()> {
        let mut xml = Writer::new_with_indent(writer, b' ', 4);
        xml.write_event(Event::Start(BytesStart::new("context")))?;

        xml.write_event(Event::Start(BytesStart::new("metadata")))?;
        xml.create_element("project_root")
            .write_text_content(BytesText::new(&config.root_path.to_string_lossy()))?;
        xml.create_element("scan_time")
            .write_text_content(BytesText::new(&Local::now().to_rfc3339()))?;

        if let Some(prov) = &config.provenance {
            xml.write_event(Event::Start(BytesStart::new("provenance")))?;
            xml.create_element("repository")
                .write_text_content(BytesText::new(&prov.repo_url))?;
            xml.create_element("commit_hash")
                .write_text_content(BytesText::new(&prov.commit_hash))?;
            xml.write_event(Event::End(BytesEnd::new("provenance")))?;
        }

        let total_tokens: usize = files.iter().map(|f| f.token_count).sum();
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

        let tree_paths: Vec<_> = files.iter().map(|f| &f.relative_path).collect();
        let tree_view = TreeRenderer::build(&tree_paths).render(&root_name);
        xml.create_element("directory_structure")
            .write_text_content(BytesText::new(&tree_view))?;
        xml.write_event(Event::End(BytesEnd::new("metadata")))?;

        xml.write_event(Event::Start(BytesStart::new("files")))?;
        for file in files {
            let mut elem = BytesStart::new("file");
            elem.push_attribute(("path", file.relative_path.to_string_lossy().as_ref()));
            elem.push_attribute(("language", file.language.as_str()));

            if file.is_suspicious {
                elem.push_attribute(("suspicious", "true"));
            }

            xml.write_event(Event::Start(elem))?;

            match &file.content {
                ContentType::Text(text) => {
                    xml.write_event(Event::CData(BytesCData::new(text)))?;
                }
                ContentType::Error(e) => {
                    let error_msg = format!("[ERROR: {}]", e);
                    xml.write_event(Event::CData(BytesCData::new(&error_msg)))?;
                }
                ContentType::Binary => {
                    xml.write_event(Event::CData(BytesCData::new("[BINARY CONTENT SKIPPED]")))?;
                }
                ContentType::Omitted => {
                    xml.write_event(Event::CData(BytesCData::new(
                        "[OMITTED FOR BREVITY: EXCEEDS TOKEN LIMIT]",
                    )))?;
                }
            }
            xml.write_event(Event::End(BytesEnd::new("file")))?;
        }

        xml.write_event(Event::End(BytesEnd::new("files")))?;
        xml.write_event(Event::End(BytesEnd::new("context")))?;
        Ok(())
    }
}
