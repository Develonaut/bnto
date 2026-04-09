# Bnto — Completed Sprints

**This file contains the full history of completed sprints.** For the active plan, see [PLAN.md](PLAN.md). For backlog items, see [PLAN-BACKLOG.md](PLAN-BACKLOG.md).

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
- [x] Homepage complete (April 2026): Developer-facing landing page with Motorways animations, kawaii sushi mascots, code editor section, recipe showcase marquee. Pieces 1-9 shipped
- [x] Smart Iteration (March 2026): `settings.iteration: "auto" | "explicit"`, engine wraps per-file processors in implicit loops, 20 golden tests (10 explicit + 10 flat)

---

## Completed Sprints (early)

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

### Sprint 8: Tier 3 Near-Term Recipes — COMPLETE

**Goal:** Expand the recipe catalog with high-SEO-value recipes that run 100% client-side. Each recipe needs: Rust engine operation, `@bnto/nodes` recipe fixture, SEO page with metadata + JSON-LD, E2E verification. This is the first product expansion since M1.

**Why now:** Sprint 7 unified the discovery infrastructure — adding a recipe to `@bnto/nodes` now automatically appears on every surface. The pipeline is ready for new recipes. Tier 3 recipes target high-volume search queries (watermark: 30K/mo, strip-exif: 15K/mo, merge-csv: 12K/mo, csv-to-json: 25K/mo).

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

### Sprint 8.5: Simplify Config, Reconnect Editor Lightweight — COMPLETE

**Goal:** Three phases. Disconnect editor (done), make recipe config schema-driven (any recipe gets controls for free), then reconnect the editor as a lightweight open+export tool with no persistence. Favorites tabled — user preferences out of MVP scope.

#### Sprint 8.5a: Disconnect Editor + Slash Dead Code

Pure deletion + reference cleanup. Low risk. All changes ship together.

**Delete entirely:**

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

**Modify (web app):**

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

**Modify (core):**

- [x] `packages/core/src/BntoCoreProvider.tsx` — remove useRecipeSync from SyncProvider
- [x] `packages/core/src/clients/recipeClient.ts` — gut to keep only `run()`, query options, cache invalidation
- [x] `packages/core/src/reactCore.ts` — remove useRecipes, useRemoveRecipe
- [x] `packages/core/src/index.ts` — remove UserRecipe, RecipeListItem, fileTransfer exports
- [x] `packages/core/src/types/recipe.ts` — remove UserRecipe and RecipeListItem
- [x] `packages/core/src/adapters/convex/recipeAdapter.ts` — remove fetchCloudRecipes

**Modify (My Recipes — temporary placeholder):**

- [x] `apps/web/app/(app)/my-recipes/page.tsx` — placeholder until Sprint 8.5b
- [x] Delete `apps/web/app/(app)/my-recipes/_components/` (entire directory)

**Critical preservation:** `core.recipes.run()` → executionService path stays intact (tool pages use it). `recipeService.ts` stays (query options). `recipeAdapter.ts` keeps getRecipesQuery, getRecipeQuery.

#### Sprint 8.5b: Favorites + My Recipes — TABLED

**Tabled (March 2026).** User preferences/engagement features deferred to post-MVP.

#### Sprint 8.5c: Schema-Driven Recipe Config

Replace ~600 LOC of hardcoded per-recipe config with dynamic schema-driven config.

- [x] `apps/web` — **DynamicRecipeConfig component**
- [x] `apps/web` — **Update recipeFlowStore config shape**
- [x] `apps/web` — **Wire execution path**
- [x] `apps/web` — **Replace config registry** (delete all per-recipe config files)
- [x] `apps/web` — **Verify all recipes**
- [x] `apps/web` — **E2E verification**

#### Sprint 8.5d: Reconnect Editor (Open + Export Only)

Bring back the `/editor` route as a lightweight open+export tool. No persistence — no save to Convex, no localStorage, no My Recipes page. sessionStorage keeps the working recipe alive across page refreshes.

- [x] `@bnto/core` — **Recipe domain**: `core.recipes` re-added as 7th domain
- [x] `apps/web` — **Editor page route**: Restored with beta dialog
- [x] `apps/web` — **Restore nav items**: NewRecipeNavButton + mobile
- [x] `apps/web` — **Restore "Open in Editor" on tool pages**
- [x] `apps/web` — **Update routes + config + copy**
- [x] `apps/web` — **sessionStorage auto-persist**
- [x] `apps/web` — **Export/download verification**
- [x] `apps/web` — **E2E tests**: 7 spec files
- [x] `apps/web` — **Verify**: Quality gate passed

---

### Sprint 9: Engine Expansion — COMPLETE

**Persona ownership:** `engine` — `/rust-expert`

#### Wave 1 (parallel — dependency system + ProcessContext)

- [x] `engine/crates/bnto-core` — Add `requires: Vec<Dependency>` to `NodeMetadata` _(PR #315)_
- [x] `engine/crates/bnto-core` — `ProcessContext` trait: `NoopContext` for browser, `NativeContext` for CLI _(PR #318)_
- [x] `engine/crates/bnto-engine` — Dependency checker _(PR #320)_
- [x] `engine/crates/bnto` — `bnto doctor` command _(PR #320)_

#### Wave 2 (parallel — video node type)

- [x] `engine/crates/bnto-video` — New crate: `video-download` processor wrapping yt-dlp _(PRs #321-#329)_
- [x] `engine/crates/bnto-video` — Register in `bnto-engine`, add `NodeTypeInfo` _(PR #321)_
- [x] `engine/crates/bnto-video` — Golden tests + recipe _(PR #321)_
- [x] Codegen — Run `task wasm:codegen`. Verify propagation through TypeScript _(PR #336)_

#### Wave 3 (parallel — CLI polish)

- [x] `engine/crates/bnto` — `bnto list` command
- [x] `engine/crates/bnto` — `bnto info <recipe>` command
- [x] `engine/crates/bnto` — Enhanced `bnto run`: progress bars, colored output, timing
- [x] `README.md` — Update to pitch CLI usage front and center

---

### Homepage & Site Polish — COMPLETE (April 2026)

**Shipped.** Homepage redesigned from recipe gallery to developer-facing landing page. Pieces 1-9 delivered. Strategy docs: [homepage-sprint-plan.md](strategy/homepage-sprint-plan.md), [brand-messaging-audit.md](strategy/brand-messaging-audit.md), [landing-page-inspiration.md](strategy/landing-page-inspiration.md).

- [x] Piece 1 — Copy polish (text-only changes)
- [x] Piece 2 — Nav restructure (ExploreDropdown, GitHub link, CTA)
- [x] Piece 3 — Hero section animations (SlideUp, FadeIn, ScaleIn, Stagger, InView)
- [x] Piece 4 — Explore page spring animations (Stagger + ScaleIn, category filter)
- [x] Pieces 5-9 — Section redesigns, recipe showcase, build your own, open kitchen, footer refresh
