use std::fs::{self, File};
use std::io::{Result, Write};
use tempfile::TempDir;

/// Shared test utilities.
/// Logic is marked as allowed dead_code because not all test crates use every helper.
#[allow(dead_code)]
pub fn create_mock_project() -> Result<TempDir> {
    let dir = tempfile::tempdir()?;
    let root = dir.path();

    File::create(root.join("main.rs"))?.write_all(b"fn main() {}")?;
    fs::create_dir(root.join("src"))?;
    File::create(root.join("src/lib.rs"))?.write_all(b"pub fn test() {}")?;
    File::create(root.join("README.md"))?.write_all(b"# Test Project")?;
    
    Ok(dir)
}