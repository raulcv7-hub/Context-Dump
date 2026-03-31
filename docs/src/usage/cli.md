# Headless Mode (CLI)

The non-interactive mode is designed for speed and automation. It allows developers to integrate context extraction into shell scripts, CI/CD pipelines, or quick terminal workflows.

## Basic Execution

To scan the current directory and generate an XML report:

```bash
context .
```

By default, this will scan the folder, apply smart ignore heuristics, extract the content, and output the result to standard output (or prompt the TUI if no specific flags indicate automation).

## Core Arguments

| Flag | Name | Description |
| :--- | :--- | :--- |
| `-o`, `--output <FILE>` | Output Path | Writes the final report to the specified file. |
| `-f`, `--format <FMT>` | Format | Forces serialization format (`xml` or `markdown`). |
| `-s`, `--stdout` | Standard Output | Forces the report to the terminal stdout stream. |
| `-c`, `--clip` | Clipboard | Copies the final report directly to the OS clipboard. |
| `-I`, `--interactive` | Force TUI | Forces the Terminal User Interface to launch. |

## Filtering Context

You can heavily restrict what is included in the dump to preserve token limits:

### Extension Whitelisting
Only include specific file types:
```bash
context . -e rs,md,toml
```

### Path Exclusion
Exclude specific directories or files containing a substring:
```bash
context . -X tests,migrations,vendor
```

### Depth Limiting
Prevent the scanner from diving too deeply into the file tree:
```bash
context . --depth 2
```

## Example: Quick PR Review Context
To copy the source code of a specific module to your clipboard for an LLM review, formatted as Markdown:

```bash
context src/api -c -f markdown -e rs
```