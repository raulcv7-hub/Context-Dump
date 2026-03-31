use std::path::PathBuf;

/// Visual representation of a filesystem node within the interactive UI.
pub struct UiNode {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub expanded: bool,
    pub selected: bool,
    pub is_hidden: bool,
    pub is_ignored: bool,
    pub is_sensitive: bool,
    pub is_git_ignored: bool,
    pub depth: usize,
    pub token_estimate: usize,
    pub children: Vec<usize>,
}

impl UiNode {
    /// Constructs a fully populated UiNode with specific expansion behavior.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        path: PathBuf,
        name: String,
        is_dir: bool,
        depth: usize,
        hidden: bool,
        ignored: bool,
        sensitive: bool,
        git_ignored: bool,
        token_estimate: usize,
        selected: bool,
        expanded: bool,
    ) -> Self {
        Self {
            path,
            name,
            is_dir,
            expanded,
            selected,
            is_hidden: hidden,
            is_ignored: ignored,
            is_sensitive: sensitive,
            is_git_ignored: git_ignored,
            token_estimate,
            depth,
            children: Vec::new(),
        }
    }
}