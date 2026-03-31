# Configuration & State Management

Context-Dump implements a silent state-persistence mechanism. This ensures that a developer's workflow remains uninterrupted across multiple executions without requiring them to repeatedly pass the same CLI flags.

## Persistence Lifecycle

When the application executes, it follows this configuration lifecycle:

1. **Load Phase:** The engine checks the user's OS-specific configuration directory (e.g., `~/.config/context/last_run.json` on Linux or `%AppData%` on Windows). If it exists, it deserializes the previous session's state.
2. **Override Phase:** Any explicit flags passed via the CLI during the current execution (e.g., `-f markdown`) override the loaded persistent state.
3. **Save Phase:** Upon successful confirmation in the TUI, the active `ContextConfig` is serialized and saved back to the disk.

*Note: Headless CLI executions (where flags like `--stdout` or `--output` are used) bypass the TUI and do not overwrite the persistent state, ensuring automated scripts do not pollute your interactive preferences.*

## Smart Ignore Heuristics

The engine employs a multi-layered approach to filter out irrelevant data via the `NoiseDetector` module. This is more aggressive than a standard `.gitignore` parser.

### 1. Standard Exclusions
Common development artifacts are hard-blocked to save IO time:
- **Version Control:** `.git`, `.svn`, `.hg`
- **Dependency Caches:** `node_modules`, `vendor`, `.venv`, `bin`, `obj`
- **Build Outputs:** `target`, `dist`, `build`, `out`

### 2. Heavy Artifact Detection
Files are evaluated based on extension and size limits. Even if a file isn't explicitly ignored, it will be flagged as noise if it exceeds safety limits:
- **Source code:** Max 50 MB (prevents reading massive minified JS files).
- **Data files (XML/JSON/CSV):** Max 250 MB.
- **Binary Office files:** Max 1 GB (handles large documentation).
- **Pure Binaries:** `.exe`, `.dll`, `.png`, `.mp4`, `.zip` are hard-blocked to prevent token wastage.
