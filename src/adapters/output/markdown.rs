use anyhow::Result;
use chrono::Local;
use std::ffi::OsStr;
use std::io::Write;

use crate::core::config::ContextConfig;
use crate::core::content::{ContentType, FileContext};
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
    fn write<W: Write>(
        &self,
        files: &[&FileContext],
        config: &ContextConfig,
        mut writer: W,
    ) -> Result<()> {
        writeln!(writer, "# Project Context Report")?;
        writeln!(
            writer,
            "> Generated on {}\n",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        )?;

        if let Some(prov) = &config.provenance {
            writeln!(writer, "## Provenance")?;
            writeln!(writer, "- **Source:** `{}`", prov.repo_url)?;
            writeln!(writer, "- **Commit:** `{}`\n", prov.commit_hash)?;
        }

        let total_tokens: usize = files.iter().map(|f| f.token_count).sum();
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

        let tree_paths: Vec<_> = files.iter().map(|f| &f.relative_path).collect();
        let tree_view = TreeRenderer::build(&tree_paths).render(&root_name);
        writeln!(writer, "## Structure\n```text\n{}```\n", tree_view)?;

        for file in files {
            writeln!(writer, "### `{}`", file.relative_path.display())?;

            if file.is_suspicious {
                writeln!(writer, "> [!WARNING]\n> Suspicious code detected (potential minification/obfuscation).\n")?;
            }

            match &file.content {
                ContentType::Text(text) => {
                    writeln!(writer, "```{}\n{}```\n", file.language, text)?;
                }
                ContentType::Error(err_msg) => {
                    writeln!(
                        writer,
                        "\n> [!CAUTION]\n> **Processing Error:** {}\n",
                        err_msg
                    )?;
                }
                ContentType::Binary => {
                    writeln!(writer, "\n> [!NOTE]\n> **Binary Content:** Omitted\n")?;
                }
                ContentType::Omitted => {
                    writeln!(
                        writer,
                        "\n> [!WARNING]\n> **[OMITTED FOR BREVITY: EXCEEDS TOKEN LIMIT]**\n"
                    )?;
                }
            }
        }
        Ok(())
    }
}
