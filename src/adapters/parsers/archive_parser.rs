use crate::adapters::parsers::FileParser;
use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Native parser for compressed archives (ZIP, TAR.GZ).
pub struct ArchiveParser;

impl ArchiveParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ArchiveParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FileParser for ArchiveParser {
    /// Extracts text from internal files of an archive. Skips binary internal files.
    fn parse(&self, path: &Path) -> Result<String> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        let mut output = String::new();

        if ext == "zip" {
            let file = File::open(path)?;
            let mut archive =
                zip::ZipArchive::new(file).map_err(|e| anyhow!("ZIP error: {}", e))?;

            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                if file.is_dir() {
                    continue;
                }

                let mut buf = Vec::new();
                if file.read_to_end(&mut buf).is_ok() {
                    if let Ok(text) = String::from_utf8(buf) {
                        output.push_str(&format!(
                            "\n--- [ARCHIVE ENTRY: {}] ---\n{}\n",
                            file.name(),
                            text
                        ));
                    }
                }
            }
        } else if ext == "gz" || ext == "tgz" {
            let file = File::open(path)?;
            let tar = flate2::read::GzDecoder::new(file);
            let mut archive = tar::Archive::new(tar);

            for entry in archive.entries()? {
                let mut file = entry?;
                if file.header().entry_type().is_dir() {
                    continue;
                }

                let mut buf = Vec::new();
                if file.read_to_end(&mut buf).is_ok() {
                    if let Ok(text) = String::from_utf8(buf) {
                        let name = file.path()?.to_string_lossy().to_string();
                        output
                            .push_str(&format!("\n--- [ARCHIVE ENTRY: {}] ---\n{}\n", name, text));
                    }
                }
            }
        } else {
            return Err(anyhow!("Unsupported archive format: {}", ext));
        }

        Ok(output)
    }
}
