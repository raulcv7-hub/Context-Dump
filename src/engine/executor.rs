use crate::cli::ContextCli;
use crate::engine::{logging, overrides, processor, remote::RemoteProject};
use context::adapters::fs_scanner::FsScanner;
use context::core::config::{persistence::ConfigPersistence, ContextConfig, Provenance};
use context::ports::scanner::ProjectScanner;
use context::ui::run_tui;
use std::path::PathBuf;

/// High-level orchestrator for the context extraction process.
pub fn execute(args: ContextCli) -> anyhow::Result<()> {
    logging::setup(args.verbose);
    let is_naked = std::env::args().count() == 1;

    let remote_resolution = RemoteProject::resolve(&args.path)?;

    let active_path = match &remote_resolution {
        Some((remote, _)) => remote.path.clone(),
        None => PathBuf::from(&args.path),
    };

    let mut config = match ConfigPersistence::load(&active_path)? {
        Some(mut cfg) => {
            if !is_naked {
                overrides::apply(&mut cfg, &args);
            }
            cfg
        }
        None => build_initial_config(&args, active_path.clone()),
    };

    config.root_path = active_path.clone();

    if let Some((_, provenance)) = remote_resolution {
        let p: Provenance = provenance;
        config.provenance = Some(p);
    }

    let scanner = FsScanner::new();
    let all_nodes = scanner.scan(&config)?;

    if is_naked || args.should_run_interactive() {
        if let Some((sel, ncfg)) = run_tui(&all_nodes, &config.root_path, config.clone())? {
            let mut final_nodes = all_nodes;
            final_nodes.retain(|n| sel.contains(&n.relative_path) && !n.is_dir && !n.is_sensitive);
            ConfigPersistence::save(&ncfg)?;
            return processor::run(final_nodes, ncfg);
        }
        return Ok(());
    }

    processor::run(filter_nodes(all_nodes, &config), config)
}

/// Final filtering stage for node selection in headless mode.
fn filter_nodes(
    nodes: Vec<context::core::file::FileNode>,
    cfg: &ContextConfig,
) -> Vec<context::core::file::FileNode> {
    nodes
        .into_iter()
        .filter(|n| {
            if n.is_dir {
                return false;
            }
            if n.is_sensitive {
                return false;
            }
            if n.is_ignored && !cfg.no_ignore {
                return false;
            }
            if n.is_hidden && !cfg.include_hidden {
                return false;
            }
            true
        })
        .collect()
}

/// Constructs the initial configuration object from CLI inputs.
fn build_initial_config(args: &ContextCli, resolved_path: PathBuf) -> ContextConfig {
    ContextConfig::build_validated(
        resolved_path,
        args.output.clone(),
        args.format,
        args.is_format_explicit(),
        args.depth,
        args.include_hidden,
        args.no_ignore,
        args.max_tokens,
        args.clip,
        args.verbose > 0,
        args.smart_ignore,
        args.extensions.clone(),
        args.exclude_extensions.clone(),
        args.include_path.clone(),
        args.exclude_path.clone(),
    )
}