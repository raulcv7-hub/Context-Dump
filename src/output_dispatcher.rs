use context::adapters::output::json::JsonWriter;
use context::adapters::output::markdown::MarkdownWriter;
use context::adapters::output::text::TextWriter;
use context::adapters::output::xml::XmlWriter;
use context::core::config::{ContextConfig, OutputFormat};
use context::core::content::FileContext;
use context::ports::writer::ContextWriter;
use std::fs::File;
use std::io::{self, Write};

/// Delivers the generated context to the configured destination.
pub fn dispatch(files: &[FileContext], config: &ContextConfig) -> anyhow::Result<()> {
    let mut buffer = Vec::new();

    match config.output_format {
        OutputFormat::Xml => XmlWriter::new().write(files, config, &mut buffer)?,
        OutputFormat::Markdown => MarkdownWriter::new().write(files, config, &mut buffer)?,
        OutputFormat::Json => JsonWriter::new().write(files, config, &mut buffer)?,
        OutputFormat::Text => TextWriter::new().write(files, config, &mut buffer)?,
    }

    if config.to_clipboard {
        let mut clip = arboard::Clipboard::new()?;
        clip.set_text(String::from_utf8(buffer.clone())?)?;
    }

    if let Some(path) = &config.output_path {
        let mut file = File::create(path)?;
        file.write_all(&buffer)?;
    } else if !config.to_clipboard {
        io::stdout().write_all(&buffer)?;
        io::stdout().flush()?;
    }

    Ok(())
}
