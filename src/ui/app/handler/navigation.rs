use crate::ui::app::core::App;
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use std::collections::HashSet;
use std::path::PathBuf;

impl App {
    /// Navigates the cursor up one element in the current view.
    pub fn move_up(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) if i > 0 => i - 1,
            _ => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Navigates the cursor down one element in the current view.
    pub fn move_down(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) if i < self.view_items.len().saturating_sub(1) => i + 1,
            Some(i) => i,
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Toggles the inclusion state of the currently highlighted file or directory.
    pub fn toggle_selection(&mut self) {
        if let Some(selected_idx) = self.list_state.selected() {
            if let Some(&node_idx) = self.view_items.get(selected_idx) {
                if self.nodes[node_idx].is_sensitive {
                    return;
                }
                let new_state = !self.nodes[node_idx].selected;
                self.set_recursive_selection(node_idx, new_state);
            }
        }
    }

    /// Recursively applies a selection state to all children of a directory node.
    fn set_recursive_selection(&mut self, idx: usize, state: bool) {
        if self.nodes[idx].is_sensitive {
            return;
        }
        self.nodes[idx].selected = state;
        let children = self.nodes[idx].children.clone();
        for child_idx in children {
            self.set_recursive_selection(child_idx, state);
        }
    }

    /// Expands or collapses the currently highlighted directory.
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

    /// Expands the currently highlighted directory.
    pub fn expand_node(&mut self) {
        if let Some(selected_idx) = self.list_state.selected() {
            if let Some(&node_idx) = self.view_items.get(selected_idx) {
                if self.nodes[node_idx].is_dir && !self.nodes[node_idx].expanded {
                    self.nodes[node_idx].expanded = true;
                    self.update_view();
                }
            }
        }
    }

    /// Collapses the currently highlighted directory.
    pub fn collapse_node(&mut self) {
        if let Some(selected_idx) = self.list_state.selected() {
            if let Some(&node_idx) = self.view_items.get(selected_idx) {
                if self.nodes[node_idx].is_dir && self.nodes[node_idx].expanded {
                    self.nodes[node_idx].expanded = false;
                    self.update_view();
                }
            }
        }
    }

    /// Recursively expands all directory nodes in the tree.
    pub fn expand_all(&mut self) {
        for node in &mut self.nodes {
            if node.is_dir {
                node.expanded = true;
            }
        }
        self.update_view();
    }

    /// Recursively collapses all directory nodes in the tree.
    pub fn collapse_all(&mut self) {
        for node in &mut self.nodes {
            if node.is_dir {
                node.expanded = false;
            }
        }
        self.update_view();
    }

    /// Selects all non-sensitive files globally across the tree.
    pub fn select_all(&mut self) {
        for node in &mut self.nodes {
            if !node.is_dir && !node.is_sensitive {
                node.selected = true;
            }
        }
    }

    /// Deselects all files globally across the tree.
    pub fn deselect_all(&mut self) {
        for node in &mut self.nodes {
            if !node.is_dir {
                node.selected = false;
            }
        }
    }

    /// Intelligently toggles selection for all test-related files based on naming heuristics.
    pub fn toggle_tests(&mut self) {
        let mut any_test_selected = false;
        for node in &self.nodes {
            if !node.is_dir && node.selected && Self::is_test_node(node) {
                any_test_selected = true;
                break;
            }
        }
        let new_state = !any_test_selected;
        for node in &mut self.nodes {
            if !node.is_dir && Self::is_test_node(node) {
                node.selected = new_state;
            }
        }
    }

    /// Evaluates heuristic rules to determine if a node represents a software test.
    fn is_test_node(node: &crate::ui::node::UiNode) -> bool {
        let has_test_component = node.path.components().any(|c: std::path::Component| {
            let s = c.as_os_str().to_string_lossy().to_lowercase();
            s == "tests"
                || s == "test"
                || s == "__tests__"
                || s == "mocks"
                || s == "spec"
                || s == "e2e"
        });

        let name = node.name.to_lowercase();
        let is_test_file = name.contains("_test.")
            || name.contains(".test.")
            || name.contains(".spec.")
            || name.starts_with("test_");

        has_test_component || is_test_file
    }

    /// Harvests the final set of PathBufs chosen by the user, explicitly dropping sensitive assets.
    pub fn get_selected_paths(&self) -> HashSet<PathBuf> {
        self.nodes
            .iter()
            .filter(|n| n.selected && !n.is_dir && !n.is_sensitive)
            .map(|n| n.path.clone())
            .collect()
    }

    /// Refreshes the active visual buffer of the hierarchy state.
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

    /// Appends visible nodes into the linear rendering vector respecting collapsed states.
    fn collect_nodes(&mut self, idx: usize) {
        self.view_items.push(idx);
        if self.nodes[idx].is_dir && self.nodes[idx].expanded {
            let children = self.nodes[idx].children.clone();
            for child_idx in children {
                self.collect_nodes(child_idx);
            }
        }
    }

    /// Maps external terminal mouse interactions into interface selection commands.
    pub fn handle_mouse(&mut self, event: MouseEvent) {
        match event.kind {
            MouseEventKind::ScrollDown => self.move_down(),
            MouseEventKind::ScrollUp => self.move_up(),
            MouseEventKind::Down(MouseButton::Left) => {
                let offset = self.list_state.offset();
                let clicked_row = event.row.saturating_sub(1) as usize;
                let target_idx = offset + clicked_row;

                if target_idx < self.view_items.len() {
                    self.list_state.select(Some(target_idx));
                    self.toggle_selection();
                }
            }
            _ => {}
        }
    }
}