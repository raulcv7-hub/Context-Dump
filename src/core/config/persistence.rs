use crate::core::config::ContextConfig;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct ConfigPersistence;

impl ConfigPersistence {
    /**
     * Resolves the path to the persistent configuration file.
     */
    fn get_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir().context("Could not find config directory")?;
        path.push("context");
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        path.push("last_run.json");
        Ok(path)
    }

    /**
     * Serializes and saves the configuration state.
     */
    pub fn save(config: &ContextConfig) -> Result<()> {
        let path = Self::get_path()?;
        let json = serde_json::to_string_pretty(config)?;
        fs::write(path, json)?;
        Ok(())
    }

    /**
     * Loads and deserializes the last saved configuration.
     */
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