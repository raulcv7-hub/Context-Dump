use crate::core::config::ContextConfig;
use std::path::Path;

/// Engine for validating files against inclusion and exclusion rules.
pub struct PathFilter;

impl PathFilter {
    /// Validates if a path matches the criteria defined in the configuration.
    /// Performs checks against path strings and file extensions.
    pub fn matches(path: &Path, config: &ContextConfig) -> bool {
        let path_str = path.to_string_lossy();

        if config.exclude_paths.iter().any(|ex| path_str.contains(ex)) {
            return false;
        }

        if !config.include_paths.is_empty()
            && !config
                .include_paths
                .iter()
                .any(|inc| path_str.contains(inc))
        {
            return false;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        if !config.exclude_extensions.is_empty() && config.exclude_extensions.contains(&ext) {
            return false;
        }

        if !config.include_extensions.is_empty() && !config.include_extensions.contains(&ext) {
            return false;
        }

        true
    }
}
