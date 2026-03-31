use crate::core::config::ContextConfig;
use crate::core::file::FileNode;
use crate::ui::app::core::App;
use crate::ui::node::UiNode;
use ratatui::widgets::ListState;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

impl App {
    /// Builds the internal hierarchical state from the linear vector of parsed FileNodes.
    pub fn new(files: &[FileNode], _root_path: &Path, config: ContextConfig) -> Self {
        let mut nodes = Vec::new();
        let mut path_to_index: HashMap<PathBuf, usize> = HashMap::new();
        let mut root_indices = Vec::new();

        for file in files {
            let relative = &file.relative_path;
            let mut current_path = PathBuf::new();
            let mut parent_idx: Option<usize> = None;

            for component in relative.components() {
                current_path.push(component);

                if !path_to_index.contains_key(&current_path) {
                    let is_dir = current_path != *relative || file.is_dir;
                    let depth = current_path.components().count().saturating_sub(1);
                    let name = component.as_os_str().to_string_lossy().to_string();

                    let tokens = if is_dir { 0 } else { file.token_estimate };

                    let is_expanded = if is_dir && current_path == *relative {
                        !file.is_ignored
                    } else {
                        true
                    };

                    let previously_selected = config.file_states.get(&current_path);
                    let is_selected = match previously_selected {
                        Some(&was_selected) => was_selected,
                        None => {
                            !file.is_hidden
                                && !file.is_ignored
                                && !file.is_sensitive
                                && !file.is_git_ignored
                                && !is_dir
                        }
                    };

                    let node = UiNode::new(
                        current_path.clone(),
                        name,
                        is_dir,
                        depth,
                        file.is_hidden,
                        file.is_ignored,
                        file.is_sensitive,
                        file.is_git_ignored,
                        tokens,
                        is_selected,
                        is_expanded,
                    );

                    let idx = nodes.len();
                    nodes.push(node);
                    path_to_index.insert(current_path.clone(), idx);

                    if let Some(p) = parent_idx {
                        nodes[p].children.push(idx);
                        nodes[p].is_dir = true;
                    } else {
                        root_indices.push(idx);
                    }
                }
                parent_idx = Some(path_to_index[&current_path]);
            }
        }

        let mut app = Self {
            nodes,
            root_indices,
            list_state: ListState::default(),
            view_items: Vec::new(),
            should_quit: false,
            confirmed: false,
            config,
            default_filename: "context_report".to_string(),
        };

        app.update_view();
        if !app.view_items.is_empty() {
            app.list_state.select(Some(0));
        }
        app
    }
}