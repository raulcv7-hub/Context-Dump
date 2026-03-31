# Introduction

**Context-Dump** is a high-performance, security-first native Rust engine designed to aggregate source code and technical documentation into a unified, token-optimized context format. It acts as a secure bridge between complex, multi-format file systems and Large Language Models (LLMs).

## The Core Problem

Software projects are increasingly complex, containing not just code, but also sensitive configuration, minified third-party assets, and deeply nested archives. Feeding this data to an LLM manually is risky and inefficient.

## The Solution

Context-Dump provides a highly concurrent binary that:
- **Secures Data:** Automatically detects and masks PII (Emails, IPs, Credit Cards) and Secrets (AWS, JWT, Stripe).
- **Detects Noise:** Flags suspicious, minified, or obfuscated code that consumes tokens without providing value.
- **Tracks Origin:** Injects "Provenance" metadata (Source URL and Commit Hash) for remote repositories.
- **Respects Privacy:** Maps `.gitignore` to its internal selection state while maintaining user control.
- **Native Extraction:** Processes PDFs, Office docs, and ZIP/TAR archives without external runtimes.

## Design Philosophy: Absolute Portability & RAII

The project follows a **"Tank" architecture**: a single static binary with no dependencies. It uses strict **RAII (Resource Acquisition Is Initialization)** patterns to guarantee that temporary data from remote clones is wiped from the disk even if the process crashes.