# Bnto — Build Plan

**Last Updated:** April 4, 2026 (groomed — Sprint 9 W1-W2 complete, video node shipped, CLI polish prioritized over TUI, TUI deferred to own sprint)
**This is the single source of truth for what's been built, what's in progress, and what's next.**

Skills and commands that reference the plan read this file. Update it after every sprint.

---

## How This Works

Tasks are organized into **sprints** (features) and **waves** (dependency groups within a sprint). All tasks in a wave can be picked up in parallel by agents. Waves must complete in order before the next wave starts.

```
- [ ]              → available, grab it
- [ ] **CLAIMED**  → an agent is working on this, pick something else
- [x]              → done
```

**Scope rule:** Each task targets ONE package. Don't touch files outside the tagged package unless the task explicitly says so.

**Branching:** Feature branches target `main` directly. Create a branch from `main` (`git checkout -b <type>/<short-description> main`), do the work, PR into `main`, squash merge. Use worktrees (`/pickup --w`) for isolation when multiple agents are active — see the pickup skill for the smart isolation model.

**Co-location decision (Feb 2026, updated March 2026):** UI components and editor features currently live in `apps/web`. Sprint 4D extracts UI to `@bnto/ui` (branded **Motorway** — Mini Motorways-inspired design system). Sprint 4E extracts editor to `@bnto/editor`. This extraction happens BEFORE the editor production sprint to establish clean package boundaries and make the complex editor work easier to follow. Engine, core API, and data layer logic stays in `@bnto/core`.

---

## Current State

- **v0.5.0 released (April 2026):** 15 predefined recipes (6 Tier 1 + 2 Tier 1B + 4 Tier 3 + 1 CLI-only video + 2 Tier 1B compositions). Sprint 9 W1-W2 complete: dependency system, ProcessContext, video node (yt-dlp), `bnto doctor`, `--param` CLI flag, extra args pass-through
- **v0.2.0 released (April 2026):** 14 predefined recipes, schema-driven config on all tool pages, editor reconnected as lightweight open+export tool
- **M1 delivered (Feb 2026):** All 6 Tier 1 bntos + 2 Tier 1B multi-node compositions run 100% client-side via Rust→WASM
- **M2 delivered (March 2026):** Editor v1 shipped — schema-driven config controls, keyboard shortcuts, accessibility audit. Accounts, execution history, PostHog telemetry all live.
- **Sprint 9 Waves 1-2 complete (April 2026):** Dependency system (`requires` on NodeMetadata), `ProcessContext` trait (NativeContext/NoopContext), `bnto doctor` command, `bnto-video` crate (video-download via yt-dlp), `InputMode` enum, `InputCardinality::Source`, `--param` CLI flag, extra args pass-through, H.264 codec preference, video title as filename
- **Sprint 8 complete (April 2026):** 4 Tier 3 engine operations (strip-exif, merge-csv, csv-to-json, image-overlay/watermark) + recipe fixtures + golden tests + codegen + SEO pages + E2E
- **Sprint 8.5 complete (March-April 2026):** Schema-driven recipe config (8.5c) + editor reconnected lightweight (8.5d) with sessionStorage persistence
- **Open-source-first positioning (April 2026):** Stripped pricing page, auth surfaces, Pro references. Monetization tabled until community traction.
- **crates.io preparation (April 2026):** All engine crates prepared for publish (v0.1.1). `cargo install bnto` path scaffolded but not yet live.
- **Community recipes:** Contributors submit `.bnto.json` via GitHub PRs. Maintainer curates. Accepted recipes auto-propagate via the Sprint 7 discovery infrastructure.
- **Tabled (deep backlog):** Code Editor (CM6), Edit/Run Mode, Sprint 5B W2-4 (LayerPanel polish, processing node accents), Favorites/My Recipes, TUI (deferred to own sprint after CLI polish)
- **Cloud infrastructure:** R2 file transit — ready for M4 (server technology TBD)
- **WASM engine:** 6 Rust crates (bnto-core, bnto-image, bnto-csv, bnto-file, bnto-video, bnto-engine), single cdylib (bnto-wasm), CLI binary (bnto-cli). 1.6MB raw / 606KB gzipped
- **Auth:** `@convex-dev/auth`. Password auth, integration tests complete, E2E auth lifecycle verified (13/13 tests)
- **Infra:** GitHub Actions CI (Rust + TypeScript + CI Gate), tag-triggered release pipeline (CI gate → Vercel preview → E2E → Lighthouse → auto-deploy Vercel + Convex to production on stable tags → GitHub Release), PostHog telemetry wired
- **Packages:** `@bnto/core` (7 domains: recipes, executions, user, auth, telemetry, registry, flags), `@bnto/auth`, `@bnto/backend`, `@bnto/nodes`, `@bnto/registry`, `@bnto/ui`, `@bnto/editor`, `@bnto/form`, `@bnto/i18n`

---

## What's Built (don't redo)

- [x] Monorepo: Turborepo + pnpm + Taskfile.dev
- [x] @bnto/core: Layered singleton (clients → services → adapters), React Query + Convex adapter, 38+ hooks
- [x] @bnto/auth: `@convex-dev/auth` integration, password auth
- [x] @bnto/backend: Convex schema (users, workflows, executions, executionLogs), auth, crons, analytics fields
- [x] @bnto/nodes: Engine-agnostic node definitions, Zod schemas, recipes, validation (node/recipe counts derived from engine catalog — see test assertions)
- [x] @bnto/ui: Extracted Motorway design system — primitives, layout, typography, feedback, surface, interaction, overlay, animation components
- [x] @bnto/editor: Extracted editor package — EditorCanvas, EditorToolbar, LayerPanel, ConfigPanel, CompartmentNode, NodePaletteMenu, adapters, hooks, store, actions
- [x] Web app: Auth flow, SEO infrastructure, middleware, landing pages (real content), privacy policy
- [x] Playwright E2E: 27+ screenshots, user journey tests, execution flow tests, site navigation (desktop + mobile)
- [x] Rust WASM engine: 6 crates (bnto-core, bnto-image, bnto-csv, bnto-file, bnto-video, bnto-engine), single cdylib (bnto-wasm), CLI binary (bnto-cli), Web Worker wrapper, progress reporting
- [x] Browser execution: All 6 Tier 1 bntos client-side via WASM, ZIP download, auto-download
- [x] Cloud execution infrastructure: R2 file transit, presigned URLs — ready for M4
- [x] Recipe page overhaul (Sprint 2D): RecipeShell, PhaseIndicator, FileCard, RecipeConfigSection, useRecipeFlow
- [x] Motorway design system: Grid, LinearProgress, ToolbarProgress, RadioGroup, NavButton, RadialSlider, surface system, Pressable + Surface composition
- [x] Per-instance browser execution stores: Factory pattern, `core.wasm.createExecution()`, no state leaks
- [x] Sprint 3 pre-work: Anonymous→password userId preservation, FIXME cleanup, Knip audit, naming audit, codebase standards review, schema analytics fields
- [x] GitHub Actions CI: Rust (fmt + clippy + unit + WASM) + TypeScript (build + lint + test) + CI Gate
- [x] convexQuery skip guards: All adapter functions use `"skip"` for falsy IDs (PR #23)
- [x] Format versioning + Zod node validation (Sprint 4G): `.bnto.json` format version constant, schema versioning, Zod parameter schemas for all node types, schema-driven config panel with registry-based controls
- [x] Editor production route (Sprint 5 W1-W2): `/editor` route, recipe loading by ID (`?recipe={id}`), compartment node redesign (icons + category colors), "Open in Editor" clone-on-click nav integration
- [x] Pipeline executor extraction (Sprint 4H): Runtime-agnostic `executePipeline()` in `@bnto/core`, `NodeRunner` contract, `processFiles()` removed from browser adapter, comprehensive TDD test suite
- [x] Editor API layer (Sprint 5D): `createEditor()` factory, 5 domain clients (nodes, definition, execution, history, panels), 5 services, React binding layer (`EditorProvider`, `useEditor`, domain hooks), full component migration, deprecated hooks deleted
- [x] Multi-node recipes (Tier 1B): optimize-images-for-web, generate-thumbnails — first multi-node predefined recipes with 3-operation pipelines inside forEach loops
- [x] Slider presets + select labels: Unified `quality` parameter (compression→quality rename with direct semantics), slider preset system, select option labels
- [x] Definition round-trip fidelity: `captureDefinition()` snapshot, `loadDefinition()` restore, fidelity test suite proving lossless round-trips
- [x] Editor execution wiring: RunButton → runExecution → core.executions.runPipeline(), ResultsTab/ResultRow in RunPanel, reset/re-run flow, per-node execution state tracking
- [x] Recipe save backend: Convex save mutation in recipes.ts, core.recipes.save() in recipeClient.ts, useSaveRecipe.ts hook
- [x] Editor Beta Launch (PR #173): Feature flag removed, beta badges on nav/CTAs, dismissible banner on `/editor`
- [x] I/O node visual hierarchy (Sprint 5B W1): Size differentiation (100×100 vs 120×120), muted color for I/O, elevation distinction, Pressable behavior split
- [x] Editor UX polish (Sprint 5A): Hover delete overlay, PlaceholderSlot, isIoNode flag, exit animations, config panel identity echo (node icon + empty state), SchemaForm field grouping (FieldGroup), LayerPanel drag-to-reorder, empty canvas auto-behaviors, E2E verification
- [x] Editor copy cleanup (Sprint 5C): Nav "Create" → "New Recipe", CTA "Customize in Editor" → "Open in Editor"
- [x] Alternating layout direction for nested containers (#162), fitView selected node priority (#161), container node layout + divider-based add nodes (#160)
- [x] Sprint 5 Editor v1: Auto-download default, config panel controls (Textarea, Combobox, KeyValueEditor), Motorway showcase, control registry wiring, inferFieldType updates, schema metadata cleanup, DRY recipe I/O nodes, save button + My Recipes integration, unsaved changes warning, E2E verification, keyboard shortcuts, accessibility audit
- [x] Input node file extension filter fix: `deriveFileInputAccept` pure function, store selector in RunButton, unit tests, E2E verification
- [x] Unified recipe model (Sprint 7): Layered types (`Recipe` in `@bnto/nodes`, `UserRecipe` in `@bnto/core`), deleted `RecipeDefinition` duplicates, `BntoEntry` derived from `Recipe`. `core.registry` as 6th domain (Zustand store, client API, React hooks). `?from={slug}` eliminated — "Open in Editor" clones template, navigates by ID. Runtime surfaces consume `core.registry`, build-time surfaces keep direct imports
- [x] Explore page (`/explore`): Full-page searchable/filterable recipe & node browser with ExploreHeader, ExploreFilters, ExploreRecipeGrid, ExploreJsonLd. Server component page with client interactive leaves (PR #281)
- [x] `@bnto/i18n` package: Centralized string management — `t()` dot-path resolver, `useT()` hook, `en.json` app strings + auto-generated `nodes.json` from engine catalog. Type-safe `StringKey` derived from JSON (PR #282)
- [x] Engine catalog codegen: Downstream TS values (format version, node metadata) derived from Rust engine catalog snapshot — single source of truth (PR #289)
- [x] Recipe flattening: Predefined recipes simplified to `settings.iteration: "auto"` — removed explicit loop/group nesting, engine handles per-file iteration implicitly (PR #278)
- [x] Sprint 8.5c (Schema-Driven Config): DynamicRecipeConfig replaces ~600 LOC of handcoded per-recipe config components. Any recipe gets config controls for free via `@bnto/form` SchemaForm (PRs #302, #303)
- [x] Sprint 8.5d (Reconnect Editor Lightweight): Editor restored as open+export tool with sessionStorage persistence. No save, no My Recipes. Beta dialog, import/export, E2E test suite (PR #305)
- [x] Tier 3 engine operations: strip-exif (PR #292), merge-csv (PRs #295, #296), csv-to-json (PR #294), image-overlay/watermark (PR #308) — all with recipe fixtures, golden tests, codegen
- [x] Watermark preview controls: Engine-parity positioning (9 positions), opacity, scale, color, live canvas preview (PR #309)
- [x] Release v0.2.0 (April 2026): 14 predefined recipes, schema-driven config, editor reconnect, 4 Tier 3 operations
- [x] Editor UX polish (Sprint 7): Config panel tabs + sync status (PR #283), unified toolbar layout (PR #284), carry flow config into editor (PR #285), unified run button (PR #286), carry dropped files into editor (PR #287), fix editor reset + returnTo redirects (PR #288)
- [x] Quality tooling: Knip dead-code detection in lefthook (PR #270), ESLint complexity rules promoted to error (PR #271), ESLint extended to all packages (PR #273), non-null assertions replaced (PR #274), SEO/README/copy improvements (PR #275), cloud→local recipe hydration (PR #276), footer links from registry (PR #277)
- [x] Sprint 9 W1 — Dependency system: `requires: Vec<Dependency>` on `NodeMetadata`, `ProcessContext` trait (`NativeContext`/`NoopContext`), dependency checker, `bnto doctor` command (PRs #315, #318, #320)
- [x] Sprint 9 W2 — Video node: `bnto-video` crate, `video-download` processor wrapping yt-dlp, `InputMode::Url`/`InputCardinality::Source`, `--param` CLI flag, H.264 codec preference, video title as filename, extra args pass-through, m3u8/HLS support, download verification tests (PRs #321-#329)
- [x] Open-source-first pivot: Stripped pricing page, auth surfaces, Pro references. Monetization tabled until community traction (PR #317)
- [x] crates.io preparation: All 6 engine crates prepared for publish at v0.1.1. `cargo install bnto` scaffolded (PRs #316, #319)
- [x] Release v0.5.0 (April 2026): 15 predefined recipes, video-download node, extra args pass-through, dependency system

---

## Revenue & Monetization Context

Pricing, revenue projections, and "ready to charge" criteria live in private business docs (`BNTO_PRIVATE_DOCS_PATH` in `.env.local`) — see `pricing-strategy.md`, `seo-monetization.md`, and `feature-funnel.md`.

**Monetization model (updated Feb 2026):** Browser execution is free unlimited. Pro sells real value — persistence, collaboration, premium compute. See ROADMAP.md for the full model.

| Sprint       | What Ships                                   | Revenue Implication                                                                                                                           |
| ------------ | -------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| Sprint 2B    | Browser execution (M1 MVP)                   | **All Tier 1 bntos run client-side.** Zero backend cost. Files never leave user's machine.                                                    |
| Sprint 2C    | Launch readiness (content + domain)          | **bnto.io live and indexable.** Real content on every page. SEO crawling begins. First real users possible.                                   |
| Sprint 2D    | Recipe page UX overhaul                      | **COMPLETE.** Progressive phase-driven flow. Motorway design language on every tool page.                                                     |
| Sprint H     | Housekeeping                                 | **COMPLETE.** FileUpload rewrite, Rust test audit, EXIF coverage, Pressable, CI, ESLint.                                                      |
| Sprint 3     | Platform features (accounts, history)        | Accounts exist. Conversion hooks scaffolded (Save, History). Usage analytics instrumented.                                                    |
| Sprint 4     | Recipe editor (headless + visual)            | Editor shipped as v1. **Now frozen** — power-user feature, not primary experience.                                                            |
| Sprint 4D-4G | Package extraction + versioning + validation | Clean architecture. Zod schemas. Packages ready for desktop (M3).                                                                             |
| Sprint 5     | Editor v1 (config controls, save, polish)    | **M2 completion.** Editor complete. Investment paused — revisit post-revenue.                                                                 |
| Sprint 8     | Tier 3 near-term recipes                     | **SEO expansion.** New browser recipes targeting high-volume search queries. Product catalog grows.                                           |
| Sprint 8.5   | Schema config + lightweight editor reconnect | **Simplification.** Schema-driven recipe config (any recipe gets controls for free), editor reconnected as open+export tool (no persistence). |
| Sprint 9     | Engine expansion (CLI, TUI, video)           | **Engine-first pivot.** Dependency system, video node, TUI, CLI polish. New capabilities tested via CLI first.                                |
| Sprint 10-11 | Desktop + server (deferred)                  | Distribution targets. Deferred to backlog. Desktop (Tauri) and server-side execution.                                                         |
| Sprint 12    | Stripe + Pro tier (tabled)                   | **Tabled.** Revenue strategy revisited when community traction emerges.                                                                       |

---

## Completed Sprints (collapsed)

### Phase 0: Foundation — COMPLETE

Monorepo restructuring, engine solidification with TDD (>90% coverage on all 10 node types), integration test fixtures, CLI smoke tests, Go API server, Convex setup, web app shell, @bnto/core hooks.

### Sprint 1: Infrastructure Migration — COMPLETE

Moved from Railway/Convex Auth to Vercel/Better Auth. Auth provider, Convex schema, Vercel deployment, proxy middleware, sign-in/sign-up/sign-out pages, route protection. Wave 4 (auth verification) was skipped — gaps caught and resolved in Sprint 2A.

### Sprint 2: Predefined Bntos + Cloud Execution — Waves 1-4 COMPLETE

6 Tier 1 fixtures, SEO URL routing, bnto registry, tool page UI (file drop, per-bnto config), R2 file transit, Railway deployment, env config (R2/Convex/Vercel/Railway), execution UI (RunButton, ExecutionProgress, ExecutionResults), predefined execution path. Wave 5 (pipeline verification) blocked by auth — deferred to Sprint 2A Wave 5.

### Sprint 2A: Auth Fix — COMPLETE

Migrated to `@convex-dev/auth` (eliminates JWT race condition). Anonymous sessions, proxy middleware, integration tests (A1-A7, C1-C3, S1-S3). Core integration test harness (ConvexHttpClient factory). Execution + upload/download integration tests. Playwright E2E pipeline verification. Auth evaluation documented in git history.

### Sprint 2.5: Codebase Polish — COMPLETE

Node.js subpath imports (`#components/*`, `#lib/*`), camelCase file rename (hooks, utils, lib), PascalCase component rename, dot-notation primitive wrappers, Button audit/migration, Button pseudo-state fix, Button animations (Mini Motorways motion language). Font review (DM Sans → Geist evaluation) deferred to backlog.

### Sprint 2B: Browser Execution (M1 MVP) — COMPLETE

All 6 Tier 1 bntos running 100% client-side via Rust→WASM. `@bnto/nodes` package (engine-agnostic definitions), Rust workspace with 5 crates, Web Worker wrapper, browser adapter in `@bnto/core`, BntoPageShell browser routing, ZIP download for multi-file results. Rust evaluation checkpoint PASSED. WASM bundle: 1.6MB raw / 606KB gzipped. 44+ Rust unit tests, WASM integration tests, Playwright E2E with screenshot assertions for all 6 bntos. **M1 milestone delivered.**

### Sprint 2C: Launch Readiness — COMPLETE

bnto.io live and indexable. All Mainline template content replaced with real bnto content (home, pricing, FAQ, privacy, footer, navbar). Messaging audit (no false claims). CSS animation refactor (JS → CSS-driven). Site navigation E2E tests. 15/15 static pages generate cleanly.

### Sprint 2D: Recipe Page UX Overhaul — COMPLETE

Progressive phase-driven flow (Files → Configure → Results) with Motorway design language. RecipeShell, PhaseIndicator, FileCard, RecipeConfigSection, useRecipeFlow, per-instance execution stores. 27+ screenshots regenerated. All 4 waves complete.

### Sprint H: Housekeeping — COMPLETE

Tech debt cleanup: FileUpload→react-dropzone, core.browser→core.wasm rename, shared ESLint config, Pressable component, React import sweep, GitHub Actions CI (PR #10), Rust test audit, EXIF orientation coverage. All tasks delivered.

### Sprint 3A: Remove Anonymous User System — COMPLETE

Eliminated anonymous Convex session system across 5 waves (backend schema, core hooks, web components, auth E2E, docs cleanup). Auth is now binary: signed in or not. 13/13 auth E2E tests passing. All anonymous references removed from schema, code, and docs.

### Sprint 3: Platform Features (M2) — COMPLETE (Wave 3 tabled)

Accounts earn their keep: execution history (IndexedDB for unauth, Convex for auth), `/my-recipes` dashboard, PostHog telemetry, Lighthouse CI, save prompt conversion hook, pricing page, browser auth verification, execution history migration on signup. Wave 3 (3 E2E test tasks) tabled — see backlog "Testing: Sprint 3 Deferred E2E Tests."

### Sprint 4: Recipe Editor (Headless-First) — COMPLETE

Headless-first editor: Wave 1 (`@bnto/nodes` pure functions — CRUD, adapters, tests), Wave 2 (Zustand store, ReactFlow adapters, hooks), Wave 3 (Motorway MVP — BentoCanvas, EditorToolbar, NodePalette, NodeConfigPanel, RecipeEditor). Architecture: `@bnto/nodes` → pure functions → Zustand store → React hooks → visual skin. Two entry points: `createBlankDefinition()` or `loadRecipe(slug)`. See [editor-architecture.md](.claude/strategy/editor-architecture.md), [visual-editor.md](.claude/strategy/visual-editor.md).

### Sprint 4C: Input & Output Nodes — COMPLETE

Self-describing recipes via `input` and `output` node types (PR #102). 4 waves: Wave 1 (`@bnto/nodes` — I/O types, schemas, recipe updates, 22 tests), Wave 2 (`@bnto/core` adapter reads I/O nodes, editor store singleton constraints), Wave 3 (generic InputRenderer/OutputRenderer, I/O compartment rendering), Wave 4 (RecipeShell migration, per-slug I/O code deleted, E2E verified). See [io-nodes.md](.claude/strategy/io-nodes.md).

### Sprint 4D: Extract `@bnto/ui` (Motorway Design System) — COMPLETE

Moved all UI primitives, design tokens, and shared components from `apps/web/components/` to `packages/ui/` as `@bnto/ui`. Zero domain knowledge — pure visual building blocks. 3 waves: package scaffold + primitives, shared components, rewire + verify (PR #103).

### Sprint 4E: Extract `@bnto/editor` — COMPLETE

Moved all editor components from `apps/web/components/editor/` to `packages/editor/` as `@bnto/editor`. Editor depends on `@bnto/ui` + `@bnto/core` + `@bnto/nodes`. 2 waves: package scaffold + move, rewire + verify. 90 editor tests + 66 web tests pass.

### Sprint 4F: Code Standards Review — COMPLETE

Audited all active code against updated `code-standards.md` (March 2026 tightened limits). 3 waves: per-package file size + structure audit (all 6 packages), cross-cutting DRY + Object.assign + Server Component audit, Zustand store ownership audit. Every file conforms.

### Sprint 4G: Versioning & Node Validation — COMPLETE

Format versioning activated across the stack. Zod schemas replaced hand-rolled `ParameterSchema` DSL for all 15 node types. Schema-driven config panel with `CONTROL_REGISTRY` map dispatching Zod-inferred `FieldControl` types to `@bnto/ui` controls. 3 waves: format version constants + schema version field, Zod migration + validation function, schema-driven `SchemaForm` + `SchemaField` components (PRs #114-#116).

### Sprint 4H: Pipeline Executor Extraction — COMPLETE

Runtime-agnostic `executePipeline()` extracted to `@bnto/core`. `NodeRunner` contract, `processFiles()` removed from browser adapter. Comprehensive TDD test suite (pure Node.js, no browser). 4 waves: types + tests, implementation, adapter cleanup, export + E2E verification.

### Sprint 5D: Editor API Layer — COMPLETE

`createEditor()` factory with `client → service → store` abstraction mirroring `@bnto/core`. 5 domain clients (nodes, definition, execution, history, panels), 5 services, React binding layer (`EditorProvider`, `useEditor`, domain hooks), full component migration, deprecated hooks deleted. 5 waves.

### Editor Beta Launch — COMPLETE

Feature flag removed, beta badges on nav/CTAs, dismissible banner on `/editor` with localStorage persistence. E2E verified (PR #173).

### Sprint 5A: Editor UX — COMPLETE

Node interaction + empty state + config polish. 5 waves: hover delete overlay + PlaceholderSlot + isIoNode flag, exit animations (react-animate-presence + tailwindcss-motion), config panel identity echo (node icon + empty state + SchemaForm field grouping), LayerPanel drag-to-reorder (@dnd-kit/sortable), empty canvas auto-behaviors (auto-open palette, auto-select Input), E2E verification.

### Sprint 5B Wave 1: I/O Node Visual Hierarchy — COMPLETE

Size differentiation (100×100 vs 120×120), muted color for I/O nodes, elevation distinction (sm vs md), Pressable behavior split (I/O not pressable-to-configure). Unit tests for all visual distinctions.

### Sprint 5C: Editor Copy + Nav Labels — COMPLETE

Renamed nav "Create" → "New Recipe", recipe page CTA "Customize in Editor" → "Open in Editor". Grep-verified no remaining old copy.

### Sprint 5: Editor v1 (M2 Completion) — COMPLETE

Editor shipped as usable v1: auto-download default, config panel controls (Textarea, Combobox, KeyValueEditor + control registry wiring), schema metadata cleanup (hidden params, DRY recipe I/O), save to account + My Recipes integration + unsaved changes warning, keyboard shortcuts (undo/redo/delete/run/export), accessibility audit. All 4 waves complete. **M2 milestone delivered.**

---

## What's Next

**Sprint 8 complete.** All 3 waves delivered. 4 Tier 3 engine operations (strip-exif, merge-csv, csv-to-json, watermark/image-overlay), recipe fixtures, golden tests, codegen, SEO pages, E2E tests, and Lighthouse audit all done. Schema-driven config (8.5c) and editor reconnect (8.5d) both complete. **v0.2.0 released.**

**After Sprint 8:** Engine expansion (M3). CLI-first development — dependency system, video node type, TUI, CLI polish. Desktop and monetization deferred. See [engine-expansion.md](strategy/engine-expansion.md) for the full strategy.

---

## Completed Sprint

### Sprint 6: Quality & Cleanup — COMPLETE

**Goal:** Lock in quality after M2. Clean up dead code, add error boundaries, audit performance, resolve triage items. No new features — stabilize what's built before expanding.

**Persona ownership:**

| Package                | Persona                                 |
| ---------------------- | --------------------------------------- |
| `apps/web`             | `/frontend-engineer` + `/nextjs-expert` |
| `packages/core`        | `/core-architect`                       |
| `packages/@bnto/nodes` | `/core-architect`                       |
| `engine`               | `/rust-expert`                          |

#### Wave 1 (parallel — error boundaries + dead code)

- [x] `apps/web` — **Global error boundary**: Create `buildGitHubIssueUrl()` pure function + `ErrorReport` component + `global-error.tsx` + `(app)/error.tsx` + `[bnto]/error.tsx`. Unit tests for URL construction. PostHog `app_error` telemetry on boundary trigger.
- [x] `packages/core` — **Dead code removal**: Verified — `processFile` already removed in Sprint 4H, `hasImplementation()` already removed, `executePipeline` is active (JS↔WASM adapter, not redundant). No action needed.
- [x] `packages/@bnto/nodes` — **Align stale schemas**: Verified — schemas are auto-generated from Rust engine catalog via `task nodes:generate`. Hand-written wrappers only add `hidden: true` on operation field. No Go-era operations remain.

#### Wave 2 (parallel — Go archive + Rust cleanup)

- [x] `archive/` — **Delete Go engine**: Deleted `archive/engine-go/`. Removed `go.work`. Updated `.gitignore`, `Taskfile.yml`, `bnto.code-workspace`, `README.md`, `CLAUDE.md`.
- [x] `archive/` — **Delete Go API**: Deleted `archive/api-go/`. Deleted `Dockerfile.api`. Updated `.dockerignore`. Updated test fixture references in `transit-helpers.ts`.
- [x] `infra` — **Clean up Taskfile + CI**: Removed all Go tasks from Taskfile. Updated `build:all`/`test:all` to Rust + TS only. Removed `dev:all`. No Go-related CI checks found.
- [x] `engine` — **Split `executor.rs`**: Split `executor/mod.rs` (523 lines) into three focused modules: `mod.rs` (299 lines — public API, dispatch, shared types), `primitive.rs` (184 lines — leaf node execution), `container.rs` (230 lines — loop/group/parallel containers + sub-pipeline). Comment density pass for consistency. All 437 tests pass.

#### Wave 3 (parallel — performance + stale references)

- [x] `apps/web` — **Server Component audit**: Removed `"use client"` from 15 pure presentational UI components (animation, layout, surface, feedback). Pushed editor page client boundary down (page.tsx → EditorShell island). Extracted recipe page static header to server-rendered page.tsx. Lazy-loaded config components. Moved currentUser fetch to self-fetching SessionMarker leaf. No `ssr: false` anti-patterns found.
- [x] `apps/web` — **Lighthouse audit**: Run `/lighthouse-audit --local` across all public pages. Fix failing a11y, SEO, best-practices assertions.
- [x] Cross-cutting — **Go reference sweep**: Grep for "Go engine", "Go API", "archive/engine-go" in non-archive code. Remove stale references. Update CLAUDE.md, architecture.md, ROADMAP.md.
- [x] `.claude/` — **Docs cleanup**: Update "What's Built" in PLAN.md, remove Go engine from CLAUDE.md Repository Structure, update architecture.md data flow diagram.

#### Wave 4 (parallel — triage batch)

- [x] `apps/web` — **Simplify My Recipes page**: Remove stat cards and history section. Show saved recipes grid or empty state.
- [x] `packages/ui` — **SelectTrigger press animation**: Add pressable spring effect matching Menu trigger.
- [x] `packages/ui` — **PopupTrigger shared component**: Unify Menu, Select, Combobox trigger styling (pressable spring, surface, chevron).
- [x] `packages/editor` — **File menu icons**: Add icons to "Open" and "Export" menu items for visual uniformity.
- [x] `packages/editor` — **Raw useStore audit**: Migrate raw `useStore(storeApi, ...)` calls to domain hook factories. All reads through editor API layer.
- [x] `apps/web` — **Fix reducedMotion type errors**: Fix `reducedMotion` type errors in E2E spec `test.use()` calls.
- [x] `apps/web` — **Remove redundant default props**: Audit for components passing props matching defaults (e.g., `size="md"` when `md` is default).
- [x] `apps/web` — **Home page marquee**: Replace static RecipeGrid with scrolling Marquee component (Magic UI pattern) to keep hero content above the fold.
- [x] `packages/editor` — **File menu transform origin**: Fix popover/menu animation direction — transform origin should account for trigger position.
- [x] `packages/editor` — **I/O node mode labels**: Display current mode (Upload, Text, URL) on Input/Output compartment nodes.
- [x] `packages/editor` — **Pre-populate extension TagPicker**: Ship Input node file extension TagPicker with a static list of common extensions (.jpg, .png, .csv, .pdf, etc.).
- [x] `apps/web` — **Kbd component + shortcuts dialog**: Create `<Kbd>` primitive for shortcut hints on menu items. Add `Cmd+/` keyboard shortcuts dialog.

#### Wave 5 (parallel — final quality + triage cleanup)

- [x] `apps/web` — **Replace competitor comparison with bnto-first benchmarks**: Rewrite the "How It Works" section's BragLayout to showcase bnto's own capabilities (50ms local WASM, zero uploads, unlimited runs, open source) instead of the TinyPNG/iLoveIMG comparison chart and feature table. Focus on the landscape of problems bnto solves.
- [x] `apps/web` — **Delete button on My Recipe cards**: Add delete action to saved recipe cards on `/my-recipes`. Wire `core.recipes.remove()` to a confirmation dialog on RecipeCard.
- [x] `packages/editor` + `@bnto/core` — **Auto-save recipes**: Replace manual Save with transparent persistence — localStorage if unauthed, Convex if authed. Download/Export remains the one manual action. Save removed from file menu and keyboard shortcut. Debounced auto-save to localStorage (PR #204, #205). Local recipe browsing for unauthed users on `/my-recipes` (PR #212).
- [x] `engine` — **Thin Rust comment density**: Reduce inline comment noise — keep file-level headers and comments on genuinely complex logic, remove obvious per-line explanations. Update CLAUDE.md Rust standards section.
- [x] Cross-cutting — **Inline handler audit**: Extract inline `onClick={() => ...}` handlers to named `handleOnX` functions across `packages/ui/`, `packages/editor/`, `apps/web/components/`.
- [x] Cross-cutting — **CSS-first interaction audit**: Identify JS `useState`/ternary className patterns for visual states that CSS pseudo-classes or `data-*` attributes could handle. Fix violations in `packages/ui/`, `packages/editor/`, `apps/web/components/`.
- [x] Cross-cutting — **Test naming unification**: Audit all test suites for naming consistency — clear action-oriented descriptions, consistent prefixing, logical grouping. Remove duplicate or vague test names.
- [x] `apps/web` — **Standardize E2E selectors on data-testid**: Audit E2E specs and replace fragile `getByRole`/`getByText` selectors with `data-testid` attributes for state detection and element targeting. Keep semantic selectors only for accessibility assertions. Priority: menu items, toolbar buttons, panel controls. _(PR #203 + #208, merged 2026-03-16)_
- [x] `apps/web` + `packages/editor` — **Local recipe persistence for unauthenticated users**: Non-authed users auto-save to localStorage but have no way to browse saved recipes outside the editor (My Recipes is auth-gated). Open `/my-recipes` to unauthenticated users with localStorage-backed recipe list. Add upsell messaging: "Your recipes are saved locally on this device. Sign in to sync across devices and never lose your work." This gives unauthenticated users a reason to explore saving, and gives us a natural conversion hook. Scope: (1) remove `/my-recipes` from `PROTECTED_PATHS`, (2) build a localStorage recipe list adapter (list/load/delete), (3) show local recipes in RecipeGrid when unauthed, Convex recipes when authed, (4) add upsell banner/card, (5) update proxy tests. _(Completed in `feat/auto-save-local-recipes` branch)_

#### Wave 6 (parallel — Button simplification + polish)

- [x] `packages/ui` — **Simplify Button behavioral props — CSS-first with data attributes**: Remove `pressed` and `hovered` JS props from Button. Replace with CSS-driven data-state attributes (like `dormant` already is). Specific changes: remove `hovered` prop (use CSS `:hover` or `data-hover`), remove `pressed` prop (replace with `data-active` driven by `:active`/`aria-pressed`/`aria-current`), evaluate `muted` vs `variant="muted"`, keep `dormant` and `toggle` (already CSS-first), audit NodeRoot (biggest consumer), swap dormant/disabled visuals (dormant darkens → should be subtler; disabled should use opacity). Remove `sm` and `lg` size variants — keep only `md`. Emphasis/hierarchy controlled through elevation, not size. Review in ButtonShowcase.
- [x] `apps/web` — **Theme menu lighting direction control**: RadialSlider (135°–225°) in NavThemeMenu, Zustand store with localStorage persistence, `--light-angle` CSS variable driving surface shadow system via sin/cos. FOUC prevention via blocking script.

---

## Completed Sprint

### Sprint 7: Explore & Discovery Infrastructure (Tier 2) — COMPLETE

**Goal:** Unify how recipes and nodes are listed across all surfaces, then build a dedicated Explore page. When this sprint is done, adding a recipe to `@bnto/nodes` automatically appears on every surface (home, Explore page, editor palette, sitemap, README). This is a prerequisite for Tier 3 recipe expansion.

**Problem:** Currently 5+ surfaces list recipes/nodes using different data sources and transforms:

- ~~Home: `RecipeGrid` → `BNTO_REGISTRY`~~ — **RESOLVED**: `RecipeMarquee` → `core.registry.useRecipes()`
- Navbar: `RecipesMenu` → `navData.ts` `buildRecipeCategories()` (build-time, keeps direct `@bnto/nodes` import)
- ~~Editor palette: `useNodePalette` → `NODE_TYPE_INFO` + `CATEGORIES` + `PROCESSORS`~~ — **RESOLVED**: `core.registry` hooks
- ~~Editor open dialog: `RecipePickerGrid` → `RECIPES` from `@bnto/nodes`~~ — **RESOLVED**: `core.registry.useRecipes()`
- Tool pages + sitemap: `bntoRegistry.ts` → `generateStaticParams` (build-time, keeps direct imports)
- README: Hardcoded recipe list — will drift as recipes grow
- ~~Editor URL: `?from={slug}` (predefined) vs `?recipe={id}` (saved)~~ — **RESOLVED**: unified to `?recipe={id}` via clone-on-click

**Persona ownership:**

| Package       | Persona                                 |
| ------------- | --------------------------------------- |
| `@bnto/core`  | `/core-architect`                       |
| `@bnto/nodes` | `/core-architect`                       |
| `apps/web`    | `/frontend-engineer` + `/nextjs-expert` |

#### Wave 1 (parallel — audit + cleanup + URL unification)

- [x] `@bnto/nodes` + `apps/web` — **Audit all listing surfaces**: Map every component/hook that lists recipes or nodes. Document data source, transform, filtering, and output shape for each. Identify divergences (missing recipes, different categories, stale hardcoded lists). Produce a comparison table. _(Results: 15 surfaces audited, README.md stale (6/8 recipes), all dynamic surfaces trace to `@bnto/nodes` RECIPES. See `strategy/unified-recipe-model.md`)_
- [x] `@bnto/core` — **Design unified recipe/node query API**: Propose how `@bnto/core` exposes a single query that all surfaces consume. Consider: should this be a core client (`core.catalog` or `core.explore`), or a query-only API? What filtering/grouping capabilities does it need? Write a brief design doc or add to `core-api.md`. _(Decision: a Recipe IS a Definition. Eliminate both `Recipe` wrapper types, delete `RecipeDefinition` duplicate. Persist `Definition` objects directly. Publishing metadata in web registry, persistence in thin store envelope. `core.catalog` client for unified surface access. Full design in `strategy/unified-recipe-model.md`)_
- [x] `apps/web` — **Unify editor URL pattern**: Eliminated `?from={slug}`. "Open in Editor" clones template into personal store via `core.recipes.createFromTemplate()`, navigates to `/editor?recipe={id}`. `editorUrl(id)` centralised in `lib/routes.ts`. All consumers updated. _(PR #228)_
- [x] `apps/web` — **Consolidate Recipe types**: Unified recipe model implemented — `Recipe` layered type in `@bnto/nodes`, `UserRecipe extends Recipe` in `@bnto/core`. `BntoEntry` preserved as thin wrapper derived from `Recipe`. _(PR #226)_

#### Wave 2 (parallel — unified recipe model: type migration)

Design doc: `strategy/unified-recipe-model.md`

- [x] `@bnto/nodes` — **Layered Recipe type**: `Recipe` wraps `Definition` with display metadata (id, slug, name, description, category, accept, features). All 8 predefined recipes use UUID ids. `deriveCategory()` added. No `SEOSpec` — web layer derives SEO from `recipe.name`. _(PR #226)_
- [x] `@bnto/core` — **Delete `RecipeDefinition`, simplify persistence**: Deleted duplicate types. `UserRecipe extends Recipe` adds persistence fields (`cloudId`, `savedAt`, `syncedAt`). `recipesStore`, `recipeClient`, `recipeService`, transforms all updated. `RecipeListItem` derived from `UserRecipe`. _(PR #226)_
- [x] `apps/web` — **Refactor `bntoRegistry.ts`**: `BntoEntry` preserved as thin wrapper, now derived from `Recipe` (not standalone). SEO derived from `recipe.name`. All consumers updated (RecipeMarquee, tool pages, sitemap, navData, BntoJsonLd). _(PR #226)_
- [x] `@bnto/core` — **Build `core.registry` client**: 6th domain on the `core` singleton. Zustand store with `populate()`. Client API: `getRecipes()`, `getNodeTypes()`, `getCategories()`, `getProcessors()`, `getRecipesByCategory()`, `getBrowserNodeTypes()`. React hooks: `useRecipes()`, `useNodeTypes()`, `useCategories()`, `useProcessors()`. _(PR #227)_

#### Wave 3 (parallel — surface migration + Explore page)

- [x] `apps/web` — **Migrate runtime surfaces to `core.registry`**: RecipeMarquee, RecipeCardShowcase consume `core.registry` hooks. Build-time surfaces (navData, sitemap, llms.txt) keep direct `@bnto/nodes` imports (SSG can't use Zustand). _(PR #229)_
- [x] `packages/editor` — **Migrate editor surfaces to `core.registry`**: `useNodePalette` and `RecipePickerGrid` consume `core.registry` hooks instead of direct `@bnto/nodes` imports. _(PR #229)_
- [x] `apps/web` — **Build `/explore` page**: Full-page searchable/filterable recipe & node browser. Categories, search, metadata cards. Server component page with client interactive leaves. Uses `core.registry`. _(PR #281)_
- [x] `apps/web` — **Migrate navbar Explore**: Replace dropdown with a link to `/explore`. Keep a compact "quick access" subset if desired, but primary action is navigating to the Explore page.

#### Wave 4 (sequential — verify + auto-generation)

- [x] `apps/web` — **SEO verification**: Ensure `generateStaticParams`, `generateMetadata`, sitemap, and `llms.txt` all derive from `core.catalog`. Adding a Definition to `@bnto/nodes` = it appears everywhere.
- [x] `apps/web` — **E2E tests**: Verify Explore page renders, search/filter works, recipe cards link to tool pages. Verify editor palette and open dialog still show correct items. Page-level screenshots for `/explore`.
- [x] Repo root — **Auto-generate README recipe list**: The predefined recipe table in `README.md` should be generated from `@bnto/nodes` RECIPES registry (like `llms.txt`). Add a script or codegen step so the README stays current as recipes grow.

---

## Tabled Sprints

### Deep Editor Features — TABLED (post-revenue)

**Editor is coming back lightweight (Sprint 8.5d) but deeper features remain tabled.** The `@bnto/editor` package is at v1 and architecturally isolated. These items resume if demand signals indicate users want advanced recipe creation tooling.

- **Edit Mode / Run Mode** — Mini Motorways edit/run switch. See `.claude/decisions/editor-ux-direction.md`.
- **Sprint 5B Waves 2-4** — LayerPanel polish, processing node accents. Cosmetic.
- **Code Editor (CM6)** — Schema-aware JSON editor. Power-user luxury.
- **Expression Input** — Pill tokens, variable picker. Needed for Tier 4+ nodes.
- **Recipe Persistence** — Save to Convex, localStorage sync, My Recipes dashboard. Revisit when favorites/persistence has product demand.
- **All editor triage items** — Consolidated in backlog under "Deferred: Editor Investment."

---

## Active Sprint

### Sprint 8: Tier 3 Near-Term Recipes

**Goal:** Expand the recipe catalog with high-SEO-value recipes that run 100% client-side. Each recipe needs: Rust engine operation, `@bnto/nodes` recipe fixture, SEO page with metadata + JSON-LD, E2E verification. This is the first product expansion since M1.

**Why now:** Sprint 7 unified the discovery infrastructure — adding a recipe to `@bnto/nodes` now automatically appears on every surface. The pipeline is ready for new recipes. Tier 3 recipes target high-volume search queries (watermark: 30K/mo, strip-exif: 15K/mo, merge-csv: 12K/mo, csv-to-json: 25K/mo).

**Prerequisite:** Sprint 7 must be complete (all surfaces unified, `/explore` page live with E2E).

**Persona ownership:**

| Package       | Persona              |
| ------------- | -------------------- |
| `engine`      | `/rust-expert`       |
| `@bnto/nodes` | `/core-architect`    |
| `apps/web`    | `/frontend-engineer` |

#### Wave 1 (parallel — engine operations)

- [x] `engine` — **`bnto-image`: image-overlay/watermark operation** — overlay text or image onto source. Needed for `/watermark-images` (Tier 3, 30K+ monthly searches). Position, opacity, scale, color params. Golden tests for multiple orientations and positions (PR #308, #309)
- [x] `engine` — **`bnto-image`: EXIF metadata strip** — strip all EXIF data from images. Needed for `/strip-exif` (Tier 3, 15K+ monthly searches)
- [x] `engine` — **`bnto-csv`: merge operation** — concatenate + deduplicate multiple CSVs. Needed for `/merge-csv` (Tier 3, 12K+ monthly searches)
- [x] `engine` — **`bnto-csv`: CSV-to-JSON conversion** — transform CSV rows to JSON objects. Needed for `/csv-to-json` (Tier 3, 25K+ monthly searches)

#### Wave 2 (parallel — recipes + codegen)

- [x] `@bnto/nodes` — **Recipe fixture + codegen for watermark-images**: `.bnto.json` definition created, codegen propagated, Zod schemas verified (PRs #308, #309)
- [x] `engine` — **CLI golden tests for watermark-images**: 10+ golden test fixtures covering orientations, positions, opacity, scale (PRs #308, #309)
- [x] `@bnto/nodes` — **Recipe fixtures for strip-exif, merge-csv, csv-to-json**: `.bnto.json` definitions created, codegen propagated (PRs #294, #296)
- [x] `engine` — **CLI golden tests for strip-exif, merge-csv, csv-to-json**: Golden test fixtures with byte-exact verification (PRs #294, #296)

#### Wave 3 (parallel — SEO pages + E2E)

- [x] `apps/web` — **SEO pages for Tier 3 recipes**: Verified — all 4 Tier 3 recipes auto-propagate via `getAllRecipes()` → `BNTO_REGISTRY`. `generateStaticParams`, `generateMetadata`, JSON-LD, sitemap all working. Added NAV_DESCRIPTIONS for strip-exif, merge-csv, csv-to-json, watermark-images
- [x] `apps/web` — **E2E tests**: Verified — all 4 Tier 3 recipes have comprehensive E2E specs (11 tests total: strip-exif 2, merge-csv 1, csv-to-json 2, watermark-images 6). Programmatic assertions with magic bytes, content validation, pixel diffs. All pass
- [x] `apps/web` — **Lighthouse audit**: All 18 public pages pass accessibility, SEO, best-practices thresholds. 4 Tier 3 pages verified clean

---

## Sprint 8.5: Simplify Config, Reconnect Editor Lightweight

**Goal:** Three phases. Disconnect editor (done), make recipe config schema-driven (any recipe gets controls for free), then reconnect the editor as a lightweight open+export tool with no persistence. Favorites tabled — user preferences out of MVP scope.

**Why now:** Tool pages need schema-driven config to support Tier 3 recipes without hand-crafted per-recipe components. The editor needs to come back — but without the persistence complexity (localStorage sync, auto-save, Convex save, My Recipes) that was stripped in 8.5a.

**Persona ownership:**

| Package         | Persona                                 |
| --------------- | --------------------------------------- |
| `apps/web`      | `/frontend-engineer` + `/nextjs-expert` |
| `@bnto/core`    | `/core-architect`                       |
| `@bnto/backend` | `/backend-engineer`                     |

### Sprint 8.5a: Disconnect Editor + Slash Dead Code

Pure deletion + reference cleanup. Low risk. All changes ship together.

#### Delete entirely

- [x] `apps/web/app/editor/` (entire directory — page, layout, loading, \_components/)
- [x] `apps/web/components/blocks/NewRecipeNavButton.tsx`
- [x] `apps/web/components/blocks/NewRecipeMobileButton.tsx` (if exists)
- [x] `apps/web/app/(app)/[bnto]/_components/OpenInEditorLink.tsx`
- [x] `apps/web/e2e/journeys/editor/` (entire directory)
- [x] `apps/web/e2e/editor/` (entire directory)
- [x] `apps/web/e2e/helpers/editor.ts`
- [x] `apps/web/e2e/helpers/editor-save.ts`
- [x] `apps/web/e2e/helpers/editor-execution.ts` (if exists)
- [x] `packages/core/src/stores/recipesStore.ts`
- [x] `packages/core/src/stores/mergeCloudRecipes.ts` + test
- [x] `packages/core/src/hooks/useRecipeSync.ts`
- [x] `packages/core/src/hooks/useRecipes.ts` (store-backed version)
- [x] `packages/core/src/hooks/useRemoveRecipe.ts`
- [x] `packages/core/src/transforms/cloudRecipeToUserRecipe.ts` + test
- [x] `packages/core/src/transforms/recipeToListItem.ts` + test
- [x] `packages/core/src/fileTransfer.ts`

#### Modify (web app)

- [x] `apps/web/components/blocks/DesktopNav.tsx` — remove NewRecipeNavButton import + JSX
- [x] `apps/web/components/blocks/MobileNavMenu/MobileNavActions.tsx` — remove NewRecipeMobileButton
- [x] `apps/web/app/(app)/[bnto]/_components/RecipeShell.tsx` — remove OpenInEditorLink
- [x] `apps/web/lib/routes.ts` — remove `editor` from ROUTES, delete `editorUrl()`
- [x] `apps/web/next.config.ts` — remove `@bnto/editor` from transpilePackages
- [x] `apps/web/components/blocks/RecipeMarquee.tsx` — remove CtaCard linking to /editor
- [x] `apps/web/components/blocks/HeroPitchPoints.tsx` — reword editor reference
- [x] `apps/web/components/blocks/PricingTiers.tsx` — reword editor feature line
- [x] `apps/web/components/blocks/FAQAccordion.tsx` — reword editor FAQ answer
- [x] `apps/web/playwright.config.ts` — remove editor project block
- [x] `Taskfile.yml` — remove `e2e:editor` task, update `e2e` task

#### Modify (core)

- [x] `packages/core/src/BntoCoreProvider.tsx` — remove useRecipeSync from SyncProvider
- [x] `packages/core/src/clients/recipeClient.ts` — gut to keep only `run()`, query options, cache invalidation
- [x] `packages/core/src/reactCore.ts` — remove useRecipes, useRemoveRecipe
- [x] `packages/core/src/index.ts` — remove UserRecipe, RecipeListItem, fileTransfer exports
- [x] `packages/core/src/types/recipe.ts` — remove UserRecipe and RecipeListItem
- [x] `packages/core/src/adapters/convex/recipeAdapter.ts` — remove fetchCloudRecipes

#### Modify (My Recipes — temporary placeholder)

- [x] `apps/web/app/(app)/my-recipes/page.tsx` — placeholder until Sprint 8.5b
- [x] Delete `apps/web/app/(app)/my-recipes/_components/` (entire directory)

#### Critical preservation

- `core.recipes.run()` → executionService path stays intact (tool pages use it)
- `recipeService.ts` stays (query options)
- `recipeAdapter.ts` keeps getRecipesQuery, getRecipeQuery

#### Verify

- `task ui:build` + `task ui:test`
- Tool pages execute end-to-end
- `/editor` 404s
- Knip clean (no dead imports)

### Sprint 8.5b: Favorites + My Recipes — TABLED

**Tabled (March 2026).** User preferences/engagement features deferred to post-MVP. Favorites, My Recipes page, and Convex-backed bookmarks moved to backlog. Revisit when engagement data signals demand. See backlog section "Tabled: Favorites + My Recipes."

### Sprint 8.5c: Schema-Driven Recipe Config

Replace ~600 LOC of hardcoded per-recipe config with dynamic schema-driven config. `@bnto/form` already exists with `SchemaForm`, `CONTROL_REGISTRY` (8 controls), `buildFormEntries()`, and `SchemaField`. The editor uses it today — this sprint wires it into tool pages.

**Why this unblocks everything:** After 8.5c, adding a recipe to `@bnto/registry` = automatic config UI on its tool page. No hand-crafted component needed. Tier 3 recipes (strip-exif, merge-csv, csv-to-json) get config controls for free.

#### Wave 1 (parallel — build + modify)

- [x] `apps/web` — **DynamicRecipeConfig component**: Create `[bnto]/_components/DynamicRecipeConfig.tsx`. Reads recipe definition, walks processing nodes, looks up each node's `NodeSchemaDefinition` from `@bnto/form`, renders `SchemaForm` per processing node. Multi-node recipes (optimize-images-for-web) render multiple config sections.
- [x] `apps/web` — **Update recipeFlowStore config shape**: Config state becomes `Record<nodeId, Record<string, unknown>>` (per-node configs). `setConfig` action accepts nodeId + partial params. Default config populated from schema defaults on mount.
- [x] `apps/web` — **Wire execution path**: Before engine call, merge per-node config back into definition's node parameters. Pure function: `applyConfigToDefinition(definition, configs) → Definition`.

#### Wave 2 (sequential — swap + delete + verify)

- [x] `apps/web` — **Replace config registry**: RecipeShell/RecipeConfigSection uses DynamicRecipeConfig instead of the lazy-loaded registry. Delete all files in `[bnto]/_components/configs/` (CompressImagesConfig, ResizeImagesConfig, ConvertFormatConfig, RenameFilesConfig, CleanCsvConfig, RenameCsvColumnsConfig, OptimizeImagesForWebConfig, GenerateThumbnailsConfig, FormatSelect, RenamePatternPreview, registry.tsx, types.ts, useConfigChange.ts, formatOptions.ts + test).
- [x] `apps/web` — **Verify all recipes**: Every recipe renders correct controls dynamically. Defaults match previous hardcoded values. Execution output identical. New recipes (strip-exif, merge-csv, csv-to-json) get config UI for free.
- [x] `apps/web` — **E2E verification**: Run existing recipe execution E2E tests. Verify config controls render and produce correct output.

### Sprint 8.5d: Reconnect Editor (Open + Export Only)

Bring back the `/editor` route as a lightweight open+export tool. No persistence — no save to Convex, no localStorage, no My Recipes page. sessionStorage keeps the working recipe alive across page refreshes (scoped to the browser tab).

**Mental model:** Every editor session starts by creating a new recipe (new UUID) from either a blank canvas or a predefined template. The UUID goes in the URL (`/editor?recipe={uuid}`) and keys the sessionStorage entry. Refresh reloads from sessionStorage. Close the tab → gone.

**What comes back (from PR #299):**

- `/editor` route (page, layout, loading)
- Nav items to navigate to editor (desktop + mobile)
- "Open in Editor" link on tool pages
- `@bnto/editor` in transpilePackages + web app dependencies
- Editor route in `lib/routes.ts`
- Editor E2E tests (minus all persistence/save tests)
- Marketing copy re-mentions editor where appropriate

**What came back (simplified):**

- `core.recipes` domain — re-added with sessionStorage persistence (no Convex)
- `recipesStore` — Zustand + sessionStorage via `createEnhancedStore`
- `recipeClient` — `createFromDefinition()`, `get()`, `remove()`, `count()`
- `UserRecipe` type — simplified (extends `Recipe`, adds `savedAt`, no `cloudId`)
- `fileTransfer` — in-memory File[] stash for SPA navigation
- `useRecipe`, `useRecipes`, `useRemoveRecipe` hooks — thin store subscribers
- Beta dialog — dismissible, explains import/export + future cloud persistence
- FAQ/Pricing copy — updated to mention editor + import/export + future account persistence

**What stays deleted (NO cloud persistence):**

- `mergeCloudRecipes`, `useRecipeSync` — no cloud sync
- `RecipeListItem`, `CloudRecipeDetail` types — no cloud types
- `my-recipes/` page and all components — no My Recipes dashboard
- `SavedRecipeCard`, `RecipeCardMenu` — no saved recipe UI
- Auto-save to Convex, localStorage sync — no cloud persistence
- Editor save E2E tests (`editor-save.spec.ts`, `recipe-persistence.spec.ts`)

**Persona ownership:**

| Package    | Persona                                 |
| ---------- | --------------------------------------- |
| `apps/web` | `/frontend-engineer` + `/nextjs-expert` |

#### Wave 1 (parallel — recipe domain + editor route)

- [x] `@bnto/core` — **Recipe domain**: `core.recipes` re-added as 7th domain. `recipesStore` (Zustand + sessionStorage), `recipeClient` (`createFromDefinition`, `get`, `remove`, `count`), `UserRecipe` type (simplified — no `cloudId`), `fileTransfer` (in-memory File[] stash), `useRecipe`/`useRecipes`/`useRemoveRecipe` hooks.
- [x] `apps/web` — **Editor page route**: Restored `app/editor/page.tsx` + `layout.tsx` + loading skeleton + `useEditorRecipe` hook. Page reads `?recipe={uuid}` search param, loads definition from `core.recipes.get()`, passes to `EditorProvider`. Beta dialog with dismissible wording about import/export + future persistence.

#### Wave 2 (parallel — nav integration + "Open in Editor")

- [x] `apps/web` — **Restore nav items**: NewRecipeNavButton ("Create" + Beta badge) and mobile equivalent. Click → navigate to `/editor`.
- [x] `apps/web` — **Restore "Open in Editor" on tool pages**: OpenInEditorLink + RecipeStepperEditButton in recipe stepper toolbar. Click → `core.recipes.createFromDefinition()` → `stashFilesForTransfer()` → navigate to `/editor?recipe={uuid}`.
- [x] `apps/web` — **Update routes + config + copy**: `editorUrl()` in `lib/routes.ts`, `@bnto/editor` in `transpilePackages` + `package.json`. FAQ updated (import/export, future accounts). Pricing updated (Pro: cloud recipe persistence). README updated (editor route). Beta dialog wording updated.

#### Wave 3 (parallel — auto-persist + export + E2E)

- [x] `apps/web` — **sessionStorage auto-persist**: Zustand `persist` middleware in `recipesStore` writes to sessionStorage automatically. Refresh reloads seamlessly. _(PR #305)_
- [x] `apps/web` — **Export/download verification**: File menu Export downloads `.bnto.json`. E2E test `XP1` verifies valid JSON output. Drift checks in custom recipe specs verify structural fidelity. _(PR #305)_
- [x] `apps/web` — **E2E tests**: Restored editor E2E config in `playwright.config.ts` + `Taskfile.yml`. 7 spec files: entry, build, custom recipes (drift checks), predefined, features (export + auto-download), from-scratch, stale recipe resilience. _(PR #305)_
- [x] `apps/web` — **Verify**: Quality gate passed. `/editor` renders. Tool page "Open in Editor" works. Export works. Refresh preserves state. _(PR #305)_

---

## Phase 2: Engine Expansion (CLI-First)

**Goal:** Make the bnto CLI a powerful, standalone tool. Dependency management shipped, video node shipped. Next: CLI polish (list, info, progress), then codegen to propagate video node through TypeScript. TUI deferred to Sprint 10 (its own sprint with proper breakdown).

**Why this over Desktop:** The next interesting recipe (download-video via yt-dlp) requires external dependencies and can't run in a browser. Instead of building a desktop wrapper (Tauri), we invest in what makes the engine powerful — local execution, composability, and the Rust CLI. Desktop is deferred to M4.

**Strategy doc:** [engine-expansion.md](strategy/engine-expansion.md)

### Sprint 9: Engine Expansion

**Persona ownership:**

| Package  | Persona        |
| -------- | -------------- |
| `engine` | `/rust-expert` |

#### Wave 1 (parallel — dependency system + ProcessContext)

- [x] `engine/crates/bnto-core` — `/rust-expert` — Add `requires: Vec<Dependency>` to `NodeMetadata` (binary name, version constraint, install hint, homepage) _(PR #315)_
- [x] `engine/crates/bnto-core` — `/rust-expert` — `ProcessContext` trait: controlled system access (run commands, temp files, env vars). `NoopContext` for browser, `NativeContext` for CLI _(PR #318)_
- [x] `engine/crates/bnto-engine` — `/rust-expert` — Dependency checker: verify all required binaries before pipeline start. Clear error with install hints on missing deps _(PR #320)_
- [x] `engine/crates/bnto-cli` — `/rust-expert` — `bnto doctor` command: check all dependencies, report missing with install hints _(PR #320)_

#### Wave 2 (parallel — video node type)

- [x] `engine/crates/bnto-video` — `/rust-expert` — New crate: `video-download` processor wrapping yt-dlp. Purpose-built typed params: URL, format, quality, output format _(PRs #321-#329)_
- [x] `engine/crates/bnto-video` — `/rust-expert` — Register in `bnto-engine`, add `NodeTypeInfo` (category: "video", platforms: ["cli", "server", "desktop"]) _(PR #321)_
- [x] `engine/crates/bnto-video` — `/rust-expert` — Golden tests with test fixtures. Recipe: `download-video.bnto.json` _(PR #321)_
- [x] Codegen — Run `task wasm:codegen`. Verify new video category + node type propagates through TypeScript _(PR #336)_

#### Wave 3 (parallel — CLI polish)

- [ ] `engine/crates/bnto-cli` — `/rust-expert` — `bnto list` command: list available recipes with descriptions and categories
- [ ] `engine/crates/bnto-cli` — `/rust-expert` — `bnto info <recipe>` command: show recipe details, required dependencies, node types
- [ ] `engine/crates/bnto-cli` — `/rust-expert` — Enhanced `bnto run`: progress bars per file, colored output, timing summary
- [ ] `README.md` — Update to pitch CLI usage front and center

---

### Sprint 10: TUI — DEFERRED

**Deferred (April 2026).** TUI is its own application — recipe browser, file picker, progress display, results panel, navigation. Needs proper sprint breakdown with multiple waves. Revisit after CLI is bomb-proof (Sprint 9 W3 complete, backlog items addressed).

**Framework:** `ratatui` + `crossterm`

**Scope (needs breakdown when activated):**

- [ ] `engine/crates/bnto-cli` — `/rust-expert` — Interactive TUI mode (`bnto tui`). Recipe browser with categories and search
- [ ] `engine/crates/bnto-cli` — `/rust-expert` — File picker (browse filesystem, multi-select, drag semantics)
- [ ] `engine/crates/bnto-cli` — `/rust-expert` — Progress display (per-file progress bars, node status)
- [ ] `engine/crates/bnto-cli` — `/rust-expert` — Results panel (output files, sizes, timing, open/copy)
- [ ] `engine/crates/bnto-cli` — `/rust-expert` — Navigation (tab between panels, keyboard shortcuts, help overlay)
- [ ] `engine/crates/bnto-cli` — `/rust-expert` — Recipe config editing (param overrides in TUI before execution)

---

### Backlog: Distribution (Desktop + Server)

**Deferred from Phase 2.** Desktop (Tauri) and server-side execution moved to backlog. The Tauri plan is intact but deprioritized in favor of engine expansion.

#### Desktop App (Sprint 10, deferred)

- [ ] `apps/desktop` — Bootstrap Tauri desktop project
- [ ] `@bnto/core` — Desktop adapter (Tauri IPC bindings)
- [ ] `engine` — Expose engine functions for desktop bindings
- [ ] `apps/desktop` — Wire native ↔ React bindings, local file browser
- [ ] `apps/desktop` — macOS/Windows/Linux builds

#### Server-Side Execution (Sprint 11, deferred)

- [ ] Cloud execution infrastructure (technology TBD)
- [ ] Server-only node types (AI inference, video processing at scale)

---

## Phase 3: Monetization + Polish — TABLED

**Tabled (April 2026).** Monetization is explicitly paused. Focus is on engine power and fun. Revenue strategy revisited when the tool has community traction. The plan below is preserved for when this becomes relevant.

**"Ready to charge" gate:** Before starting, confirm: real users running bntos, conversion hooks built and tested, people return voluntarily, at least one server-side bnto (AI or shell) ready for Pro tier.

### Sprint 12: Stripe + Pro Tier (M5) — TABLED

**Goal:** First revenue. Pro sells real value — not artificial limits on browser-native operations.

**What Pro includes:** $8/month or $69/year. Saved workflows, execution history (30-day retention), team sharing (up to 5 members), server-side premium bntos (AI, shell, video — M4), priority processing, API access.

**What stays free forever:** All browser-capable bntos, unlimited runs, desktop app. See ROADMAP.md trust commitments.

**Persona ownership:**
| Package | Persona |
| ---------------- | ---------------------------------- |
| `apps/web` | `/frontend-engineer` |
| `@bnto/backend` | `/backend-engineer` |
| M4 cloud service | TBD (per M4 architecture decision) |

#### Wave 1 (parallel — payments)

- [ ] `apps/web` — `/frontend-engineer` — Stripe integration (checkout session, webhook handler, subscription sync to Convex)
- [ ] `@bnto/backend` — `/backend-engineer` — `planTier` updated on successful Stripe webhook (free → pro)
- [ ] `apps/web` — `/frontend-engineer` — Upgrade page (`/upgrade`) — pricing, Pro benefits, Stripe checkout CTA
- [ ] `apps/web` — `/frontend-engineer` — Billing management page (current plan, cancel, manage via Stripe portal)

#### Wave 2 (parallel — Pro feature gates)

- [ ] `@bnto/backend` — `/backend-engineer` — Pro feature gates: 30-day history retention, team sharing (up to 5 members), priority processing queue
- [ ] M4 cloud service — Server-side execution quota enforcement (applies to premium server-side bntos only — AI, shell, video). Technology TBD per M4 architecture decision
- [ ] `apps/web` — `/frontend-engineer` — File size enforcement at R2 presigned URL generation for server-side recipes (Pro-only, size limits TBD based on usage data)

#### Wave 3 (sequential — test)

- [ ] `apps/web` — `/frontend-engineer` — Playwright E2E: free user sees Pro conversion hooks (save, history, premium bntos)
- [ ] `apps/web` — `/frontend-engineer` — Playwright E2E: Pro user has access to saved workflows and execution history

---

## Immediate Backlog

### Editor: Smart Iteration — DELIVERED

**Status:** Delivered (March 2026)

Added `settings.iteration: "auto" | "explicit"` to the Definition. When `"auto"`, the engine wraps contiguous per-file processor sequences in implicit per-file loops — users get batch processing without explicit loop/group containers. Both modes produce byte-identical output (proven via 20 golden tests: 10 explicit + 10 flat). Recipe settings panel in ConfigPanel when no node is selected. See [smart-iteration.md](strategy/smart-iteration.md).

**What shipped:**

- Rust: `PipelineSettings`, `IterationMode`, `InputCardinality` types + `run_auto_iteration()` executor + JSON Schema
- TypeScript: types propagated through `@bnto/nodes` → `@bnto/registry` → `@bnto/core`
- 10 flat recipe fixtures + 10 golden equivalence tests
- Recipe Settings Panel (iteration mode toggle) in ConfigPanel when no node selected
- `rfNodesToDefinition` preserves `settings` on export

---

## Backlog

### Tabled: Favorites + My Recipes

**Tabled March 2026.** User preferences deferred to post-MVP. Revisit when engagement data signals demand.

Full scope when ready:

- `@bnto/backend` — `favorites` table (userId, recipeSlug, favoritedAt) with indexes, list/toggle/isFavorited mutations
- `@bnto/core` — `core.favorites` domain (adapter, service, client, hooks: useFavorites, useToggleFavorite, useIsFavorited)
- `apps/web` — FavoriteButton (heart toggle, prompts sign-in if unauthed), FavoritesGrid, My Recipes page rebuild, PROTECTED_PATHS update
- E2E: favorite → My Recipes → unfavorite

### Growth: Product Hunt Launch

**Priority: Backlog.** Launch bnto on Product Hunt when the product feels complete enough to show off. Ideal timing: after Sprint 8.5d (editor reconnected) + a few more Tier 3 recipes, so the catalog feels substantial and the editor provides a "wow" moment. Coordinate with a README polish pass and landing page review.

- [ ] Prepare Product Hunt listing (tagline, description, screenshots, maker comment)
- [ ] Review landing page + README for launch readiness
- [ ] Submit and engage on launch day

### UX: Unified Popup/FloatingSurface Primitive — COMPLETE

**Delivered.** `Popup` primitive in `@bnto/ui` (`packages/ui/src/overlay/Popup.tsx`) — wraps `Card elevation="lg"` + `ScaleIn from={0.6} easing="spring-bouncier"` + z-index/pointer-events. `PopupContent` and `PopupTrigger` compose with Radix. Dialog, Menu, and AuthGate all delegate to Popup.

- [x] `packages/ui` — Audit + design: shared floating surface patterns identified across Dialog, Menu, AuthGate
- [x] `packages/ui` — Implement `Popup` + `PopupContent` + `PopupTrigger` in `overlay/`
- [x] `packages/ui` — Dialog.Content uses `<Popup>`, Menu uses `<PopupContent>`, AuthGate uses both via composition

### UX: Standardize Forms with React Hook Form + Zod

**Priority: Medium.** React Hook Form + Zod for traditional forms (auth, settings). Decision doc: [decisions/form-library.md](decisions/form-library.md). Does NOT apply to recipe config (Zustand), NodeConfigPanel (`@bnto/nodes`), or code editor (CM6).

- [x] Evaluate form library — Decision: RHF + Zod
- [ ] `packages/@bnto/form` — Create package with auth schemas, `useSignInForm`, `useSignUpForm`
- [ ] `apps/web` — Refactor auth forms to use `@bnto/form` hooks
- [ ] `apps/web` — Migrate future forms (settings/profile) as they're built

### Infra: Shared Test Fixtures Package (`@bnto/test-fixtures`)

**Priority: Low.** Wrap `test-fixtures/` in a TS package with helpers. Currently served by direct file references.

- [ ] `packages/@bnto/test-fixtures` — Create package with TS helpers, add sample CSVs
- [ ] `apps/web` — Update E2E tests to import from shared package

### Security/Performance: File Count Limits & Abuse Guardrails Audit

**Priority: Medium.** Stress-test file count limits per recipe for performance (WASM heap, ZIP generation) and abuse prevention. Document safe boundaries per recipe type.

- [ ] `apps/web` + `engine` — Stress test file counts (50/100/200+ per recipe type), measure memory + processing time
- [ ] `apps/web` — Document recommended limits per recipe, decide enforcement strategy (soft warning vs hard cap vs batching)
- [ ] `apps/web` — UI performance audit at scale (FileCard grid, BouncyStagger, responsive layout)
- [ ] `@bnto/core` — Profile `createZipBlob` memory limits for large batches
- [ ] `.claude/strategy/` — Write `file-limits.md` with results and decisions

### Engine: Unmigrated Node Operations (Rust WASM)

**Priority: Medium.** Multi-step orchestration delivered (Smart Iteration + Rust PipelineExecutor). Remaining items are Tier 4+ prerequisites.

**Orchestration (multi-step recipe support) — DELIVERED:**

- [x] `engine` — **Multi-step recipe orchestration**: Rust `PipelineExecutor` handles full graph walking with topological ordering. Smart Iteration (`settings.iteration: "auto"`) wraps contiguous processor sequences in implicit per-file loops. Proven by Tier 1B recipes (`optimize-images-for-web`, `generate-thumbnails`) and 20+ golden equivalence tests.

**Remaining (Tier 4+ prerequisites):**

- [ ] `engine` — **Expression evaluation in browser**: Choose a JS expression evaluator to replace `expr-lang/expr` for `transform` node and `loop` while/break conditions. Candidates: `expr-eval`, `filtrex`, custom safe evaluator. Not needed until Tier 4 nodes ship. See [expression-input-ux.md](strategy/expression-input-ux.md).

**Excel support:**

- [ ] `engine` — **`bnto-csv`: Excel (.xlsx) read/write** — Rust options: `calamine` (read) + `rust_xlsxwriter` (write). Lower priority than CSV operations

### Engine: `pdf` Browser Node — Future (Tier 3)

**Priority: Medium.** PDF to Images Bnto (Tier 3, 50K+ monthly searches). Browser-side via pdf.js + Canvas (JS), not Go engine. Rewrite of the Go `pdfcpu` approach for client-side execution.

- [ ] `engine` or `apps/web` — Implement browser-side PDF → image conversion (pdf.js + Canvas)
- [ ] `engine` — Unit tests for PDF → image conversion
- [ ] `@bnto/nodes` — Add `pdf` node type definition, recipe fixture `pdf-to-images.bnto.json`

### Infra: Clean Up Convex Dev Environment (Better Auth Remnants)

Convex dev (`zealous-canary-422`) has stale Better Auth records and test artifacts. Cleanup mutations written; stale execution cleanup automated via hourly cron.

- [x] `@bnto/backend` — Audit tables, write cleanup mutation: `_dev_cleanup.ts` (`cleanTestAccounts` — cascade deletes auth sessions, accounts, recipes, executions, logs, events, rate limits; preserves predefined test accounts). `cleanup_stale.ts` (`markStaleAsFailed` + `cleanupStaleExecutions` — marks pending/running >2h as failed, cleans R2). Hourly cron wired in `crons.ts`.
- [ ] `@bnto/backend` — Run `cleanTestAccounts` against dev, verify table health
- [ ] `@bnto/backend` — (If needed) Run against production

### Infra: Configure R2 Lifecycle Rules — M4 (cloud execution)

**Milestone: M4.** R2 is only used for cloud (server-side) execution. Not needed for M1 browser execution.

| Bucket                              | Prefix        | Auto-delete after |
| ----------------------------------- | ------------- | ----------------- |
| `bnto-transit` + `bnto-transit-dev` | `uploads/`    | 1 hour            |
| `bnto-transit` + `bnto-transit-dev` | `executions/` | 24 hours          |

- [ ] `infra` — Configure R2 lifecycle rules in Cloudflare dashboard (prod + dev buckets)

### Infra: Domain Setup — API Domain (M4)

`bnto.io` delivered in Sprint 2C. API domain deferred to M4.

- [ ] `infra` — (M4) Configure `api.bnto.io` for M4 cloud service (technology and hosting TBD)

### Infra: Graduate SEO Validation from E2E to Unit Tests

**Priority: Medium.** Graduate SEO validation from slow E2E to unit tests (metadata, registry↔sitemap sync). Keep thin E2E for noindex/redirect/404. Lighthouse CI already delivered.

- [x] `apps/web` — Move metadata validation to unit tests (`bntoRegistry.test.ts`): comprehensive unit tests exist — validates all bntos present, slug format, no reserved-path collisions, required metadata fields (title, description, h1, fixture, features), title format (`-- bnto` suffix), unique slugs, BNTO_REGISTRY↔getAllRecipes() parity
- [ ] `apps/web` — Slim E2E to redirects + 404 + noindex only (seo-metadata.spec.ts still includes 200+ metadata assertions that duplicate the unit tests)

### Testing: Sprint 3 Deferred E2E Tests

**Deferred from Sprint 3 Wave 3 (March 2026).** Platform features are built and working. Test coverage deferred until editor MVP ships.

- [ ] `apps/web` — Playwright E2E: AuthGate conversion flow
- [ ] `apps/web` — Playwright E2E: browser-local execution history
- [ ] `@bnto/backend` — Unit tests for execution analytics queries

### Testing: Concurrent Quota Race Condition — M4/M5 (server-side quotas)

**Milestone: M4/M5 (Sprint 10+).** Quota enforcement only applies to server-side bntos. Browser bntos are free unlimited. This race condition matters when server-side execution has limits.

- [ ] `@bnto/core` — Integration test: fire 2+ concurrent `startPredefined` calls for a user at limit-1 runs, verify at most 1 succeeds
- [ ] `@bnto/backend` — If race confirmed, investigate Convex transaction isolation guarantees or atomic increment patterns

### UX: Per-File Format Override for Convert Image Format

**Priority: Medium.** Per-file format override on `convert-image-format` FileCards. Touches UI (inline Select), state (per-file config map in Zustand), and engine (per-file config passthrough).

- [ ] `apps/web` — Per-file format override state + inline Select on FileCard
- [ ] `@bnto/core` — Update `browserExecute` for per-file config overrides
- [ ] `engine` — Verify Rust WASM supports per-file format params

### Auth: Enable OAuth Social Providers

Google and Discord OAuth configured in `convex/auth.ts` but commented out — need OAuth credentials.

- [ ] `@bnto/backend` — Uncomment `socialProviders` in `convex/auth.ts`
- [ ] `@bnto/backend` — Set Google and Discord OAuth credentials in Convex env vars
- [ ] `apps/web` — Add Google and Discord sign-in buttons to `SignInForm`

### Growth: Referral Program — M5+

Referral links with Pro trial or extended history as reward. Open question: exact reward (Pro trial vs extended history vs early access).

- [ ] `@bnto/backend` — `referrals` table + `applyReferral` mutation
- [ ] `@bnto/core` — Referral service/hooks
- [ ] `apps/web` — Referral link generation UI + landing page `?ref=CODE` capture

### UX: Expression Input — Pill Tokens & Variable Picker

**Priority: Medium.** Template expression fields (rename patterns, loop items, break conditions) are plain `<Input>` elements with placeholder hints. Users write `{{name}}-compressed.{{ext}}` with zero editor assistance. This is fine for Tier 1-2 recipes (structured controls handle everything), but becomes a usability cliff when `transform`, `http-request`, and `ai` nodes ship.

**Strategy doc:** [expression-input-ux.md](strategy/expression-input-ux.md) — full competitor analysis (Zapier, Make.com, n8n, Apple Shortcuts, Power Automate, Retool), recommended approach, engine changes, phased rollout.

**Phased delivery:**

**Phase 1 (current — no work needed):** Tier 1-3 recipes use structured controls exclusively. Template fields are hidden or pre-filled. Users never write expressions.

**Phase 2 (when Tier 4 nodes ship — transform, http-request):**

- [ ] `engine` — Add `template_variables: Option<Vec<TemplateVariable>>` to `ParameterDef` in `metadata.rs`. Each variable declares name, label, description, source, example value. Populate in processors that have template params (file-system rename pattern, loop items)
- [ ] `packages/@bnto/nodes` — Update codegen (`generate-from-catalog.ts`) to propagate `templateVariables` into `NodeSchemaDefinition` params
- [ ] `packages/editor` — **ExpressionInput component**: Rich text input that renders `{{var}}` as visual pill tokens. Backspace selects/deletes pills. Underlying value stays a template string
- [ ] `packages/editor` — **Variable picker popover**: Grouped by source (file metadata, upstream outputs, loop context). Search/filter. Inserts pill at cursor
- [ ] `packages/editor` — **SchemaField dispatch**: If `templateVariables` is set on a param, render `ExpressionInput` instead of `TextControl`
- [ ] `packages/editor` — **Fixed/Expression toggle**: Per-field toggle (n8n-style) that switches between structured control and expression input. Trailing icon on SchemaField
- [ ] `apps/web` — E2E: Verify pill token rendering, variable picker insertion, Fixed/Expression toggle

**Phase 3 (when ai nodes ship — Tier 5):**

- [ ] `packages/editor` — Expression validation feedback (red underline for unknown variables, type mismatches)
- [ ] `packages/editor` — Autocomplete for function names and variable paths (beyond pill insertion)
- [ ] `packages/editor` — Function reference tab in variable picker (document available template functions)

---

### Performance: WASM Bundle Size & Processing Benchmarks

**Deferred from Sprint 2B.** WASM bundle: 1.6MB raw / 606KB gzipped. ~20% above 500KB target. Not blocking M1.

- [ ] `engine` — Profile bundle size per crate, evaluate code splitting vs single bundle
- [ ] `apps/web` — Processing speed + memory benchmarks per node type

### Performance: Next.js Server Component Audit (follow-up)

**Initial audit delivered in Sprint 6 W3.** Pushed client boundaries to leaves, lazy-loaded configs, extracted server-rendered static headers. Remaining follow-ups:

- [ ] `apps/web` — Restructure `my-recipes/page.tsx` — Server Component page with client leaf islands (eliminate `ssr: false` anti-pattern)
- [ ] `apps/web` — Audit remaining `dynamic({ ssr: false })` usage, replace with proper server/client composition
- [ ] `apps/web` — Eliminate barrel imports in client components, lazy load heavy components

### Infra: Vercel Preview Deployment Verification

**Deferred from Sprint 2A Wave 5.** Verify auth flow end-to-end on Vercel preview deployment. Not blocking M1 browser execution.

- [ ] `apps/web` — Verify auth flow on Vercel preview deployment (cookie behavior, proxy redirects, sign-in/sign-out)

### Infra: Convex Preview Deployments for Release Verification

**Priority: Low.** The release pipeline tests E2E against a Vercel preview + dev Convex, then promotes to production Vercel + prod Convex — that exact combination is never verified together. Convex supports [preview deployments](https://docs.convex.dev/production/hosting/preview-deployments) that could pair with Vercel previews for full-stack verification.

- [ ] `infra` — Evaluate Convex preview deployments for the release pipeline
- [ ] `infra` — Wire `npx convex deploy --preview-name <tag>` into `release.yml` before E2E step
- [ ] `infra` — Pass preview Convex URL as `NEXT_PUBLIC_CONVEX_URL` to the Vercel preview build
- [ ] `infra` — Clean up preview deployments after release (or let them auto-expire)

---

### UX: Conversion Hook Messaging Audit — M2/M5

**M2 (Sprint 3) for hook UX, M5 (Sprint 11) for Stripe.** Value-driven conversion hooks (Save, History, Premium Bntos, Team) — no "limit reached" messaging for browser bntos.

- [ ] `@bnto/backend` — Separate browser (no limits) from server-side (quota) error paths
- [ ] `apps/web` — Design conversion hook components with value-driven CTAs

### UX: Execution Activity Feed — M2 (Sprint 3)

**Updated from "Animated Run Counter."** With browser-first, there's no run limit to count down. Instead, show an activity feed / recent executions indicator that reinforces the value of signing up (persistence, history).

- [ ] `apps/web` — Design activity indicator for bnto tool pages (recent executions, total runs)
- [ ] `apps/web` — For anonymous users: "You've run 12 bntos this session. Sign up to save your history."
- [ ] `apps/web` — For authenticated users: animated activity feed with execution count and last-run status

### Premium: Cloud Drive Export (Post-MVP) — M5+

Pro users auto-save results to Google Drive/OneDrive/Dropbox — removes the "download then upload" friction.

- [ ] `apps/web` — "Save to..." post-execution UX
- [ ] `apps/api` — OAuth integration for cloud drive providers
- [ ] `@bnto/backend` — Store connected drive credentials (Pro only)

### Recursive Workflow Composability (Web App)

The engine supports recursive `Definition.Nodes`. The web app must preserve this composability. Guard rails (not new tasks — apply when building related features):

- Config panels must work at any nesting depth
- Execution progress must be recursive (group nodes show children's progress)
- JSON editor must represent recursive structure faithfully
- Visual editor (Sprint 4) must support drill-down into group nodes

### Triage: Test Naming & Description Unification Pass

**Priority: Triage.** Audit all test suites (Vitest unit + Playwright E2E) for naming consistency and organization. Ensure `describe` blocks, `test`/`it` statements, and test IDs follow a unified convention — clear action-oriented descriptions, consistent prefixing (e.g., FA1, PR1), and logical grouping. Remove duplicate or vague test names.

Files: `packages/*/src/**/*.test.ts`, `apps/web/e2e/**/*.spec.ts`

### Triage: Editor Store Performance Pass

**Priority: Triage.** Audit React context usage vs store selectors across `@bnto/editor`. Ensure components use direct store subscriptions (`useStore` + selector) instead of React context for state reads. General cleanup: memoize selectors, remove unnecessary re-renders, verify slice granularity.

Files: `packages/editor/src/components/`, `packages/editor/src/hooks/`, `packages/editor/src/context.ts`

### Deep Backlog: Code Editor (CodeMirror 6) — Post-M5

**Tabled indefinitely (March 2026).** Schema-aware `.bnto.json` code editor for power users — CM6 over Monaco (60x smaller), slash commands, JSON Schema validation, store sync with visual canvas. The visual editor is the product; code editor is a power-user luxury. Design doc: [code-editor.md](.claude/strategy/code-editor.md). May revisit post-M5 if demand emerges.

### Triage: Thin Rust comment density

**Priority: Triage.** Rust code is now readable without every-line explanations. Keep file-level header comments (purpose, how it fits) but remove most inline comments — only keep them for unorthodox patterns or genuinely complex logic. Update CLAUDE.md "Rust Code Standards" section to reflect the new lighter standard. Applies to all files in `engine/crates/`.

### Triage: iLovePNG recipe parity — next wave candidates

**Priority: Triage.** When planning the next recipe wave, evaluate iLovePNG's offerings for feasibility in Rust WASM: Resize IMAGE, Crop IMAGE, Rotate IMAGE, Watermark IMAGE, Blur face, Upscale, Convert to/from JPG, HTML to IMAGE, Meme generator. Several (resize, crop, rotate, watermark) are likely doable with our existing `image` crate. Others (upscale, blur face, HTML to IMAGE) may need server-side or new deps.

### Triage: AuthGate & ProGate badge/wrapper components

**Priority: Triage.** Create `<AuthGate>` and `<ProGate>` wrapper components with `variant="popup" | "dialog"` that intercept user interaction on gated features. When an unauthenticated (or non-Pro) user clicks a gated control, show a signup prompt (popup for soft nudge, dialog for hard gate). Include `<AuthGateBadge>` and `<ProGateBadge>` icon badges for visual indication. Share common gate logic between both via a base `<FeatureGate>` component. This is the mechanism for dangling the upgrade carrot to users.

Files: new `packages/ui/src/interaction/FeatureGate/`, `apps/web/` consumers

---

### Triage: Palette → primitive node type → mode/operation selection UX

**Priority: Triage.** Evaluate a flow where the node palette lists primitive node types (e.g. "Image") instead of operations (e.g. "Compress"). After choosing a type, the user picks the mode/operation, which loads the correct config. The config panel would have a mode selector at the top so users can switch operations without removing/re-adding the node. Trade-offs: simpler palette (fewer items) vs. extra click to reach config; explicit mode control vs. current direct-to-operation approach.

Files: `packages/editor/src/components/EditorToolbar.tsx` (palette), `packages/editor/src/components/ConfigPanel/`

---

### Triage: Surface-aware typography and icon color system

**Priority: Triage.** Research how design systems (shadcn/Radix, Chakra, Mantine, Ark UI) handle text/icon color when components sit on colored surfaces (e.g. a Card with `color="primary"`). Currently `Text`, `Heading`, `Badge`, and `IconBadge` use hardcoded color tokens (`text-muted-foreground`, `bg-primary/10 text-primary`) that don't adapt when the parent surface changes. This forces consumers to manually pass `onSurface` props to every sub-component.

**Goal:** A systematic approach where typography and icon primitives automatically adapt to their parent surface color — either via CSS custom property inheritance, data attributes, or a lightweight variant system. Audit all `@bnto/ui` primitives and `@bnto/editor` node components for manual color overrides that this system would eliminate.

Files: `packages/ui/src/typography/`, `packages/ui/src/blocks/RecipeCard/`, `packages/editor/src/components/nodes/Node/NodeIcon.tsx`, `apps/web/app/surface.css`

---

### Triage: Audit and remove useEditorStoreApi — migrate to client/service API

**Priority: Triage.** Audit all uses of `useEditorStoreApi`, `storeApi.setState`, `storeApi.getState`, and `storeApi.subscribe` in `packages/editor/src/hooks/` and `packages/editor/src/components/`. Migrate each to use the proper `editor.definition.*`, `editor.nodes.*`, etc. client/service methods. Once all consumers are ported, remove the `useEditorStoreApi` export from `context.ts`.

### Triage: Adopt DialogBody in all existing editor dialogs

**Priority: Triage.** OpenRecipeDialog, HelpDialog, and any other dialogs that compose `DialogHeader`/`DialogFooter` without `DialogBody` should be updated to use the standard `DialogHeader`/`DialogBody`/`DialogFooter` composition for consistent spacing. `RecipeDialog` already follows the pattern — backport to the rest.

### Triage: Adopt useDialog hook across all dialog use cases

**Priority: Triage.** `useDialog` has been added to `@bnto/ui` (standardized open/close state for dialogs). Adopt it in all existing dialog consumers — OpenRecipeDialog, HelpDialog, and any other components that manually manage dialog open/close with `useState`. Replace manual `useState(false)` + `setOpen` patterns with the standardized `useDialog()` hook for consistent props and state management.

---

### Infra: Conventional Commits + Auto-Changelog

**Priority: Medium.** Enforce `feat:`, `fix:`, `BREAKING CHANGE:` commit format. Auto-generate `CHANGELOG.md` from commit history on release tags. Enables semantic version bumping.

- [ ] `infra` — Add `commitlint` + `@commitlint/config-conventional` to pre-commit hooks
- [ ] `infra` — Add changelog generation step to `release.yml`
- [ ] `infra` — Include changelog in GitHub Release body

### Infra: Production Deploy Protection (GitHub Environments)

**Priority: Medium.** Require manual approval in GitHub Actions before promoting a release to production. Uses GitHub's environment protection rules.

- [ ] `infra` — Create `production` environment in GitHub repo settings with required reviewers
- [ ] `infra` — Gate the promote-production job behind the `production` environment

### Infra: Wire Version into App Build

**Priority: Medium.** Wire `NEXT_PUBLIC_APP_VERSION` from the git tag into the Next.js build. Display in error boundary, footer, and dev tools. Enables user bug reports to include the deployed version.

- [ ] `apps/web` — Add `NEXT_PUBLIC_APP_VERSION` env var, populated from `${{ github.ref_name }}` in release workflow
- [ ] `apps/web` — Display version in error boundary report and footer (dev mode)

### Chore: Upgrade Convex 1.31.7 → 1.33.1

**Priority: Low.** Minor Convex JS SDK update. [Changelog](https://github.com/get-convex/convex-js/blob/main/CHANGELOG.md#changelog). Update `convex` in `packages/@bnto/backend/`, verify schema/function compatibility, run full test suite.

- [ ] `packages/@bnto/backend` — Bump `convex` to `1.33.1`
- [ ] Run `task check` — full quality gate (lint + test + build)

---

### Infra: Upgrade GitHub Actions to Node.js 24

**Priority: Low (deadline: June 2, 2026).** `actions/checkout@v4` runs on Node.js 20, which GitHub is deprecating. After June 2, 2026, actions will be forced to Node.js 24. Upgrade to `actions/checkout@v5` (or set `FORCE_JAVASCRIPT_ACTIONS_TO_NODE24=true`) when v5 is available. Also audit other action dependencies (`actions/setup-node@v4`, `actions/upload-artifact@v4`, etc.).

- [ ] `infra` — Upgrade `actions/checkout` to v5 in `ci.yml`, `release.yml`, `lighthouse.yml` when available
- [ ] `infra` — Audit all GitHub Actions dependencies for Node.js 24 compatibility

---

### Triage: Audit and consolidate E2E journey tests

**Priority: Triage.** Review all journey E2E specs (`e2e/journeys/browser/`, `e2e/journeys/editor/`, `e2e/editor/`) for overlap and redundancy after recent development. Consolidate tests that cover the same flows, remove duplicated assertions, and ensure each spec is testing a distinct user journey rather than repeating similar steps across multiple files. Aim for clear, high-level journey tests that cover critical paths without excessive duplication. For example, if multiple specs test the auth flow, consider centralizing that in a single `auth.spec.ts` and referencing it from other journey tests. Evaluate any cases that are better suited as unit/integration tests rather than E2E, and migrate accordingly.

---

### Triage: Revisit skipped auth E2E tests in editor-save.spec.ts

**Priority: Triage.** Two tests in `e2e/journeys/editor/editor-save.spec.ts` are unconditionally skipped — SV1 (save recipe) and SV3 (load saved recipe). They require auth infrastructure (signed-in user + Convex) and the Save menu item was removed from the toolbar. Revisit when auth test helpers exist and Save is re-introduced. `editor-save.spec.ts:70`

---

### Triage: Dumb components pass — extract logic from heavy component files

**Priority: Triage.** Components like `packages/editor/src/components/NodePaletteDialog/NodePaletteDialogRoot.tsx` carry too much inline logic. Audit for opportunities to 1) extract reusable utils/patterns and 2) keep components dumb (data in, render out).

---

### Triage: Engine documentation — auto-generated docs for Rust engine

**Priority: Triage.** Set up auto-generated documentation for the Rust engine. Explore `cargo doc`, GitHub wiki integration, or a `docs/` directory at engine root that documents the engine architecture, crate responsibilities, and API surface. Goal: replace the tutorial-style comments removed in `chore/thin-rust-comments` with proper external documentation.

---

### Triage: Type inheritance audit for wrapper components

**Priority: Triage.** Wrapper components (e.g. SavedRecipeCard) redefine props like `loading`, `href`, `className` that already exist on the underlying primitive (Card, RecipeCard). Audit all wrapper components to use `Pick<ComponentProps<typeof Base>, ...>` or `extends` instead of manual redefinition. Flagged on PR #212 SavedRecipeCard.tsx.

---

### Triage: E2E tests for editor keyboard shortcuts

**Priority: Triage.** The 7 editor shortcuts (undo, redo, delete, run, export, escape, help) have unit test coverage for guard logic but zero E2E tests using `page.keyboard.press()`. The existing undo test uses the toolbar button, not the keyboard. Add Playwright tests that verify actual keyboard presses trigger expected actions.

---

### Triage: Remove Lighthouse CI from normal PR pipeline

**Priority: Low.** Lighthouse CI currently runs on every PR via `.github/workflows/lighthouse.yml`. It's valuable for release verification but adds latency to the normal dev loop without catching issues that change frequently. Move Lighthouse to release-only: keep it in `release.yml` (already there), remove the standalone `lighthouse.yml` workflow (or make it `workflow_dispatch` only so it can be triggered manually). The `task seo:audit` local command remains for on-demand developer use.

### Triage: TS2353 errors on `reducedMotion` in custom Playwright fixtures

**Priority: Triage.** Files using `test.use({ reducedMotion: "reduce" })` with the custom `test` from `e2e/fixtures.ts` produce TS2353 errors — `reducedMotion` isn't in the extended fixture type. Build passes because Turbopack doesn't typecheck `e2e/` files. Fix: remove per-file `test.use` calls (already set globally in `playwright.config.ts` via `contextOptions`) or widen the fixture type. Affected: `e2e/editor/node-progress.spec.ts`, `e2e/journeys/auth/auth-behavior.spec.ts`, `e2e/journeys/auth/auth-lifecycle.spec.ts`, `e2e/pages/site-navigation.spec.ts`.

### Triage: E2E teardown cleanup fails in release pipeline (missing CONVEX_DEPLOYMENT)

**Priority: Triage.** E2E teardown logs `cleanup failed — test accounts may persist` because `npx convex run _dev_cleanup:cleanTestAccounts` requires `CONVEX_DEPLOYMENT` which isn't set in the release pipeline runner. Either pass the env var to the E2E job or skip cleanup when running against a Vercel preview.

Files: `apps/web/e2e/`, `.github/workflows/release.yml`

### Triage: Investigate proper Convex auth error handling

**Priority: Triage.** We're catching Convex auth errors broadly and risk missing real Convex errors (query failures, mutation errors, schema validation). Need to differentiate auth errors from operational errors so real issues aren't silently swallowed.

### Triage: Add forgot password / password reset flow

**Priority: Triage.** Users have no way to reset their password. Need a forgot password link on sign-in, email-based reset flow, and reset confirmation screen. Check what `@convex-dev/auth` provides out of the box.

Files: `app/(auth)/`, `packages/@bnto/auth/`

### Triage: Verify PostHog reverse proxy after production deploy

**Priority: Triage.** After PR #225 is merged and deployed, run curl checks against `bnto.io/ingest/*` endpoints, confirm trailing slash behavior on `/ingest/e/`, and verify events appear in PostHog Live Events. Can only be tested in production.

### Triage: Sync recipes on my-recipes page mount

**Superseded by Sprint 8.5 pivot.** My Recipes page removed. Recipe persistence infrastructure stripped. Editor reconnects with sessionStorage only (Sprint 8.5d).

### Triage: Code-driven feature flag definitions for self-hosters and contributors

**Priority: Triage.** Current feature flags are dashboard-driven (PostHog UI only), which doesn't scale to self-hosters (no PostHog access) or open-source contributors (can't test flag-gated features). Evaluate defining flag keys, variants, and defaults in the repo with PostHog as a runtime override layer. Options: local defaults file, Vercel Flags SDK, Convex flags table.

Files: `.claude/rules/feature-flags.md` (open source consideration section), `.claude/decisions/feature-flags.md`

### Triage: Definition/recipe version migration tool

**Priority: Triage.** When breaking changes occur to node parameters (e.g., `compression`→`quality` unification with value inversion), users with existing `.bnto.json` recipes need a migration path. Build a versioned migration system that detects definition version, applies sequential transforms (v1→v2→v3), handles value conversions (not just renames), and reports what changed. Could be CLI (`bnto migrate`) and/or automatic migration on recipe load. The `version` field already exists in the `Definition` type.

Files: `packages/@bnto/nodes/src/definition.ts` (Definition type with version field), `engine/crates/bnto-core/` (engine-side validation)

### Triage: Remove DevTab and all dev-only execution controls

**Priority: Triage.** Rip out DevTab, DevNodeControls, devMockData, and the node-progress E2E spec (~500 lines of dead code). Also remove `setNodeStatus`, `setNodeProgress`, and `forceExecutionState` from ExecutionService interface and implementation — these are dev-only methods with no production consumers.

Files to delete: `DevTab.tsx`, `DevNodeControls.tsx`, `devMockData.ts`, `node-progress.spec.ts`. Files to modify: `RunPanelRoot.tsx`, `editorTypes.ts`, `executionService.ts`, `createEditorStore.test.ts`.

### Triage: Redesign homepage as developer-facing landing page

**Priority: Triage.** Rework bnto.io homepage from a recipe gallery into a developer-facing landing page for the tool/engine (like Tauri, Deno, Bun). Pitch the composable automation engine, run-anywhere story, and getting started (`cargo install bnto`). Recipe pages stay as the SEO showcase; homepage becomes the pitch for the tool itself.

### Triage: Secret/environment variable management for recipes

**Priority: Triage.** Recipes will need to reference secrets (API keys, auth tokens, env vars) without embedding them in `.bnto.json`. No recipe needs this yet, but HTTP, AI, and shell nodes will. Design needed: how recipes reference variables, how secrets resolve per target (CLI reads env/dotfiles, server reads vault, browser prompts user), how the editor surfaces variable placeholders without exposing values.

### Triage: Publish bnto CLI to crates.io (prerequisite for homepage rework)

**Priority: Triage.** Publish `bnto-core`, `bnto-engine`, and `bnto-cli` to crates.io so `cargo install bnto` works. Crate names are available. Requires: (1) convert path deps to crates.io-compatible deps for the publish chain, (2) add crates.io metadata to all published crates, (3) add `cargo publish` job to `release.yml` on stable tags, (4) configure `CARGO_REGISTRY_TOKEN` secret. **Must land before homepage rework advertises CLI install.** Phase 2: Homebrew tap via `cargo-dist` for non-Rust users.

## Reference

| Document                                                         | Purpose                                                                                                           |
| ---------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| `.claude/journeys/`                                              | User journey test matrices — auth, engine, API, web app, editor                                                   |
| `.claude/strategy/bntos.md`                                      | Predefined Bnto registry — slugs, fixtures, SEO targets, tiers                                                    |
| `.claude/strategy/editor-architecture.md`                        | Shared editor layer — store, hooks, package strategy, switchable editors                                          |
| `.claude/strategy/editor-user-journey.md`                        | Editor user journey — stages, flows, success criteria, phased delivery                                            |
| `.claude/strategy/visual-editor.md`                              | Bento box visual editor — compartment design, grid layout, execution state                                        |
| `.claude/strategy/code-editor.md`                                | Code editor design — CM6, slash commands, JSON Schema                                                             |
| `.claude/strategy/visual-editor.md`                              | Bento box visual editor — compartment design, grid layout, execution state                                        |
| `.claude/strategy/go-engine-migration.md`                        | HISTORICAL — Go node parameter inventory (engine deleted, git history preserves source)                           |
| `.claude/strategy/cloud-desktop-strategy.md`                     | Architecture, cost analysis, cloud execution topology                                                             |
| `.claude/strategy/core-principles.md`                            | Trust commitments, "For Claude Code" guidance                                                                     |
| `.claude/rules/`                                                 | Auto-loaded rules (architecture, code-standards, components, etc.)                                                |
| `.claude/skills/`                                                | Agent skills (pickup, project-manager, code-review, pre-commit)                                                   |
| Private business docs (`BNTO_PRIVATE_DOCS_PATH` in `.env.local`) | Pricing strategy, revenue projections, SEO monetization, feature funnel, brand, personas, competitive positioning |
