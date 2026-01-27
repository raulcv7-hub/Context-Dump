use crate::core::content::minifier::strategies::{
    AggressiveStrategy, IndentPreservingStrategy, MarkdownStrategy, MinificationStrategy,
    NoopStrategy,
};

pub struct MinifierFactory;

impl MinifierFactory {
    /// Selecciona la estrategia de reducción de tokens basada en el lenguaje.
    pub fn get_strategy(language: &str) -> Box<dyn MinificationStrategy> {
        let lang = language.to_lowercase();

        match lang.as_str() {
            // Documentos técnicos y lenguajes indentados
            "py" | "python" | "yaml" | "yml" => Box::new(IndentPreservingStrategy),

            // Nueva estrategia específica para preservar Markdown generado por PDF/DOCX
            "md" | "markdown" => Box::new(MarkdownStrategy),

            // Lenguajes de llaves (Minificación agresiva de espacios)
            "rs" | "rust" | "js" | "javascript" | "ts" | "typescript" | "c" | "cpp" | "java"
            | "cs" => Box::new(AggressiveStrategy),

            _ => Box::new(NoopStrategy),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_mapping() {
        let s1 = MinifierFactory::get_strategy("rs");
        let res = s1.process("  content  ");
        assert_eq!(res, "content\n");

        let s2 = MinifierFactory::get_strategy("py");
        let res = s2.process("    pass    ");
        assert_eq!(res, "    pass\n");
    }
}
