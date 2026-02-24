# Bnto — Strategic Roadmap

**Last Updated:** February 24, 2026
**Purpose:** High-level strategy, milestones, and big decisions. PLAN.md tracks sprint tasks. This tracks the "why" and "where we're going."

---

## Vision

Bnto is the one place small teams go to get things done — compress images, clean a CSV, rename files, call an API — without the overhead of a platform or the fragility of a script.

**Four execution targets, one product:**

| Target | When | Cost to Us | Cost to User |
|--------|------|-----------|--------------|
| **Browser** (Rust→WASM) | M1 (now) | $0 | Free forever |
| **Cloud** (Go or Rust, per M1 outcome) | M4 (premium) | ~$5/mo base | Pro tier |
| **Desktop** (Tauri or Wails, per M1 outcome) | M3 | $0 | Free forever |
| **CLI** (Go binary) | Already built | $0 | Free forever (OSS) |

**The insight:** Browser execution for Tier 1 bntos costs us nothing and gives users unlimited free runs. Cloud execution becomes a premium feature for things the browser can't do (AI, shell commands, video). This inverts the old model where cloud execution was the default and free runs were artificially capped.

---

## Milestone Map

```
M1: Browser Execution (MVP)          ← ACTIVE
    Ship all Tier 1 bntos running in-browser via Rust→WASM.
    Zero backend for core experience. "Your files never leave your computer."

    Rust WASM is the M1 execution engine. The 6 MVP nodes are built
    in Rust, compiled to WASM via wasm-pack, and run in Web Workers.

    ┌──────────────────────────────────────────────────────┐
    │  ESCAPE HATCH: If Rust sucks, bail to JS             │
    │                                                      │
    │  After 1-2 Rust nodes, evaluate honestly:            │
    │  - Is development velocity acceptable?               │
    │  - Is the WASM boundary manageable?                  │
    │  - Is bundle size reasonable?                        │
    │                                                      │
    │  If no → switch to JS library adapters (jSquash,     │
    │          PapaParse, etc.) for the remaining nodes.   │
    │          No harm, no foul — it's only 6 nodes.      │
    │                                                      │
    │  @bnto/nodes stays either way.                       │
    └──────────────────────────────────────────────────────┘

M2: Platform Features
    Save workflows, execution history, user accounts.
    Convex-backed. This is where accounts earn their keep.

M3: Desktop App
    Desktop app with local execution. Free forever, unlimited.
    Full node support including shell-command and BYOK AI.

    If Rust WASM succeeded in M1:
      → Desktop = Tauri (Rust-native). One codebase for all targets.
      → Go engine becomes legacy (CLI keeps working, no new development).

    If we bailed to JS in M1:
      → Desktop = Wails v2 (Go-native). Go engine powers desktop + CLI.
      → JS adapters stay for browser.

M4: Premium Server-Side Bntos
    Server-side engine (Go or Rust, per M1/M3 outcome) for
    things browsers can't do: AI inference, shell commands,
    video processing, large file operations.

M5: Monetization
    Stripe. Pro tier. Revenue.
    Charges for real value (AI, collaboration, history) —
    not artificial run limits on browser-native operations.
```

**Key:** Milestones are sequential but overlap. M2 work starts before M1 closes. M1 itself is the Rust evaluation — by the time we ship MVP, we'll know if Rust works for us. The M3 desktop decision falls out naturally from M1's outcome.

---

## Browser Execution: Tech Matrix

All Tier 1 bntos run 100% client-side via Rust→WASM. No server round-trip, no R2 file transit, no Railway.

| Bnto | Slug | Rust Crate(s) | WASM Strategy | Notes |
|------|------|---------------|---------------|-------|
| Compress Images | `/compress-images` | `image`, `mozjpeg-sys`, `oxipng` | wasm-pack + wasm-bindgen | MozJPEG for JPEG, OxiPNG for PNG, WebP via `image` |
| Resize Images | `/resize-images` | `image` (resize module) | wasm-pack + wasm-bindgen | Lanczos3/CatmullRom filters |
| Convert Image Format | `/convert-image-format` | `image` (decode any → encode any) | wasm-pack + wasm-bindgen | JPEG, PNG, WebP, AVIF, GIF, BMP, TIFF |
| Clean CSV | `/clean-csv` | `csv` + `serde` | wasm-pack + wasm-bindgen | Rust `csv` crate is battle-tested |
| Rename CSV Columns | `/rename-csv-columns` | `csv` + `serde` | wasm-pack + wasm-bindgen | Header rewrite, same engine as clean-csv |
| Rename Files | `/rename-files` | Pure JS (no Rust needed) | JS | Pattern matching + rename on File objects — no binary processing |

**Why Rust WASM:** We're betting that building the engine in Rust now — even for just 6 MVP nodes — gives us a head start on the unified engine vision. If Rust works well here, the same code powers desktop (Tauri), CLI, and cloud. One language, one codebase.

**CSV in Rust WASM:** The JS/WASM string boundary is a known concern. Rust's `csv` crate is fast enough that the boundary overhead may be acceptable for our use case (structured operations, not free-form parsing). We'll benchmark against PapaParse after the first CSV node ships. If Rust CSV is measurably slower, CSV nodes can bail to JS while image nodes stay in Rust.

**Rename Files stays JS.** No binary processing — it's purely File object manipulation. Rust adds nothing here.

**Web Workers are mandatory.** All WASM processing runs off the main thread. Progress callbacks report back to the UI via `postMessage`.

### JS Fallback Libraries (escape hatch)

If Rust doesn't work out after 1-2 nodes, these are the proven JS alternatives:

| Bnto | JS Library | Notes |
|------|-----------|-------|
| Image processing | jSquash (MozJPEG, OxiPNG, WebP, AVIF) | Discourse uses for 50MB+ images |
| CSV processing | PapaParse | 1M rows in ~5s. JS beats WASM for strings |
| SVG optimization | Vexy SVGO (Rust→WASM, ironically) | 12x faster than Node SVGO |

### Tier 2+ Browser Candidates

| Bnto | Approach | Feasibility |
|------|----------|-------------|
| Strip EXIF | Rust `image` (metadata strip) or JS piexifjs | High — trivial with either |
| Optimize SVGs | Rust (custom) or Vexy SVGO (Rust→WASM) | High |
| Convert CSV to JSON | Rust `csv` + `serde_json` | High — trivial transform |
| Validate/Format JSON | Pure JS (JSON.parse + stringify) | High — no Rust needed |
| Merge CSVs | Rust `csv` | High — concat + dedupe |
| PDF to Images | pdf.js + Canvas (JS) | Medium — quality varies, Rust PDF libs immature |

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

## Shared Node Registry

**Problem:** Frontend currently hardcodes per-node config shapes. Two sources of truth (Go engine + frontend). Browser adapter needs a third.

**Solution:** `@bnto/nodes` package — single source of truth for node definitions, schemas, recipes, and validation.

```
packages/@bnto/nodes/
├── definitions/      # Node type definitions (what each node does)
├── schemas/          # Input/output schemas (drives config panel UI)
├── recipes/          # Predefined bnto recipes (metadata + definition)
└── validators/       # Workflow validation (works in browser, CLI, desktop)
```

**Consumed by every execution target:**
- **Rust WASM browser engine** (M1) — executes each node type client-side
- **Web app config UI** — generates forms from schemas (Atomiton pattern: `createFieldsFromSchema`)
- **Go engine** (CLI, potentially M3/M4) — existing 28k-line implementation, validates against same schemas
- **Desktop app** — same validation regardless of engine choice (Tauri if Rust wins, Wails if Go)

**This is the engine-agnostic foundation.** No matter what happens — whether Rust WASM succeeds or we bail to JS adapters — `@bnto/nodes` stays. It's the single source of truth that any execution engine consumes. Build it first.

**This replaces** the current pattern where `engine/pkg/menu/` owns recipes in Go and the web app hardcodes config shapes. Both converge on `@bnto/nodes`.

---

## Monetization Model

### Old Model (cloud-first, run-capped)
```
Free: 25 runs/month → hit wall day 3 → upgrade or leave
Pro: $8/month for 500 runs
```
**Problem:** Punishes users for using the product. Compressing images shouldn't cost money when the user's browser is doing the work.

### New Model (browser-first, value-driven)

| Tier | Price | What You Get |
|------|-------|-------------|
| **Free (Browser)** | $0 forever | All browser-capable bntos, unlimited runs, files never leave your machine |
| **Free (Desktop)** | $0 forever | Everything local — shell commands, BYOK AI, full filesystem |
| **Pro (Web)** | $8/mo or $69/yr | Saved workflows, execution history, team sharing, server-side AI, priority processing, API access |

**Why this is better:**
- Users don't resent paying for AI inference or collaboration. They resent "you've compressed too many images this month."
- Browser execution costs us $0. Capping it is artificial and hostile.
- Pro tier sells real value: persistence, collaboration, premium compute.
- Desktop remains free forever (trust commitment in `core-principles.md`).

---

## Conversion Funnel

Users convert when they want something the browser can't provide alone. These are natural upgrade hooks — not artificial limits.

| Hook | Trigger | What They're Buying |
|------|---------|-------------------|
| **Save** | "I want to keep this workflow" | Persistence (Convex-backed workflow storage) |
| **History** | "I need my execution history for audit" | Execution log retention (30-day Pro) |
| **Premium Bntos** | "I need AI to classify these images" | Server-side compute (Railway) |
| **Team** | "My team needs shared workflows" | Collaboration (up to 5 members, no per-seat) |

**Lazy anonymous sessions:** Users run browser bntos instantly — no signup, no account, no friction. Convex logs executions with a browser fingerprint. When they sign up, history is available retroactively. Zero backend until they choose to engage.

---

## Architecture Decisions

| Decision | Status | Rationale |
|----------|--------|-----------|
| **Rust WASM for MVP browser nodes** | Active (M1) | Build 6 MVP nodes in Rust, compile to WASM. Learn Rust, prove the unified engine vision. Escape hatch: bail to JS after 1-2 nodes if it sucks. |
| **JS adapters as fallback** | Standby | jSquash + PapaParse are production-proven. If Rust doesn't work, switch to JS for remaining nodes. No harm, no foul. |
| **Go engine paused for web** | Paused | 28k lines of proven code. Ready for M3 (desktop) and M4 (premium). Not deleted. |
| **`@bnto/nodes` is engine-agnostic** | Approved | Schemas, recipes, validation in TS. Survives any engine choice. The safety net. |
| **Railway deprioritized** | Backlog (M4) | Only needed for premium server-side bntos. |
| **R2 deprioritized** | Backlog (M4) | Not needed for browser execution. File transit only for cloud path. |
| **Lazy anonymous sessions** | Approved | Zero backend friction. Convex logs when accounts exist. |
| **Convex execution logging** | Approved | Records who ran what. Ties to history when user signs up. |
| **Web Workers mandatory** | Approved | All WASM processing off main thread. Progress via postMessage. |
| **Zip + individual downloads** | Approved | Both options for multi-file result retrieval. |

### Engine Decision: M1 Is the Evaluation

**The approach:** We're building the 6 MVP browser nodes in Rust, compiled to WASM. M1 itself is the Rust evaluation — by the time we ship, we'll know if Rust works for us.

**The escape hatch is clean.** After 1-2 Rust WASM nodes, we evaluate honestly. If development velocity is bad, the WASM boundary is painful, or bundle size is unreasonable — we bail to JS library adapters for the remaining nodes. It's only 6 nodes. No harm, no foul.

**Evaluation checkpoints (after each node):**

| Question | Good sign | Bad sign |
|----------|-----------|----------|
| Development velocity | Building nodes gets faster as patterns emerge | Each node is a slog with new problems |
| WASM boundary | Data passes cleanly, File/Blob handling works | Constant serialization headaches, memory leaks |
| Bundle size | < 500KB gzipped for all nodes | Multi-MB WASM blobs per node |
| Ecosystem coverage | `image` and `csv` crates cover our needs | Fighting the crate ecosystem, writing custom codecs |
| Developer experience | wasm-pack builds are fast, debugging is manageable | Build times are painful, errors are cryptic |

**If Rust succeeds (likely outcome by end of M1):**
- Rust becomes the engine for all targets (browser WASM, desktop native, CLI, cloud)
- Desktop (M3) uses Tauri (Rust-native) — one codebase, one language
- Go engine becomes legacy (CLI keeps working, no new development)
- The unified engine vision is real: one Rust codebase powering every execution target

**If we bail to JS:**
- JS library adapters (jSquash, PapaParse, etc.) power browser execution
- Go stays as the engine for desktop (Wails v2), CLI, and cloud (Railway)
- Two execution implementations maintained, but they share `@bnto/nodes` schemas
- Rust nodes already built still work — they just don't become the unified engine

**The safety net: `@bnto/nodes` is engine-agnostic.** Node definitions, schemas, recipes, and validation rules live in TypeScript. They don't care whether the executor is Rust WASM, JS, or Go. No matter what happens, `@bnto/nodes` stays.

**There is no timeline. This is for fun.** We're learning Rust, building something cool, and proving out the unified engine vision. If it works, we've leapfrogged the "two implementations" problem. If not, we have a working product on JS adapters.

---

## What Lives Where

| Document | Scope |
|----------|-------|
| **ROADMAP.md** (this file) | Milestones, strategic direction, big decisions, "why" |
| **PLAN.md** | Sprint tasks, waves, what's claimed/done/next |
| **cloud-desktop-strategy.md** | Detailed architecture, tech decisions, data model, deployment topology |
| **architecture.md** | Rules: layered architecture, data flow, execution model |
| **bntos.md** | Bnto registry: slugs, tiers, fixtures, node requirements |
| **core-principles.md** | Trust commitments, design philosophy |
| **Notion** | Pricing details, revenue projections, search volume data, competitive analysis |

---

## Principles That Constrain This Roadmap

From `core-principles.md`:

1. **Free tier never gets worse.** Browser execution is free forever. No artificial caps.
2. **Desktop is free forever.** No "desktop Pro." Local execution is always unlimited.
3. **MIT license stays MIT.** The engine is always open.
4. **No dark patterns.** Upgrade hooks are natural (save, history, AI) — not artificial limits.
5. **If bnto shuts down, the engine stays open.** No lock-in, ever.

---

*This document is the strategic layer. For sprint-level task tracking, see [PLAN.md](PLAN.md). For detailed architecture, see [strategy/cloud-desktop-strategy.md](strategy/cloud-desktop-strategy.md).*
