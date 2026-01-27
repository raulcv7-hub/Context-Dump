use std::collections::BTreeMap;
use std::path::Path;

#[derive(Default)]
pub struct TreeRenderer {
    children: BTreeMap<String, TreeRenderer>,
}

impl TreeRenderer {
    /// Constructs a tree structure from a slice of relative paths.
    pub fn build<P: AsRef<Path>>(paths: &[P]) -> Self {
        let mut root = Self::default();
        for path in paths {
            root.insert(path.as_ref());
        }
        root
    }

    /// Recursively inserts path components into the internal map.
    fn insert(&mut self, path: &Path) {
        let mut current = self;
        for component in path.components() {
            let key = component.as_os_str().to_string_lossy().to_string();
            current = current.children.entry(key).or_default();
        }
    }

    /// Returns the ASCII representation of the tree.
    pub fn render(&self, root_name: &str) -> String {
        let mut buffer = String::new();
        buffer.push_str(root_name);
        buffer.push('\n');
        self.render_recursive("", &mut buffer);
        buffer
    }

    /// Internal recursive drawing logic.
    fn render_recursive(&self, prefix: &str, buffer: &mut String) {
        let count = self.children.len();
        for (i, (name, node)) in self.children.iter().enumerate() {
            let is_last = i == count - 1;
            let connector = if is_last { "└── " } else { "├── " };

            buffer.push_str(prefix);
            buffer.push_str(connector);
            buffer.push_str(name);
            buffer.push('\n');

            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            node.render_recursive(&new_prefix, buffer);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_rendering_logic() {
        let paths = vec!["src/main.rs", "src/lib.rs", "Cargo.toml"];
        let renderer = TreeRenderer::build(&paths);
        let output = renderer.render("my_project");

        assert!(output.contains("my_project"));
        assert!(output.contains("├── Cargo.toml"));
        assert!(output.contains("└── src"));
        assert!(output.contains("    ├── lib.rs"));
        assert!(output.contains("    └── main.rs"));
    }
}
