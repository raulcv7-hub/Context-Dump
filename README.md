# Context

[![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

This project is a high-performance ingestion engine designed to transform complex repositories and technical documentation into a unified context, optimized for LLMs (ChatGPT, Claude, Llama, Gemini).

## Key Features

*   **Multi-threaded Performance**: Massive parallel processing powered by `Rayon`.
*   **State Persistence**: Automatically remembers your last configuration (format, filters, output destination).
*   **Smart Mode**: If invoked with arguments (e.g., `context .`), it executes the last saved configuration instantly.
*   **Smart Ignore**: Dynamic heuristics to omit heavy artifacts (>1MB) and known binary noise.
*   **Native Multi-platform Parsers**: High-speed, built-in support for **PDF, DOCX, Excel, and Text** without external dependencies (no Python required).

## Strategic Usage

### 1. Interactive Flow
Run the command without arguments to open the TUI:
```bash
context
```
*Configure your preferences, navigate the tree to select/deselect files, and confirm with `Enter`. This setup will be saved as your new default.*

## TUI Interface

| Key | Action |
| :--- | :--- |
| `Space` | **Toggle Selection** (applies recursively to directories). |
| `Enter` | **Confirm and Process**. |
| `c` | Toggle **Clipboard** (ON/OFF). |
| `o` | Toggle **Output File** (File vs. Stdout). |
| `f` | Cycle **Format** (XML ↔ Markdown). |
| `Arrows` | Navigate the file tree. |
| `Left/Right` | Collapse / Expand directories. |
| `Esc / q` | **Exit** the application. |

## CLI Options

| Flag | Description |
| :--- | :--- |
| `-o, --output <FILE>` | Output path. Automatically detects format by extension. |
| `-s, --stdout` | Dumps to terminal (disables TUI and overrides output file). |
| `-c, --clip` | Copies the result to the system clipboard. |
| `-S, --smart-ignore` | Enable/Disable noise heuristics (default: true). |
| `-e, --extensions` | Whitelist extensions (e.g., `rs,py,ts`). |
| `-x, --exclude` | Blacklist extensions. |
| `-i, --include-path` | Inclusion filter by string in path. |
| `-X, --exclude-path` | Exclusion filter by string in path. |
| `-I, --interactive` | Force TUI mode, ignoring other flags. |
| `-v, --verbose` | Log level (`-v` INFO, `-vv` DEBUG). |

## Architecture

The project follows a **Modular Hexagonal Native Architecture** for maximum portability:

```text
src/
├── core/           # Domain (Config, Persistence, Models)
├── ports/          # Interfaces (Traits for Scanner, Reader, Writer)
├── adapters/       # Technical Implementations
│   ├── fs_scanner/ # Smart Ignore engine
│   ├── output/     # XML and Markdown generators
│   └── parsers/    # Native PDF, Office, and Text extractors
└── ui/             # Reactive TUI (Ratatui)
```
