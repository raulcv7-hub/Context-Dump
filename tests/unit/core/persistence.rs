use context::core::config::{ContextConfig, OutputFormat};
use context::core::config::persistence::ConfigPersistence;

/// Verifies the full serialization and deserialization cycle of the application configuration.
/// Uses struct update syntax to avoid clippy warnings about reassignment.
#[test]
fn test_config_persistence_cycle() {
    let config = ContextConfig {
        to_clipboard: true,
        output_format: OutputFormat::Markdown,
        exclude_paths: vec!["target".to_string(), "node_modules".to_string()],
        ..ContextConfig::default()
    };

    ConfigPersistence::save(&config).expect("Should save config");

    let loaded = ConfigPersistence::load().expect("Should load config");
    
    assert!(loaded.is_some());
    let loaded_cfg = loaded.unwrap();
    
    assert!(loaded_cfg.to_clipboard);
    assert_eq!(loaded_cfg.output_format, OutputFormat::Markdown);
    assert!(loaded_cfg.exclude_paths.contains(&"target".to_string()));
}