use crate::ui::app::core::App;
use std::collections::HashSet;
use std::path::PathBuf;

impl App {
    /// Moves the selection cursor one position up in the visible list.
    pub fn move_up(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) if i > 0 => i - 1,
            _ => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Moves the selection cursor one position down in the visible list.
    pub fn move_down(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) if i < self.view_items.len() - 1 => i + 1,
            Some(i) => i,
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Toggles the selection status for the currently highlighted node and its descendants.
    pub fn toggle_selection(&mut self) {
        if let Some(selected_idx) = self.list_state.selected() {
            if let Some(&node_idx) = self.view_items.get(selected_idx) {
                let new_state = !self.nodes[node_idx].selected;
                self.set_recursive_selection(node_idx, new_state);
            }
        }
    }

    /// Internal recursive helper to update selection state down the tree.
    fn set_recursive_selection(&mut self, idx: usize, state: bool) {
        self.nodes[idx].selected = state;
        let children = self.nodes[idx].children.clone();
        for child_idx in children {
            self.set_recursive_selection(child_idx, state);
        }
    }

    /// Expands or collapses the selected directory node.
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

    /// Aggregates all currently selected file paths into a HashSet.
    pub fn get_selected_paths(&self) -> HashSet<PathBuf> {
        self.nodes
            .iter()
            .filter(|n| n.selected && !n.is_dir)
            .map(|n| n.path.clone())
            .collect()
    }

    /// Rebuilds the flat list of visible items based on expansion states.
    pub fn update_view(&mut self) {
        self.view_items.clear();
        let roots = self.root_indices.clone();
        for root_idx in roots {
            self.collect_nodes(root_idx);
        }
        if self.list_state.selected().is_none() && !self.view_items.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    /// Recursively traverses and adds visible nodes to the view list.
    fn collect_nodes(&mut self, idx: usize) {
        self.view_items.push(idx);
        if self.nodes[idx].is_dir && self.nodes[idx].expanded {
            let children = self.nodes[idx].children.clone();
            for child_idx in children {
                self.collect_nodes(child_idx);
            }
        }
    }
}