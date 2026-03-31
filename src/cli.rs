use clap::{ArgAction, Parser};
use context::core::config::OutputFormat;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ContextCli {
    #[arg(default_value = ".")]
    pub path: String,

    #[arg(short, long)]
    pub output: Option<PathBuf>,

    #[arg(short, long, value_enum, default_value_t = OutputFormat::Xml)]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = false)]
    pub clip: bool,

    #[arg(short = 'I', long, default_value_t = false)]
    pub interactive: bool,

    #[arg(short = 's', long, default_value_t = false)]
    pub stdout: bool,

    #[arg(short = 'S', long, default_value_t = true, action = ArgAction::Set)]
    pub smart_ignore: bool,

    #[arg(short, long)]
    pub depth: Option<usize>,

    #[arg(long, default_value_t = false)]
    pub include_hidden: bool,

    #[arg(long, default_value_t = false)]
    pub no_ignore: bool,

    #[arg(long, default_value_t = 50000)]
    pub max_tokens: usize,

    #[arg(short = 'e', long, value_delimiter = ',')]
    pub extensions: Vec<String>,

    #[arg(short = 'x', long, value_delimiter = ',')]
    pub exclude_extensions: Vec<String>,

    #[arg(short = 'i', long)]
    pub include_path: Vec<String>,

    #[arg(short = 'X', long)]
    pub exclude_path: Vec<String>,

    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,
}

impl ContextCli {
    /// Determines if the output format was explicitly requested.
    pub fn is_format_explicit(&self) -> bool {
        let args: Vec<String> = std::env::args().collect();
        args.iter().any(|a| a == "-f" || a == "--format")
    }

    /// Determines if the TUI should be launched based on current arguments.
    pub fn should_run_interactive(&self) -> bool {
        if self.interactive {
            return true;
        }
        if self.stdout || self.clip || self.output.is_some() {
            return false;
        }

        let has_filters = !self.extensions.is_empty()
            || !self.exclude_extensions.is_empty()
            || !self.include_path.is_empty()
            || !self.exclude_path.is_empty()
            || self.depth.is_some();

        !has_filters
    }
}
