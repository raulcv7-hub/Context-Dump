use context::adapters::output::markdown::MarkdownWriter;
use context::adapters::output::xml::XmlWriter;
use context::core::config::{ContextConfig, OutputFormat};
use context::core::content::FileContext;
use context::ports::writer::ContextWriter;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Dispatches the generated context to a single destination based on priority.
pub fn dispatch(files: &[FileContext], config: &ContextConfig) -> anyhow::Result<()> {
    let mut buffer = Vec::new();

    let mut sorted_refs: Vec<&FileContext> = files.iter().collect();
    sorted_refs.sort_by(|a, b| {
        let p1 = get_priority(&a.relative_path);
        let p2 = get_priority(&b.relative_path);
        p1.cmp(&p2)
            .then_with(|| a.relative_path.cmp(&b.relative_path))
    });

    match config.output_format {
        OutputFormat::Xml => XmlWriter::new().write(&sorted_refs, config, &mut buffer)?,
        OutputFormat::Markdown => MarkdownWriter::new().write(&sorted_refs, config, &mut buffer)?,
    }

    if config.to_clipboard {
        if let Err(e) = dispatch_clipboard(&buffer) {
            return fallback_to_file(&buffer, config, &e);
        }
        return Ok(());
    }

    if let Some(path) = &config.output_path {
        let mut file = File::create(path)?;
        file.write_all(&buffer)?;
    } else {
        io::stdout().write_all(&buffer)?;
        io::stdout().flush()?;
    }

    Ok(())
}

/// Architecture-aware clipboard dispatcher.
/// Linux environments drop data if the CLI process dies quickly. We use cross-process detached daemons instead.
#[cfg(target_os = "linux")]
fn dispatch_clipboard(buffer: &[u8]) -> Result<(), String> {
    use std::process::{Command, Stdio};

    // Cross-Process Persistent Clipboard Daemon
    // Attempt 1: Wayland (wl-copy)
    if let Ok(mut child) = Command::new("wl-copy").stdin(Stdio::piped()).spawn() {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(buffer);
        }
        if child.wait().is_ok() {
            return Ok(());
        }
    }

    // Attempt 2: X11 (xclip)
    if let Ok(mut child) = Command::new("xclip")
        .args(["-selection", "clipboard"])
        .stdin(Stdio::piped())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(buffer);
        }
        if child.wait().is_ok() {
            return Ok(());
        }
    }

    // Attempt 3: X11 Alternative (xsel)
    if let Ok(mut child) = Command::new("xsel")
        .args(["--clipboard", "--input"])
        .stdin(Stdio::piped())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(buffer);
        }
        if child.wait().is_ok() {
            return Ok(());
        }
    }

    Err("No clipboard daemon found. Please install wl-copy, xclip, or xsel.".into())
}

#[cfg(not(target_os = "linux"))]
fn dispatch_clipboard(buffer: &[u8]) -> Result<(), String> {
    use std::thread;
    use std::time::Duration;

    let mut clip = arboard::Clipboard::new().map_err(|e| format!("Clipboard error: {}", e))?;
    let text = String::from_utf8(buffer.to_vec()).map_err(|_| "Invalid UTF-8".to_string())?;

    clip.set_text(text)
        .map_err(|e| format!("Failed to set text: {}", e))?;
    thread::sleep(Duration::from_millis(400)); // Minor grace period for Windows/macOS
    Ok(())
}

fn fallback_to_file(buffer: &[u8], config: &ContextConfig, reason: &str) -> anyhow::Result<()> {
    eprintln!("\n⚠️ [CLIPBOARD ERROR] {}", reason);
    eprintln!("💾 Automatically falling back to file output to prevent data loss.");

    let ext = match config.output_format {
        OutputFormat::Xml => "xml",
        OutputFormat::Markdown => "md",
    };
    let fallback = format!("context_fallback.{}", ext);
    let mut file = File::create(&fallback)?;
    file.write_all(buffer)?;

    eprintln!("✅ Output safely redirected to: {}", fallback);
    Ok(())
}

fn get_priority(path: &Path) -> usize {
    let name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase();
    let ext = path
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase();

    match name.as_str() {
        "readme.md" | "architecture.md" | "summary.md" | "contributing.md" => 0,
        "cargo.toml" | "package.json" | "dockerfile" | "makefile" | "tsconfig.json" | "go.mod"
        | "pom.xml" | "build.gradle" => 1,
        "main.rs" | "lib.rs" | "index.js" | "index.ts" | "main.go" | "app.js" | "app.ts" => 2,
        n if n.contains("types") || n.contains("interface") || ext == "d.ts" => 3,
        _ if ["rs", "go", "ts", "js", "py", "java", "c", "cpp", "h", "hpp"]
            .contains(&ext.as_str()) =>
        {
            4
        }
        _ => 5,
    }
}
