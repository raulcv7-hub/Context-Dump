use anyhow::{Context, Result};
use context::core::config::Provenance;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Manages the lifecycle of a remote repository cloned into a temporary directory.
pub struct RemoteProject {
    /// The self-destructing temporary directory mechanism.
    _dir: TempDir,
    /// The physical path to the cloned contents.
    pub path: PathBuf,
}

impl RemoteProject {
    /// Clones a remote repository and extracts Provenance (Commit Hash & Origin).
    pub fn resolve(target: &str) -> Result<Option<(Self, Provenance)>> {
        let is_remote = target.starts_with("http://")
            || target.starts_with("https://")
            || target.starts_with("git@");

        if !is_remote {
            return Ok(None);
        }

        eprintln!("🌐 Remote repository detected. Cloning shallowly...");
        let temp_dir =
            tempfile::tempdir().context("Failed to create secure temporary directory")?;
        let clone_path = temp_dir.path().to_path_buf();

        let status = Command::new("git")
            .arg("clone")
            .arg("--depth")
            .arg("1")
            .arg(target)
            .arg(&clone_path)
            .status()
            .context("Failed to execute 'git clone'.")?;

        if !status.success() {
            anyhow::bail!("Git clone failed for target: {}", target);
        }

        let output = Command::new("git")
            .arg("rev-parse")
            .arg("HEAD")
            .current_dir(&clone_path)
            .output()?;

        let commit_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let provenance = Provenance {
            repo_url: target.to_string(),
            commit_hash: if commit_hash.is_empty() {
                "UNKNOWN".to_string()
            } else {
                commit_hash
            },
        };

        Ok(Some((
            Self {
                path: clone_path,
                _dir: temp_dir,
            },
            provenance,
        )))
    }
}

impl Drop for RemoteProject {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}
