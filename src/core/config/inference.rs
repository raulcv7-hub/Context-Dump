use crate::core::config::models::{ContextConfig, OutputFormat};
use std::collections::HashSet;
use std::path::Path;

impl ContextConfig {
    /**
     * Builds a validated configuration by merging user inputs and inferring formats.
     */
    #[allow(clippy::too_many_arguments)]
    pub fn build_validated(
        root_path: std::path::PathBuf,
        output_path: Option<std::path::PathBuf>,
        format: OutputFormat,
        is_format_explicit: bool,
        max_depth: Option<usize>,
        include_hidden: bool,
        no_ignore: bool,
        to_clipboard: bool,
        verbose: bool,
        smart_ignore: bool,
        extensions: Vec<String>,
        exclude_extensions: Vec<String>,
        include_path: Vec<String>,
        exclude_path: Vec<String>,
    ) -> Self {
        let mut final_format = format;

        if !is_format_explicit {
            if let Some(ref path) = output_path {
                final_format = infer_from_path(path).unwrap_or(format);
            }
        }

        Self {
            root_path,
            output_path,
            output_format: final_format,
            max_depth,
            include_hidden,
            no_ignore,
            to_clipboard,
            verbose,
            smart_ignore,
            include_extensions: extensions
                .iter()
                .map(|s: &String| s.to_lowercase())
                .collect::<HashSet<_>>(),
            exclude_extensions: exclude_extensions
                .iter()
                .map(|s: &String| s.to_lowercase())
                .collect::<HashSet<_>>(),
            include_paths: include_path,
            exclude_paths: exclude_path,
        }
    }
}

/**
 * Maps file extensions to supported output formats.
 */
fn infer_from_path<P: AsRef<Path>>(path: P) -> Option<OutputFormat> {
    path.as_ref()
        .extension()?
        .to_str()
        .map(|ext: &str| match ext.to_lowercase().as_str() {
            "xml" => OutputFormat::Xml,
            "md" | "markdown" => OutputFormat::Markdown,
            _ => OutputFormat::Xml,
        })
}