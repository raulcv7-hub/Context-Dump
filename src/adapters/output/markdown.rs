use anyhow::Result;
use chrono::Local;
use std::ffi::OsStr;
use std::io::Write;

use crate::core::config::ContextConfig;
use crate::core::content::{ContentMinifier, ContentType, FileContext};
use crate::core::tree::TreeRenderer;
use crate::ports::writer::ContextWriter;

#[derive(Default)]
pub struct MarkdownWriter;

impl MarkdownWriter {
    pub fn new() -> Self {
        Self
    }
}

impl ContextWriter for MarkdownWriter {
    /// Genera un documento Markdown estructurado.
    /// Diferencia visualmente entre código fuente, errores de ingesta y archivos binarios.
    fn write<W: Write>(
        &self,
        files: &[FileContext],
        config: &ContextConfig,
        mut writer: W,
    ) -> Result<()> {
        writeln!(writer, "# Project Context Report")?;
        writeln!(
            writer,
            "> Generated on {}\n",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        )?;

        let total_tokens: usize = files.iter().map(|f: &FileContext| f.token_count).sum();
        let root_name = config
            .root_path
            .file_name()
            .map(|n: &OsStr| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| ".".to_string());

        writeln!(writer, "## Metadata")?;
        writeln!(
            writer,
            "- **Files:** {}\n- **Tokens:** {}\n",
            files.len(),
            total_tokens
        )?;

        let tree_paths: Vec<_> = files
            .iter()
            .map(|f: &FileContext| &f.relative_path)
            .collect();
        let tree_view = TreeRenderer::build(&tree_paths).render(&root_name);
        writeln!(writer, "## Structure\n```text\n{}```\n", tree_view)?;

        for file in files {
            writeln!(writer, "### `{}`", file.relative_path.display())?;
            match &file.content {
                ContentType::Text(text) => {
                    let out = if config.minify {
                        ContentMinifier::minify(text, &file.language)
                    } else {
                        text.clone()
                    };
                    writeln!(writer, "```{}\n{}```\n", file.language, out)?;
                }
                ContentType::Error(err_msg) => {
                    // Nota: El formato debe coincidir exactamente con el test unitario
                    writeln!(
                        writer,
                        "\n> [!CAUTION]\n> **Processing Error:** {}\n",
                        err_msg
                    )?;
                }
                ContentType::Binary => {
                    writeln!(
                        writer,
                        "\n> [!NOTE]\n> **Binary Content:** Omitted (Weight optimization)\n"
                    )?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::content::ContentType;
    use std::path::PathBuf;

    #[test]
    fn test_markdown_rendering_diagnostics() {
        let config = ContextConfig::default();
        let mut buf = Vec::new();
        let files = vec![FileContext::new(
            PathBuf::from("large_book.pdf"),
            PathBuf::from("large_book.pdf"),
            ContentType::Error("Subprocess Timeout".into()),
            "pdf".into(),
            0,
        )];

        MarkdownWriter::new()
            .write(&files, &config, &mut buf)
            .unwrap();
        let output = String::from_utf8(buf).unwrap();

        // CORRECCIÓN: Se añaden los asteriscos para coincidir con el formato del writer
        assert!(output.contains("**Processing Error:** Subprocess Timeout"));
        assert!(output.contains("[!CAUTION]"));
    }
}
