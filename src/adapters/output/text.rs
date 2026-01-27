use anyhow::Result;
use chrono::Local;
use std::ffi::OsStr;
use std::io::Write;

use crate::core::config::ContextConfig;
use crate::core::content::{ContentMinifier, ContentType, FileContext};
use crate::core::tree::TreeRenderer;
use crate::ports::writer::ContextWriter;

#[derive(Default)]
pub struct TextWriter;

impl TextWriter {
    pub fn new() -> Self {
        Self
    }
}

impl ContextWriter for TextWriter {
    fn write<W: Write>(
        &self,
        files: &[FileContext],
        config: &ContextConfig,
        mut writer: W,
    ) -> Result<()> {
        let separator = "=".repeat(80);
        let sub_separator = "-".repeat(80);

        writeln!(
            writer,
            "{}\nPROJECT CONTEXT REPORT\n{}",
            separator, separator
        )?;
        writeln!(
            writer,
            "Generated Date: {}",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        )?;
        writeln!(writer, "Project Root:   {}", config.root_path.display())?;

        let total_tokens: usize = files.iter().map(|f: &FileContext| f.token_count).sum();
        writeln!(writer, "Total Files:    {}", files.len())?;
        writeln!(writer, "Total Tokens:   {} (Estimated)\n", total_tokens)?;

        writeln!(writer, "DIRECTORY STRUCTURE\n{}", sub_separator)?;
        let root_name = config
            .root_path
            .file_name()
            .and_then(|n: &OsStr| n.to_str())
            .unwrap_or(".");

        let tree_paths: Vec<_> = files
            .iter()
            .map(|f: &FileContext| &f.relative_path)
            .collect();
        let tree_view = TreeRenderer::build(&tree_paths).render(root_name);
        writeln!(writer, "{}\n", tree_view.trim_end())?;

        writeln!(writer, "FILE CONTENTS\n{}", separator)?;
        for file in files {
            writeln!(
                writer,
                "\nFILE: {}\nLANGUAGE: {}\n{}",
                file.relative_path.display(),
                file.language,
                sub_separator
            )?;

            match &file.content {
                ContentType::Text(text) => {
                    let body = if config.minify {
                        ContentMinifier::minify(text, &file.language)
                    } else {
                        text.clone()
                    };
                    writeln!(writer, "{}", body)?;
                }
                ContentType::Binary => writeln!(writer, "[BINARY CONTENT SKIPPED]")?,
                ContentType::Error(e) => writeln!(writer, "[ERROR: {}]", e)?,
            }
            writeln!(writer, "{}", sub_separator)?;
        }
        writeln!(writer, "END OF REPORT")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_text_writer_output() {
        let config = ContextConfig::default();
        let files = vec![FileContext::new(
            PathBuf::from("test.rs"),
            PathBuf::from("test.rs"),
            ContentType::Text("fn main(){}".into()),
            "rust".into(),
            5,
        )];
        let mut buf = Vec::new();
        TextWriter::new().write(&files, &config, &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("PROJECT CONTEXT REPORT"));
        assert!(output.contains("FILE: test.rs"));
    }
}
