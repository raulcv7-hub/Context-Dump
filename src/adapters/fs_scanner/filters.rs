use crate::core::config::ContextConfig;
use std::path::Path;

pub struct PathFilter;

impl PathFilter {
    /// Validates if a path matches the inclusion and exclusion criteria of the config.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::ContextConfig;
    use std::collections::HashSet;

    #[test]
    fn test_filter_exclusion_priority() {
        let mut config = ContextConfig::default();
        config.exclude_paths.push("tests".to_string());
        config.include_paths.push("src".to_string());

        assert!(!PathFilter::matches(Path::new("tests/main.rs"), &config));
        assert!(PathFilter::matches(Path::new("src/main.rs"), &config));
    }

    #[test]
    fn test_filter_extension_whitelist() {
        let mut config = ContextConfig::default();
        let mut set = HashSet::new();
        set.insert("rs".to_string());
        config.include_extensions = set;

        assert!(PathFilter::matches(Path::new("main.rs"), &config));
        assert!(!PathFilter::matches(Path::new("main.py"), &config));
    }
}
