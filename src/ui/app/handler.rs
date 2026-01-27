use crate::ui::app::core::App;
use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

impl App {
    /// Calculates the language distribution only for currently selected files.
    pub fn get_language_distribution(&self) -> Vec<(String, f64)> {
        let mut counts = BTreeMap::new();
        let mut total = 0;
        for node in self.nodes.iter().filter(|n| n.selected && !n.is_dir) {
            let ext = node
                .path
                .extension()
                .and_then(|e| e.to_str())
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

    pub fn get_total_selected_tokens(&self) -> usize {
        self.nodes
            .iter()
            .filter(|n| n.selected && !n.is_dir)
            .map(|n| n.token_estimate)
            .sum()
    }

    pub fn get_selected_count(&self) -> usize {
        self.nodes
            .iter()
            .filter(|n| n.selected && !n.is_dir)
            .count()
    }

    /// Returns how many files were auto-ignored by smart heuristics.
    pub fn get_smart_ignored_count(&self) -> usize {
        self.nodes
            .iter()
            .filter(|n| n.is_ignored && !n.is_dir)
            .count()
    }

    pub fn move_up(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) if i > 0 => i - 1,
            _ => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn move_down(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) if i < self.view_items.len() - 1 => i + 1,
            Some(i) => i,
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn toggle_selection(&mut self) {
        if let Some(selected_idx) = self.list_state.selected() {
            if let Some(&node_idx) = self.view_items.get(selected_idx) {
                let new_state = !self.nodes[node_idx].selected;
                self.set_recursive_selection(node_idx, new_state);
            }
        }
    }

    fn set_recursive_selection(&mut self, idx: usize, state: bool) {
        self.nodes[idx].selected = state;
        let children = self.nodes[idx].children.clone();
        for child_idx in children {
            self.set_recursive_selection(child_idx, state);
        }
    }

    pub fn toggle_expand(&mut self) {
        if let Some(selected_idx) = self.list_state.selected() {
            if let Some(&node_idx) = self.view_items.get(selected_idx) {
                if self.nodes[node_idx].is_dir {
                    self.nodes[node_idx].expanded = !self.nodes[node_idx].expanded;
                    self.update_view();
                }
            }
        }
    }

    pub fn get_selected_paths(&self) -> HashSet<PathBuf> {
        self.nodes
            .iter()
            .filter(|n| n.selected && !n.is_dir)
            .map(|n| n.path.clone())
            .collect()
    }

    /// Advanced view update that applies the search filter to the node tree.
    pub fn update_view(&mut self) {
        self.view_items.clear();
        let roots = self.root_indices.clone();
        for root_idx in roots {
            self.collect_filtered(root_idx);
        }
        if self.list_state.selected().is_none() && !self.view_items.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    fn collect_filtered(&mut self, idx: usize) {
        let matches = self.search_query.is_empty()
            || self.nodes[idx]
                .name
                .to_lowercase()
                .contains(&self.search_query.to_lowercase());

        let has_matching_child = self.has_matching_child(idx);

        if matches || has_matching_child {
            self.view_items.push(idx);
            let expanded = self.nodes[idx].expanded || !self.search_query.is_empty();
            if self.nodes[idx].is_dir && expanded {
                let children = self.nodes[idx].children.clone();
                for child_idx in children {
                    self.collect_filtered(child_idx);
                }
            }
        }
    }

    fn has_matching_child(&self, idx: usize) -> bool {
        for &child_idx in &self.nodes[idx].children {
            if self.nodes[child_idx]
                .name
                .to_lowercase()
                .contains(&self.search_query.to_lowercase())
                || self.has_matching_child(child_idx)
            {
                return true;
            }
        }
        false
    }
}
