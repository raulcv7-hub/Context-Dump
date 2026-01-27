use crate::core::config::ContextConfig;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct ConfigPersistence;

impl ConfigPersistence {
    /// Returns the path to the persistence file in the user's config directory.
    fn get_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir().context("Could not find config directory")?;
        path.push("context");
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        path.push("last_run.json");
        Ok(path)
    }

    /// Saves the provided configuration to disk.
    pub fn save(config: &ContextConfig) -> Result<()> {
        let path = Self::get_path()?;
        let json = serde_json::to_string_pretty(config)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Loads the last saved configuration from disk.
    pub fn load() -> Result<Option<ContextConfig>> {
        let path = Self::get_path()?;
        if !path.exists() {
            return Ok(None);
        }
        let data = fs::read_to_string(path)?;
        let config: ContextConfig = serde_json::from_str(&data)?;
        Ok(Some(config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization_cycle() {
        let config = ContextConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let decoded: ContextConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.output_format, decoded.output_format);
    }
}
