mod factory;
mod strategies;

use factory::MinifierFactory;

pub struct ContentMinifier;

impl ContentMinifier {
    /// Entry point for minifying content based on its language.
    pub fn minify(content: &str, language: &str) -> String {
        let strategy = MinifierFactory::get_strategy(language);
        strategy.process(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_facade_integration() {
        let result = ContentMinifier::minify("  fn test() {}  ", "rs");
        assert_eq!(result, "fn test() {}\n");
    }
}
