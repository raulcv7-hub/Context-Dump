use context::adapters::output::markdown::MarkdownWriter;
use context::adapters::output::xml::XmlWriter;
use context::core::config::{ContextConfig, OutputFormat};
use context::core::content::FileContext;
use context::ports::writer::ContextWriter;
use std::fs::File;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

/// Dispatches the generated context to a single destination based on priority:
/// 1. Clipboard (if enabled)
/// 2. File (if output_path is set)
/// 3. Stdout (default)
pub fn dispatch(files: &[FileContext], config: &ContextConfig) -> anyhow::Result<()> {
    let mut buffer = Vec::new();

    match config.output_format {
        OutputFormat::Xml => XmlWriter::new().write(files, config, &mut buffer)?,
        OutputFormat::Markdown => MarkdownWriter::new().write(files, config, &mut buffer)?,
    }

    if config.to_clipboard {
        let mut clip = arboard::Clipboard::new()?;
        clip.set_text(String::from_utf8(buffer)?)?;
        
        thread::sleep(Duration::from_millis(200));
        return Ok(());
    }

    if let Some(path) = &config.output_path {
        let mut file = File::create(path)?;
        file.write_all(&buffer)?;
    } else {
        io::stdout().write_all(&buffer)?;
        io::stdout().flush()?;
    }

    Ok(())
}