# Bnto — Free Online File Tools Powered by Rust & WebAssembly

[![CI](https://github.com/Develonaut/bnto/actions/workflows/ci.yml/badge.svg)](https://github.com/Develonaut/bnto/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-WASM-black?logo=rust)](https://www.rust-lang.org/)
[![Next.js](https://img.shields.io/badge/Next.js-16-black?logo=next.js)](https://nextjs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5-blue?logo=typescript&logoColor=white)](https://www.typescriptlang.org/)

**Compress images, clean CSVs, rename files, and convert formats — online, free, no sign-up.**

Everything runs client-side via Rust compiled to WebAssembly. Your files never leave your browser. No uploads, no accounts, no limits.

> **Try it now:** [bnto.io](https://bnto.io)

## Tools

| Tool | What it does |
|------|-------------|
| [Compress Images](https://bnto.io/compress-images) | Shrink JPEG, PNG, and WebP images without uploading to a server |
| [Resize Images](https://bnto.io/resize-images) | Resize to exact dimensions or percentages |
| [Convert Image Format](https://bnto.io/convert-image-format) | Convert between PNG, JPEG, WebP, and GIF |
| [Clean CSV](https://bnto.io/clean-csv) | Remove empty rows, trim whitespace, deduplicate |
| [Rename CSV Columns](https://bnto.io/rename-csv-columns) | Rename column headers in bulk |
| [Rename Files](https://bnto.io/rename-files) | Batch rename files with patterns |

## Why Bnto?

- **Private:** Files never leave your browser. Processing happens 100% client-side via Rust compiled to WebAssembly. No server uploads, ever
- **Free:** All browser-based tools are free with no limits. No signup required
- **Fast:** Near-native performance from compiled Rust, not interpreted JavaScript
- **Offline-capable:** Once loaded, tools work without an internet connection
- **Open Source:** MIT licensed. The entire engine and web app are right here

## How It Works

Pick a tool, drop your files, get results. Processing happens instantly in your browser using a Rust engine compiled to WebAssembly — no server round-trips, no waiting.

```
You drop files
  → Browser loads Rust/WASM engine in a Web Worker
  → Files are processed entirely in-browser
  → You download the results
```

Your files never touch a server. The browser IS the server.

## For Developers

### Prerequisites

- Node.js 18+
- [pnpm](https://pnpm.io)
- [Task](https://taskfile.dev)
- Rust toolchain (for engine work only -- install via [rustup](https://rustup.rs))

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
task check              # Full check (vet + test + build)
```

## Architecture

Bnto follows the **Bento Box Principle** -- every file, function, and package does one thing well.

The browser is the execution engine. Rust compiles to a single WASM binary that runs inside a Web Worker. The web app loads the WASM module, sends files in, and gets results back. No server round-trips for processing.

```
Browser (WASM execution)
  |
  +-- Web Worker loads bnto_wasm.wasm
  |     |-- bnto-image (compress, resize, convert)
  |     |-- bnto-csv (clean, rename columns)
  |     +-- bnto-file (rename)
  |
  +-- Next.js app (Vercel)
  +-- Convex Cloud (auth, data persistence)
```

`@bnto/core` is the transport-agnostic API layer. UI components never call backend services directly -- they use core hooks that route requests to the right backend depending on runtime.

### Repository Structure

```
bnto/
├── apps/
│   ├── web/                     # Next.js on Vercel (bnto.io)
│   │   └── components/          # UI components (co-located)
│   └── desktop/                 # Tauri frontend (planned)
├── packages/
│   ├── core/                    # @bnto/core -- Transport-agnostic API
│   └── @bnto/
│       ├── auth/                # @bnto/auth -- Cloud auth
│       ├── backend/             # @bnto/backend -- Convex schema + functions
│       └── nodes/               # @bnto/nodes -- Engine-agnostic node definitions
├── engine/                      # Rust WASM engine (browser execution)
│   └── crates/
│       ├── bnto-core/           # Core types, traits, progress reporting
│       ├── bnto-image/          # Image compression/resize/convert
│       ├── bnto-csv/            # CSV clean/rename columns
│       ├── bnto-file/           # File rename
│       └── bnto-wasm/           # cdylib entry point (single WASM binary)
└── archive/                     # Preserved reference code (not active)
    ├── engine-go/               # Go CLI + engine (~33K LOC)
    └── api-go/                  # Go HTTP API server (~2.5K LOC)
```

## Contributing

Contributions are welcome. To get started:

1. Fork and clone the repository
2. Install prerequisites (Node.js 18+, pnpm, Task, Rust)
3. Run `pnpm install` to set up dependencies
4. Run `task dev` to start the development servers
5. Run `task check` before submitting a pull request

Please follow the existing code patterns and the [Bento Box Principle](.claude/rules/code-standards.md) -- small, focused files and functions with clear boundaries.

## License

[MIT](LICENSE) -- Copyright 2024-2026 [Develonaut](https://github.com/Develonaut)
