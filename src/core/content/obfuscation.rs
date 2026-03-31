/// Analyzes text structures to determine if code is obfuscated or minified.
pub struct CodeAnalyzer;

impl CodeAnalyzer {
    /// Returns true if the content exhibits characteristics of minification or obfuscation.
    pub fn is_suspicious(text: &str) -> bool {
        let lines: Vec<&str> = text.lines().collect();
        let line_count = lines.len();

        if line_count == 0 {
            return false;
        }
        if text.len() < 500 {
            return false;
        } // Too short to accurately guess

        let max_line_length = lines.iter().map(|l| l.len()).max().unwrap_or(0);

        if max_line_length > 2000 {
            return true;
        }

        let avg_line_length = text.len() / line_count;
        if avg_line_length > 300 {
            return true;
        }

        false
    }
}
