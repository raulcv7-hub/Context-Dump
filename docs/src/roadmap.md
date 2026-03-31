# Future Roadmap & Epics

The following epics outline the planned features and architectural expansions for Context-Dump. They are categorized by technical impact and priority.

## Epic 1: Security & Protection (High Priority)
Preventing the accidental leakage of sensitive credentials to LLMs.
- **Hardcoded Blacklist:** Absolute denial of parsing for standard key files (`.env`, `id_rsa`, `*.pem`, `*.key`).
- **TUI Security Warnings:** A dedicated UI panel that flashes red to alert the user if a wallet or authentication directory is detected.
- **Entropy Filtering:** Pre-parsing analysis to detect high-entropy strings (potential API keys or passwords) and sanitize them before output.

## Epic 2: Token Economics & Budgeting
Enhancing the user's control over the exact payload size sent to the LLM.
- **Test Exclusion Shortcut:** A dedicated TUI shortcut to instantly deselect all localized test files (e.g., `*_test.rs`, `*.spec.ts`, `tests/`).
- **Token Weight Heatmap:** Rendering file names in the TUI using color gradients (Green/Yellow/Red) based on their `token_estimate` size.
- **Context Truncation:** Setting a hard token limit per file; if exceeded, the parser will append an `<omitted for brevity>` marker instead of halting or flooding the context.
- **Priority Dumping:** Reordering the final XML/Markdown output so critical files (`README.md`, configuration files) appear at the top, capitalizing on the LLM's primary attention window.

## Epic 3: UI Quality of Life & State Persistence
- **Mass Collapse/Expand:** Shortcuts (e.g., `Shift+Arrows`) to recursively expand or collapse the entire file tree.
- **Project-Specific Persistence:** Modifying the state manager to remember file selections on a per-project basis. If new files are added, they are selected by default, while previously unselected files remain ignored.
- **Clipboard Limits:** Checking the native OS clipboard buffer capacity and warning the user if the project dump exceeds it.

## Epic 4: External Integrations
- **Remote Repository Ingestion:** Allowing the CLI to accept a Git URL, cloning it to a temporary directory, extracting the context, and self-cleaning.
- **Web Content Ingestion:** A lightweight, headless HTTP crawler to extract text from a provided documentation URL without requiring heavy external browser binaries.
- **Third-Party File Filtering:** Specific detection algorithms to auto-exclude generated `dist` or `vendor` content injected by package managers.