use std::path::PathBuf;

/// UI-specific representation of a node in the interactive tree.
pub struct UiNode {
    /// The filesystem path associated with this node.
    pub path: PathBuf,
    /// The display name in the TUI explorer.
    pub name: String,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Whether the directory is currently expanded in the view.
    pub expanded: bool,
    /// Whether the node is selected for the final dump.
    pub selected: bool,
    /// Inherited hidden status from the domain model.
    pub is_hidden: bool,
    /// Inherited ignored status from the domain model.
    pub is_ignored: bool,
    /// Visual depth level for correct indentation rendering.
    pub depth: usize,
    /// Estimated tokens to be shown in the UI.
    pub token_estimate: usize,
    /// Indices of child nodes within the App's node vector.
    pub children: Vec<usize>,
}

impl UiNode {
    /// Constructs a new UiNode with default expansion and smart selection logic.
    pub fn new(
        path: PathBuf,
        name: String,
        is_dir: bool,
        depth: usize,
        hidden: bool,
        ignored: bool,
        token_estimate: usize,
    ) -> Self {
        Self {
            path,
            name,
            is_dir,
            expanded: true,
            selected: !hidden && !ignored,
            is_hidden: hidden,
            is_ignored: ignored,
            token_estimate,
            depth,
            children: Vec::new(),
        }
    }
}