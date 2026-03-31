use crate::core::config::ContextConfig;
use anyhow::{Context, Result};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

pub struct ConfigPersistence;

impl ConfigPersistence {
    /// Hashes the absolute project path to create a unique identifier for state memory.
    fn get_project_hash(path: &Path) -> String {
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        format!("project_{}.json", hasher.finish())
    }

    /// Resolves the path to the persistent configuration file based on the specific project.
    fn get_path(project_path: &Path) -> Result<PathBuf> {
        let mut path = dirs::config_dir().context("Could not find config directory")?;
        path.push("context");
        path.push("projects");
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }

        // Convert to absolute path to ensure uniqueness
        let absolute =
            fs::canonicalize(project_path).unwrap_or_else(|_| project_path.to_path_buf());
        path.push(Self::get_project_hash(&absolute));

        Ok(path)
    }

    /// Serializes and saves the configuration state.
    pub fn save(config: &ContextConfig) -> Result<()> {
        let path = Self::get_path(&config.root_path)?;
        let json = serde_json::to_string_pretty(config)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Loads and deserializes the last saved configuration for the specific project.
    pub fn load(project_path: &Path) -> Result<Option<ContextConfig>> {
        let path = Self::get_path(project_path)?;
        if !path.exists() {
            return Ok(None);
        }
        let data = fs::read_to_string(path)?;
        let config: ContextConfig = serde_json::from_str(&data)?;
        Ok(Some(config))
    }
}
