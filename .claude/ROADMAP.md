# Bnto — Strategic Roadmap

**Last Updated:** April 5, 2026
**Purpose:** High-level strategy, milestones, and big decisions. PLAN.md tracks sprint tasks. This tracks the "why" and "where we're going."

---

## Vision

Bnto is workflow automation through composable parts. Each node encapsulates a single capability — compress an image, call an API, run a shell command, download a video. Chain nodes into recipes. Run them anywhere.

**The CLI is the product.** `cargo install bnto` gets you started. Define recipes as `.bnto.json` files, run them with `bnto run`. The power is in composition: any workflow you can describe as a sequence of steps, bnto can automate. One Rust engine compiles to every target — your terminal today, your browser, your desktop, and a server tomorrow. The recipe doesn't care where it runs.

**The architecture makes this possible:** One Rust engine, multiple compilation targets. Write a node once. The engine handles execution, progress, error handling, and platform differences. CLI nodes get full system access. Browser nodes compile to WASM. Server nodes get managed infrastructure.

**Execution targets:**

| Target                  | Status                 | Cost to Us  | Cost to User       |
| ----------------------- | ---------------------- | ----------- | ------------------ |
| **CLI** (Rust native)   | **Primary (now)**      | $0          | Free forever (OSS) |
| **TUI** (ratatui)       | Next (Sprint 10)       | $0          | Free forever (OSS) |
| **Browser** (Rust→WASM) | Delivered, maintenance | $0          | Free forever       |
| **Desktop** (Tauri)     | Backlog (M4)           | $0          | Free forever       |
| **Cloud** (server-side) | Backlog (M4)           | ~$5/mo base | Pro tier           |

**The insight:** The engine is the stable API. Nodes are the building blocks. Recipes are the workflows. Targets are just compilation modes. Revenue strategy is tabled — focus is on making the engine powerful and fun.

**The web:** bnto.io is a landing page that directs to `cargo install bnto`. SEO recipe pages showcase what bnto can do. Browser execution (Rust→WASM) is delivered and maintained but is not the primary experience — the CLI is.

---

## Milestone Map

```
M1: Browser Execution (MVP)          ← DELIVERED (Feb 2026)
    All 6 Tier 1 bntos running in-browser via Rust→WASM.
    Zero backend for core experience. "Your files never leave your computer."

    Rust evaluation: PASSED. All 6 nodes built in Rust. Development
    velocity good, WASM boundary clean, ecosystem sufficient.
    Bundle: 1.6MB raw / 606KB gzipped (single cdylib, all 6 nodes).

    Cloud execution pipeline (Sprints 1-2A) also COMPLETE — M4
    infrastructure delivered ahead of schedule.

    Tier 1B multi-node compositions (March 2026): optimize-images-for-web,
    generate-thumbnails — first multi-node predefined recipes.

M2: Platform Features                ← DELIVERED (March 2026)
    Web platform features shipped: save workflows, execution history,
    accounts, recipe editor v1. Then PIVOTED: auth stripped, editor
    frozen as lightweight open+export tool, web reduced to landing page.
    Community recipes via GitHub PRs, curated by maintainer.

M3: Engine Expansion + CLI           ← ACTIVE (April 2026)
    CLI is the product. Dependency system DELIVERED. Video node DELIVERED.
    ProcessContext DELIVERED. CLI commands DELIVERED. TUI DELIVERED (6 screens,
    278 tests). Schema-driven config DELIVERED (Sprint 11). v0.5.0 shipped
    to crates.io.

    Next: Data persistence + Home + Library (Sprint 12A), then bnto-form
    crate (Sprint 11.5), then TUI recipe editors (Sprints 12-18).

M4: Distribution (backlog)
    Desktop app (Tauri, Rust-native). Server-side execution for premium
    recipes. Technology for cloud execution TBD.

M5: Monetization (tabled)
    Tabled. Focus is on making the engine powerful and fun.
    Revenue strategy revisited when the tool has community traction.
```

**Key:** M1 (browser) and M2 (platform) delivered. M3 active: CLI is the primary product surface, crates.io live, v0.5.0 shipped. TUI delivered (Sprint 10). Schema-driven config delivered (Sprint 11). Next: data persistence + Home + Library (Sprint 12A), then bnto-form (Sprint 11.5), then recipe editors (Sprints 12-18). Desktop (M4) and monetization (M5) are backlog.

---

## CLI-First Development

**A node is a universal capability.** Build it once in Rust, and the engine takes care of running it on every target. CLI nodes get full system access. Browser-capable nodes compile to WASM and work at bnto.io. Server nodes get managed infrastructure. The node author doesn't think about targets — the engine does.

**The CLI is the product.** `cargo install bnto` gives you 15 recipes out of the box. New node types are built and tested via `bnto run` — the CLI is both the development workflow and the primary user experience:

1. Build the processor in Rust (TDD-first, golden tests)
2. Test via `bnto run <recipe> [files...]`
3. Prove it works end-to-end in the CLI
4. The engine's `platforms` metadata determines where it surfaces — browser, desktop, server, or all of the above

**Extensibility is the point.** The 15 predefined recipes are a starting point. Anyone can add a node for any capability — image processing, data transforms, API calls, shell commands, video manipulation — and it automatically composes with every other node in the system. Recipes are just compositions of nodes. The engine handles execution, iteration, progress, and error handling.

**TUI is live.** `bnto tui` launches an interactive terminal UI (ratatui + crossterm) — recipe browser, file picker, progress display, results panel. Same engine, richer interface. Sprint 10 delivered 6 screens with 278 tests. Sprint 11 adds schema-driven parameter controls.

**Dependency system:** Node types can declare external dependencies (`yt-dlp`, `ffmpeg`, `imagemagick`). The engine checks them before pipeline execution. `bnto doctor` reports missing dependencies with install hints.

**ProcessContext:** A trait giving processors controlled system access (run commands, temp files, env vars). Pure WASM processors don't use it. CLI/desktop processors get a real implementation.

---

## Browser Execution: Tech Matrix (delivered, maintenance)

All Tier 1 bntos run 100% client-side via Rust→WASM. No server round-trip, no R2 file transit, no Railway. This work is complete — the browser is a secondary execution target behind the CLI.

| Bnto                 | Slug                    | Rust Crate(s)                     | WASM Strategy            | Notes                                                              |
| -------------------- | ----------------------- | --------------------------------- | ------------------------ | ------------------------------------------------------------------ |
| Compress Images      | `/compress-images`      | `image`, `mozjpeg-sys`, `oxipng`  | wasm-pack + wasm-bindgen | MozJPEG for JPEG, OxiPNG for PNG, WebP via `image`                 |
| Resize Images        | `/resize-images`        | `image` (resize module)           | wasm-pack + wasm-bindgen | Lanczos3/CatmullRom filters                                        |
| Convert Image Format | `/convert-image-format` | `image` (decode any → encode any) | wasm-pack + wasm-bindgen | JPEG, PNG, WebP, AVIF, GIF, BMP, TIFF                              |
| Clean CSV            | `/clean-csv`            | `csv` + `serde`                   | wasm-pack + wasm-bindgen | Rust `csv` crate is battle-tested                                  |
| Rename CSV Columns   | `/rename-csv-columns`   | `csv` + `serde`                   | wasm-pack + wasm-bindgen | Header rewrite, same engine as clean-csv                           |
| Rename Files         | `/rename-files`         | `bnto-file` (Rust `regex`)        | wasm-pack + wasm-bindgen | Pattern matching + regex rename — built in Rust for uniform engine |

**Why Rust WASM:** The bet paid off. All 6 nodes built in Rust, including `rename-files` (originally planned as JS — built in Rust for uniform engine). The same code will power desktop (Tauri), CLI, and cloud. One language, one codebase.

**CSV in Rust WASM:** Rust's `csv` crate handles structured operations well. The JS/WASM string boundary overhead is acceptable for our use case.

**Web Workers are mandatory.** All WASM processing runs off the main thread. Progress callbacks report back to the UI via `postMessage`.

### JS Libraries (reference, not needed for M1)

Rust succeeded — these are not needed for Tier 1. Kept as reference for Tier 2+ candidates where JS may be simpler:

| Bnto             | JS Library                            | Notes                                              |
| ---------------- | ------------------------------------- | -------------------------------------------------- |
| Image processing | jSquash (MozJPEG, OxiPNG, WebP, AVIF) | Discourse uses for 50MB+ images                    |
| CSV processing   | PapaParse                             | 1M rows in ~5s. Potential for very large CSV bntos |
| SVG optimization | Vexy SVGO (Rust→WASM, ironically)     | 12x faster than Node SVGO                          |

### Tier 2+ Browser Candidates

| Bnto                 | Approach                                     | Feasibility                                     |
| -------------------- | -------------------------------------------- | ----------------------------------------------- |
| Strip EXIF           | Rust `image` (metadata strip) or JS piexifjs | High — trivial with either                      |
| Optimize SVGs        | Rust (custom) or Vexy SVGO (Rust→WASM)       | High                                            |
| Convert CSV to JSON  | Rust `csv` + `serde_json`                    | High — trivial transform                        |
| Validate/Format JSON | Pure JS (JSON.parse + stringify)             | High — no Rust needed                           |
| Merge CSVs           | Rust `csv`                                   | High — concat + dedupe                          |
| PDF to Images        | pdf.js + Canvas (JS)                         | Medium — quality varies, Rust PDF libs immature |

---

## Bnto Classification

Every bnto falls into one of three execution categories:

### Browser-Only (free, unlimited)

Runs entirely in the user's browser. Files never leave the machine. No account needed.

- All Tier 1 bntos (compress, resize, convert, clean CSV, rename CSV columns, rename files)
- Most Tier 2 bntos (strip EXIF, optimize SVG, CSV to JSON, validate JSON, merge CSV)

### CLI/Desktop (free, unlimited)

Runs locally via the CLI or desktop app. Full system access. No browser limitations.

- All browser-capable recipes (same engine, native binary)
- External dependency recipes (video download via yt-dlp, shell commands via ffmpeg)
- Filesystem operations (directory traversal, batch rename with real paths)

### Server-Only (premium, future)

Requires server-side execution. These are the Pro tier differentiators.

- AI nodes — API keys shouldn't be exposed client-side; needs server proxy
- Video processing at scale — ffmpeg WASM exists but is impractically large
- Advanced PDF operations — server-side libraries are more capable

---

## Shared Node Registry — DELIVERED

`@bnto/nodes` is the engine-agnostic foundation — single source of truth for node definitions, schemas, recipes, and validation. **Built and shipping since Sprint 2B.** Zod parameter schemas, codegen from Rust engine catalog (`catalog.snapshot.json` → `generate-from-catalog.ts`). Node type and recipe counts are derived from the engine catalog — see test assertions in `nodeTypes.test.ts` and `recipesCatalog.test.ts` for current numbers.

```
packages/@bnto/nodes/
├── generated/        # Auto-generated from engine catalog (types, schemas, catalog)
├── schemas/          # Per-node-type Zod parameter schemas
├── recipes/          # Predefined bnto recipes (metadata + definition)
└── validators/       # Definition validation (works in browser, CLI, desktop)
```

**Consumed by:** `@bnto/core` (execution pipeline + registry), Rust WASM engine (browser), `@bnto/editor` (node CRUD, adapters — frozen), web app config UI (schema-driven forms). The CLI consumes the Rust engine directly — `@bnto/nodes` is the TypeScript mirror.

---

## Monetization Model

**Tabled.** Focus is on making the engine powerful and fun. Revenue strategy revisited when the tool has community traction.

The previous pricing model (browser free, server Pro) is preserved as a reference in [pricing-model.md](strategy/pricing-model.md). The core principle remains:

> **Nodes that can run locally are free. Nodes that need a managed server cost money.**

But monetization work is explicitly paused. No Stripe, no Pro tier, no feature gates until the engine has proven value and community adoption.

---

## Community Recipes

**Curated, not user-generated.** For MVP, recipes are maintained by the project maintainer and community contributors via GitHub.

| Source         | Mechanism                                                 | Quality control     |
| -------------- | --------------------------------------------------------- | ------------------- |
| **Predefined** | `@bnto/registry` recipes, auto-propagated to all surfaces | Maintainer-authored |
| **Community**  | GitHub PRs to `@bnto/registry/src/recipes/`               | PR review + CI gate |

**Why GitHub, not a platform:** Building recipe publishing, moderation, and discovery infrastructure is premature. GitHub PRs give us version control, review workflow, CI validation, and community contribution — all for free. Accepted recipes flow through the existing codegen pipeline and appear on every surface automatically.

---

## Architecture Decisions

| Decision                             | Status                  | Rationale                                                                                                                                       |
| ------------------------------------ | ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| **Rust WASM for browser nodes**      | Delivered (M1 complete) | All 6 Tier 1 nodes built in Rust, compiled to WASM. Unified engine vision proven. 606KB gzipped bundle.                                         |
| **JS adapters as fallback**          | Not needed              | Rust succeeded. JS libraries available for Tier 2+ if specific nodes warrant it.                                                                |
| **Go engine deleted**                | Archived (March 2026)   | Removed in Sprint 6. Rust is the unified engine for all targets. Source preserved in git history.                                               |
| **`@bnto/nodes` is engine-agnostic** | Approved                | Schemas, recipes, validation in TS. Survives any engine choice. The safety net.                                                                 |
| **CLI-first development**            | Decided (April 2026)    | New capabilities built and tested via CLI before browser/web. Engine is the product.                                                            |
| **Desktop deprioritized**            | Decided (April 2026)    | Tauri plan intact but deferred to M4. Engine expansion (M3) is more interesting and impactful.                                                  |
| **Monetization tabled**              | Decided (April 2026)    | No Stripe, no Pro tier until community traction. Focus on engine power and fun.                                                                 |
| **No-account browser execution**     | Approved                | Zero backend friction. Convex logs when accounts exist.                                                                                         |
| **Web Workers mandatory**            | Approved                | All WASM processing off main thread. Progress via postMessage.                                                                                  |
| **`@bnto/ui` extracted**             | Delivered (March 2026)  | Motorway design system as independent package. Primitives, layout, animation, surface system.                                                   |
| **`@bnto/editor` extracted**         | Delivered (March 2026)  | Headless-first editor package. ReactFlow canvas, schema-driven config, editor API layer.                                                        |
| **Smart Iteration**                  | Delivered (March 2026)  | `settings.iteration: "auto"\|"explicit"` on Definition. Auto wraps per-file processors in implicit loops. 20 golden tests prove equivalence.    |
| **Editor lightweight (open+export)** | Delivered (March 2026)  | Editor persistence stripped (8.5a). Reconnected as open+export tool with sessionStorage (8.5d). No save, no My Recipes. Deep features deferred. |
| **Schema-driven recipe config**      | Delivered (March 2026)  | DynamicRecipeConfig replaces handcoded per-recipe components. Adding a recipe = automatic config UI.                                            |
| **Image overlay/watermark**          | Delivered (April 2026)  | `image-overlay` operation in `bnto-image`. Text watermark with position/opacity/scale/color. 10+ golden tests.                                  |
| **v0.2.0 released**                  | Shipped (April 2026)    | 14 recipes, schema-driven config, editor reconnect, 4 Tier 3 operations.                                                                        |
| **`platforms` passthrough**          | Shipped (April 2026)    | Full `platforms: string[]` from engine catalog instead of lossy `browserCapable: boolean`. Enables correct CLI/server/browser filtering.        |
| **v0.5.0 released**                  | Shipped (April 2026)    | 15 recipes, video-download node, extra args pass-through, dependency system, H.264 codec preference, ProcessContext trait.                      |
| **TUI deferred to own sprint**       | Decided (April 2026)    | TUI is a full application (editor, navigation, recipe browser). Needs proper sprint breakdown. CLI polish comes first — make it bomb-proof.     |
| **CLI/TUI-first pivot**              | Decided (April 2026)    | CLI is the product. Web reduced to landing page. Editor frozen. Auth stripped. TUI (ratatui) is next UI surface. Frontend/premium work on hold. |
| **Open-source-first pivot**          | Decided (April 2026)    | Stripped pricing, auth surfaces, Pro references. Monetization tabled. Web → landing page for `cargo install bnto`.                              |

### Engine Decision: Rust Won (Feb 2026)

**The evaluation is complete.** All 6 MVP browser nodes were built in Rust, compiled to WASM. M1 was the Rust evaluation — and Rust passed.

**Evaluation results:**

| Question             | Result                                                                                      |
| -------------------- | ------------------------------------------------------------------------------------------- |
| Development velocity | **PASS** — Each node built faster as patterns emerged                                       |
| WASM boundary        | **PASS** — ArrayBuffer transfers clean, Web Worker wrapper solid                            |
| Bundle size          | **ACCEPTABLE** — 606KB gzipped for all 6 nodes (above 500KB target by ~20%, but reasonable) |
| Ecosystem coverage   | **PASS** — `image`, `csv`, `serde`, `regex` crates cover all Tier 1 needs                   |
| Developer experience | **PASS** — wasm-pack builds fast, errors debuggable, tooling solid                          |

**What this means going forward:**

- Rust is the engine for all targets (browser WASM, desktop native, CLI, cloud)
- Desktop (M4) uses Tauri (Rust-native) — one codebase, one language
- Go engine deleted (Sprint 6, March 2026) — source preserved in git history
- The unified engine vision is real: one Rust codebase powering every execution target

**The safety net remains: `@bnto/nodes` is engine-agnostic.** Node definitions, schemas, recipes, and validation rules live in TypeScript. They survive any future engine decisions.

**There is no timeline. This is for fun.** We're learning Rust, building something cool, and it's working.

---

## What Lives Where

| Document                                                         | Scope                                                                                                  |
| ---------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| **ROADMAP.md** (this file)                                       | Milestones, strategic direction, big decisions, "why"                                                  |
| **PLAN.md**                                                      | Sprint tasks, waves, what's claimed/done/next                                                          |
| **engine-expansion.md**                                          | Engine expansion strategy: dependency system, ProcessContext, TUI, node taxonomy                       |
| **pricing-model.md**                                             | Reference: free vs premium model (tabled, preserved for future use)                                    |
| **cloud-desktop-strategy.md**                                    | Detailed architecture, tech decisions, data model, deployment topology                                 |
| **architecture.md**                                              | Rules: layered architecture, data flow, execution model                                                |
| **bntos.md**                                                     | Recipe registry: slugs, tiers, fixtures, node requirements                                             |
| **core-principles.md**                                           | Trust commitments, design philosophy                                                                   |
| Private business docs (`BNTO_PRIVATE_DOCS_PATH` in `.env.local`) | Revenue projections, search volume data, competitive analysis, brand, pricing strategy, feature funnel |

---

## Principles That Constrain This Roadmap

From `core-principles.md`:

1. **CLI is the product.** The terminal experience comes first. Web showcases, it doesn't gate.
2. **Free tier never gets worse.** CLI and browser execution are free forever. No artificial caps.
3. **Desktop is free forever.** No "desktop Pro." Local execution is always unlimited.
4. **MIT license stays MIT.** The engine is always open.
5. **No dark patterns.** Upgrade hooks are natural — not artificial limits.
6. **If bnto shuts down, the engine stays open.** No lock-in, ever.

---

_This document is the strategic layer. For sprint-level task tracking, see [PLAN.md](PLAN.md). For engine expansion strategy, see [strategy/engine-expansion.md](strategy/engine-expansion.md). For detailed architecture, see [strategy/cloud-desktop-strategy.md](strategy/cloud-desktop-strategy.md)._
