use context::adapters::parsers::{FileParser, NativeOfficeParser, PlainTextParser, NativePdfParser};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;
use zip::write::SimpleFileOptions;

/// Verifies that the PlainTextParser correctly reads standard UTF-8 files.
#[test]
fn test_plain_text_parsing() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    let content = "Hello, context engine!";
    
    File::create(&file_path).unwrap().write_all(content.as_bytes()).unwrap();
    
    let parser = PlainTextParser::new();
    let result = parser.parse(&file_path).unwrap();
    
    assert_eq!(result, content);
}

/// Verifies the native Word (.docx) parser by creating a minimal valid ZIP structure.
/// Fixes the zip v2.x FileOptions ambiguity by using SimpleFileOptions.
#[test]
fn test_native_docx_parsing_logic() {
    let dir = tempdir().unwrap();
    let docx_path = dir.path().join("test.docx");
    
    let file = File::create(&docx_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    
    let options = SimpleFileOptions::default();
    zip.start_file("word/document.xml", options).unwrap();
    
    let xml_content = r#"
        <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
        <w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
            <w:body>
                <w:p><w:r><w:t>Extracted Text From Docx</w:t></w:r></w:p>
            </w:body>
        </w:document>
    "#;
    zip.write_all(xml_content.as_bytes()).unwrap();
    zip.finish().unwrap();

    let parser = NativeOfficeParser::new();
    let result = parser.parse(&docx_path).unwrap();
    
    assert!(result.contains("Extracted Text From Docx"));
}

/// Verifies that the PDF parser correctly returns an error when the file is not a valid PDF.
#[test]
fn test_pdf_parser_invalid_file() {
    let dir = tempdir().unwrap();
    let fake_pdf = dir.path().join("fake.pdf");
    File::create(&fake_pdf).unwrap().write_all(b"not a pdf content").unwrap();
    
    let parser = NativePdfParser::new();
    let result = parser.parse(&fake_pdf);
    
    assert!(result.is_err());
}

/// Verifies that the Office parser correctly identifies and rejects unsupported formats.
#[test]
fn test_office_parser_unsupported_format() {
    let dir = tempdir().unwrap();
    let invalid_path = dir.path().join("test.png");
    File::create(&invalid_path).unwrap();
    
    let parser = NativeOfficeParser::new();
    let result = parser.parse(&invalid_path);
    
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Unsupported office format"));
}

/// Verifies basic parsings.
#[test]
fn test_text_parser_basic_read() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("main.rs");
    File::create(&path).unwrap().write_all(b"fn main() {}").unwrap();
    
    let parser = PlainTextParser::new();
    let result = parser.parse(&path).unwrap();
    assert_eq!(result, "fn main() {}");
}