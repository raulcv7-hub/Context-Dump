use std::path::PathBuf;

/// Representación atómica de un elemento en la interfaz de usuario.
pub struct UiNode {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub expanded: bool,
    pub selected: bool,
    pub is_hidden: bool,
    pub is_ignored: bool,
    pub token_estimate: usize,
    pub depth: usize,
    pub children: Vec<usize>,
}

impl UiNode {
    /// Crea un nuevo nodo de interfaz con soporte para metadatos de visibilidad y peso.
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
