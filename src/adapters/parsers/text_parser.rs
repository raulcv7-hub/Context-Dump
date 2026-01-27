use crate::adapters::parsers::FileParser;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct PlainTextParser;

impl PlainTextParser {
    /// Factory for PlainTextParser.
    pub fn new() -> Self {
        Self
    }
}

impl Default for PlainTextParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FileParser for PlainTextParser {
    /// Direct filesystem read for source code or plain text.
    fn parse(&self, path: &Path) -> Result<String> {
        Ok(fs::read_to_string(path)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_parser_default() {
        let _ = PlainTextParser;
    }
}
