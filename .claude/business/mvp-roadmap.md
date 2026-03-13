# MVP Scope & Feature Roadmap

**Last Updated:** February 2026
**Status:** Phase 1 in progress
**Previously:** Notion — "MVP Scope & Feature Roadmap"

**Timeline Philosophy:** Steady, deliberate progress. No breakneck pace. Some weeks we ship features, some weeks we don't. Quality over speed.

---

## North Star

**Get a real user running a real Bnto on the web as fast as possible.**

That user is a designer, a small team member, a solo founder. They open a browser, pick a predefined Bnto, drop their files, and it runs. Free. No account required for the core experience. No download. No setup. It just works.

Everything else — the JSON editor, the desktop app, the full workflow builder — is layered on top of that working foundation.

---

## What's Built (Phase 0 — Complete)

**Go Engine (archived, legacy):**
- 10 node types, all >90% test coverage
- Integration tests with fixture .bnto.json files
- CLI smoke tests (run, validate, list, dry-run)
- Go HTTP API server with 20+ integration tests
- BntoService shared API layer
- **Status:** Archived in `archive/engine-go/` and `archive/api-go/`. Rust engine is now primary.

**Rust WASM Engine (M1, delivered):**
- All 6 Tier 1 nodes built in Rust, compiled to WASM
- Uniform Rust engine — no JS fallback needed
- 606KB gzipped single cdylib bundle (all 6 nodes)
- Web Worker wrapper with typed message protocol
- Unit tests + WASM integration tests + Playwright E2E
- **The unified engine vision is real:** one Rust codebase will power browser (WASM), desktop (Tauri native), CLI (native binary), and cloud (compiled service)

**Monorepo Infrastructure:**
- Turborepo + pnpm workspaces + Taskfile.dev + go.work
- @bnto/core: React Query + Convex adapter, hooks, runtime detection
- @bnto/backend: Convex schema (users, workflows, executions, executionLogs)
- @bnto/auth: `@convex-dev/auth` (stateless JWT auth)
- @bnto/nodes: Engine-agnostic node definitions, schemas, recipes, validation
- UI components co-located in `apps/web/components/` (future @bnto/ui / Motorway)
- Playwright E2E infrastructure

---

## Revised Phase Order

> **Updated February 2026:** Strategy has shifted from cloud-first to browser-first. Browser execution is M1. Cloud execution (already built) moves to M4 (premium). See `ROADMAP.md` for the full strategic direction.

```
Old order: Web UI → Desktop execution → Cloud execution
v2 order:  Web UI + Cloud execution → JSON editor → Desktop app
v3 order (current): Browser execution (WASM) → Platform features → Desktop app → Premium cloud
```

---

## Phase 1: Browser Execution + Predefined Bntos (In Progress)

### Goal

Ship a web app where anyone can open a browser, pick a predefined Bnto, and run it **entirely client-side**. All Tier 1 bntos execute via Rust WASM in Web Workers. Files never leave the user's machine. Free. No account needed. No backend for core experience.

Cloud execution (Go API on Railway, R2 file transit) is **already built and tested** — 6/6 integration E2E tests pass. This infrastructure is ready for M4 (premium server-side bntos).

Auth (`@convex-dev/auth`, stateless JWT) exists for users who want to save flows and build their own. But the entry experience requires nothing.

### Sprint 1: Infrastructure (Auth + Deployment)

- Wave 1: @bnto/auth (Better Auth), @bnto/backend (Convex updates), Vercel deployment — **COMPLETE**
- Wave 2: BntoProvider + SessionProvider, proxy middleware, AppGate, sign-in/sign-up pages, route definitions — **IN PROGRESS**
- Wave 3: Sign-out flow, remove passphrase gate and old auth
- Wave 4: Verify auth flow end-to-end, Playwright E2E tests

### Sprint 2: Predefined Bntos + Execution

This is the MVP moment. The first time a real user drops files, runs a Bnto, and gets their output back. Everything else builds on top of this working.

#### Predefined Bntos at Launch

Curated around the highest-value single-purpose tool replacements. Each Bnto maps directly to something people currently Google for a separate tool.

**Image Bntos:**
- **Compress Images** — JPEG/PNG/WebP compression with quality slider. Replaces TinyPNG. Batch, no limits, no $25/year.
- **Resize Images** — Set target width, height, or both. Aspect ratio lock option.
- **Convert Image Format** — PNG → WebP, JPEG → PNG, HEIC → JPEG, etc. The most Googled image task.
- **PDF to Images** — Convert each PDF page to PNG or JPEG. Replaces half a dozen single-purpose sites.

**File Bntos:**
- **Rename Files** — Pattern-based batch rename. Prefix, suffix, sequential numbering, find/replace in filename. Replaces Bulk Rename Utility and similar tools.

**Data Bntos (CSV):**
- **Rename CSV Columns** — Map old column names to new ones. Drop files, set mappings, download clean CSV. No Excel required.
- **Format CSV Column Values** — Standardize date formats, trim whitespace, uppercase/lowercase, find/replace within a column.

**Why CSV is a sleeper hit:** There is no good simple web tool for basic CSV cleanup. Everyone either opens Excel, writes a Python script, or suffers. Bnto's spreadsheet node handles this natively and the use cases are extremely high search volume.

**Expansion based on user demand** — these launch Bntos are the opening lineup, not the ceiling.

**Future Bnto category: HTTP / API (BYOK)**

Not MVP, but a natural unlock once the JSON editor exists. The `http-request` node already exists in the Go engine. The pattern is simple:
- User brings their own API key — stored in their Bnto config, never on our servers
- Predefined Bnto templates for common patterns: call an API, transform the response, write to a file
- No Bnto-managed integrations — we don't build 1,000 connectors. We build one composable node and get out of the way.
- Examples: hit an AI API and save the response, call a webhook, fetch JSON from an endpoint and convert to CSV

This is the right way to handle integrations for a lean solo project: abstract the complexity of *running* the request, but never own the credentials or the connection. BYOK means zero liability, zero infrastructure, and infinite flexibility.

#### User Experience Per Bnto

Every predefined Bnto follows the same simple pattern:

1. **Pick the Bnto** from the gallery
2. **Drop your files** — drag and drop, batch supported
3. **Set options** — minimal, context-specific controls (quality slider, target format, column mapping, etc.)
4. **Run** — one button
5. **Download** — zip of output files, direct download, no account needed

No account required. No login wall. Results delivered immediately.

#### File Transfer Architecture

**Pattern: Browser → R2 → Railway → R2 → Browser**

Files are never stored permanently. The full lifecycle is:

1. User drops files in browser
2. Browser uploads directly to Cloudflare R2 (temp bucket, TTL-keyed paths)
3. Convex mutation creates an execution record, triggers Railway via HTTP
4. Railway Go API pulls input files from R2, executes the `.bnto.json` flow
5. Railway pushes output files back to R2 (same temp path structure)
6. Convex subscription notifies browser that execution is complete
7. Browser generates signed download URL from R2, user downloads zip
8. R2 objects deleted after download or after 1-hour TTL

**Why R2 and not direct blob to Railway:**
- Railway has a 100MB request body limit — large image batches will hit this
- R2 free tier: 10GB storage cap, 1M write ops, 10M read ops, **zero egress fees**
- Files never persist — upload → process → download → delete keeps storage usage near zero
- S3-compatible API means standard tooling on both sides

**Cost reality:** At MVP volume, R2 usage will be well within the free tier.

#### Execution Progress

**Real-time via Convex subscriptions** (already in stack).

Pattern:
1. Convex mutation creates execution record with status `pending`
2. Railway API updates status via Convex HTTP action: `running` → `complete` / `failed`
3. Browser subscribes to execution record — UI updates in real time
4. On `complete`, download button appears with signed R2 URL

**What the user sees:** A calm, satisfying progress state. Not a spinner. Node-by-node progress where each compartment fills as it completes.

### Sprint 3: Dashboard & Workflow Management (Authenticated)

For users who sign up — save flows, track history, manage their Bntos.

- WorkflowCard, StatusBadge, RunButton, EmptyState components
- Dashboard page, new workflow page
- Execution history
- **Execution analytics** — track execution events per user in Convex. Browser executions free and unlimited. Server-node execution tracked for Pro billing.
- **Upgrade prompt scaffolding** — UI component at natural value moments (Save, History, Server Nodes, Team). No Stripe yet, but UX built and tested.
- Playwright E2E tests

### Sprint 4: JSON Editor

For users who want to go deeper — write or customize .bnto.json files in-browser.

- Monaco/CodeMirror editor component with schema validation and syntax highlighting
- Template selector — start from a predefined Bnto, customize
- Zustand editor state store
- Playwright E2E tests

---

## Phase 2: Desktop App (Free, Local, Unlimited)

### Goal

Free desktop app using Tauri (Rust-native). Same React frontend, local Rust engine execution. For users who want offline execution, no cloud dependency, and no limits whatsoever.

> **Updated February 2026:** Desktop tech changed from Wails v2 (Go) to Tauri (Rust-native) after Rust won the M1 engine evaluation.

**Core Bntos are always free on desktop.** Forever.

**BYOK AI (desktop advantage):** Desktop is the natural home for AI-powered nodes. Users already have API keys for Claude, OpenAI, etc. BYOK means zero inference costs for Bnto, zero data privacy concerns, and zero rate limit headaches.

### Sprint 5: Tauri Bootstrap
- Bootstrap Tauri project
- Implement Tauri adapter in @bnto/core
- Expose Rust engine functions via Tauri IPC commands
- Wire up runtime detection (Tauri webview vs browser)

### Sprint 6: Local Execution
- Execute workflows via Tauri Rust bindings (all node types)
- Execution progress streaming via Tauri events
- Results view, error handling, cancellation
- macOS + Windows + Linux builds

---

## Phase 3: Polish + Monetization Infrastructure

### Goal

Revenue infrastructure, polish, and the visual editor. By this point we have real users, real signal, and a working product worth paying for.

- **Stripe integration** — Pro ($8/month or $69/year). Pro sells persistence, collaboration, and server-node execution. Browser bntos remain free unlimited. See `pricing-model.md`.
- Cloud file upload/download polish
- Execution history with detailed logs (Pro feature — 30 days)
- Workflow versioning and duplication
- Visual workflow editor (drag-and-drop nodes)
- Priority processing queue (Pro users skip the line)
- Usage dashboard

---

## Revenue Milestones by Sprint

> **Note:** Monetization model is value-driven (browser free unlimited, Pro sells persistence/collaboration/premium compute). See `pricing-model.md`.

| Sprint | What ships | Revenue implication |
|---|---|---|
| Sprint 1 | Auth + deployment | Foundation only |
| Sprint 2 | Cloud execution pipeline | Infrastructure for M4 (premium). Already complete. |
| Sprint 2B | **Browser execution (M1)** | **All Tier 1 bntos free, unlimited, client-side.** SEO pages live. Audience building starts. |
| Sprint 3 | Platform features (accounts, history) | **Accounts exist.** Conversion hooks: Save, History. |
| Sprint 4 | JSON editor | Power users self-identify. Pro signal. |
| Sprint 5-6 | Desktop app | Top of funnel. Trust builds. |
| Sprint 7 | Stripe + Pro tier | **First revenue.** Pro: persistence, collaboration, server-side AI. |

**The mantra:** Every sprint either builds the product users love or builds the system that lets them pay for it. Nothing is wasted.

---

## SEO Strategy: URL-Driven Tool Pages

SEO is bnto's highest-leverage acquisition channel. File-processing keywords have enormous search volume and clear commercial intent.

### The core idea: intent-preloaded URLs

Every predefined Bnto gets its own dedicated, indexable URL that loads the app with that Bnto already selected and ready to run:

```
bnto.io/compress-images
bnto.io/convert-png-to-webp
bnto.io/resize-images
bnto.io/pdf-to-images
bnto.io/rename-files
bnto.io/clean-csv
bnto.io/rename-csv-columns
```

This is not a separate landing page. It IS the app. The URL determines which Bnto is pre-selected.

### Target queries at launch

| URL | Target query | Est. monthly searches |
|---|---|---|
| /compress-images | "compress images online free" | 100K+ |
| /convert-png-to-webp | "convert png to webp" | 50K+ |
| /resize-images | "resize images online" | 200K+ |
| /pdf-to-images | "pdf to png free" | 50K+ |
| /rename-files | "batch rename files online" | 10K+ |
| /clean-csv | "clean csv online" | 5K+ |
| /rename-csv-columns | "rename csv columns" | 5K+ |

### Supporting content (Phase 3+)

Write SEO content that embeds bnto as the tool:
- "How to compress images for the web without losing quality" → uses bnto Compress Images
- "The easiest way to convert PNG to WebP" → uses bnto Convert Format
- "How to clean a CSV without Excel" → uses bnto Clean CSV

This is the Bannerbear playbook — documentation and tutorials drove more conversions than any other marketing activity.

---

## R2 as Temp Storage: Why It's Free Forever

Cloudflare R2 is bnto's file transit layer, not a storage product. Files exist in R2 for minutes, not days.

- R2 free tier: 10GB storage, 1M write operations, 10M read operations, **zero egress fees**
- At any given moment, R2 contains only actively-processing files from the last hour
- A 100-user active day might have 50MB in R2 at peak
- If volume grows to tens of thousands of daily users, R2 storage costs are $0.015/GB/month — never meaningful

---

## Feature Prioritization Framework

When deciding what to build next, ask:

1. **Does it get a casual user to their first successful run?** If yes, high priority.
2. **Can this be done with the CLI already?** If yes, ship the UI wrapper, don't reinvent.
3. **Does it work locally?** Local-first always. Cloud extends, not replaces.
4. **Can it be built simply?** No over-engineering for v1.
5. **Would the founder use this today?** Dogfooding is the best validation.

---

## The MVP Mantra

**Ship predefined recipes running in the browser, free and unlimited. Your files never leave your machine. Let real users tell us what to build next.**

---

*Reference this document when making scope decisions. If a feature isn't in the current phase, it waits.*
