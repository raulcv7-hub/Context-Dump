use crate::adapters::parsers::FileParser;
use anyhow::{anyhow, Result};
use calamine::{Reader, Xlsx, Data};
use std::path::Path;
use std::fs::File;
use std::io::Read;
use quick_xml::events::Event;
use quick_xml::reader::Reader as XmlReader;

pub struct NativeOfficeParser;

impl NativeOfficeParser {
    /// Factory for the native Office parser.
    pub fn new() -> Self {
        Self
    }

    /// Extracts text from Excel sheets using the calamine engine.
    fn parse_xlsx(&self, path: &Path) -> Result<String> {
        let mut workbook: Xlsx<_> = calamine::open_workbook(path)
            .map_err(|e| anyhow!("Excel error: {}", e))?;
        
        let mut output = String::new();
        let sheet_names = workbook.sheet_names().to_vec();

        for sheet in sheet_names {
            output.push_str(&format!("\n--- Sheet: {} ---\n", sheet));
            if let Ok(range) = workbook.worksheet_range(&sheet) {
                for row in range.rows() {
                    let row_str: Vec<String> = row.iter().map(|cell| {
                        match cell {
                            Data::Empty => "".to_string(),
                            Data::String(s) => s.clone(),
                            Data::Float(f) => f.to_string(),
                            Data::Int(i) => i.to_string(),
                            Data::Bool(b) => b.to_string(),
                            _ => "".to_string(),
                        }
                    }).collect();
                    output.push_str(&row_str.join(" | "));
                    output.push('\n');
                }
            }
        }
        Ok(output)
    }

    /**
     * Extracts text from Word documents by parsing the internal document.xml.
     * Only content inside <w:t> tags is extracted to ensure clean text.
     */
    fn parse_docx(&self, path: &Path) -> Result<String> {
        let file = File::open(path)?;
        let mut archive = zip::ZipArchive::new(file)?;
        
        let mut document_xml = archive.by_name("word/document.xml")
            .map_err(|_| anyhow!("Invalid DOCX: missing word/document.xml"))?;
        
        let mut buffer = Vec::new();
        document_xml.read_to_end(&mut buffer)?;

        let mut reader = XmlReader::from_reader(&buffer[..]);
        let mut output = String::new();
        let mut buf = Vec::new();
        let mut in_text_tag = false;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    if e.name().as_ref() == b"w:t" {
                        in_text_tag = true;
                    }
                }
                Ok(Event::End(e)) => {
                    if e.name().as_ref() == b"w:t" {
                        in_text_tag = false;
                    }
                }
                Ok(Event::Text(e)) => {
                    if in_text_tag {
                        let decoded = reader.decoder().decode(e.as_ref())?;
                        output.push_str(&decoded);
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(anyhow!("XML parse error in DOCX: {}", e)),
                _ => {}
            }
            buf.clear();
        }

        Ok(output)
    }
}

impl FileParser for NativeOfficeParser {
    /// Routes the parsing logic based on the specific office extension.
    fn parse(&self, path: &Path) -> Result<String> {
        let ext = path.extension()
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