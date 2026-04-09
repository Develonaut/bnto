# Bnto — Build Plan

**Last Updated:** April 5, 2026 (CLI/TUI-first pivot — rewrote current state, collapsed 23+ frozen backlog items, promoted Sprint 10 TUI)
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

**CLI is the product.** `cargo install bnto` gets you 15 recipes. The web is a landing page.

- **v0.5.0 released (April 2026):** 15 recipes, video-download node (yt-dlp), dependency system, ProcessContext, `bnto list/info/run/doctor` commands. Published to crates.io
- **Engine (Rust):** Library crates (bnto-core, bnto-image, bnto-csv, bnto-file, bnto-video, bnto-engine), WASM entry point (bnto-wasm), CLI binary (bnto). CLI is the primary consumer, browser (WASM) is secondary
- **M1-M2 delivered:** Browser execution (WASM), editor v1, accounts, execution history — all shipped but web is now maintenance mode
- **CLI/TUI-first pivot (April 2026):** Web reduced to landing page. Editor frozen. Auth stripped. Frontend/premium work on hold. Focus: engine, CLI, TUI, infra
- **Next: TUI (Sprint 10)** — `bnto tui` via ratatui + crossterm (recipe browser, file picker, progress, results)
- **crates.io live:** All crates published. Release pipeline auto-publishes on stable tags
- **Open source (MIT):** Monetization tabled. Focus on engine power and community traction
- **Infra:** GitHub Actions CI, tag-triggered release pipeline (CI → preview → E2E → Lighthouse → production deploy → GitHub Release)
- **Homepage complete (April 2026):** Developer-facing landing page with Motorways animations, kawaii sushi mascots, code editor section, recipe showcase marquee. Pieces 1-9 shipped
- **Frozen:** Editor (`@bnto/editor`), auth (`@bnto/auth`), premium features, frontend investment. Web packages maintained but not actively developed

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
- [x] Rust WASM engine: library crates (bnto-core, bnto-image, bnto-csv, bnto-file, bnto-video, bnto-engine), cdylib entry point (bnto-wasm), CLI binary (bnto), Web Worker wrapper, progress reporting
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
- [x] crates.io published: All crates live on crates.io. `cargo publish` job in `release.yml` publishes in dependency order on stable tags. `cargo install bnto` works
- [x] Release v0.5.0 (April 2026): 15 predefined recipes, video-download node, extra args pass-through, dependency system

---

## Revenue & Monetization Context

**Tabled (April 2026).** Monetization is explicitly paused. The CLI is free, open-source (MIT), and the focus is on making the engine powerful and fun. Revenue strategy revisited when the tool has community traction. Private business docs (`BNTO_PRIVATE_DOCS_PATH` in `.env.local`) preserve the original pricing analysis for future reference.

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

**Sprint 9 complete.** CLI is solid: 15 recipes, dependency system, video node, `bnto list/info/run/doctor`. v0.5.0 shipped to crates.io.

**Next up: TUI (Sprint 10).** `bnto tui` launches an interactive terminal UI via ratatui + crossterm — recipe browser, file picker, progress display, results panel. Same engine, richer interface. See Sprint 10 below.

**After TUI:** More node types (Excel, PDF, shell), recipe expansion. Desktop (Tauri) and monetization are deep backlog. See [engine-expansion.md](strategy/engine-expansion.md).

---

## Completed Sprints (recent)

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

### Sprint 8: Tier 3 Near-Term Recipes — COMPLETE

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
- [x] `engine/crates/bnto` — `/rust-expert` — `bnto doctor` command: check all dependencies, report missing with install hints _(PR #320)_

#### Wave 2 (parallel — video node type)

- [x] `engine/crates/bnto-video` — `/rust-expert` — New crate: `video-download` processor wrapping yt-dlp. Purpose-built typed params: URL, format, quality, output format _(PRs #321-#329)_
- [x] `engine/crates/bnto-video` — `/rust-expert` — Register in `bnto-engine`, add `NodeTypeInfo` (category: "video", platforms: ["cli", "server", "desktop"]) _(PR #321)_
- [x] `engine/crates/bnto-video` — `/rust-expert` — Golden tests with test fixtures. Recipe: `download-video.bnto.json` _(PR #321)_
- [x] Codegen — Run `task wasm:codegen`. Verify new video category + node type propagates through TypeScript _(PR #336)_

#### Wave 3 (parallel — CLI polish)

- [x] `engine/crates/bnto` — `/rust-expert` — `bnto list` command: list available recipes with descriptions and categories
- [x] `engine/crates/bnto` — `/rust-expert` — `bnto info <recipe>` command: show recipe details, required dependencies, node types
- [x] `engine/crates/bnto` — `/rust-expert` — Enhanced `bnto run`: progress bars per file, colored output, timing summary
- [x] `README.md` — Update to pitch CLI usage front and center

---

### Sprint 10: TUI — NEXT

**Next sprint.** `bnto tui` launches an interactive terminal UI — recipe browser, file picker, progress display, results panel. Same engine, richer interface than raw CLI.

**Framework:** `ratatui` + `crossterm`

**Dependencies:** `ratatui`, `crossterm` added to `bnto` Cargo.toml

#### Wave 1 (parallel — foundation)

- [ ] `engine/crates/bnto` — `/rust-expert` — TUI module scaffolding (`src/tui/`). App struct, event loop, terminal setup/teardown
- [ ] `engine/crates/bnto` — `/rust-expert` — Recipe browser panel (list all recipes with categories, search/filter, selection)
- [ ] `engine/crates/bnto` — `/rust-expert` — Basic navigation (tab between panels, keyboard shortcuts, help overlay, quit)

#### Wave 2 (parallel — execution flow)

- [ ] `engine/crates/bnto` — `/rust-expert` — File picker panel (browse filesystem, multi-select files for recipe input)
- [ ] `engine/crates/bnto` — `/rust-expert` — Progress display (per-file progress bars, node status, live update during execution)
- [ ] `engine/crates/bnto` — `/rust-expert` — Recipe config editing (param overrides in TUI before execution)

#### Wave 3 (sequential — polish + test)

- [ ] `engine/crates/bnto` — `/rust-expert` — Results panel (output files, sizes, timing, open-in-finder/copy-path)
- [ ] `engine/crates/bnto` — `/rust-expert` — Integration tests for TUI mode (headless terminal testing)
- [ ] `engine/crates/bnto` — `/rust-expert` — `bnto tui` documentation + README update

---

### Deep Backlog: Distribution (Desktop + Server)

**Deferred.** Desktop (Tauri) and server-side execution in deep backlog. Revisit after TUI ships and community traction emerges.

- Desktop app (Tauri) — links engine natively like CLI, system webview for React frontend
- Server-side execution — cloud infrastructure for premium nodes (AI, shell, video at scale)

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

### Growth: Product Hunt Launch

**Priority: Backlog.** Launch bnto on Product Hunt when the product feels complete enough to show off. Ideal timing: after TUI ships + a few more recipes. Homepage is polished and ready. Coordinate with a README polish pass.

- [ ] Prepare Product Hunt listing (tagline, description, screenshots, maker comment)
- [ ] Review landing page + README for launch readiness
- [ ] Submit and engage on launch day

### Engine: File Count Limits & Performance Benchmarks

**Priority: Low.** Stress-test file count limits per recipe in the CLI for performance. Document safe boundaries per recipe type.

- [ ] `engine` — Benchmark file counts (50/100/200+ per recipe type), measure memory + processing time in CLI
- [ ] `engine` — Document recommended limits per recipe, decide enforcement strategy

### Engine: Future Node Operations

**Priority: Medium.** Multi-step orchestration delivered. Remaining items are future node prerequisites.

- [ ] `engine` — **Expression evaluation**: Expression evaluator for `transform` node and `loop` conditions. Candidates: custom Rust evaluator, `expr-eval` (for browser). Not needed until Tier 4 nodes ship
- [ ] `engine` — **Excel (.xlsx) read/write** in `bnto-csv`: Rust options `calamine` (read) + `rust_xlsxwriter` (write)

### Engine: `pdf` Node — Future

**Priority: Low.** PDF processing (split, merge, extract images, pdf-to-images). CLI-first via native Rust PDF libraries. Browser support TBD.

- [ ] `engine` — Evaluate Rust PDF crates (`lopdf`, `pdf-extract`, `printpdf`)
- [ ] `engine` — Implement `bnto-pdf` crate with initial processor(s)
- [ ] `engine` — Recipe fixture `pdf-to-images.bnto.json` + golden tests

### Auth: All Auth Features — FROZEN (auth stripped)

**Frozen until auth is re-enabled.** Auth surfaces stripped in open-source-first pivot (April 2026). The following items are blocked and will be revisited when auth returns:

- OAuth social providers, forgot password, AuthGate/ProGate components
- Convex auth error handling, deferred E2E tests, conversion hook messaging
- Execution activity feed, Vercel preview auth verification

### Premium & Growth — FROZEN (monetization tabled)

**Frozen (April 2026).** Monetization tabled. These items revisit when revenue strategy returns:

- Referral program (referral links, Pro trial rewards, `?ref=CODE` capture)
- Cloud Drive export (Google Drive/OneDrive/Dropbox post-execution save — M5+)
- Quota race condition (concurrent server-side quota enforcement — M4/M5)
- Feature flag definitions for self-hosters (code-driven flag defaults vs PostHog-only)
- Per-file format override (per-file config on convert-image-format FileCards)

### Editor & Frontend — FROZEN (CLI/TUI-first pivot)

**Frozen (April 2026).** Editor, frontend investment, and web UX work on hold. Focus is CLI/TUI. Revisit when/if the web editor is reactivated:

- Expression input (pill tokens, variable picker, fixed/expression toggle — Phases 2-3). Strategy: [expression-input-ux.md](strategy/expression-input-ux.md)
- Editor store performance pass (periodic audit — no issues found April 2026)
- Palette → primitive node type → mode/operation selection UX redesign
- Surface-aware typography and icon color system (`@bnto/ui` primitives)
- `useEditorStoreApi` usage audit (6 pipeline hooks — documented, intentional)
- `useDialog` hook adoption across dialog consumers
- Dumb components pass (extract logic from heavy editor component files)
- Type inheritance audit for wrapper components
- Editor keyboard shortcuts E2E (7 shortcuts have unit tests, no Playwright coverage)
- Recursive workflow composability (config panels at any depth, recursive progress, drill-down)
- Next.js Server Component audit follow-up (`my-recipes/page.tsx`, barrel imports, lazy loading)
- E2E journey test consolidation (deduplicate overlapping specs, migrate to unit where appropriate)
- Test naming & description unification pass (Vitest + Playwright naming conventions)

---

### Engine: WASM Bundle Size Optimization

**Priority: Low.** WASM bundle: 1.6MB raw / 606KB gzipped. ~20% above 500KB target. Not blocking anything — CLI is primary. Profile per-crate contribution if browser perf becomes a concern.

### Infra: Web-Only — FROZEN (web in maintenance mode)

**Frozen (April 2026).** Web infra items that only matter when web is actively developed:

- Convex preview deployments for release verification (pair preview Convex with preview Vercel)
- SEO validation graduation (slim E2E to redirects + 404 only, metadata validated in unit tests)
- Convex dev environment cleanup (run `cleanTestAccounts` against dev, verify table health)
- Wire version into app build (`NEXT_PUBLIC_APP_VERSION` from git tag)

### Triage: iLovePNG recipe parity — next wave candidates

**Priority: Medium.** When planning the next recipe wave, evaluate iLovePNG's offerings for feasibility: Resize IMAGE, Crop IMAGE, Rotate IMAGE, Watermark IMAGE (done), Blur face, Upscale, Convert to/from JPG, HTML to IMAGE, Meme generator. Several (resize, crop, rotate) are doable with existing `image` crate.

### Triage: Engine documentation — auto-generated docs

**Priority: Low.** Set up `cargo doc` or docs site for the Rust engine. Document crate responsibilities, API surface, architecture. `engine/crates/`.

### Triage: Definition/recipe version migration tool

**Priority: Medium.** `bnto migrate` CLI command for breaking changes to `.bnto.json` node parameters (e.g., `compression`→`quality`). Versioned migration system: detect version, apply sequential transforms, report changes. The `version` field already exists in `Definition`.

### @bnto/i18n: Interpolation + Raw Text Migration

**Priority: Low.** Add `{{variable}}` interpolation support to `t()` so dynamic values (recipe counts, etc.) can live in `en.json` instead of template literals in components. Then migrate all hardcoded `<Text>` strings in landing page components to `t()` calls.

- [ ] `packages/@bnto/i18n` — Add optional `vars` parameter to `t()`: `t(key, { count: 15 })` replaces `{{count}}` in the resolved string
- [ ] `packages/@bnto/i18n` — Unit tests for interpolation (single var, multiple vars, missing var, no vars)
- [ ] `apps/web` — Migrate hardcoded strings in landing page section components to `t()` calls
- [ ] `packages/@bnto/i18n` — Move dynamic recipe count strings to `en.json` with `{{count}}` placeholders

### @bnto/ui: `<SpringIn>` Entrance Animation Component

**Priority: Low.** Homepage shipped using `Card dormant` prop + `ScaleIn`/`SlideUp` instead. `SpringIn` is a nice-to-have refinement for future card-heavy sections, not a blocker.

The springable surface system (grounded → raised with bouncy spring) is the most satisfying animation in Motorways, but it's currently only available as a **state toggle** on `<Card loading>` / `<Surface grounded>`. You have to manage a boolean to trigger it. There's no way to use it as a one-shot entrance animation composable with `<Stagger>`.

`<SpringIn>` would bridge this gap: a keyframe-based entrance animation where elements start grounded (flat, muted, no elevation) and spring up to their natural elevated state on mount — the "building materializing on the map" feeling, usable anywhere `<ScaleIn>` or `<SlideUp>` is used today.

**The gap:**

- `ScaleIn` = scale + opacity (2D, no depth change)
- `SlideUp` = translate + opacity (2D, no depth change)
- Springable surfaces = grounded → raised (3D elevation, but requires state toggle)
- `SpringIn` (new) = grounded → raised as a one-shot keyframe entrance, composable with `Stagger`

**Implementation approach:**

- [ ] `packages/ui` — Create `@keyframes spring-in` in `animations.css`: starts with `translate(0, 0)` + muted colors + collapsed walls, ends at elevated rest position. Uses `--ease-spring-pressable` (the bounciest 3-oscillation curve)
- [ ] `packages/ui` — Create `SpringIn` component in `packages/ui/src/animation/Animate/SpringIn.tsx` following the same pattern as `ScaleIn` (forwardRef, `asChild`, `index`, `easing`, `buildStyle`)
- [ ] `packages/ui` — Add `spring` prop: `"bouncy" | "bouncier" | "bounciest"` (default `"bounciest"`) to control oscillation intensity
- [ ] `packages/ui` — Add `elevation` prop: `"sm" | "md" | "lg"` (default `"md"`) to set the target height the element springs up to
- [ ] `packages/ui` — Ensure composability with `<Stagger>` via `--stagger-index` delay
- [ ] `packages/ui` — Respect `motion-safe:` prefix (reduced motion shows element at final state, no animation)
- [ ] `packages/ui` — Add to animation component barrel export and update Motorway showcase page
- [ ] `packages/ui` — Unit tests: renders, respects asChild, stagger index sets delay, reduced motion applies
- [ ] `apps/web` — Add `SpringIn` demo to Motorway animation showcase tab

**Usage vision:**

```tsx
// Cards spring up from the ground one by one
<Stagger interval={80}>
  {recipes.map((r, i) => (
    <SpringIn key={r.id} index={i} elevation="md">
      <Card elevation="md">{r.name}</Card>
    </SpringIn>
  ))}
</Stagger>

// Hero element springs up dramatically
<SpringIn spring="bounciest" elevation="lg">
  <Card elevation="lg">Hero content</Card>
</SpringIn>
```

**Key decision:** The `<SpringIn>` component wraps the child (like `ScaleIn` wraps). It does NOT need the child to be a `<Surface>` — it applies its own keyframe animation. But when wrapping a `<Card>`, the card's elevation should match the `SpringIn` elevation for visual consistency (the card's resting shadow matches where the spring animation lands).

---

### Homepage & Site Polish — COMPLETE (April 2026)

**Shipped.** Homepage redesigned from recipe gallery to developer-facing landing page. Pieces 1-9 delivered: copy polish, nav restructure, hero animations, explore page animations, "What's in the box" redesign with mascots, recipe showcase marquee, "Build Your Own" code editor section, "Open Kitchen" section, footer refresh. Kawaii sushi mascots integrated (5 purchased from Catalyst Labs). Strategy docs: [homepage-sprint-plan.md](strategy/homepage-sprint-plan.md), [brand-messaging-audit.md](strategy/brand-messaging-audit.md), [landing-page-inspiration.md](strategy/landing-page-inspiration.md).

**Piece 1 — Copy polish (text-only, no component changes):**

- [x] Revise hero subheading — one sentence, lean into bento metaphor ("15 recipes included. Or pack your own.")
- [x] Revise section divider labels — "What's in the box", "Open kitchen"
- [x] Revise pitch points — add personality ("Pick your ingredients", "Your kitchen, your rules", "Open kitchen")
- [x] Revise footer tagline — "Pack. Run. Done."
- [x] Revise "How it works" body — shorter, punchier ("One node, one job. Chain them together, run them anywhere.")

**Piece 2 — Nav restructure:**

- [x] Rename "Create (beta)" → "Editor (beta)" in nav
- [x] Remove FAQ from top nav (keep in footer)
- [x] Build `ExploreDropdown` mega-menu — recipes grouped by category, sourced from `core.registry`
- [x] Add "Get started" CTA button + GitHub star link
- [x] Relocate theme toggle out of nav (footer or remove)

**Piece 3 — Hero section animations:**

- [x] `SlideUp` on hero headline, `FadeIn` with delay on sub-headline
- [x] `ScaleIn` with `spring-bouncy` on CTA buttons (staggered)
- [x] `ScaleIn` on terminal mockup card
- [x] `Stagger` + `SlideUp` on pitch points
- [x] Scroll-trigger utility — `IntersectionObserver` adding animation classes on viewport entry (`InView` component)

**Piece 4 — Explore page spring animations:**

- [x] Wrap recipe cards in `Stagger` + `ScaleIn` with stagger index
- [x] Spring selection animation on category filter (Select dropdown)
- [x] `FadeIn` on page header

**Pieces 5-9 — all shipped** (section redesigns, recipe showcase, build your own, open kitchen, footer refresh)

**Remaining polish (low priority, deferred):**

- [ ] Piece 10: Recipe page animations (`SlideUp` on header, `ScaleIn` on drop zone, `FadeIn` on config)
- [ ] Piece 11 remaining: Purchase 3-4 category mascot characters, convert to SVG components with size variants
- [ ] Piece 12: FAQ page `ScaleIn` entrance animations

### Triage: Secret/environment variable management for recipes

**Priority: Medium.** Recipes will need secrets (API keys, tokens, env vars) without embedding in `.bnto.json`. Design: how recipes reference variables, how secrets resolve per target (CLI reads env/dotfiles, server reads vault, browser prompts user).

### Triage: E2E teardown cleanup fails in release pipeline

**Priority: Low.** E2E teardown logs `cleanup failed` because `CONVEX_DEPLOYMENT` isn't set in release pipeline. Either pass env var to E2E job or skip cleanup against Vercel preview.

### Triage: Rename registry constructors

**Priority: Low.** `create_default_registry()` → `create_browser_registry()`, `create_native_registry()` → `create_registry()`. The "full" registry should be the default name, the WASM-constrained one the exception.

### Infra: Conventional Commits + Auto-Changelog

**Priority: Low.** Enforce `feat:`, `fix:`, `BREAKING CHANGE:` commit format. Auto-generate `CHANGELOG.md` on release tags. Not blocking anything.

### Infra: Production Deploy Protection (GitHub Environments)

**Priority: Low.** Manual approval step via GitHub Environments for production deploys. Existing tag-based workflow already prevents accidental deploys.

### Infra: Upgrade GitHub Actions to Node.js 24

**Priority: Low (deadline: June 2, 2026).** Upgrade `actions/checkout` to v5 when available. Audit all actions for Node.js 24 compatibility.

### Chore: Upgrade Convex 1.31.7 → 1.33.1

**Priority: Low.** Minor Convex JS SDK update. Bump in `packages/@bnto/backend/`, run `task check`.

### Triage: Responsive GridItem props

**Priority: Triage.** `GridItem` props (`colSpan`, `rowSpan`, `colStart`, `rowStart`) should accept `ResponsiveProp<T>` like the `Grid` `cols` prop does, so spans and positions can vary by breakpoint (mobile/tablet/desktop). Currently only `cols` is responsive — all placement props are static.

`packages/ui/src/layout/Grid.tsx`

### Triage: File Node Ecosystem — BRU-Style Composable File Operations

**Priority: Triage.** Expand the `file` category from 1 recipe to 6-8 with composable node processors inspired by Bulk Rename Utility. Enhance `file-rename` (counter, extension params), add new nodes (`file-collect`, `file-copy`, `file-filter`, `file-sanitize`, `file-metadata`, `svg-optimize`), and extend `image-convert` for vector formats (EPS/AI→SVG, SVG→PNG via `resvg`). Each node unlocks standalone recipes and custom compositions. Full strategy: [file-node-ecosystem.md](.claude/strategy/file-node-ecosystem.md)

### Triage: Homepage hero — BRU-style file recipe showcase

**Priority: Triage.** Add a file operation composition (e.g. `collect → sanitize → rename → copy`) as a "Build Your Own" hero snippet in `BuildYourOwnSection`. Demonstrates composable power vs monolithic tools. Blocked on file node ecosystem implementation.

`apps/web/app/(app)/_components/BuildYourOwnSection.tsx`, `recipeSnippets.ts`

---

## Reference

| Document                                                         | Purpose                                                                        |
| ---------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| `.claude/strategy/bntos.md`                                      | Predefined Bnto registry — slugs, fixtures, SEO targets, tiers                 |
| `.claude/strategy/engine-execution.md`                           | Engine execution architecture — pipeline executor, progress events             |
| `.claude/strategy/cloud-desktop-strategy.md`                     | Architecture, cost analysis, cloud execution topology                          |
| `.claude/strategy/core-principles.md`                            | Trust commitments, key principles                                              |
| `.claude/strategy/expression-input-ux.md`                        | Expression input UX (frozen — reference for future)                            |
| `.claude/rules/`                                                 | Auto-loaded rules (architecture, code-standards, engine-node-patterns, etc.)   |
| `.claude/skills/`                                                | Agent skills (pickup, project-manager, code-review, pre-commit)                |
| Private business docs (`BNTO_PRIVATE_DOCS_PATH` in `.env.local`) | Pricing strategy, revenue projections, SEO monetization (historical — on hold) |
