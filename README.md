# bnto

[![CI](https://github.com/Develonaut/bnto/actions/workflows/ci.yml/badge.svg)](https://github.com/Develonaut/bnto/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-WASM-black?logo=rust)](https://www.rust-lang.org/)
[![Next.js](https://img.shields.io/badge/Next.js-16-black?logo=next.js)](https://nextjs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5-blue?logo=typescript&logoColor=white)](https://www.typescriptlang.org/)

**Free tools that run in your browser. Your files never leave your machine.**

bnto (like "bento") is a collection of instant, composable tools powered by Rust compiled to WebAssembly. Compress images, clean CSVs, rename files, convert formats, or build your own multi-step recipes with a visual editor. No uploads, no accounts, no limits.

> **Try it now:** [bnto.io](https://bnto.io)

---

## Why bnto?

|                          | bnto                       | TinyPNG           | Squoosh                 | CloudConvert       |
| ------------------------ | -------------------------- | ----------------- | ----------------------- | ------------------ |
| **Free**                 | Unlimited, forever         | 20 images/mo      | Unlimited (1 at a time) | Limited credits    |
| **Private**              | Files stay in your browser | Uploads to server | Browser-based           | Uploads to server  |
| **Batch processing**     | Unlimited files            | 20 max            | One at a time           | Per-conversion fee |
| **Composable**           | Chain tools into recipes   | Single tool       | Single tool             | Single tool        |
| **Visual recipe editor** | Yes                        | No                | No                      | No                 |
| **Offline**              | Yes, once loaded           | No                | Yes                     | No                 |
| **Open source**          | MIT licensed               | No                | Yes                     | No                 |

bnto replaces the five bookmarks small teams cobble together for everyday tasks. One place for compressing, converting, renaming, cleaning. Same interface, composable.

---

## Recipes

Recipes are what bnto calls its tools. Each one is a pipeline of nodes that process your files. Predefined recipes work out of the box. The visual editor lets you build your own.

<!-- BEGIN AUTO-GENERATED RECIPES TABLE -->

| Recipe                  | What it does                                                                                                | Try it                                                                     |
| ----------------------- | ----------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| Compress Images         | Compress PNG, JPEG, and WebP images instantly in your browser. No upload limits, no signup.                 | [bnto.io/compress-images](https://bnto.io/compress-images)                 |
| Resize Images           | Resize images to exact dimensions or percentages. Free, no signup required.                                 | [bnto.io/resize-images](https://bnto.io/resize-images)                     |
| Convert Image Format    | Convert between PNG, JPEG, WebP, and GIF formats instantly. Free, no signup.                                | [bnto.io/convert-image-format](https://bnto.io/convert-image-format)       |
| Rename Files            | Batch rename files with patterns. Free, no signup required.                                                 | [bnto.io/rename-files](https://bnto.io/rename-files)                       |
| Clean CSV               | Remove empty rows, trim whitespace, deduplicate CSV data. Free, no signup.                                  | [bnto.io/clean-csv](https://bnto.io/clean-csv)                             |
| Rename CSV Columns      | Rename CSV column headers in bulk. Free, no signup required.                                                | [bnto.io/rename-csv-columns](https://bnto.io/rename-csv-columns)           |
| CSV to JSON             | Convert CSV files to JSON format with configurable delimiters. Free, no signup.                             | [bnto.io/csv-to-json](https://bnto.io/csv-to-json)                         |
| Optimize Images for Web | Resize, convert to WebP, and compress images for fast web loading. Free, no signup.                         | [bnto.io/optimize-images-for-web](https://bnto.io/optimize-images-for-web) |
| Generate Thumbnails     | Resize images to thumbnail size, convert to WebP, and add a prefix. Free, no signup.                        | [bnto.io/generate-thumbnails](https://bnto.io/generate-thumbnails)         |
| Compress & Rename       | Compress images and add a suffix so originals and compressed versions are distinguishable. Free, no signup. | [bnto.io/compress-and-rename](https://bnto.io/compress-and-rename)         |
| Standardize CSV         | Clean up messy CSV data and rename column headers in one step. Free, no signup.                             | [bnto.io/standardize-csv](https://bnto.io/standardize-csv)                 |
| Strip EXIF              | Remove EXIF metadata from images instantly in your browser. No upload limits, no signup.                    | [bnto.io/strip-exif](https://bnto.io/strip-exif)                           |

<!-- END AUTO-GENERATED RECIPES TABLE -->

### Visual Recipe Editor

Don't see what you need? Build it. The visual editor lets you compose nodes into multi-step recipes — chain a resize into a format conversion into a compression, all in one run. Save your recipes and reuse them.

Open the editor at [bnto.io/editor](https://bnto.io/editor).

---

## How It Works

Pick a recipe, drop your files, get results. Processing happens in your browser using a Rust engine compiled to WebAssembly — no server round-trips.

```
You drop files
  → Browser loads Rust/WASM engine in a Web Worker
  → Files are processed entirely in-browser
  → You download the results
```

Your files never touch a server. The browser IS the server.

---

## The Bento Box

bnto is named after the bento box (弁当) — a Japanese lunch container where each compartment holds one thing, serves one purpose, and fits together into a complete meal.

That idea runs through everything:

- **Nodes** are compartments — each does one thing well (compress, resize, rename)
- **Recipes** are boxes — portable, organized, complete
- **Boxes compose** — a recipe can contain other recipes. One node or twenty, the mental model never changes

---

## Where This Is Going

bnto starts with file tools in the browser, but the engine is designed to grow.

**Today:** 10 predefined recipes running client-side via Rust/WASM. A visual editor for building custom recipes. Free, unlimited, no account required.

**Next:** A desktop app (Tauri) with full local execution — including nodes the browser can't run. Shell commands (`ffmpeg`, `yt-dlp`, `imagemagick`), filesystem operations, and BYOK AI. Same recipes, same engine, running natively.

**Later:** Cloud execution for server-side nodes (AI inference, video processing, unrestricted HTTP). Pro tier for persistence, collaboration, and premium compute.

One Rust engine powering browser (WASM), desktop (native), CLI, and cloud. Recipes are portable `.bnto.json` files that work everywhere.

---

## For Developers

### Prerequisites

- Node.js 18+
- [pnpm](https://pnpm.io)
- [Task](https://taskfile.dev)
- Rust toolchain (for engine work — install via [rustup](https://rustup.rs))

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

# Rust WASM engine
task wasm:build         # Build WASM crates (release, web target)
task wasm:test          # Run Rust unit + WASM integration tests
task wasm:lint          # Run clippy
task wasm:fmt           # Format Rust code

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

bnto follows the **Bento Box Principle** — every file, function, and package does one thing well.

The browser is the execution engine. Rust compiles to a single WASM binary that runs inside a Web Worker. The web app loads the WASM module, sends files in, and gets results back.

```
Browser (WASM execution)
  │
  ├── Web Worker loads bnto_wasm.wasm
  │     ├── bnto-image (compress, resize, convert)
  │     ├── bnto-csv (clean, rename columns)
  │     └── bnto-file (rename)
  │
  ├── Next.js app (Vercel)
  └── Convex Cloud (auth, data persistence)
```

`@bnto/core` is the transport-agnostic API layer. UI components never call backend services directly — they use core hooks that swap backends depending on runtime (browser, desktop, or cloud).

### Repository Structure

```
bnto/
├── apps/
│   └── web/                     # Next.js on Vercel (bnto.io)
├── packages/
│   ├── core/                    # @bnto/core — Transport-agnostic API
│   ├── ui/                      # @bnto/ui — Motorway design system
│   ├── editor/                  # @bnto/editor — Visual recipe editor
│   └── @bnto/
│       ├── auth/                # @bnto/auth — Cloud auth
│       ├── backend/             # @bnto/backend — Convex schema + functions
│       ├── form/                # @bnto/form — Schema-driven config forms
│       ├── nodes/               # @bnto/nodes — Engine-generated node catalog
│       └── registry/            # @bnto/registry — Recipe curation + discovery
└── engine/                      # Rust WASM engine
    └── crates/
        ├── bnto-core/           # Core types, traits, progress reporting
        ├── bnto-engine/         # Pipeline executor + node registry
        ├── bnto-image/          # Image compression/resize/convert
        ├── bnto-csv/            # CSV clean/rename columns
        ├── bnto-file/           # File rename
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

Follow the existing code patterns and the [Bento Box Principle](.claude/rules/code-standards.md) — small, focused files and functions with clear boundaries.

---

## Support

If bnto saves you time, consider buying me a coffee.

<a href="https://buymeacoffee.com/develonaut"><img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me a Coffee" height="60"></a>

## License

[MIT](LICENSE) — Copyright 2024-2026 [Develonaut](https://github.com/Develonaut)
