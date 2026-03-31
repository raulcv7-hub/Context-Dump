use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Provenance {
    pub repo_url: String,
    pub commit_hash: String,
}

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
    pub provenance: Option<Provenance>,
    pub max_depth: Option<usize>,
    pub include_hidden: bool,
    pub no_ignore: bool,
    pub max_tokens_per_file: usize,
    pub to_clipboard: bool,
    pub verbose: bool,
    pub smart_ignore: bool,
    pub include_extensions: HashSet<String>,
    pub exclude_extensions: HashSet<String>,
    pub include_paths: Vec<String>,
    pub exclude_paths: Vec<String>,
    pub file_states: HashMap<PathBuf, bool>,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            root_path: PathBuf::from("."),
            output_path: None,
            output_format: OutputFormat::Xml,
            provenance: None,
            max_depth: None,
            include_hidden: false,
            no_ignore: false,
            max_tokens_per_file: 30_000,
            to_clipboard: false,
            verbose: false,
            smart_ignore: true,
            include_extensions: HashSet::new(),
            exclude_extensions: HashSet::new(),
            include_paths: Vec::new(),
            exclude_paths: Vec::new(),
            file_states: HashMap::new(),
        }
    }
}
