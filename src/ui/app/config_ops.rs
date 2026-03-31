use crate::core::config::OutputFormat;
use crate::ui::app::core::App;
use std::path::PathBuf;

impl App {
    /// Cycles through output destinations: Terminal -> File -> Clipboard.
    pub fn cycle_output_destination(&mut self) {
        if !self.config.to_clipboard && self.config.output_path.is_none() {
            self.set_output_to_file();
        } else if self.config.output_path.is_some() {
            self.config.output_path = None;
            self.config.to_clipboard = true;
        } else {
            self.config.to_clipboard = false;
        }
    }

    /// Sets the output to a file based on the current format and project name.
    fn set_output_to_file(&mut self) {
        let format = self.config.output_format;
        let ext = self.format_to_ext(format);
        let base_name = self
            .config
            .root_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "context_report".to_string());

        self.config.output_path = Some(PathBuf::from(format!("{}.{}", base_name, ext)));
        self.config.to_clipboard = false;
    }

    /// Alternates between XML and Markdown formats.
    pub fn cycle_format(&mut self) {
        let new_format = match self.config.output_format {
            OutputFormat::Xml => OutputFormat::Markdown,
            OutputFormat::Markdown => OutputFormat::Xml,
        };

        self.config.output_format = new_format;
        let ext = self.format_to_ext(new_format);

        if let Some(path) = self.config.output_path.as_mut() {
            path.set_extension(ext);
        }
    }

    /// Returns the file extension for the current format.
    fn format_to_ext(&self, format: OutputFormat) -> &'static str {
        match format {
            OutputFormat::Xml => "xml",
            OutputFormat::Markdown => "md",
        }
    }
}
