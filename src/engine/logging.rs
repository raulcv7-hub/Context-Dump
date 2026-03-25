use std::io;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

/// Configures the global tracing subscriber for the engine.
/// Sets the log level based on the verbosity count from the CLI.
pub fn setup(verbosity: u8) {
    let level = match verbosity {
        0 => Level::WARN,
        1 => Level::INFO,
        _ => Level::DEBUG,
    };

    let _ = FmtSubscriber::builder()
        .with_max_level(level)
        .with_writer(io::stderr)
        .try_init();
}