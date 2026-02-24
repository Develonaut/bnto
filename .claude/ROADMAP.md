# Bnto — Strategic Roadmap

**Last Updated:** February 24, 2026
**Purpose:** High-level strategy, milestones, and big decisions. PLAN.md tracks sprint tasks. This tracks the "why" and "where we're going."

---

## Vision

Bnto is the one place small teams go to get things done — compress images, clean a CSV, rename files, call an API — without the overhead of a platform or the fragility of a script.

**Four execution targets, one product:**

| Target | When | Cost to Us | Cost to User |
|--------|------|-----------|--------------|
| **Browser** (JS/WASM) | M1 (now) | $0 | Free forever |
| **Cloud** (Go on Railway) | M4 (premium) | ~$5/mo base | Pro tier |
| **Desktop** (Go via Wails) | M3 | $0 | Free forever |
| **CLI** (Go binary) | Already built | $0 | Free forever (OSS) |

**The insight:** Browser execution for Tier 1 bntos costs us nothing and gives users unlimited free runs. Cloud execution becomes a premium feature for things the browser can't do (AI, shell commands, video). This inverts the old model where cloud execution was the default and free runs were artificially capped.

---

## Milestone Map

```
M1: Browser Execution (MVP)          ← ACTIVE
    Ship all Tier 1 bntos running in-browser via JS/WASM.
    Zero backend for core experience. "Your files never leave your computer."

M2: Platform Features
    Save workflows, execution history, user accounts.
    Convex-backed. This is where accounts earn their keep.

M3: Desktop App + Engine Evaluation
    Desktop app with local execution. Free forever, unlimited.
    Full node support including shell-command and BYOK AI.

    ┌──────────────────────────────────────────────────────┐
    │  DECISION POINT: Go vs Rust for Engine               │
    │                                                      │
    │  Take one cycle to rebuild the MVP browser nodes     │
    │  with Rust WASM. Evaluate: did it go well?           │
    │                                                      │
    │  Yes → Rust becomes the engine. Desktop = Tauri.     │
    │        One codebase: browser + desktop + CLI + cloud. │
    │                                                      │
    │  No  → Stay with Go. Desktop = Wails v2.             │
    │        JS adapters stay for browser. Go for rest.     │
    │                                                      │
    │  Either way, @bnto/nodes (schemas, recipes,          │
    │  validation) stays — it's engine-agnostic.           │
    │  See "Post-MVP Engine Decision" below.               │
    └──────────────────────────────────────────────────────┘

M4: Premium Server-Side Bntos
    Server-side engine (Go or Rust, per M3 decision) for
    things browsers can't do: AI inference, shell commands,
    video processing, large file operations.

M5: Monetization
    Stripe. Pro tier. Revenue.
    Charges for real value (AI, collaboration, history) —
    not artificial run limits on browser-native operations.
```

**Key:** Milestones are sequential but overlap. M2 work starts before M1 closes. The decision point in M3 is explicit — we take one cycle to rebuild MVP nodes in Rust WASM and evaluate before committing M4's server-side engine to either language.

---

## Browser Execution: Tech Matrix

All Tier 1 bntos run 100% client-side. No server round-trip, no R2 file transit, no Railway.

| Bnto | Slug | Library | Runtime | Notes |
|------|------|---------|---------|-------|
| Compress Images | `/compress-images` | jSquash (MozJPEG, OxiPNG, WebP, AVIF) | WASM | Discourse uses for 50MB+ images |
| Resize Images | `/resize-images` | jSquash resize / Pica | WASM/JS | ~1s for 5000x3000 on desktop |
| Convert Image Format | `/convert-image-format` | jSquash (decode any → encode any) | WASM | Google Squoosh is the reference |
| Clean CSV | `/clean-csv` | PapaParse | JS (Web Worker) | 1M rows in ~5s. JS beats WASM for strings |
| Rename CSV Columns | `/rename-csv-columns` | PapaParse | JS (Web Worker) | Same engine as clean-csv |
| Rename Files | `/rename-files` | Native JS | JS | Pattern matching + rename on File objects |

**Why PapaParse over WASM for CSV:** String allocation across the JS/WASM boundary is expensive. PapaParse in a Web Worker outperforms any WASM CSV parser. This is a known property of the boundary — don't fight it.

**Web Workers are mandatory.** All processing runs off the main thread. Progress callbacks report back to the UI via `postMessage`.

### Tier 2+ Browser Candidates

| Bnto | Library | Feasibility |
|------|---------|-------------|
| Strip EXIF | jSquash / piexifjs | High — metadata strip is trivial |
| Optimize SVGs | Vexy SVGO (Rust→WASM) | High — 12x faster than Node SVGO |
| Convert CSV to JSON | PapaParse + JSON.stringify | High — trivial transform |
| Validate/Format JSON | Native JS | High — JSON.parse + stringify |
| Merge CSVs | PapaParse | High — concat + dedupe |
| PDF to Images | pdf.js + Canvas | Medium — quality varies |

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
- **JS browser adapter** (M1) — knows how to execute each node type client-side
- **Web app config UI** — generates forms from schemas (Atomiton pattern: `createFieldsFromSchema`)
- **Go engine** (M3/M4) — reference implementation, validates against same schemas
- **Rust WASM engine** (M3 evaluation) — if Rust wins, consumes the same schemas
- **Desktop app** — same validation regardless of engine choice

**This is the engine-agnostic foundation.** No matter what happens at the Go vs Rust decision point, `@bnto/nodes` stays. It's the single source of truth that both engine candidates consume. Build it first, before either engine evaluation.

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
| **JS browser adapters first** | Active (M1) | Ships in weeks. jSquash + PapaParse are production-proven. $0 compute. |
| **Go engine paused for web** | Paused | 28k lines of proven code. Ready for M3 (desktop) and M4 (premium). Not deleted. |
| **Rust evaluation in M3** | Planned | One cycle to rebuild MVP nodes in Rust WASM. Evaluate, then decide. See below. |
| **`@bnto/nodes` is engine-agnostic** | Approved | Schemas, recipes, validation in TS. Survives any engine choice. The safety net. |
| **Railway deprioritized** | Backlog (M4) | Only needed for premium server-side bntos. |
| **R2 deprioritized** | Backlog (M4) | Not needed for browser execution. File transit only for cloud path. |
| **Lazy anonymous sessions** | Approved | Zero backend friction. Convex logs when accounts exist. |
| **Convex execution logging** | Approved | Records who ran what. Ties to history when user signs up. |
| **Web Workers mandatory** | Approved | All browser processing off main thread. Progress via postMessage. |
| **Zip + individual downloads** | Approved | Both options for multi-file result retrieval. |

### Post-MVP Engine Decision: Go vs Rust

**When:** During M3, after the MVP product (browser execution) is established and shipping.

**How:** Take one development cycle to rebuild the MVP browser nodes (image processing, CSV processing, file rename) using Rust compiled to WASM. Same `@bnto/nodes` schemas, same test fixtures, same UI — just a different execution backend. Then evaluate.

**What we'll have at that point:**
- A working JS browser adapter (M1) proving which execution patterns matter
- A working Go engine powering CLI (and potentially desktop)
- Real usage data showing which bntos people actually use
- Experience with the WASM boundary and its constraints from M1
- A Rust WASM prototype of the same MVP nodes to compare directly

**The evaluation:**

| Question | If Rust wins | If Go wins |
|----------|-------------|-----------|
| Was Rust WASM development velocity acceptable? | Continue with Rust | Stay with Go |
| Does Rust WASM match or beat JS adapter performance? | Unified engine possible | JS adapters stay for browser |
| Is bundle size reasonable for browser delivery? | Ship Rust WASM to browser | Keep JS libraries |
| Does the Rust ecosystem cover our node types well? | Expand to all nodes | Keep Go for server/desktop nodes |

**If Rust:**
- Rust becomes the engine for all targets (browser WASM, desktop native, CLI, cloud)
- Desktop uses Tauri (Rust-native) instead of Wails (Go-native)
- JS browser adapters retired — Rust WASM replaces them
- Go engine becomes legacy (still works for CLI, but no new development)
- One codebase, one language, one test suite

**If Go:**
- Go stays as the engine for desktop (Wails v2), CLI, and cloud (Railway)
- JS browser adapters stay for browser execution
- Two execution implementations maintained (JS + Go), but they share `@bnto/nodes` schemas
- Trade-off accepted: dual implementations vs. Rust learning curve

**The safety net: `@bnto/nodes` is engine-agnostic.** Node definitions, schemas, recipes, and validation rules live in TypeScript. They don't care whether the executor is JS, Go, or Rust. No matter which engine wins, `@bnto/nodes` stays. This is why we build it first — it de-risks the decision.

**This is explicitly NOT a decision we make now.** We ship M1 with JS adapters (fast, proven, low-risk), build the product, get users, and evaluate Rust with data and a working baseline. The worst outcome of deferring is we throw away some JS adapter code. The worst outcome of committing prematurely is we're stuck with a language choice made without experience.

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
