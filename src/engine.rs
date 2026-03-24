use crate::cli::ContextCli;
use crate::output_dispatcher;
use context::adapters::fs_reader::FsReader;
use context::adapters::fs_scanner::FsScanner;
use context::core::config::{persistence::ConfigPersistence, ContextConfig};
use context::ports::reader::FileReader;
use context::ports::scanner::ProjectScanner;
use context::ui::run_tui;

use indicatif::{ProgressBar, ProgressStyle};
use tracing::{info, warn};

/**
 * Orchestrates the full execution logic and handles UI-to-Engine transitions.
 */
pub fn execute(args: ContextCli) -> anyhow::Result<()> {
    setup_logging(args.verbose);

    let is_naked_call = std::env::args().count() == 1;

    let mut config = if let Some(mut last_config) = ConfigPersistence::load()? {
        if !is_naked_call {
            apply_cli_overrides(&mut last_config, &args);
        }
        last_config
    } else {
        ContextConfig::build_validated(
            args.path.clone(),
            args.output.clone(),
            args.format,
            args.is_format_explicit(),
            args.depth,
            args.include_hidden,
            args.no_ignore,
            args.clip,
            args.verbose > 0,
            args.smart_ignore,
            args.extensions.clone(),
            args.exclude_extensions.clone(),
            args.include_path.clone(),
            args.exclude_path.clone(),
        )
    };

    config.root_path = args.path.clone();

    let scanner = FsScanner::new();
    let all_nodes = scanner.scan(&config)?;

    if is_naked_call || args.should_run_interactive() {
        if let Some((selected, new_cfg)) = run_tui(&all_nodes, &config.root_path, config.clone())? {
            let mut final_nodes = all_nodes;
            final_nodes.retain(|n| selected.contains(&n.relative_path));

            ConfigPersistence::save(&new_cfg)?;
            return run_processing(final_nodes, new_cfg);
        }
        return Ok(());
    }

    let filtered_nodes: Vec<_> = all_nodes
        .into_iter()
        .filter(|n| {
            if n.is_ignored && !config.no_ignore {
                return false;
            }
            if n.is_hidden && !config.include_hidden {
                return false;
            }
            true
        })
        .collect();

    run_processing(filtered_nodes, config)
}

/**
 * Handles file ingestion and triggers the final output with a clean summary.
 */
fn run_processing(
    nodes: Vec<context::core::file::FileNode>,
    config: ContextConfig,
) -> anyhow::Result<()> {
    if nodes.is_empty() {
        warn!("No files to process.");
        return Ok(());
    }

    info!("Processing {} files...", nodes.len());

    let pb = ProgressBar::new(nodes.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")?
            .progress_chars("#>-"),
    );

    let reader = FsReader::new();
    let mut contexts = Vec::with_capacity(nodes.len());

    for node in nodes {
        let file_name = node.path.file_name().unwrap_or_default().to_string_lossy();
        pb.set_message(format!("Ingesting: {}", file_name));

        let ctx = reader.read_file(&node, &config);
        contexts.push(ctx);

        pb.inc(1);
    }

    pb.finish_with_message("Ingestion complete");

    let total_tokens: usize = contexts.iter().map(|c| c.token_count).sum();
    let file_count = contexts.len();

    output_dispatcher::dispatch(&contexts, &config)?;

    println!("\n🚀 --- EXECUTION SUMMARY ---");
    println!("📂 Files processed:  {}", file_count);
    println!("🪙 Total tokens:     {}", total_tokens);
    println!("📄 Format:           {:?}", config.output_format);

    if config.to_clipboard {
        println!("📋 Destination:      SYSTEM CLIPBOARD (File output skipped)");
    } else if let Some(p) = &config.output_path {
        println!("💾 Destination:      FILE ({})", p.display());
    } else {
        println!("📺 Destination:      STANDARD OUTPUT");
    }
    println!("-----------------------------\n");

    Ok(())
}

/**
 * Maps CLI flags to the internal configuration state.
 */
fn apply_cli_overrides(config: &mut ContextConfig, args: &ContextCli) {
    if args.clip {
        config.to_clipboard = true;
    }
    if args.stdout {
        config.output_path = None;
        config.to_clipboard = false;
    }
    if !args.smart_ignore {
        config.smart_ignore = false;
    }
    if let Some(out) = &args.output {
        config.output_path = Some(out.clone());
        config.to_clipboard = false;
    }
    if args.is_format_explicit() {
        config.output_format = args.format;
    }
    if args.include_hidden {
        config.include_hidden = true;
    }
    if args.no_ignore {
        config.no_ignore = true;
    }
    if let Some(depth) = args.depth {
        config.max_depth = Some(depth);
    }

    if !args.extensions.is_empty() {
        config.include_extensions = args.extensions.iter().map(|s| s.to_lowercase()).collect();
    }
    if !args.exclude_extensions.is_empty() {
        config.exclude_extensions = args
            .exclude_extensions
            .iter()
            .map(|s| s.to_lowercase())
            .collect();
    }
    if !args.include_path.is_empty() {
        config.include_paths = args.include_path.clone();
    }
    if !args.exclude_path.is_empty() {
        config.exclude_paths = args.exclude_path.clone();
    }
}

/**
 * Initializes the tracing subscriber for logging.
 */
fn setup_logging(verbosity: u8) {
    use std::io;
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    let level = match verbosity {
        0 => Level::WARN,
        1 => Level::INFO,
        _ => Level::DEBUG,
    };

    let _ = FmtSubscriber::builder()
        .with_max_level(level)
        .with_writer(io::stderr)
        .try_init();
}
