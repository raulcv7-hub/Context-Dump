/// Interfaz para las estrategias de reducción de contenido.
pub trait MinificationStrategy: Send + Sync {
    fn process(&self, content: &str) -> String;
}

/// Estrategia para lenguajes donde la indentación es semántica (Python/YAML).
pub struct IndentPreservingStrategy;

impl MinificationStrategy for IndentPreservingStrategy {
    fn process(&self, content: &str) -> String {
        content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| format!("{}\n", line.trim_end()))
            .collect()
    }
}

/// Estrategia para Markdown y Documentos
/// Preserva la estructura visual (tablas, listas) pero elimina el aire innecesario.
pub struct MarkdownStrategy;

impl MinificationStrategy for MarkdownStrategy {
    fn process(&self, content: &str) -> String {
        let mut output = String::with_capacity(content.len());
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if trimmed.starts_with('|') || trimmed.starts_with('*') || trimmed.starts_with('-') {
                output.push_str(line.trim_end());
            } else {
                output.push_str(trimmed);
            }
            output.push('\n');
        }
        output
    }
}

/// Estrategia agresiva para lenguajes tipo C (Rust, JS, Java).
pub struct AggressiveStrategy;

impl MinificationStrategy for AggressiveStrategy {
    fn process(&self, content: &str) -> String {
        content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| format!("{}\n", line.trim()))
            .collect()
    }
}

pub struct NoopStrategy;

impl MinificationStrategy for NoopStrategy {
    fn process(&self, content: &str) -> String {
        content.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_preservation() {
        let md = "# Title\n\n| Col 1 | Col 2 |\n|---|---|\n| val | val |\n\n* List item";
        let strategy = MarkdownStrategy;
        let result = strategy.process(md);

        assert!(result.contains("| Col 1 | Col 2 |")); // Tabla intacta
        assert!(result.contains("* List item")); // Lista intacta
        assert!(!result.contains("\n\n")); // Sin saltos dobles
    }
}
