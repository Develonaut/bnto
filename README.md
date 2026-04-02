# bnto

[![CI](https://github.com/Develonaut/bnto/actions/workflows/ci.yml/badge.svg)](https://github.com/Develonaut/bnto/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-WASM-black?logo=rust)](https://www.rust-lang.org/)
[![Next.js](https://img.shields.io/badge/Next.js-16-black?logo=next.js)](https://nextjs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5-blue?logo=typescript&logoColor=white)](https://www.typescriptlang.org/)

**A composable pipeline tool for developers.** Compress images, clean CSVs, rename files, download videos, call APIs — from your terminal or browser. Powered by a Rust engine that compiles to native binaries and WebAssembly.

```bash
# Compress all JPEGs in the current directory
bnto run compress-images *.jpg

# Clean up a messy CSV
bnto run clean-csv data.csv

# Chain tools into multi-step recipes
bnto run optimize-images-for-web photos/
```

Recipes are portable `.bnto.json` files that run everywhere — CLI, browser, desktop.

> **Also runs in your browser:** [bnto.io](https://bnto.io) — same engine, compiled to WebAssembly. Files never leave your machine.

---

## Recipes

Recipes are what bnto calls its tools. Each one is a pipeline of nodes that process your files. Predefined recipes work out of the box.

<!-- BEGIN AUTO-GENERATED RECIPES TABLE -->

| Recipe                  | What it does                                                                                                                                 | Try it                                                                     |
| ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| Compress Images         | Compress PNG, JPEG, and WebP images instantly in your browser. No upload limits, no signup.                                                  | [bnto.io/compress-images](https://bnto.io/compress-images)                 |
| Resize Images           | Resize images to exact dimensions or percentages. Free, no signup required.                                                                  | [bnto.io/resize-images](https://bnto.io/resize-images)                     |
| Convert Image Format    | Convert between PNG, JPEG, WebP, and GIF formats instantly. Free, no signup.                                                                 | [bnto.io/convert-image-format](https://bnto.io/convert-image-format)       |
| Rename Files            | Batch rename files with patterns. Free, no signup required.                                                                                  | [bnto.io/rename-files](https://bnto.io/rename-files)                       |
| Clean CSV               | Remove empty rows, trim whitespace, deduplicate CSV data. Free, no signup.                                                                   | [bnto.io/clean-csv](https://bnto.io/clean-csv)                             |
| Rename CSV Columns      | Rename CSV column headers in bulk. Free, no signup required.                                                                                 | [bnto.io/rename-csv-columns](https://bnto.io/rename-csv-columns)           |
| CSV to JSON             | Convert CSV files to JSON format with configurable delimiters. Free, no signup.                                                              | [bnto.io/csv-to-json](https://bnto.io/csv-to-json)                         |
| Merge CSV               | Combine multiple CSV files into one with header reconciliation. Free, no signup.                                                             | [bnto.io/merge-csv](https://bnto.io/merge-csv)                             |
| Optimize Images for Web | Resize, convert to WebP, and compress images for fast web loading. Free, no signup.                                                          | [bnto.io/optimize-images-for-web](https://bnto.io/optimize-images-for-web) |
| Generate Thumbnails     | Resize images to thumbnail size, convert to WebP, and add a prefix. Free, no signup.                                                         | [bnto.io/generate-thumbnails](https://bnto.io/generate-thumbnails)         |
| Compress & Rename       | Compress images and add a suffix so originals and compressed versions are distinguishable. Free, no signup.                                  | [bnto.io/compress-and-rename](https://bnto.io/compress-and-rename)         |
| Standardize CSV         | Clean up messy CSV data and rename column headers in one step. Free, no signup.                                                              | [bnto.io/standardize-csv](https://bnto.io/standardize-csv)                 |
| Strip EXIF              | Remove EXIF metadata from images instantly in your browser. No upload limits, no signup.                                                     | [bnto.io/strip-exif](https://bnto.io/strip-exif)                           |
| Watermark Images        | Add a logo or watermark to images. Position, size, and opacity are fully configurable. Runs in your browser. Files never leave your machine. | [bnto.io/watermark-images](https://bnto.io/watermark-images)               |

<!-- END AUTO-GENERATED RECIPES TABLE -->

### Visual Recipe Editor

Don't see what you need? Build it. The visual editor lets you compose nodes into multi-step recipes. Chain a resize into a format conversion into a compression, all in one run.

Open the editor at [bnto.io/editor](https://bnto.io/editor).

---

## How It Works

### CLI

```bash
# Install
cargo install bnto

# Run a recipe
bnto run compress-images photos/*.jpg

# List available recipes
bnto list

# Check recipe details
bnto info compress-images
```

The CLI runs recipes using the native Rust engine. No WASM, no browser — direct native execution with full system access.

### Browser

Pick a recipe at [bnto.io](https://bnto.io), drop your files, get results. Processing happens entirely in your browser using the same Rust engine compiled to WebAssembly. No server round-trips.

```
You drop files
  → Browser loads Rust/WASM engine in a Web Worker
  → Files are processed entirely in-browser
  → You download the results
```

Your files never touch a server. The browser IS the server.

### The `.bnto.json` Format

Recipes are portable JSON files that describe a pipeline:

```json
{
  "formatVersion": "1",
  "nodes": [
    {
      "id": "input",
      "type": "input",
      "parameters": { "mode": "file-upload", "accept": ["image/jpeg", "image/png"] }
    },
    { "id": "compress", "type": "image-compress", "parameters": { "quality": 80 } },
    { "id": "output", "type": "output", "parameters": { "mode": "download" } }
  ],
  "settings": { "iteration": "auto" }
}
```

The same `.bnto.json` runs in the CLI, browser, and (future) desktop. Recipes are composable — chain nodes into multi-step pipelines.

---

## The Bento Box

bnto is named after the bento box, a Japanese lunch container where each compartment holds one thing, serves one purpose, and fits together into a complete meal.

That idea runs through everything:

- **Nodes** are compartments. Each does one thing well (compress, resize, rename)
- **Recipes** are boxes. Portable, organized, complete
- **Boxes compose.** A recipe can contain other recipes. One node or twenty, the mental model never changes

---

## Where This Is Going

**Today:** 14 predefined recipes running via CLI and browser. A visual editor for building custom recipes. Free, unlimited, no account required.

**Next:** Engine expansion — dependency system for external tools (yt-dlp, ffmpeg), video download node type, TUI for interactive recipe execution, and CLI polish. The CLI becomes the primary development surface.

**Later:** Desktop app (Tauri) for native local execution. Cloud execution for server-side nodes (AI inference, video processing). Pro tier for persistence and collaboration.

One Rust engine powering CLI, browser (WASM), desktop (native), and cloud. Recipes are portable `.bnto.json` files that work everywhere.

---

## For Developers

### Prerequisites

- Node.js 18+
- [pnpm](https://pnpm.io)
- [Task](https://taskfile.dev)
- Rust toolchain (for engine work, install via [rustup](https://rustup.rs))

### Setup

```bash
git clone https://github.com/Develonaut/bnto.git
cd bnto
pnpm install
```

### Commands

```bash
# Development
task dev                # Start web + Convex dev servers (Next.js on port 4000)

# Rust engine
task wasm:build         # Build WASM crates (release, web target)
task wasm:test          # Run Rust unit + WASM integration tests
task wasm:lint          # Run clippy
task wasm:fmt           # Format Rust code

# Native CLI
task cli:build          # Build native CLI binary
task cli:test           # Run CLI unit + integration + golden tests

# Frontend
task ui:build           # Build all TS packages
task ui:test            # Run TS tests
task ui:lint            # Lint all TS packages

# E2E tests (requires task dev running)
task e2e                # Run Playwright E2E tests

# Quality gate
task check              # Full check (lint + test + build)
```

### Architecture

bnto follows the **Bento Box Principle**: every file, function, and package does one thing well.

The Rust engine is the core. It compiles to a native CLI binary for local execution and to a WASM binary for browser execution.

```
Rust Engine
  │
  ├── Native CLI (bnto run, bnto list, bnto doctor)
  │     ├── bnto-image (compress, resize, convert, overlay)
  │     ├── bnto-csv (clean, rename columns, merge, csv-to-json)
  │     └── bnto-file (rename, strip-exif)
  │
  ├── WASM (browser, via Web Worker)
  │     └── Same crates, compiled to wasm32-unknown-unknown
  │
  ├── Next.js app (bnto.io, Vercel)
  └── Convex Cloud (auth, data persistence)
```

`@bnto/core` is the transport-agnostic API layer. UI components never call backend services directly. They use core hooks that swap backends depending on runtime (browser, desktop, or cloud).

### Repository Structure

```
bnto/
├── apps/
│   └── web/                     # Next.js on Vercel (bnto.io)
├── packages/
│   ├── core/                    # @bnto/core - Transport-agnostic API
│   ├── ui/                      # @bnto/ui - Motorway design system
│   ├── editor/                  # @bnto/editor - Visual recipe editor
│   └── @bnto/
│       ├── auth/                # @bnto/auth - Cloud auth
│       ├── backend/             # @bnto/backend - Convex schema + functions
│       ├── form/                # @bnto/form - Schema-driven config forms
│       ├── nodes/               # @bnto/nodes - Engine-generated node catalog
│       └── registry/            # @bnto/registry - Recipe curation + discovery
└── engine/                      # Rust engine (WASM + native CLI)
    └── crates/
        ├── bnto-core/           # Core types, traits, progress reporting
        ├── bnto-engine/         # Pipeline executor + node registry
        ├── bnto-image/          # Image compression/resize/convert/overlay
        ├── bnto-csv/            # CSV clean/rename columns/merge/convert
        ├── bnto-file/           # File rename, EXIF strip
        ├── bnto-cli/            # Native CLI binary
        └── bnto-wasm/           # cdylib entry point (single WASM binary)
```

---

## Contributing

Contributions are welcome. To get started:

1. Fork and clone the repository
2. Install prerequisites (Node.js 18+, pnpm, Task, Rust)
3. Run `pnpm install` to set up dependencies
4. Run `task dev` to start the development servers
5. Run `task check` before submitting a pull request

Follow the existing code patterns and the [Bento Box Principle](.claude/rules/code-standards.md): small, focused files and functions with clear boundaries.

---

## Support

If bnto saves you time, consider buying me a coffee.

<a href="https://buymeacoffee.com/develonaut"><img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me a Coffee" height="60"></a>

## License

[MIT](LICENSE) - Copyright 2024-2026 [Develonaut](https://github.com/Develonaut)
