# Interactive Mode (TUI)

The Terminal User Interface (TUI) provides a deterministic, visual way to explore a project's architecture and selectively include files into the context dump.

## Initialization

Running the application without explicit filtering flags or output targets launches the TUI:

```bash
context
```

## Interface Layout

The interface is divided into three distinct panels:
1. **Project Explorer (Top):** A hierarchical view of the scanned file system. Hidden or ignored files are displayed in dark gray and deselected by default.
2. **Context Summary (Middle):** Real-time statistics showing the total selected files, total token budget, target output destination, and a language distribution breakdown.
3. **Controls (Bottom):** A reference bar for keyboard shortcuts.

## Navigation and Controls

| Action | Keybinding | Description |
| :--- | :--- | :--- |
| **Move Cursor** | `Up` / `Down` | Navigates vertically through the visible list. |
| **Expand/Collapse** | `Right` / `Left` | Opens or closes the currently selected directory. |
| **Toggle Selection** | `Space` | Selects or deselects the highlighted file. If a directory is highlighted, it recursively toggles all its children. |
| **Cycle Destination** | `o` | Toggles the output target between `TERMINAL`, `FILE`, and `SYSTEM CLIPBOARD`. |
| **Cycle Format** | `f` | Toggles the output structure between `XML` and `MARKDOWN`. |
| **Execute** | `Enter` | Confirms the selection, exits the TUI, and begins the extraction phase. |
| **Quit** | `q` or `Esc` | Aborts the application without processing. |

## Smart Selection

The TUI automatically pre-selects files based on the engine's noise detection heuristics. Build artifacts, lockfiles, and hidden directories (`.git`) are automatically excluded from the initial selection, ensuring that a rapid `Enter` press yields a clean, LLM-ready context block.
