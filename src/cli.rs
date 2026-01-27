use clap::{ArgAction, Parser};
use context::core::config::OutputFormat;
use std::path::PathBuf;

/// High-performance Context Generator.
/// If no arguments are provided, it defaults to Interactive Mode.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ContextCli {
    /// Path to the project root to scan.
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Optional output file path. Inferred format if extension is present.
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Output format (xml, json, markdown, text).
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Xml)]
    pub format: OutputFormat,

    /// Copy the result to the system clipboard.
    #[arg(short, long, default_value_t = false)]
    pub clip: bool,

    /// Minify output (reduce whitespace) to save tokens.
    #[arg(short = 'm', long, default_value_t = false)]
    pub minify: bool,

    /// Force Interactive mode (TUI). Default if no other flags are set.
    #[arg(short = 'I', long, default_value_t = false)]
    pub interactive: bool,

    /// Force dumping output to terminal (Stdout).
    #[arg(short = 's', long, default_value_t = false)]
    pub stdout: bool,

    /// Use smart heuristics to ignore heavy binary artifacts and noise.
    #[arg(short = 'S', long, default_value_t = true, action = ArgAction::Set)]
    pub smart_ignore: bool,

    /// Maximum depth to traverse.
    #[arg(short, long)]
    pub depth: Option<usize>,

    /// Include hidden files and directories in the analysis.
    #[arg(long, default_value_t = false)]
    pub include_hidden: bool,

    /// Ignore gitignore and .ignore files.
    #[arg(long, default_value_t = false)]
    pub no_ignore: bool,

    /// Filter by extension (comma separated, e.g., rs,py).
    #[arg(short = 'e', long, value_delimiter = ',')]
    pub extensions: Vec<String>,

    /// Exclude extensions (comma separated).
    #[arg(short = 'x', long, value_delimiter = ',')]
    pub exclude_extensions: Vec<String>,

    /// Only include paths containing this string.
    #[arg(short = 'i', long)]
    pub include_path: Vec<String>,

    /// Exclude paths containing this string.
    #[arg(short = 'X', long)]
    pub exclude_path: Vec<String>,

    /// Turn debugging information on (-v, -vv).
    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,
}

impl ContextCli {
    pub fn is_format_explicit(&self) -> bool {
        let args: Vec<String> = std::env::args().collect();
        args.iter().any(|a| a == "-f" || a == "--format")
    }

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
