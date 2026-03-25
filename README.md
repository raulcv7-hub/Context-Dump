# Context-Dump

[![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Context-Dump is a high-performance command-line utility and terminal user interface (TUI) designed for the systematic aggregation of project source code and technical documentation into a unified context format. The engine is optimized for ingestion by Large Language Models (LLMs), ensuring high data fidelity and precise token management.

## Technical Overview

The application is engineered in Rust to provide a memory-safe, zero-dependency executable. It addresses the challenge of project context window limitations by allowing users to selectively aggregate disparate file formats into structured XML or Markdown reports.

### Key Capabilities

- **High-Concurrency Ingestion**: Utilizes the Rayon data-parallelism library to execute multi-threaded file scanning and content extraction.
- **Native Format Support**: Implements built-in, pure-Rust parsers for PDF, Microsoft Office (XLSX, DOCX), Jupyter Notebooks (IPYNB), and standard UTF-8 text files.
- **Precision Tokenization**: Employs the `cl100k_base` BPE encoding (standardized by GPT-4 and Claude models) via `tiktoken-rs` for exact context window budgeting.
- **Dependency-Free Portability**: Compiles to a static binary with no external runtime requirements (e.g., Python, C-shared libraries, or virtual environments).
- **State Persistence**: Automatically retains user configuration, including inclusion/exclusion filters and output preferences, between execution cycles.

## Installation

### Prerequisites

- Rust Toolchain (1.80 or higher)
- C Compiler (for internal linking on specific targets)

### Building from Source

To compile and install the optimized release binary to your local path:

```bash
make install
```

This command invokes the Rust compiler with maximum optimization flags and relocates the binary to the standard user path (`~/.local/bin` or equivalent).

## Operation Modes

### Interactive Mode (TUI)

Executing the utility without arguments initializes the interactive Terminal User Interface:

```bash
context
```

**Keybindings and Navigation:**

| Command | Action |
|:---|:---|
| Arrows | Traverse the hierarchical file tree. |
| Space | Toggle recursive selection/deselection of nodes. |
| o | Cycle output target (Standard Output, File System, System Clipboard). |
| f | Cycle serialization format (XML, Markdown). |
| Enter | Commit selection and initialize processing. |
| Esc / q | Terminate the application. |

### Headless Mode (CLI)

The utility supports non-interactive execution for integration into automated pipelines and shell redirection:

```bash
context <PATH> [OPTIONS]
```

**Common Flags:**

- `-o, --output <FILE>`: Specifies the target path for the report.
- `-s, --stdout`: Redirects the report content to the standard output stream (Feedback is redirected to stderr).
- `-f, --format <FORMAT>`: Manually selects the serialization format (`xml` or `markdown`).
- `-e, --extensions <LIST>`: Implements a whitelist filter for specific file extensions.
- `-X, --exclude-path <LIST>`: Implements a blacklist filter for path substrings.

## System Architecture

The project adheres to a Modular Native Architecture, separating domain logic from infrastructure adapters:

- **Core Layer**: Manages BPE tokenization, domain models (`FileNode`, `FileContext`), and the ASCII tree rendering engine.
- **Adapter Layer**: Contains specialized native extractors for binary and structured formats.
- **Engine Layer**: Orchestrates the execution lifecycle, CLI argument parsing, and progress reporting via `stderr`.
- **UI Layer**: Implements a state-driven reactive TUI using the `ratatui` framework.

## Data Serialization Formats

### XML (Structured)
Optimized for programmatic LLM ingestion. It provides clear demarcations between file metadata, directory structure, and content within CDATA blocks to prevent escape character corruption.

### Markdown (Readable)
Optimized for human auditing and LLM prompting. It utilizes standard Markdown syntax and fenced code blocks with language identifiers.

## License

This software is released under the MIT License. For further information, refer to the [LICENSE](./LICENSE) file included in this repository.
