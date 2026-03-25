use crate::cli::ContextCli;
use crate::engine::{logging, overrides, processor};
use context::adapters::fs_scanner::FsScanner;
use context::core::config::{persistence::ConfigPersistence, ContextConfig};
use context::ports::scanner::ProjectScanner;
use context::ui::run_tui;

/// High-level orchestrator for the context extraction process.
pub fn execute(args: ContextCli) -> anyhow::Result<()> {
    logging::setup(args.verbose);
    let is_naked = std::env::args().count() == 1;

    let mut config = match ConfigPersistence::load()? {
        Some(mut cfg) => {
            if !is_naked { overrides::apply(&mut cfg, &args); }
            cfg
        }
        None => build_initial_config(&args),
    };

    config.root_path = args.path.clone();
    let scanner = FsScanner::new();
    let all_nodes = scanner.scan(&config)?;

    if is_naked || args.should_run_interactive() {
        if let Some((sel, ncfg)) = run_tui(&all_nodes, &config.root_path, config.clone())? {
            let mut final_nodes = all_nodes;
            final_nodes.retain(|n| sel.contains(&n.relative_path));
            ConfigPersistence::save(&ncfg)?;
            return processor::run(final_nodes, ncfg);
        }
        return Ok(());
    }

    processor::run(filter_nodes(all_nodes, &config), config)
}

/// Filters scanned nodes based on visibility and ignore settings.
fn filter_nodes(nodes: Vec<context::core::file::FileNode>, cfg: &ContextConfig) -> Vec<context::core::file::FileNode> {
    nodes.into_iter().filter(|n| {
        if n.is_ignored && !cfg.no_ignore { return false; }
        if n.is_hidden && !cfg.include_hidden { return false; }
        true
    }).collect()
}

/// Helper to create a fresh configuration from CLI arguments.
fn build_initial_config(args: &ContextCli) -> ContextConfig {
    ContextConfig::build_validated(
        args.path.clone(), args.output.clone(), args.format,
        args.is_format_explicit(), args.depth, args.include_hidden,
        args.no_ignore, args.clip, args.verbose > 0, args.smart_ignore,
        args.extensions.clone(), args.exclude_extensions.clone(),
        args.include_path.clone(), args.exclude_path.clone(),
    )
}