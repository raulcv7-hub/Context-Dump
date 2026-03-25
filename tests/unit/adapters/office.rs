use context::adapters::parsers::{FileParser, NativeOfficeParser};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;
use zip::write::SimpleFileOptions;

/// Verifies the native Word parser with a structure containing paragraph breaks.
#[test]
fn test_native_docx_paragraph_spacing() {
    let dir = tempdir().unwrap();
    let docx_path = dir.path().join("para.docx");
    let file = File::create(&docx_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    
    zip.start_file("word/document.xml", SimpleFileOptions::default()).unwrap();
    let xml = r#"
        <w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
            <w:body>
                <w:p><w:r><w:t>Hello</w:t></w:r></w:p>
                <w:p><w:r><w:t>World</w:t></w:r></w:p>
            </w:body>
        </w:document>
    "#;
    zip.write_all(xml.as_bytes()).unwrap();
    zip.finish().unwrap();

    let parser = NativeOfficeParser::new();
    let result = parser.parse(&docx_path).unwrap();
    
    assert!(result.contains("Hello\nWorld"));
}

/// Verifies that multiple text runs within the same paragraph are concatenated without spaces.
#[test]
fn test_native_docx_inline_runs() {
    let dir = tempdir().unwrap();
    let docx_path = dir.path().join("runs.docx");
    let file = File::create(&docx_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    
    zip.start_file("word/document.xml", SimpleFileOptions::default()).unwrap();
    let xml = r#"
        <w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
            <w:body>
                <w:p>
                    <w:r><w:t>He</w:t></w:r>
                    <w:r><w:t>llo</w:t></w:r>
                </w:p>
            </w:body>
        </w:document>
    "#;
    zip.write_all(xml.as_bytes()).unwrap();
    zip.finish().unwrap();

    let parser = NativeOfficeParser::new();
    let result = parser.parse(&docx_path).unwrap();
    
    assert!(result.contains("Hello"));
}