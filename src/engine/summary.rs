use crate::output_dispatcher;
use context::core::config::ContextConfig;
use context::core::content::FileContext;

/// Dispatches output and prints an execution summary to the stderr stream.
/// Redirection to stderr ensures stdout remains clean for data piping.
pub fn print(contexts: &[FileContext], config: &ContextConfig) -> anyhow::Result<()> {
    output_dispatcher::dispatch(contexts, config)?;
    let total_tokens: usize = contexts.iter().map(|c| c.token_count).sum();

    eprintln!("\n🚀 --- EXECUTION SUMMARY ---");
    eprintln!("📂 Files processed:  {}", contexts.len());
    eprintln!("🪙 Total tokens:     {}", total_tokens);
    eprintln!("📄 Format:           {:?}", config.output_format);

    if config.to_clipboard {
        eprintln!("📋 Destination:      SYSTEM CLIPBOARD");
    } else if let Some(p) = &config.output_path {
        eprintln!("💾 Destination:      FILE ({})", p.display());
    } else {
        eprintln!("📺 Destination:      STANDARD OUTPUT");
    }
    eprintln!("-----------------------------\n");
    Ok(())
}
