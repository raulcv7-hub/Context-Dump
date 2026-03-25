use crate::adapters::parsers::FileParser;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::path::Path;
use std::fs;

#[derive(Deserialize)]
struct Notebook {
    cells: Vec<Cell>,
}

#[derive(Deserialize)]
struct Cell {
    cell_type: String,
    source: serde_json::Value,
}

/// Native parser for Jupyter Notebooks (.ipynb).
pub struct NotebookParser;

impl NotebookParser {
    /// Creates a new NotebookParser instance.
    pub fn new() -> Self {
        Self
    }

    /// Extracts the source content from a notebook cell, handling both string and array formats.
    fn extract_source(&self, source: &serde_json::Value) -> String {
        if let Some(arr) = source.as_array() {
            arr.iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join("")
        } else {
            source.as_str().unwrap_or("").to_string()
        }
    }
}

impl FileParser for NotebookParser {
    /// Parses the JSON structure of a .ipynb file and aggregates markdown and code cells.
    fn parse(&self, path: &Path) -> Result<String> {
        let content = fs::read_to_string(path)?;
        let nb: Notebook = serde_json::from_str(&content)
            .map_err(|e| anyhow!("Invalid Notebook JSON: {}", e))?;

        let mut output = String::new();
        for (i, cell) in nb.cells.iter().enumerate() {
            let source = self.extract_source(&cell.source);
            output.push_str(&format!("\n# --- Cell [{}] ({}) ---\n", i, cell.cell_type));
            output.push_str(&source);
            output.push('\n');
        }

        Ok(output)
    }
}

impl Default for NotebookParser {
    /// Provides a default instance of the NotebookParser.
    fn default() -> Self {
        Self::new()
    }
}