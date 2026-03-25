use crate::cli::ContextCli;
use context::core::config::ContextConfig;

/// Overrides a base configuration with explicit CLI flags.
pub fn apply(config: &mut ContextConfig, args: &ContextCli) {
    if args.clip { config.to_clipboard = true; }
    if args.stdout {
        config.output_path = None;
        config.to_clipboard = false;
    }
    if !args.smart_ignore { config.smart_ignore = false; }
    if let Some(out) = &args.output {
        config.output_path = Some(out.clone());
        config.to_clipboard = false;
    }
    if args.is_format_explicit() { config.output_format = args.format; }
    if args.include_hidden { config.include_hidden = true; }
    if args.no_ignore { config.no_ignore = true; }
    if let Some(d) = args.depth { config.max_depth = Some(d); }

    sync_filters(config, args);
}

/// Synchronizes inclusion/exclusion filters from CLI.
fn sync_filters(config: &mut ContextConfig, args: &ContextCli) {
    if !args.extensions.is_empty() {
        config.include_extensions = args.extensions.iter().map(|s| s.to_lowercase()).collect();
    }
    if !args.exclude_extensions.is_empty() {
        config.exclude_extensions = args.exclude_extensions.iter().map(|s| s.to_lowercase()).collect();
    }
    if !args.include_path.is_empty() { config.include_paths = args.include_path.clone(); }
    if !args.exclude_path.is_empty() { config.exclude_paths = args.exclude_path.clone(); }
}