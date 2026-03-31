use crate::ui::app::core::App;
use std::collections::BTreeMap;

impl App {
    /// Computes the language distribution of the currently selected files.
    pub fn get_language_distribution(&self) -> Vec<(String, f64)> {
        let mut counts = BTreeMap::new();
        let mut total = 0;
        for node in self.nodes.iter().filter(|n| n.selected && !n.is_dir) {
            let ext = node
                .path
                .extension()
                .and_then(|e: &std::ffi::OsStr| e.to_str())
                .unwrap_or("text")
                .to_lowercase();
            *counts.entry(ext).or_insert(0) += 1;
            total += 1;
        }
        if total == 0 {
            return Vec::new();
        }
        let mut dist: Vec<_> = counts
            .into_iter()
            .map(|(l, c)| (l, (c as f64 / total as f64) * 100.0))
            .collect();
        dist.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        dist
    }

    /// Calculates the sum of token estimates for all selected files.
    pub fn get_total_selected_tokens(&self) -> usize {
        self.nodes
            .iter()
            .filter(|n| n.selected && !n.is_dir)
            .map(|n| n.token_estimate)
            .sum()
    }

    /// Returns the total number of files currently selected.
    pub fn get_selected_count(&self) -> usize {
        self.nodes
            .iter()
            .filter(|n| n.selected && !n.is_dir)
            .count()
    }

    /// Returns the number of files hidden from selection by smart heuristics.
    pub fn get_smart_ignored_count(&self) -> usize {
        self.nodes
            .iter()
            .filter(|n| n.is_ignored && !n.is_dir)
            .count()
    }
}