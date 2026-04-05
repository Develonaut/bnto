# bnto

[![CI](https://github.com/Develonaut/bnto/actions/workflows/ci.yml/badge.svg)](https://github.com/Develonaut/bnto/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-WASM-black?logo=rust)](https://www.rust-lang.org/)
[![Next.js](https://img.shields.io/badge/Next.js-16-black?logo=next.js)](https://nextjs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5-blue?logo=typescript&logoColor=white)](https://www.typescriptlang.org/)

**Workflow automation through composable parts.** Build a node for anything — image processing, data transforms, API calls, video downloads, shell commands — then chain nodes into recipes that automate your workflow. One Rust engine that compiles to native binaries, WebAssembly, and (soon) desktop and server. Write a recipe once, run it anywhere.

```bash
# Run a predefined recipe
bnto run compress-images *.jpg

# Chain nodes into multi-step workflows
bnto run optimize-images-for-web photos/

# Any workflow you can describe, bnto can run
bnto run my-custom-pipeline data/
```

Recipes are portable `.bnto.json` files — composable pipelines that run everywhere: CLI, browser, desktop, server.

> **Also runs in your browser:** [bnto.io](https://bnto.io) — same engine, compiled to WebAssembly. Files never leave your machine.

---

## Recipes

A recipe is a pipeline of nodes. Each node does one thing — compress, resize, rename, convert, download, call an API — and you chain them together into a workflow. Need something that doesn't exist? Build a node for it, and the engine takes care of running it everywhere.

15 predefined recipes work out of the box:

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

### Build Your Own

Don't see what you need? Build it. Compose nodes into custom recipes using the visual editor or write `.bnto.json` by hand. Any node you create automatically works on every target the engine supports — browser, CLI, desktop, server.

Open the editor at [bnto.io/editor](https://bnto.io/editor).

---

## How It Works

### CLI

```bash
# Install
cargo install bnto-cli

# Compress all JPEGs in a directory
bnto run compress-images photos/*.jpg

# Download a video
bnto run download-video https://www.youtube.com/watch?v=dQw4w9WgXcQ

# Browse available recipes
bnto list

# Show recipe details and dependencies
bnto info download-video

# Check external tool availability
bnto doctor
```

The CLI runs recipes using the native Rust engine — direct native execution with full system access. Progress bars, colored output, and timing summaries keep you informed.

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

A recipe is a portable JSON file that describes a pipeline of nodes:

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

The same `.bnto.json` runs in the CLI, browser, desktop, and server. Nodes are the building blocks — each one encapsulates a single capability. Recipes compose them into workflows. The engine handles execution, progress reporting, and error handling across every target.

---

## The Bento Box

bnto is named after the bento box — a Japanese lunch container where each compartment holds one thing, serves one purpose, and fits together into a complete meal.

That idea is the architecture:

- **Nodes** are compartments. Each encapsulates one capability — compress an image, call an API, run a shell command, download a video. Build a node for anything you want to automate.
- **Recipes** are boxes. Compose nodes into multi-step workflows. Portable, shareable, version-controlled `.bnto.json` files.
- **The engine** is the tray. One Rust codebase compiles to every target: native CLI binary, WebAssembly for the browser, native desktop app, server-side execution. A node you write today runs everywhere the engine runs — without changes.
- **Boxes compose.** A recipe can contain other recipes. One node or twenty, the mental model never changes.

---

## Where This Is Going

**Today:** 15 predefined recipes running via CLI and browser. A visual editor for building custom recipes. Video download (yt-dlp), image processing, CSV transforms, file operations — all from one tool. Free, unlimited, no account required.

**Next:** New node types (shell commands, HTTP requests), TUI for interactive recipe execution, more video operations. The node catalog grows; every addition is automatically available on every target.

**Later:** Desktop app (Tauri) for native local execution. Cloud execution for server-side nodes (AI inference, video processing). Community-contributed node types and recipes.

**The vision:** Any workflow you can describe as a sequence of steps, bnto can automate. Build the node, compose the recipe, run it anywhere. One Rust engine powering CLI, browser, desktop, and cloud — with recipes as portable `.bnto.json` files that work on every target.

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

The Rust engine is the core — a single codebase that compiles to every execution target. Write a node once, and it runs everywhere the engine runs.

```
Rust Engine (single codebase, multiple targets)
  │
  ├── Native CLI (bnto run, bnto list, bnto doctor)
  ├── WASM (browser, via Web Worker — same crates, same code)
  ├── Desktop (Tauri, planned — native Rust, no WASM overhead)
  └── Server (cloud execution, planned — same engine, managed infra)

Node Crates (each domain gets its own crate)
  ├── bnto-image (compress, resize, convert, overlay, strip-exif)
  ├── bnto-csv (clean, rename columns, merge, csv-to-json)
  ├── bnto-file (rename)
  └── bnto-video (download via yt-dlp, native-only)
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
        ├── bnto-video/          # Video download (yt-dlp, native-only)
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
