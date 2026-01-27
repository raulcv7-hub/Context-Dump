use crate::core::config::OutputFormat;
use crate::ui::app::core::App;
use std::path::PathBuf;

impl App {
    /// Activa o desactiva la copia al portapapeles del sistema.
    pub fn toggle_clipboard(&mut self) {
        self.config.to_clipboard = !self.config.to_clipboard;
    }

    /// Activa o desactiva la minificación de espacios en blanco.
    pub fn toggle_minify(&mut self) {
        self.config.minify = !self.config.minify;
    }

    /// Alterna el destino de salida entre la terminal y un archivo físico.
    pub fn toggle_output_destination(&mut self) {
        if self.config.output_path.is_some() {
            self.config.output_path = None;
        } else {
            let format = self.config.output_format;
            let ext = self.format_to_ext(format);
            let base_name = self
                .config
                .root_path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "context_report".to_string());

            self.config.output_path = Some(PathBuf::from(format!("{}.{}", base_name, ext)));
        }
    }

    /// Cicla entre los formatos disponibles y actualiza la extensión del archivo si existe.
    pub fn cycle_format(&mut self) {
        let new_format = match self.config.output_format {
            OutputFormat::Xml => OutputFormat::Markdown,
            OutputFormat::Markdown => OutputFormat::Json,
            OutputFormat::Json => OutputFormat::Text,
            OutputFormat::Text => OutputFormat::Xml,
        };

        self.config.output_format = new_format;

        let ext = self.format_to_ext(new_format);
        if let Some(path) = self.config.output_path.as_mut() {
            path.set_extension(ext);
        }
    }

    /// Mapeo puro de formato a extensión de archivo.
    fn format_to_ext(&self, format: OutputFormat) -> &'static str {
        match format {
            OutputFormat::Xml => "xml",
            OutputFormat::Markdown => "md",
            OutputFormat::Json => "json",
            OutputFormat::Text => "txt",
        }
    }
}
