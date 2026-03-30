# Bnto — Strategic Roadmap

**Last Updated:** March 30, 2026
**Purpose:** High-level strategy, milestones, and big decisions. PLAN.md tracks sprint tasks. This tracks the "why" and "where we're going."

---

## Vision

Bnto is the one place small teams go to get things done — compress images, clean a CSV, rename files, call an API — without the overhead of a platform or the fragility of a script.

**Four execution targets, one product:**

| Target                                 | When          | Cost to Us  | Cost to User       |
| -------------------------------------- | ------------- | ----------- | ------------------ |
| **Browser** (Rust→WASM)                | M1 (now)      | $0          | Free forever       |
| **Cloud** (Go or Rust, per M1 outcome) | M4 (premium)  | ~$5/mo base | Pro tier           |
| **Desktop** (Tauri, Rust-native)       | M3            | $0          | Free forever       |
| **CLI** (Go binary)                    | Already built | $0          | Free forever (OSS) |

**The insight:** Browser execution for Tier 1 bntos costs us nothing and gives users unlimited free runs. Cloud execution becomes a premium feature for things the browser can't do (AI, shell commands, video). This inverts the old model where cloud execution was the default and free runs were artificially capped.

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
    Save workflows, execution history, user accounts, recipe editor v1.
    Convex-backed. Editor shipped with schema-driven config, save/My Recipes,
    keyboard shortcuts. Packages extracted: @bnto/ui, @bnto/editor.

    PIVOT (March 2026): Editor persistence stripped (Sprint 8.5a). Editor
    reconnecting as lightweight open+export tool with sessionStorage only
    (Sprint 8.5d) — no save to Convex, no localStorage, no My Recipes.
    Favorites/user preferences tabled to post-MVP. Deep editor features
    (code editor, expression input, edit/run mode) deferred to post-revenue.
    Community recipes via GitHub PRs, curated by maintainer.

M3: Desktop App
    Desktop app with local execution. Free forever, unlimited.
    Full node support including shell-command and BYOK AI.

    Rust succeeded in M1 → Desktop = Tauri (Rust-native).
    One codebase for browser + desktop + CLI.
    Go engine deleted (Sprint 6, March 2026). Rust is the unified engine.

M4: Premium Server-Side Bntos
    Server-side Rust engine for things browsers can't do:
    AI inference, shell commands, video processing, large
    file operations. Technology TBD (compiled service vs container).

M5: Monetization
    Stripe. Pro tier. Revenue.
    Charges for real value (AI, collaboration, history) —
    not artificial run limits on browser-native operations.
```

**Key:** Milestones are sequential but overlap. M1 and M2 are delivered. The M3 desktop decision is made: Tauri (Rust-native). Next up: schema-driven recipe config (Sprint 8.5c), reconnect editor lightweight (Sprint 8.5d), finish Tier 3 recipes (Sprint 8), then Desktop (M3) or Monetization (M5).

---

## Browser Execution: Tech Matrix

All Tier 1 bntos run 100% client-side via Rust→WASM. No server round-trip, no R2 file transit, no Railway.

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
- Most Tier 2 bntos (strip EXIF, optimize SVG, CSV↔JSON, validate JSON, merge CSV)

### Hybrid (browser + optional cloud)

Works in browser with limitations. Cloud unlocks the full experience.

- `http-request` — CORS limits which APIs are reachable from browser
- Large file operations — browser memory limits (~2GB practical max)
- Chained workflows — complex multi-step pipelines may benefit from server orchestration

### Server-Only (premium)

Requires server-side execution. These are the Pro tier differentiators.

- `shell-command` — impossible in browser (ffmpeg, imagemagick, etc.)
- AI nodes — API keys shouldn't be exposed client-side; needs server proxy
- Video processing — ffmpeg WASM exists but is impractically large
- Advanced PDF operations — server-side libraries are more capable
- Filesystem operations — real path access, directory traversal

---

## Shared Node Registry — DELIVERED

`@bnto/nodes` is the engine-agnostic foundation — single source of truth for node definitions, schemas, recipes, and validation. **Built and shipping since Sprint 2B.** 12 node types, Zod parameter schemas, 8 predefined recipes, codegen from Rust engine catalog (`catalog.snapshot.json` → `generate-from-catalog.ts`).

```
packages/@bnto/nodes/
├── generated/        # Auto-generated from engine catalog (types, schemas, catalog)
├── schemas/          # Per-node-type Zod parameter schemas
├── recipes/          # Predefined bnto recipes (metadata + definition)
└── validators/       # Definition validation (works in browser, CLI, desktop)
```

**Consumed by:** Rust WASM engine (browser), web app config UI (schema-driven `SchemaForm` + `CONTROL_REGISTRY`), `@bnto/editor` (node CRUD, adapters), `@bnto/core` (execution pipeline). Future: desktop (Tauri), CLI.

---

## Monetization Model

> **Single source of truth:** [pricing-model.md](strategy/pricing-model.md) defines the complete free vs premium framework — three layers (nodes, recipes, platform features), terminology, and the full feature matrix. This section is the strategic summary.

### The Dividing Line

> **Nodes that can run in your browser are free. Nodes that need a server cost money.**
>
> The node _definitions_ are always available to everyone (they're in `@bnto/nodes`, MIT licensed). The _execution_ of server nodes is what costs money.

This is the only principle you need. Everything else follows from it.

### Three Layers

1. **Nodes** — Browser nodes (image, csv, file, transform) are free, unlimited. Server nodes (ai, shell-command, video) are Pro, usage-based. Desktop: everything free (BYOK).
2. **Recipes** — Predefined + community-curated recipes are always free. Editor is a lightweight open+export tool (no persistence). Custom recipe creation is a power-user feature, not the primary experience.
3. **Platform features** — Persistence, history, team sharing, API access = Pro. Favorites (tabled) will be the free account creation hook when implemented.

### Pricing

| Tier               | Price           | What You Get                                                                                         |
| ------------------ | --------------- | ---------------------------------------------------------------------------------------------------- |
| **Free (Browser)** | $0 forever      | All browser-capable recipes, unlimited runs, recipe editor, export. Files never leave your machine.  |
| **Free (Desktop)** | $0 forever      | Everything — all nodes including AI (BYOK) and shell-command. Unlimited.                             |
| **Pro (Web)**      | $8/mo or $69/yr | Save recipes, execution history, team sharing, server-side compute, API access, priority processing. |

**Why this works:**

- Browser execution costs us $0. Capping it is artificial and hostile.
- Pro tier sells real value: persistence, collaboration, premium compute.
- Favorites (tabled) will create a natural sign-up moment when implemented.
- Desktop remains free forever (trust commitment in `core-principles.md`).

---

## Conversion Funnel

Users convert when they want something the browser can't provide alone. These are natural upgrade hooks — not artificial limits.

| Hook             | Trigger                                        | What They're Buying                          | Tier | Status  |
| ---------------- | ---------------------------------------------- | -------------------------------------------- | ---- | ------- |
| **Favorites**    | "I want to bookmark this recipe for next time" | Personalized recipe collection               | Free | Tabled  |
| **History**      | "I need my execution history for audit"        | Execution log retention (30-day Pro)         | Pro  | Planned |
| **Server nodes** | "I need AI to classify these images"           | Server-side compute (usage-based)            | Pro  | M4      |
| **Team**         | "My team needs shared recipes"                 | Collaboration (up to 5 members, no per-seat) | Pro  | M5      |

**No account required for browser execution:** Users run browser recipes instantly — no signup, no account, no friction. Zero backend until they choose to engage. Favorites (tabled) will become the first conversion hook when implemented — "bookmark this recipe" requires a free account.

---

## Community Recipes

**Curated, not user-generated.** For MVP, recipes are maintained by the project maintainer and community contributors via GitHub.

| Source         | Mechanism                                                 | Quality control     |
| -------------- | --------------------------------------------------------- | ------------------- |
| **Predefined** | `@bnto/registry` recipes, auto-propagated to all surfaces | Maintainer-authored |
| **Community**  | GitHub PRs to `@bnto/registry/src/recipes/`               | PR review + CI gate |

**Why GitHub, not a platform:** Building recipe publishing, moderation, and discovery infrastructure is premature. GitHub PRs give us version control, review workflow, CI validation, and community contribution — all for free. Accepted recipes flow through the existing codegen pipeline and appear on every surface automatically.

**Future (post-revenue):** If demand signals emerge for self-serve recipe creation and sharing, the editor infrastructure is already built (`@bnto/editor` is extracted and functional). The question is when, not whether.

---

## Architecture Decisions

| Decision                             | Status                   | Rationale                                                                                                                                        |
| ------------------------------------ | ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Rust WASM for browser nodes**      | Delivered (M1 complete)  | All 6 Tier 1 nodes built in Rust, compiled to WASM. Unified engine vision proven. 606KB gzipped bundle.                                          |
| **JS adapters as fallback**          | Not needed               | Rust succeeded. JS libraries available for Tier 2+ if specific nodes warrant it.                                                                 |
| **Go engine deleted**                | Archived (March 2026)    | Removed in Sprint 6. Rust is the unified engine for all targets. Source preserved in git history.                                                |
| **`@bnto/nodes` is engine-agnostic** | Approved                 | Schemas, recipes, validation in TS. Survives any engine choice. The safety net.                                                                  |
| **Railway deprioritized**            | Backlog (M4)             | Only needed for premium server-side bntos.                                                                                                       |
| **R2 deprioritized**                 | Backlog (M4)             | Not needed for browser execution. File transit only for cloud path.                                                                              |
| **No-account browser execution**     | Approved                 | Zero backend friction. Convex logs when accounts exist.                                                                                          |
| **Convex execution logging**         | Approved                 | Records who ran what. Ties to history when user signs up.                                                                                        |
| **Web Workers mandatory**            | Approved                 | All WASM processing off main thread. Progress via postMessage.                                                                                   |
| **Zip + individual downloads**       | Approved                 | Both options for multi-file result retrieval.                                                                                                    |
| **`@bnto/ui` extracted**             | Delivered (March 2026)   | Motorway design system as independent package. Primitives, layout, animation, surface system.                                                    |
| **`@bnto/editor` extracted**         | Delivered (March 2026)   | Headless-first editor package. ReactFlow canvas, schema-driven config, editor API layer.                                                         |
| **Smart Iteration**                  | Delivered (March 2026)   | `settings.iteration: "auto"\|"explicit"` on Definition. Auto wraps per-file processors in implicit loops. 20 golden tests prove equivalence.     |
| **Editor lightweight (open+export)** | In progress (March 2026) | Editor persistence stripped (8.5a). Reconnecting as open+export tool with sessionStorage (8.5d). No save, no My Recipes. Deep features deferred. |
| **Favorites tabled**                 | Tabled (March 2026)      | User preferences deferred to post-MVP. Revisit when engagement data signals demand.                                                              |
| **Community recipes via GitHub**     | Decided (March 2026)     | Contributors submit `.bnto.json` PRs. Maintainer curates. No publishing platform needed for MVP.                                                 |
| **Code Editor (CM6) tabled**         | Deep backlog             | Power-user luxury. Visual editor is the product. May revisit post-M5.                                                                            |

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
- Desktop (M3) uses Tauri (Rust-native) — one codebase, one language
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
| **pricing-model.md**                                             | Single source of truth: free vs premium, three layers, terminology, feature matrix                     |
| **cloud-desktop-strategy.md**                                    | Detailed architecture, tech decisions, data model, deployment topology                                 |
| **architecture.md**                                              | Rules: layered architecture, data flow, execution model                                                |
| **bntos.md**                                                     | Recipe registry: slugs, tiers, fixtures, node requirements                                             |
| **core-principles.md**                                           | Trust commitments, design philosophy                                                                   |
| Private business docs (`BNTO_PRIVATE_DOCS_PATH` in `.env.local`) | Revenue projections, search volume data, competitive analysis, brand, pricing strategy, feature funnel |

---

## Principles That Constrain This Roadmap

From `core-principles.md`:

1. **Free tier never gets worse.** Browser execution is free forever. No artificial caps.
2. **Desktop is free forever.** No "desktop Pro." Local execution is always unlimited.
3. **MIT license stays MIT.** The engine is always open.
4. **No dark patterns.** Upgrade hooks are natural (save, history, AI) — not artificial limits.
5. **If bnto shuts down, the engine stays open.** No lock-in, ever.

---

_This document is the strategic layer. For sprint-level task tracking, see [PLAN.md](PLAN.md). For detailed architecture, see [strategy/cloud-desktop-strategy.md](strategy/cloud-desktop-strategy.md)._
