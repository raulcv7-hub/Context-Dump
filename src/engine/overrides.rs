use crate::cli::ContextCli;
use context::core::config::ContextConfig;

pub fn apply(config: &mut ContextConfig, args: &ContextCli) {
    let raw_args: Vec<String> = std::env::args().collect();

    if has_arg(&raw_args, "-c", "--clip") {
        config.to_clipboard = true;
        config.output_path = None;
    }

    if has_arg(&raw_args, "-s", "--stdout") {
        config.output_path = None;
        config.to_clipboard = false;
    }

    if has_arg(&raw_args, "-S", "--smart-ignore") {
        config.smart_ignore = args.smart_ignore;
    }

    if let Some(out) = &args.output {
        config.output_path = Some(out.clone());
        config.to_clipboard = false;
    }

    if args.is_format_explicit() {
        config.output_format = args.format;
    }

    if has_arg(&raw_args, "", "--include-hidden") {
        config.include_hidden = true;
    }

    if has_arg(&raw_args, "", "--no-ignore") {
        config.no_ignore = true;
    }

    if let Some(d) = args.depth {
        config.max_depth = Some(d);
    }

    if has_arg(&raw_args, "", "--max-tokens") {
        config.max_tokens_per_file = args.max_tokens;
    }

    sync_filters(config, args, &raw_args);
}

/// Checks if a specific flag was explicitly provided in the command line arguments.
fn has_arg(raw: &[String], short: &str, long: &str) -> bool {
    raw.iter()
        .any(|a| (!short.is_empty() && a == short) || a == long)
}

/// Synchronizes inclusion and exclusion filters from CLI while respecting command line presence.
fn sync_filters(config: &mut ContextConfig, args: &ContextCli, raw: &[String]) {
    if has_arg(raw, "-e", "--extensions") {
        config.include_extensions = args.extensions.iter().map(|s| s.to_lowercase()).collect();
    }
    if has_arg(raw, "-x", "--exclude-extensions") {
        config.exclude_extensions = args
            .exclude_extensions
            .iter()
            .map(|s| s.to_lowercase())
            .collect();
    }
    if has_arg(raw, "-i", "--include-path") {
        config.include_paths = args.include_path.clone();
    }
    if has_arg(raw, "-X", "--exclude-path") {
        config.exclude_paths = args.exclude_path.clone();
    }
}
