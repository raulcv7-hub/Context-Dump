use crate::adapters::parsers::FileParser;
use anyhow::{anyhow, Result};
use calamine::{Data, Reader, Xlsx};
use quick_xml::events::Event;
use quick_xml::reader::Reader as XmlReader;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Native parser for Microsoft Office formats (DOCX, XLSX).
pub struct NativeOfficeParser;

impl NativeOfficeParser {
    /// Creates a new instance of the Office parser.
    pub fn new() -> Self {
        Self
    }

    fn parse_xlsx(&self, path: &Path) -> Result<String> {
        let mut workbook: Xlsx<_> =
            calamine::open_workbook(path).map_err(|e| anyhow!("Excel error: {}", e))?;
        let mut output = String::new();
        for sheet in workbook.sheet_names().to_vec() {
            output.push_str(&format!("\n--- Sheet: {} ---\n", sheet));
            if let Ok(range) = workbook.worksheet_range(&sheet) {
                for row in range.rows() {
                    let row_str: Vec<String> = row
                        .iter()
                        .map(|cell| match cell {
                            Data::String(s) => s.clone(),
                            Data::Float(f) => f.to_string(),
                            Data::Int(i) => i.to_string(),
                            Data::Bool(b) => b.to_string(),
                            _ => "".to_string(),
                        })
                        .collect();
                    output.push_str(&format!("{}\n", row_str.join(" | ")));
                }
            }
        }
        Ok(output)
    }

    fn parse_docx(&self, path: &Path) -> Result<String> {
        let file = File::open(path)?;
        let mut archive = zip::ZipArchive::new(file)?;
        let mut doc_xml = archive.by_name("word/document.xml")?;
        let mut buffer = Vec::new();
        doc_xml.read_to_end(&mut buffer)?;
        let mut reader = XmlReader::from_reader(&buffer[..]);
        let (mut output, mut buf, mut in_text) = (String::new(), Vec::new(), false);

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) if e.name().as_ref() == b"w:t" => in_text = true,
                Ok(Event::End(e)) => {
                    let name = e.name();
                    if name.as_ref() == b"w:t" {
                        in_text = false;
                    }
                    if name.as_ref() == b"w:p" {
                        output.push('\n');
                    }
                }
                Ok(Event::Text(e)) if in_text => {
                    let bytes = e.into_inner();
                    output.push_str(&reader.decoder().decode(&bytes)?);
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(anyhow!("XML error: {}", e)),
                _ => {}
            }
            buf.clear();
        }
        Ok(output)
    }
}

impl Default for NativeOfficeParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FileParser for NativeOfficeParser {
    fn parse(&self, path: &Path) -> Result<String> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        match ext.as_str() {
            "xlsx" | "xls" => self.parse_xlsx(path),
            "docx" => self.parse_docx(path),
            _ => Err(anyhow!("Unsupported office format: {}", ext)),
        }
    }
}
