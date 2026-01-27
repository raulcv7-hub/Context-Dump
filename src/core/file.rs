use std::path::PathBuf;

/// Representa un archivo en el dominio con metadatos de visibilidad y peso estimado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileNode {
    pub path: PathBuf,
    pub relative_path: PathBuf,
    pub is_hidden: bool,
    pub is_ignored: bool,
    pub token_estimate: usize,
}

impl FileNode {
    /// Crea un nuevo nodo de archivo con todos sus metadatos.
    pub fn new(
        path: PathBuf,
        relative_path: PathBuf,
        is_hidden: bool,
        is_ignored: bool,
        token_estimate: usize,
    ) -> Self {
        Self {
            path,
            relative_path,
            is_hidden,
            is_ignored,
            token_estimate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_node_token_carry() {
        let node = FileNode::new(PathBuf::from("a"), PathBuf::from("a"), false, false, 100);
        assert_eq!(node.token_estimate, 100);
    }
}
