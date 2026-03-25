use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
pub enum OutputFormat {
    Xml,
    Markdown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextConfig {
    pub root_path: PathBuf,
    pub output_path: Option<PathBuf>,
    pub output_format: OutputFormat,
    pub max_depth: Option<usize>,
    pub include_hidden: bool,
    pub no_ignore: bool,
    pub to_clipboard: bool,
    pub verbose: bool,
    pub smart_ignore: bool,
    pub include_extensions: HashSet<String>,
    pub exclude_extensions: HashSet<String>,
    pub include_paths: Vec<String>,
    pub exclude_paths: Vec<String>,
}

impl Default for ContextConfig {
    /// Sets the default configuration with XML as the primary output format.
    fn default() -> Self {
        Self {
            root_path: PathBuf::from("."),
            output_path: None,
            output_format: OutputFormat::Xml,
            max_depth: None,
            include_hidden: false,
            no_ignore: false,
            to_clipboard: false,
            verbose: false,
            smart_ignore: true,
            include_extensions: HashSet::new(),
            exclude_extensions: HashSet::new(),
            include_paths: Vec::new(),
            exclude_paths: Vec::new(),
        }
    }
}