# Context-Dump

[![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Docs](https://img.shields.io/badge/docs-mdBook-blue.svg)](https://raulcv7-hub.github.io/Context-Dump/)

**Context-Dump** is a high-performance, security-hardened native engine designed to aggregate massive project source code and technical documentation into unified, token-optimized context reports for Large Language Models (LLMs). 

It transforms disparate file systems into clean, structured XML or Markdown payloads while strictly enforcing data privacy and context window economics.

## Security & Privacy

Context-Dump is built for environments where data leakage is not an option. It features a multi-layered security pipeline:

- **PII & Secret Redaction**: Native Regex-based masking of Emails, IPv4 addresses, AWS Access Keys (`AKIA...`), JWTs, and Stripe Live Tokens.
- **Luhn Algorithm Validation**: Smart credit card detection that validates 13-19 digit sequences mathematically before redaction to prevent false positives.
- **Suspicious Code Detection**: Heuristically identifies minified, obfuscated, or machine-generated assets (e.g., massive single-line JS bundles) to save your token budget and alert the LLM.
- **Zero-Trace Remote Ingestion**: Pass a Git URL directly. The engine performs a shallow clone into a temporary directory and utilizes strict **RAII patterns** to ensure the repository is wiped from the disk even if the process is interrupted.
- **Hard-Coded Security Firewall**: Absolute denial-of-service for sensitive credentials like `.env` files, `id_rsa` keys, `kubeconfig`, and certificates.

## Advanced Capabilities

- **X-Ray Archive Parsing**: Natively "sees" inside `.zip` and `.tar.gz` files. It extracts internal text content without requiring manual decompression.
- **Token Heatmaps**: The TUI dynamically colors files (Green/Yellow/Red) based on their `cl100k_base` BPE token weight.
- **Intelligent Truncation**: Prevents LLM context overflows by truncating files that exceed a specific token limit (default: 30k), injecting a `[OMITTED FOR BREVITY]` marker.
- **Priority Dumping**: Intelligently sorts the output so that `README.md`, architectural docs, and project configurations (`Cargo.toml`, `package.json`) appear at the top of the report.
- **Cross-Process Clipboard (Linux)**: Bypasses standard clipboard timeouts on X11/Wayland by spawning background daemons (`wl-copy`, `xclip`) to ensure massive payloads persist after the CLI exits.

## Installation

### Prerequisites
- **Rust Toolchain** (1.80 or higher)
- **Git** (Required only for remote repository features)

### Build and Install
```bash
make install
```
This command compiles an optimized release binary and moves it to your local bin directory (typically `~/.local/bin` or `~/.cargo/bin`).

## Operation Modes

### Interactive Mode (TUI)
Initializing the utility without arguments or target flags launches the interactive explorer:
```bash
context
```

**Keybindings and Navigation:**
| Command | Action |
|:---|:---|
| `Arrows` / `Mouse Wheel` | Traverse the hierarchical file tree. |
| `Left Click` / `Space` | Toggle selection of a file or recursively for a directory. |
| `a` / `d` | **Select All** or **Deselect All** valid files in the project. |
| `t` | **Toggle Tests**: Smart-select/deselect all test files and `/tests` directories. |
| `E` / `C` | **Expand All** or **Collapse All** folders in the tree. |
| `e` / `c` | **Expand** or **Collapse** only the currently highlighted node. |
| `o` / `f` | Cycle Output target (Clipboard, File, Stdout) or Format (XML, Markdown). |
| `Enter` | Commit selection and initialize processing. |

### Headless & Remote Mode (CLI)
For automation, CI/CD pipelines, or quick remote extractions:

```bash
# Clone a remote repo, extract context, and copy to clipboard in one step:
context https://github.com/rust-lang/regex --clip

# Scan local directory with strict extension filters and output to stdout:
context ./src --stdout --format markdown -e rs,toml --max-tokens 50000
```

## Persistence & Governance

- **Project-Specific Memory**: The tool generates a unique hash for every project root, remembering your exact file selections and configuration preferences across sessions.
- **Provenance Tracking**: Every report includes metadata indicating the source repository URL and the specific SHA1 commit hash for auditing purposes.
- **.gitignore Awareness**: Respects your existing ignore rules by deselecting matched files by default while keeping them visible for manual override.

## Technical Documentation
For deep-dives into the internal architecture, security heuristics, and deployment strategies, visit the [Context-Dump Documentation Site](https://RaulCarrillo.github.io/context).

## License
Released under the MIT License. Copyright (c) 2026 Raúl Carrillo Vicente.
