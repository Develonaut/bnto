# Bnto — Build Plan

**Last Updated:** April 27, 2026 (Groom: Sprint 16 complete, Sprint 17 defined — BRU file ecosystem + engine polish)
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

**CLI is the product.** `cargo install bnto` gets you 20 recipes. The web is a landing page.

- **v0.12.0 released (April 2026):** 20 recipes, video-download node (yt-dlp), dependency system, ProcessContext, `bnto list/info/run/doctor/install/dry-run` commands, TUI execution progress, vector operations (SVG). Published to crates.io
- **Engine (Rust):** Library crates (bnto-core, bnto-image, bnto-csv, bnto-file, bnto-shell, bnto-engine), WASM entry point (bnto-wasm), CLI binary (bnto). CLI is the primary consumer, browser (WASM) is secondary
- **M1-M2 delivered:** Browser execution (WASM), editor v1, accounts, execution history — all shipped but web is now maintenance mode
- **CLI/TUI-first pivot (April 2026):** Web reduced to landing page. Editor frozen. Auth stripped. Frontend/premium work on hold. Focus: engine, CLI, TUI, infra
- **TUI delivered (Sprint 10):** `bnto tui` via ratatui + crossterm — 7 screens (home, browser/recipes, detail, picker, execution, results, settings), 400+ tests
- **TUI schema-driven config (Sprint 11):** Type-aware parameter controls (boolean toggles, enum selects, number sliders, validation), engine-owned node schema, ~930 LOC hand-written TS deleted
- **Recipe-level deps + shell-command (Sprint 12B):** `PipelineDefinition.requires`, `shell-command` processor with security boundary, `download-video` migrated from dedicated crate, `bnto-video` deleted. Recipe fields (`{{fields.*}}`) delivered
- **Data persistence + Home + Library (Sprint 12A):** XDG-compliant storage (`BntoPaths`), atomic writes, TOML config, Home screen, My Library, `bnto` = TUI default
- **`tonkotsu` crate (Sprint 11.5, renamed from bnto-form):** Standalone ratatui form widget library (TextInput, Select, Confirm, Number), TEA-native, zero bnto dependency. ~105 tests
- **TUI List Editor (Sprint 12):** Full recipe editing — add/remove/reorder nodes, inline param editing, undo/redo, save workflow, multiple entry points. ~75 tests
- **TUI Wizard (Sprint 13):** Guided recipe creation — category → operation → config → done. Hands off to List editor
- **TUI controls polish (Sprints 14-15):** Bubbles-inspired display/edit modes, FilePath field type, TextArea, fuzzy Select filter, picker search/metadata/breadcrumbs, vim keybindings. `tonkotsu` now at huh parity for shipped controls
- **Sprint 16 delivered:** tonkotsu huh parity, file-rename enhanced, template expressions, version constraints, `bnto migrate`, 2 design spikes (execution progress UX, recipe secrets). 20 recipes
- **Next:** Sprint 17 — BRU file ecosystem (file-filter, file-collect, file-copy, file-metadata), engine polish (execution progress Phase 1, secrets Phase 1, recipe trust), image crop/rotate, format expansion (CSV sort/filter, Excel, EPS→SVG). Target: 26+ recipes
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
- [x] Vector format support (April 2026): `bnto-vector` crate, svg-to-png, svg-to-jpeg, optimize-svg recipes. 18 total recipes
- [x] Sprint 14 Engine Hardening (April 2026): Bento Box audit, TUI execution progress, TOCTOU fix, `bnto dry-run`, `bnto install`, Node.js 24 Actions. v0.12.0 released

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

### Sprint 6: Quality & Cleanup — COMPLETE

Post-M2 stabilization. Error boundaries, Go archive deletion, Rust executor split, Server Component audit, Lighthouse audit, inline handler extraction, CSS-first interaction audit, Button simplification, theme menu lighting. 6 waves.

### Sprint 7: Explore & Discovery Infrastructure (Tier 2) — COMPLETE

Unified recipe/node listing across all surfaces. `core.registry` 6th domain. `/explore` page. Layered `Recipe` type, `RecipeDefinition` deleted, `?from={slug}` eliminated. Adding a recipe to `@bnto/nodes` auto-appears everywhere.

### Sprint 8: Tier 3 Near-Term Recipes — COMPLETE

4 high-SEO-value recipes: watermark-images, strip-exif, merge-csv, csv-to-json. All 100% client-side via WASM. 11 E2E tests, Lighthouse clean.

### Sprint 8.5: Simplify Config, Reconnect Editor Lightweight — COMPLETE

8.5a: Editor disconnection + dead code slash. 8.5c: Schema-driven recipe config (DynamicRecipeConfig replaces ~600 LOC). 8.5d: Editor reconnected as open+export tool with sessionStorage (PR #305). 8.5b (favorites) tabled.

### Sprint 9: Engine Expansion — COMPLETE

Dependency system (`requires: Vec<Dependency>`), `ProcessContext` trait, `bnto doctor`, `video-download` processor (yt-dlp), CLI commands (`bnto list/info/run`). Phase 2 engine expansion foundation.

### Sprint 10: TUI — COMPLETE

`bnto tui` via ratatui + crossterm. 6 screens (browser, detail, picker, execution, results, settings), 278 tests. TEA architecture. Sprint 10 totals: 32 Rust files.

### Sprint 11: Engine-Owned Node Schema + TUI Schema-Driven Config — COMPLETE

Engine single source of truth for node schemas + document types. ~930 LOC hand-written TS deleted from `@bnto/nodes`. TUI type-aware controls (boolean, enum, number). Codegen overhaul. 7 PRs.

### Sprint 12B: Recipe-Level Dependencies + Shell Command — COMPLETE

`PipelineDefinition.requires`, `shell-command` processor (bnto-shell crate), `download-video` migration from `bnto-video` to shell-command + recipe deps. `bnto-video` crate deleted. Recipe fields (`{{fields.*}}` template syntax) delivered as follow-up. ~4 PRs, ~30 tests.

### Sprint 12A: Data Persistence + Home + Library — COMPLETE

XDG-compliant storage (`BntoPaths`), atomic writes, TOML config, Home screen, My Library, `bnto` (no args) = TUI. ~8 PRs, ~65 tests.

### Sprint 11.5: `tonkotsu` (formerly bnto-form) — TUI Form Widget Crate — COMPLETE

Standalone ratatui form crate: TextInput (grapheme-aware), Select (compact cycling + filter), Confirm, Number (vendored slider). TEA-native, zero bnto dependency. ~6 PRs, ~105 tests.

### Sprint 12: TUI List Editor — COMPLETE

TUI recipe editor: editor state model, add/remove/reorder nodes, inline param editing with type-aware controls, undo/redo, save workflow, multiple entry points (--new, file arg, browser e, detail e). ~8 PRs, ~75 tests.

### Sprint 13: TUI Wizard — COMPLETE

Guided recipe creation: category → operation → config → done. Wizard-to-editor handoff with auto-naming. ~3 PRs, ~25 tests.

### Homepage & Site Polish — COMPLETE (April 2026)

Developer-facing landing page. Pieces 1-9: copy polish, nav restructure, hero animations, explore page animations, "What's in the box" mascots, recipe showcase marquee, "Build Your Own" code editor section, "Open Kitchen" section, footer refresh.

### Editor: Smart Iteration — DELIVERED (March 2026)

`settings.iteration: "auto" | "explicit"` — engine wraps per-file processors in implicit loops. 20 golden equivalence tests. Recipe Settings Panel.

---

## What's Next

**Sprint 16 complete.** tonkotsu at full huh parity, file-rename enhanced (counter + sanitize), template expressions (`{{env.*}}`, `{{ctx.*}}`, `{{node.*}}`), version constraints, `bnto migrate`, execution progress + secrets design spikes delivered. 20 recipes shipped.

**Next: Sprint 17 — BRU File Ecosystem + Engine Polish.** Achieve BRU (Bulk Rename Utility) composable file operations goal: 4 new file processors (file-filter, file-collect, file-copy, file-metadata) that chain together for BRU-level power. Parallel engine polish (execution progress Phase 1, secrets Phase 1, recipe trust/consent). Image crop/rotate. Format expansion (CSV sort/filter, Excel, EPS→SVG). Target: 26+ recipes.

Desktop (Tauri) and monetization are deep backlog.

---

## Sprint 14: Engine Hardening — COMPLETE

**Goal:** Make the current experience bulletproof. Fix the worst UX gaps, address security quick wins, reduce friction for new users.

**Persona ownership:**

| Package                    | Persona        |
| -------------------------- | -------------- |
| `engine/crates/bnto`       | `/rust-expert` |
| `engine/crates/bnto-core`  | `/rust-expert` |
| `engine/crates/bnto-shell` | `/rust-expert` |
| `.github/workflows/`       | Infra          |

#### Wave 0 — Bento Box Audit (before feature work)

**Do this first.** Audit all Rust files against Bento Box size limits. Several files have grown well past 250 production lines:

| File                        | Prod Lines | Issue                                                                     |
| --------------------------- | ---------- | ------------------------------------------------------------------------- |
| `bnto/src/tui/app.rs`       | ~1305      | TEA update() — idiomatic match arms, but extract screen-specific handlers |
| `bnto-core/src/metadata.rs` | ~673       | Node metadata registry — extract per-category modules                     |
| `tonkotsu/src/form.rs`      | ~523       | Form widget — extract widget-specific logic                               |
| `bnto/src/tui/keys.rs`      | ~668       | Key dispatch — idiomatic match arms, evaluate extraction                  |

Note: TEA `update()` match blocks and `handle_*_key()` are idiomatic Rust (per MEMORY.md) — splitting them would be worse. But `app.rs` at 1305 prod lines has room to extract screen-specific update handlers into separate modules. `metadata.rs` and `form.rs` are not TEA patterns and should be broken up.

- [x] `engine/crates/bnto` — **Audit + extract `app.rs`**: Extracted 24 handler functions into `app_helpers/` module directory (6 submodules: navigation, editor, wizard, home_detail, library, settings). `app.rs` reduced from 1305 → 300 prod lines
- [x] `engine/crates/bnto-core` — **Break up `metadata.rs`**: Extracted `node_types.rs` (252 prod) and `parameters.rs` (107 prod). `metadata.rs` reduced to 85 prod lines (re-export hub)
- [x] `engine/crates/tonkotsu` — **Break up `form.rs`**: Extracted `controls/dispatch.rs` (301 prod — TEA dispatch, idiomatic). `form.rs` reduced to 142 prod lines
- [x] `engine/crates/` — **Sweep remaining 250+ files**: Audited 37 files over 250 prod lines. 6 are TEA dispatch (idiomatic exception), 10 are test files, 21 are production. `node_types.rs` (252) is a data registry — splitting would reduce cohesion

#### Wave 1 — Execution Progress + Security (parallel)

- [x] `engine/crates/bnto` — **TUI execution progress feedback**: Stream stderr from child processes in `run_command()`, surface live activity indicator (elapsed time, spinner, or parsed progress). Long-running recipes (download-video) currently show a blank screen for 10+ minutes. RED tests: stderr streaming, elapsed timer, cancellation (~5 tests)
- [x] `engine/crates/bnto-core` — **TOCTOU fix in `NativeContext::temp_file()`**: Replace nanosecond timestamp with `tempfile` crate (`mkstemp` semantics) to eliminate symlink race. RED tests: concurrent temp file creation, no collisions (~3 tests)
- [x] `engine/crates/bnto` — **`bnto dry-run <recipe>`**: Dry-run mode showing exactly which commands a recipe will execute before running. Resolves templates, shows final args. Builds trust for shell-command recipes. RED tests: dry-run output format, template resolution display (~4 tests)

#### Wave 2 — Dependency UX (sequential, depends on Wave 1)

- [x] `engine/crates/bnto` — **`bnto install <recipe>`**: Auto-install recipe dependencies with OS/package manager detection (brew/apt/choco/pacman). Reads `requires` from recipe definition. RED tests: detection per OS, install command generation, already-installed skip (~6 tests)

#### Wave 3 — Infra (parallel with Wave 2)

- [x] `.github/workflows/` — **Node.js 24 GitHub Actions upgrade**: Upgrade `actions/checkout` to v5+, audit all action versions for Node.js 24 compatibility. Hard deadline: June 2, 2026. RED tests: CI passes with updated actions

**Sprint 14 totals: ~5 PRs, ~18 tests**

---

## Sprint 15: TUI Controls — Bubbles-Quality UX — COMPLETE

**Goal:** Make `tonkotsu` controls and the file picker feel as polished as [Charm Bubbles](https://github.com/charmbracelet/bubbles). Form fields get display/edit modes. File selection becomes a form control. Picker gets search, metadata, breadcrumbs.

**Strategy doc:** [tui-controls-bubbles.md](strategy/tui-controls-bubbles.md)

**Persona ownership:**

| Package                  | Persona        |
| ------------------------ | -------------- |
| `engine/crates/tonkotsu` | `/rust-expert` |
| `engine/crates/bnto`     | `/rust-expert` |

#### Wave 1 — Form Control Interaction Model (sequential)

- [x] `engine/crates/tonkotsu` — **Display/Edit mode for form fields**: Each field renders a compact display line (label + value). Enter opens edit mode (full control). Enter/Esc returns to display. RED tests: display rendering, mode transitions, value preservation (~6 tests)
- [x] `engine/crates/tonkotsu` — **FilePath field type**: New `FieldKind::FilePath` renders as path string in display mode. Edit mode opens inline directory browser (picker logic extracted into bnto-form). RED tests: display, browser nav, file selection, ext filter (~8 tests)

#### Wave 2 — Picker Polish (parallel with Wave 1 completion)

- [x] `engine/crates/bnto` — **Picker search/filter**: Inline text search filters entries by filename (case-insensitive). Backspace clears. Shows match count. RED tests: filter narrows, clear restores, empty state (~5 tests)
- [x] `engine/crates/bnto` — **Picker file metadata columns**: Aligned perms + human-readable sizes. Toggle with `p`. Symlink `->` indicator. RED tests: size format, perms display, symlink indicator (~4 tests)
- [x] `engine/crates/bnto` — **Picker breadcrumb path**: Styled path segments replacing plain directory string. Current dir highlighted. RED tests: breadcrumb rendering, segment styling (~3 tests)

#### Wave 3 — Form Control Refinements (depends on Wave 1)

- [x] `engine/crates/tonkotsu` — **Select with fuzzy filter**: Typing filters options by fuzzy substring. Cycling preserved when no filter. RED tests: fuzzy match, cycling fallback, clear filter (~4 tests)
- [x] `engine/crates/tonkotsu` — **TextArea field type**: Multi-line input. Display shows first line + count. Edit shows scrollable editor. RED tests: multi-line, scroll, display truncation (~5 tests)

**Sprint 15 totals: ~7 PRs, ~35 tests**

---

## Sprint 16: Recipe Expansion + huh Parity — COMPLETE

**Goal:** Rename `bnto-form` → `tonkotsu` and make it the Rust equivalent of Charm's [huh](https://github.com/charmbracelet/huh) library. Expand recipe catalog with new file operations. Strengthen engine infrastructure for future recipes. Grew recipe count from 18→20. Wave 5 (image crop/rotate) deferred to Sprint 17.

**Strategy docs:** [tonkotsu-strategy.md](strategy/tonkotsu-strategy.md) (§ huh Parity), [file-node-ecosystem.md](strategy/file-node-ecosystem.md) (Phases 1-2), [tui-controls-bubbles.md](strategy/tui-controls-bubbles.md)

**Persona ownership:**

| Package                     | Persona        |
| --------------------------- | -------------- |
| `engine/crates/tonkotsu`    | `/rust-expert` |
| `engine/crates/bnto-image`  | `/rust-expert` |
| `engine/crates/bnto-file`   | `/rust-expert` |
| `engine/crates/bnto-engine` | `/rust-expert` |
| `engine/crates/bnto`        | `/rust-expert` |
| `engine/crates/bnto-core`   | `/rust-expert` |

#### Wave 0 — Rename bnto-form → tonkotsu (prerequisite)

Rename the `bnto-form` crate to `tonkotsu` — a playful ramen-themed name for the ratatui ecosystem. `tonkotsu` is available on crates.io. The broth holds the whole bowl together, just like the form library holds the whole TUI interaction together.

- [x] `engine/crates/bnto-form` → `engine/crates/tonkotsu` — **Rename crate to tonkotsu**: Rename directory (`git mv`), update `Cargo.toml` (package name, bin name → `tonkotsu-demo`), update workspace `Cargo.toml` member, update `bnto/Cargo.toml` dependency, update all `use bnto_form::` → `use tonkotsu::` imports (~15 source files), update `Taskfile.yml` (`form:demo` task), update strategy docs + PLAN.md + CLAUDE.md references. Verify `task wasm:lint && task cli:test` pass clean. (~0 tests — pure rename, existing tests validate)

#### Wave 1 — tonkotsu: huh Parity (parallel, depends on Wave 0)

Bring `tonkotsu` (formerly `bnto-form`) to full feature parity with Charm's huh library. See [tonkotsu-strategy.md § huh Parity](strategy/tonkotsu-strategy.md) for the gap analysis.

- [x] `engine/crates/tonkotsu` — **FullScreenEdit form mode**: Third `FormMode` variant. Display mode identical to DisplayEdit (compact one-liners). Edit mode hides all other fields, renders dedicated panel with label header + full control + helper footer. Becomes default demo mode. RED tests: display rendering, edit panel visibility, mode transitions, all field types, panel framing, helper text (~8 tests)
- [x] `engine/crates/tonkotsu` — **MultiSelect field type**: New `FieldKind::MultiSelect`. Display: `"Tags: image, vector (2 selected)"`. Edit: checkboxes with Space to toggle, Enter to confirm. Wrapping navigation. RED tests: toggle selection, display formatting, confirm/cancel, empty selection (~5 tests)
- [x] `engine/crates/tonkotsu` — **Field grouping**: `FieldGroup` wraps fields into named sections. In FullScreenEdit, groups render as navigable pages (next/prev). In DisplayEdit, groups render as visual sections with headers. RED tests: group rendering, page navigation, field-to-group mapping (~5 tests)
- [x] `engine/crates/tonkotsu` — **Note field type**: Read-only `FieldKind::Note` for informational text between fields. Not editable, not focusable. Display: styled text block. RED tests: renders text, skipped by focus navigation, not editable (~3 tests)

#### Wave 2 — File Node Expansion (parallel with Wave 1)

BRU-style composable file operations. See [file-node-ecosystem.md](strategy/file-node-ecosystem.md) Phase 1 (enhance file-rename) and Phase 2 (file-sanitize).

- [x] `engine/crates/bnto-file` — **Enhance file-rename: counter + extension params**: Add `counter_start` (integer, default 1), `counter_pad` (integer, default 0), `extension` (string) params. New `{{counter}}` template variable (auto-incrementing, respects start/pad). RED tests: counter formatting, zero-pad widths, extension replacement, counter across files (~5 tests)
- [x] `engine/crates/bnto-file` — **File-sanitize processor**: New `file-sanitize` processor. Params: `mode` (slugify/strip/normalize), `separator` (default `-`), `max_length` (default 0 = no limit). Pure string manipulation, browser+CLI. RED tests: each mode, unicode normalization, max length truncation, separator replacement (~5 tests)
- [x] `engine/crates/bnto-engine` + `engine/recipes/` — **File recipes + codegen**: `number-files.bnto.json` (file-rename with counter), `sanitize-filenames.bnto.json` (file-sanitize). Register processors. Golden tests. Codegen updates. SEO slugs: `/number-files`, `/sanitize-filenames`. RED tests: recipe execution, golden output verification (~4 tests)

#### Wave 3 — Engine Infrastructure (parallel with Wave 2)

Template expressions, version constraints, and migration tooling. Strengthens the engine for future recipe complexity.

- [x] `engine/crates/bnto-core` — **Template expression expansion**: Extend `{{fields.*}}` template system with `{{env.*}}` (environment variables), `{{ctx.*}}` (execution context — temp dir, working dir, platform), `{{node.<id>.*}}` (inter-node output references). RED tests: each namespace resolution, missing var handling, nested references (~6 tests)
- [x] `engine/crates/bnto-core` — **Version constraint enforcement**: Parse `<binary> --version` output, validate against `Dependency.version` semver constraint. Fail pipeline before execution if version doesn't satisfy. RED tests: semver parsing, constraint matching, version extraction from output (~5 tests)
- [x] `engine/crates/bnto` — **`bnto migrate` CLI command**: Migrate `.bnto.json` files across breaking parameter changes (e.g., `compression`→`quality`). Detect version, apply sequential transforms, report changes. RED tests: version detection, migration transforms, idempotent re-run (~5 tests)

#### Wave 4 — Design Spikes (parallel with Wave 3, strategy docs not code)

Write strategy docs to unblock future sprints. No code — research, mockups, and architecture decisions.

- [x] `.claude/strategy/execution-progress-ux.md` — **Rich execution progress UX design spike**: Competitive audit (Claude Code, cargo, docker, Bubbles), Unicode indicator inventory (spinners, progress bars, frames), metrics design (elapsed, throughput, ETA, file count), layout mockups (CLI single-line vs TUI multi-line), architecture review (engine events vs rendering), phased scope recommendation
- [x] `.claude/strategy/recipe-secrets.md` — **Secret/env variable management design spike**: How recipes reference secrets without embedding in `.bnto.json`. Resolution per target (CLI reads env/dotfiles, server reads vault, browser prompts). Integration with `{{env.*}}` template namespace (Wave 4). Threat model for secret exposure

#### Wave 5 — Image Recipe Expansion — DEFERRED → Sprint 17

Deferred to Sprint 17 Wave 3. Image crop/rotate is lower priority than BRU file ecosystem.

**Sprint 16 totals: ~12 PRs, ~57 tests, 2 new recipes (18→20), 2 strategy docs, 1 crate rename**

---

## Sprint 17: BRU File Ecosystem + Engine Polish — ACTIVE

**Goal:** Achieve the BRU (Bulk Rename Utility) composable file operations goal. Four new file processors that chain together to give BRU-level power. Parallel engine polish for execution progress, secrets, and trust model. Image crop/rotate (deferred from Sprint 16). Format expansion (CSV sort/filter, Excel, EPS→SVG). Grows recipe count from 20→26+.

**Strategy docs:** [file-node-ecosystem.md](strategy/file-node-ecosystem.md) (Phases 2+), [execution-progress-ux.md](strategy/execution-progress-ux.md) (Phase 1), [recipe-secrets.md](strategy/recipe-secrets.md) (Phase 1)

**Persona ownership:**

| Package                     | Persona        |
| --------------------------- | -------------- |
| `engine/crates/bnto`        | `/rust-expert` |
| `engine/crates/bnto-core`   | `/rust-expert` |
| `engine/crates/bnto-file`   | `/rust-expert` |
| `engine/crates/bnto-image`  | `/rust-expert` |
| `engine/crates/bnto-csv`    | `/rust-expert` |
| `engine/crates/bnto-vector` | `/rust-expert` |
| `engine/crates/bnto-engine` | `/rust-expert` |
| `engine/crates/bnto-editor` | `/rust-expert` |

#### Wave 1 — Engine Polish (parallel, no deps)

Improve all recipe execution. Can start immediately.

- [x] `engine/crates/bnto` — **Execution progress Phase 1**: Completion summary line (`Completed 10 files in 2.4s`), per-file elapsed (`3/10 photo.jpg (1.2s)`), braille spinner + elapsed for indeterminate shell-command nodes. TUI: rename "NODES" → "STEPS", inline file count next to active node, per-node elapsed. Zero engine event changes — rendering only. RED tests: summary format, elapsed display, spinner rendering, TUI label changes (~5 tests). Strategy: [execution-progress-ux.md](strategy/execution-progress-ux.md) § Phase 1
- [x] `engine/crates/bnto-core` + `engine/crates/bnto` — **Secrets Phase 1: dotenv + pre-flight validation**: Load `.env` from working directory and `~/.config/bnto/.env` in `NativeContext`. Simple `KEY=VALUE` parser (no crate). Add `secrets` array to `PipelineDefinition` schema. Pre-flight check: fail with clear message if required secrets missing. `bnto doctor` shows secret status. RED tests: dotenv parsing, resolution order (system > project > user), pre-flight validation, missing secret error (~6 tests). Strategy: [recipe-secrets.md](strategy/recipe-secrets.md) § Phase 1
- [x] `engine/crates/bnto` — **Recipe trust/consent**: Distinguish built-in vs local vs community recipes. First-run consent prompt before executing `shell-command` nodes from untrusted sources. Cache approval per recipe hash in `BntoPaths` config. TUI: trust badge on recipe detail. RED tests: trust level classification, consent prompt, approval caching, bypass for built-in (~5 tests)

#### Wave 2 — BRU File Processors (parallel, the core BRU nodes)

Four new processors in `bnto-file`. Each does one thing, composes with the others. This is the core BRU delivery.

- [x] `engine/crates/bnto-file` — **`file-filter` processor**: Filter files by extension, name pattern, or size. Params: `extensions` (string, comma-separated), `name_pattern` (string, glob/regex), `min_size` (integer, bytes), `max_size` (integer, bytes). Files that don't match are dropped from the pipeline. Browser+CLI. 22 tests
- [x] `engine/crates/bnto-file` — **`file-collect` processor**: Directory traversal + glob matching. Params: `pattern` (string, glob), `recursive` (boolean, default true), `flatten` (boolean, default true). Accepts directory path as input, outputs matched files into pipeline. CLI-only (filesystem traversal). 12 tests
- [x] `engine/crates/bnto-file` — **`file-copy` processor**: Place output files in destination directory. Params: `destination` (string, path), `create_dirs` (boolean, default true), `conflict` (enum: skip/overwrite/rename, default skip). CLI-only (filesystem write). 10 tests
- [x] `engine/crates/bnto-file` — **`file-metadata` processor**: Extract file properties (size, extension, mime_type, SHA-256 hash). Enriches file metadata map. Browser+CLI (pure-Rust SHA-256, no native deps). 18 tests

#### Wave 3 — Image Processors + All Recipes + Codegen (depends on Wave 2)

Ship image crop/rotate (deferred from Sprint 16) alongside all new file recipes and codegen.

- [ ] `engine/crates/bnto-image` — **Crop image processor**: New `image-crop` processor. Params: `x`, `y`, `width`, `height`, `anchor` (center/top-left/top-right/bottom-left/bottom-right). Auto EXIF orientation via existing `decode_with_orientation()`. RED tests: crop dimensions, bounds validation, anchor positioning, EXIF handling (~6 tests)
- [ ] `engine/crates/bnto-image` — **Rotate image processor**: New `image-rotate` processor. Params: `degrees` (enum: 90/180/270), `flip_horizontal` (bool), `flip_vertical` (bool). Uses existing `image::imageops::rotate*()` + `flip_*()`. RED tests: each rotation angle, flip combinations, rotation+flip compound (~5 tests)
- [ ] `engine/crates/bnto-engine` + `engine/recipes/` — **File + image recipes + codegen**: `flatten-folder.bnto.json` (file-collect → file-copy), `collect-and-rename.bnto.json` (file-collect → file-rename), `crop-images.bnto.json`, `rotate-images.bnto.json`. Register all new processors. Golden tests. Codegen updates. SEO slugs: `/flatten-folder`, `/collect-and-rename`, `/crop-images`, `/rotate-images`. RED tests: recipe execution, golden output verification (~6 tests)

#### Wave 4 — Format Expansion (parallel with Wave 3)

Spreadsheet operations and vector format conversion. Extends recipe breadth.

- [ ] `engine/crates/bnto-csv` — **CSV sort/filter processor**: New `csv-sort` processor. Params: `sort_by` (string, column name), `order` (enum: asc/desc), `filter_column` (string), `filter_value` (string), `filter_op` (enum: equals/contains/greater/less). Browser+CLI. RED tests: sort ascending/descending, filter by value, combined sort+filter, missing column handling (~5 tests)
- [ ] `engine/crates/bnto-csv` — **Excel (.xlsx) read/write**: New `spreadsheet-convert` processor. Read via `calamine`, write via `rust_xlsxwriter`. Params: `direction` (enum: xlsx-to-csv/csv-to-xlsx), `sheet` (string, sheet name for xlsx input). CLI-only initially (WASM binary size concern — evaluate `#[cfg(feature = "native")]`). RED tests: xlsx→csv, csv→xlsx, sheet selection, multi-sheet (~5 tests)
- [ ] `engine/crates/bnto-vector` — **EPS→SVG processor**: CLI-only shell-out via Inkscape or Ghostscript. New `vector-convert` processor. Params: `format` (enum: svg). `bnto doctor` checks for Inkscape/Ghostscript availability. Recipe: `convert-eps-to-svg.bnto.json`. RED tests: EPS conversion, missing binary error, doctor check (~4 tests)

#### Wave 5 — Crate Extraction (parallel with Wave 4)

- [ ] `engine/crates/bnto-editor` — **Extract `bnto-editor` crate**: Move `EditorModel`, `EditorNode`, `EditorSnapshot`, `EditorCommand`, recipe I/O, wizard state model, validation from `bnto/src/tui/screens/` to `engine/crates/bnto-editor/`. TUI becomes a consumer (editor state + TUI rendering). All existing editor unit tests move to crate. RED tests: `EditorModel` is `Send + Sync`, `EditorCommand::apply` is pure (~5 new tests)

**Sprint 17 totals: ~14 PRs, ~72 tests, 6 new recipes (20→26), 4 new file processors, 2 new image processors, 2 new format processors, 1 crate extraction**

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

### TUI Code + Graph Views — DEPRIORITIZED

> **Deprioritized (April 2026).** All editor sprints deprioritized. TUI List editor and Wizard already deliver a solid editing experience. $EDITOR integration and ASCII graph are nice-to-haves, not priorities.

- [ ] `engine/crates/bnto` — **$EDITOR integration**: Press `c` in Editor → export to temp `.bnto.json` → open in `$EDITOR` → validate JSON on return → update EditorModel
- [ ] `engine/crates/bnto` — **ASCII graph renderer**: Press `g` in Editor → read-only box-drawing view of recipe structure

---

### ~~Sprint 15: `bnto-editor` Crate Extraction~~ → Sprint 17 Wave 5

**Promoted to Sprint 17 Wave 5.** Extract the shared editor state model from TUI into standalone `bnto-editor` crate.

---

### Sprint 16: Web List Editor — DEPRIORITIZED

> **Deprioritized (April 2026).** TUI List editor and Wizard already provide a solid editing experience. Web editor investment deferred until demand signals indicate users want browser-based recipe creation beyond the existing open+export tool.

**Goal:** Add the List editor to the web `@bnto/editor` package alongside the existing Visual editor.

**Strategy doc:** [recipe-editors.md](strategy/recipe-editors.md) (§ Web Platform)
**Depends on:** Sprint 11 (engine-owned node schema). Independent of TUI sprints.

**Persona ownership:**

| Package               | Persona              |
| --------------------- | -------------------- |
| `packages/editor`     | `/frontend-engineer` |
| `packages/@bnto/form` | `/frontend-engineer` |

#### Wave 1 — List View Component (sequential)

- [ ] `packages/editor` — **List editor component**: `ListEditor` renders store nodes as ordered step list. Expand/collapse steps. Collapsed shows label + hero param. Keyboard navigation (Arrow keys, Enter). RED tests: renders all nodes, expand/collapse, hero param display (~6 tests)
- [ ] `packages/editor` — **Reorder + Add/Remove in List**: DnD reorder with `@dnd-kit` + keyboard (Shift+Arrow). Node picker popover for adding. Delete with undo. RED tests: drag reorder, keyboard reorder, add from picker, delete with undo (~5 tests)

#### Wave 2 — Editor Switcher (parallel)

- [ ] `packages/editor` — **Editor type switcher**: Toolbar control to switch between Visual (existing), List (new), and Code (Sprint 18). State preserved across switches. Preference persisted to localStorage. RED tests: state preservation across switches, preference persistence (~5 tests)
- [ ] `packages/editor` — **Per-node JSON toggle**: In List editor, each expanded step has "Show JSON" toggle revealing raw JSON for that node. Read-only in list view. RED tests: toggle renders JSON, read-only, per-node state (~3 tests)

**Sprint 16 totals: ~4 PRs, ~19 tests**

---

### Sprint 17: Web Wizard — DEPRIORITIZED

> **Deprioritized (April 2026).** Web editor investment deferred. TUI Wizard (Sprint 13) covers guided creation.

**Goal:** Guided recipe creation for web users. Step-by-step: category → operation → config → done.

**Strategy doc:** [recipe-editors.md](strategy/recipe-editors.md) (§ Wizard Editor)
**Depends on:** Sprint 16 (web List editor for handoff target)

**Persona ownership:**

| Package               | Persona              |
| --------------------- | -------------------- |
| `packages/editor`     | `/frontend-engineer` |
| `packages/@bnto/form` | `/frontend-engineer` |

#### Wave 1 — Wizard Flow (sequential)

- [ ] `packages/editor` — **Wizard flow component**: Step-by-step form (category card grid → operation radio list → config via `@bnto/form` SchemaForm → complete summary). Produces EditorModel, hands off to List editor. Auto-naming. Back navigation. RED tests: step rendering, navigation, config uses SchemaForm, handoff to List editor, skip-to-end (~8 tests)

**Sprint 17 totals: ~1 PR, ~8 tests**

---

### Sprint 18: Web Code Editor (CM6) — DEPRIORITIZED

> **Deprioritized (April 2026).** Web editor investment deferred. TUI Code view covers power-user JSON editing via $EDITOR (also deprioritized).

**Goal:** JSON code editor with CodeMirror 6, following the existing [code-editor.md](strategy/code-editor.md) strategy.

**Strategy doc:** [code-editor.md](strategy/code-editor.md)
**Depends on:** Sprint 16 (editor switcher)

**Persona ownership:**

| Package           | Persona               |
| ----------------- | --------------------- |
| `packages/editor` | `/code-editor-expert` |
| `packages/editor` | `/frontend-engineer`  |

#### Wave 1 — CM6 Integration (sequential)

- [ ] `packages/editor` — **CodeMirror 6 editor view**: JSON editing with validation, hover info, autocompletion from engine JSON Schema. Store sync (debounced). External update annotation. RED tests: renders JSON, validation errors, autocompletion, store sync (~6 tests)

#### Wave 2 — Slash Commands (sequential)

- [ ] `packages/editor` — **Slash command insertion**: `/` trigger shows node type menu, inserts complete valid node JSON block with defaults. Position-aware (only at valid insertion points). RED tests: slash menu, insert JSON, defaults, position check (~4 tests)

**Sprint 18 totals: ~2 PRs, ~10 tests**

---

**After Sprint 15:** Engine hardening, triage items, making the current experience bulletproof. File picker UX overhaul, file node ecosystem expansion, more node types, recipe expansion. All editor sprints (14-18) deprioritized. Distribution (desktop + server) in deep backlog.

---

### Deep Backlog: Distribution (Desktop + Server)

**Deferred.** Desktop (Tauri) and server-side execution in deep backlog. Revisit after TUI ships and community traction emerges.

- Desktop app (Tauri) — links engine natively like CLI, system webview for React frontend
- Server-side execution — cloud infrastructure for premium nodes (AI, shell, video at scale)

---

## Phase 3: Monetization + Polish — TABLED

**Tabled (April 2026).** Monetization is explicitly paused. Focus is on engine power and fun. Revenue strategy revisited when the tool has community traction. The plan below is preserved for when this becomes relevant.

**"Ready to charge" gate:** Before starting, confirm: real users running bntos, conversion hooks built and tested, people return voluntarily, at least one server-side bnto (AI or shell) ready for Pro tier.

### Stripe + Pro Tier (M5) — TABLED

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

## Backlog

### Engine: Sprint 12B Follow-Up (remaining)

**Priority: Medium.** Items unlocked by Sprint 12B (recipe-level deps + shell-command). See [recipe-deps-strategy.md](strategy/recipe-deps-strategy.md).

1. ~~**`bnto install <recipe>`**~~ — **Done.** Sprint 14 Wave 2 (PR #453)
2. ~~**Version constraint enforcement**~~ — **→ Sprint 16 Wave 3**
3. **Per-platform install hints** — Detect OS, show correct package manager command (`apt`, `choco`, `pacman`)
4. ~~**Template expression expansion**~~ — **→ Sprint 16 Wave 4** (`{{env.*}}`, `{{ctx.*}}`, `{{node.<id>.*}}`)

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
- ~~[ ] `engine` — **Excel (.xlsx) read/write**~~ — **→ Sprint 17 Wave 4**

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

### ~~Triage: iLovePNG recipe parity~~ → Sprint 17 Wave 3

**Promoted.** Crop + Rotate promoted to Sprint 17 Wave 3 (deferred from Sprint 16 Wave 5). Remaining iLovePNG candidates (blur face, upscale, HTML to image, meme generator) require ML or headless browser — deep backlog.

### Triage: Engine documentation — auto-generated docs

**Priority: Low.** Set up `cargo doc` or docs site for the Rust engine. Document crate responsibilities, API surface, architecture. `engine/crates/`.

### ~~Triage: Definition/recipe version migration tool~~ → Sprint 16 Wave 4

**Promoted.** `bnto migrate` promoted to Sprint 16 Wave 4.

### ~~@bnto/i18n: Interpolation + Raw Text Migration~~ — ARCHIVED (web frozen)

**Archived (April 2026).** Web is maintenance mode. i18n interpolation is web-only, no CLI/TUI impact.

---

### Homepage & Site Polish — COMPLETE

**Shipped (April 2026).** All pieces delivered. See PLAN-HISTORY.md for full breakdown.

### ~~Triage: Secret/environment variable management for recipes~~ → Sprint 16 Wave 5

**Promoted.** Design spike promoted to Sprint 16 Wave 5 (`strategy/recipe-secrets.md`). Depends on `{{env.*}}` template namespace (Wave 4).

### Triage: E2E teardown cleanup fails in release pipeline

**Priority: Low.** E2E teardown logs `cleanup failed` because `CONVEX_DEPLOYMENT` isn't set in release pipeline. Either pass env var to E2E job or skip cleanup against Vercel preview.

### Infra: Conventional Commits + Auto-Changelog

**Priority: Low.** Enforce `feat:`, `fix:`, `BREAKING CHANGE:` commit format. Auto-generate `CHANGELOG.md` on release tags. Not blocking anything.

### Infra: Production Deploy Protection (GitHub Environments)

**Priority: Low.** Manual approval step via GitHub Environments for production deploys. Existing tag-based workflow already prevents accidental deploys.

### Chore: Upgrade Convex 1.31.7 → 1.33.1

**Priority: Low.** Minor Convex JS SDK update. Bump in `packages/@bnto/backend/`, run `task check`.

### Engine: Vector Format Support — `bnto-vector` Crate (2 of 3 Phases Complete)

**Priority: Backlog (EPS → SVG CLI-only remaining).** New `vector` node category (counterpart to `image`/raster) with three incremental phases. Phases 1-2 shipped. New `bnto-vector` crate houses all vector operations. Full strategy: [file-node-ecosystem.md](.claude/strategy/file-node-ecosystem.md)

**Phase 1 — SVG → Raster (extend `image-convert`): DONE**

- [x] `engine/crates/bnto-vector` — New crate with `resvg` + `usvg` + `tiny-skia` dependencies (PR #364)
- [x] `engine/crates/bnto-image` — Extend `image-convert` to detect SVG input, rasterize via `resvg`, encode to PNG/JPEG/WebP (PR #369)
- [x] `engine/crates/bnto-image` — New `dpi` parameter (default 96, range 72–300) for rasterization resolution (PR #369)
- [x] `engine/crates/bnto-core` — Add `vector` category to `NodeTypeInfo` metadata (PR #364)
- [x] `engine/recipes/` — `svg-to-png.bnto.json`, `svg-to-jpeg.bnto.json` (PR #370)
- [x] Codegen + golden tests + test count updates (PRs #370, #372)
- [x] **Delivers:** `/svg-to-png`, `/svg-to-jpeg` recipe pages (browser + CLI)

**Phase 2 — SVG Optimization (`vector-optimize` processor): DONE**

> Custom XML-level optimizer using roxmltree/xmlwriter (already in WASM binary via resvg transitive chain — zero new deps). 9 cleanup passes: remove metadata, comments, DOCTYPE/PI, editor namespaces, empty containers, empty attributes, unused xmlns, collapse redundant groups, minify whitespace.

- [x] `engine/crates/bnto-vector` — `vector-optimize` processor (custom roxmltree/xmlwriter approach) (PR #379)
- [x] Params: `precision`, `removeComments`, `removeMetadata`, `collapseGroups`, `minify`
- [x] `engine/recipes/` — `optimize-svg.bnto.json`
- [x] Codegen + golden tests + test count updates
- [x] **Delivers:** `/optimize-svg` recipe page

**Phase 3 — EPS → SVG (CLI-only shell-out): → Sprint 17 Wave 4**

Promoted to Sprint 17 Wave 4.

### ~~Backlog: File Node Ecosystem — BRU-Style Composable File Operations~~ → Sprint 17 Wave 2

**Promoted to Sprint 17 Wave 2.** `file-rename` enhancement and `file-sanitize` shipped in Sprint 16. Remaining nodes (`file-filter`, `file-collect`, `file-copy`, `file-metadata`) promoted to Sprint 17. Full strategy: [file-node-ecosystem.md](.claude/strategy/file-node-ecosystem.md)

### Triage: Homepage hero — BRU-style file recipe showcase

**Priority: Triage.** Add a file operation composition (e.g. `collect → sanitize → rename → copy`) as a "Build Your Own" hero snippet in `BuildYourOwnSection`. Demonstrates composable power vs monolithic tools. Blocked on file node ecosystem implementation.

`apps/web/app/(app)/_components/BuildYourOwnSection.tsx`, `recipeSnippets.ts`

### Triage: Context-aware result item actions

**Priority: Triage.** Result items currently only have a download button. Add richer actions depending on the recipe/result type: before/after comparison slider for size-reduction recipes (compress, optimize), copy-to-clipboard for text-based outputs (SVG, CSV, JSON), image preview for raster outputs. Actions should be driven by the result's MIME type and metadata.

`CompletedRow.tsx`, `ResultRow.tsx`

---

---

### ~~Triage: TUI Execution Screen Progress Feedback~~ → Sprint 14 Wave 1

**Promoted to Sprint 14 Wave 1.** See Sprint 14: Engine Hardening above.

---

### Triage: Power Recipe Infrastructure

**Priority: Triage.** Implement foundational engine capabilities (recipe variables, template expressions, data-driven forEach, inter-node data passing) and core node types (shell-command, file-system, spreadsheet-read, http-request) to support complex, data-driven custom recipes like the Etsy Product Image Pipeline. See [power-recipes.md](strategy/power-recipes.md) for full gap analysis, node maps, priority tiers (Tier 0 foundation → Tier 1 nodes → Tier 2 resilience → Tier 3 recipe-as-node), and acceptance test matrix.

### Triage: Security hardening follow-ups (shell-command audit)

**Priority: Triage.** Seven deferred security items from the `shell-command` processor threat model. Each is independent and can be triaged separately:

1. ~~**Recipe trust levels + first-run consent**~~ — **→ Sprint 17 Wave 1**
2. ~~**`bnto inspect <recipe>` command**~~ — **Done.** Shipped as `bnto dry-run` in Sprint 14 Wave 1 (PR #451)
3. **Path traversal prevention in shell-command args** — Sandbox file path arguments to execution working directory. Reject absolute paths and `..` traversal. Canonicalize + verify
4. ~~**Fix `extra_args` whitespace splitting in yt-dlp adapter**~~ — **Resolved.** `bnto-video` crate deleted; `download-video` migrated to shell-command recipe with `{{fields.*}}` templates. The old whitespace-splitting code path no longer exists
5. ~~**TOCTOU fix in NativeContext::temp_file()**~~ — **Done.** Fixed in Sprint 14 Wave 1 (PR #450)
6. **Network capability classification** — Classify recipes as local-only vs network-capable. Warn on network binaries (`curl`, `wget`, `nc`, `ssh`). Future: outbound domain allowlist
7. **Recipe signatures** — Sign built-in recipes. Unsigned community recipes trigger warnings. Foundation for verified registry

### ~~Triage: FullScreenEdit FormMode (default)~~ — DONE (PR #462)

**Delivered in Sprint 16 Wave 1.** `FormMode::FullScreenEdit` shipped in tonkotsu.

### ~~Triage: Rich execution progress UX for CLI/TUI~~ — DESIGN SPIKE DELIVERED (PR #471)

**Design spike delivered.** Strategy doc at `strategy/execution-progress-ux.md`. Three implementation phases defined below, each independently promotable. See the spike for competitive audit, indicator inventory, layout mockups, and architecture review.

### ~~Triage: Execution progress Phase 1~~ → Sprint 17 Wave 1

**Promoted to Sprint 17 Wave 1.** CLI/TUI rendering polish — completion summary, per-file elapsed, braille spinner.

### Triage: Execution progress Phase 2 — Throughput + animated indicators

**Priority: Low (depends on Phase 1 feedback).** Add computed metrics and animated indicators. Still no engine event changes. Single PR, ~4 tests.

- Throughput metric (`4.2 files/s`) when batch >= 5 files
- Animated braille spinner for active node in TUI
- ETA for large batches (10+ files)

Strategy: [execution-progress-ux.md](strategy/execution-progress-ux.md) § Phase 2. `engine/crates/bnto`

### Triage: Execution progress Phase 3 — Size-aware completion summary

**Priority: Low (depends on Phase 2).** Show input→output size comparison on completion. Requires new `PipelineSummary` event or compute-from-disk in CLI renderer. Single PR, ~3 tests.

- Summary: `Completed 10 files in 2.4s (12.4 MB → 3.1 MB, 75% smaller)`

Strategy: [execution-progress-ux.md](strategy/execution-progress-ux.md) § Phase 3. `engine/crates/bnto`, `engine/crates/bnto-core`

### Triage: Rich execution progress UX — animated Unicode indicators

**Priority: Medium.** Unicode progress indicators, animated spinners, token/byte counters, elapsed time. Inspired by Claude Code's progress display (e.g. `✳ Quantumizing… (3m 7s · ↓ 4.6k tokens)`). Make recipe execution feel alive with animated unicode characters, progress bars with percentage, file-level status indicators, and streaming throughput metrics. Covers both CLI (`bnto run`) and TUI execution screen. Depends on Phase 1 (Sprint 17 Wave 1) shipping first.

`engine/crates/bnto`, `engine/crates/bnto-core`

### Triage: Cloud Execution via Railway

**Priority: Triage.** Build `bnto-server` crate (Axum HTTP service wrapping `bnto-engine`), `ServerContext` (sandboxed ProcessContext), Dockerfile with pre-installed binaries, Railway Pro deployment with scale-to-zero, update `execution_engine.ts` to target Railway, R2 presigned URL I/O, execution token auth, SSE progress streaming. Exit criteria: `download-video` runs on bnto.io via cloud execution. Full strategy and phased plan: [`.claude/strategy/cloud-execution.md`](strategy/cloud-execution.md).

`engine/crates/bnto-server/` (new), `packages/@bnto/backend/convex/execution_engine.ts`, `packages/core/src/adapters/`

### Triage: Data-Driven Loop Iteration (O(1) Memory)

**Priority: Triage.** Replace N-files row iteration with `NodeOutput.data` + lazy iteration in loop container. Track node outputs in `run_node_chain`, loop queries upstream structured data and iterates without materializing all files upfront. Also fixes dormant `{{node.*}}` template system (`node_outputs` always empty today). Prerequisite: `spreadsheet-read` processor ships first with pragmatic N-files approach.

`engine/crates/bnto-core/src/executor/container.rs`, `engine/crates/bnto-core/src/executor/mod.rs`, `engine/crates/bnto-core/src/processor.rs`

### Triage: Pipeline Environment-Agnosticism Audit

**Priority: Triage.** Audit the pipeline/execution code for environment-specific logic that should be pushed to the environment layer. The pipeline should stay dumb — produce output bytes — and let each environment (CLI, browser/WASM, TUI) read output node metadata and decide what to do with I/O. Check: progress reporting wiring, output path decisions, file I/O handling, anything that makes the pipeline aware of which environment it's running in. Goal: execution code is simple, straightforward, and environment-agnostic.

`engine/crates/bnto-core/src/executor/`, `engine/crates/bnto-engine/src/`, `engine/crates/bnto/src/main.rs`

---

### Triage: bulk-video-download output routing

**Priority: Triage.** The bulk-video-download recipe currently hardcodes the download path in the shell-command's `-o` arg. Instead, the output node's `directory` param should control where files end up, and the shell-command should write to `{{output_dir}}` (engine-managed temp). The loop's `progressive` output mode should populate the Downloads folder incrementally as iterations complete. Investigate whether the output node + progressive output pipeline is sufficient, or if yt-dlp's `-o` needs a special path for its own subdirectory structure (`{{item.group}}/`).

`/Users/Ryan/.bnto/recipes/bulk-video-download.bnto.json`

---

### Triage: BntoPaths audit — no rogue path building

**Priority: Triage.** Audit entire codebase for path formation that bypasses `BntoPaths`. All derivation of bnto home, recipes, logs, cache, and state dirs must go through `BntoPaths` module — no hardcoded `~/.bnto/`, no manual `PathBuf::from` construction outside `BntoPaths`. Fix any violations.

`engine/crates/bnto/src/storage/paths.rs`, grep for `.join("recipes")`, `.join(".bnto")`, `home_dir()` outside `BntoPaths`

---

### Triage: Home path change — offer to migrate data

**Priority: Triage.** When a user changes the Home Path setting in the TUI, prompt them: "Do you want to move your existing data from `<old path>` to `<new path>`?" If yes, move the contents of the old home (recipes, state, cache) to the new location. If no, just update the config pointer. Prevents orphaned data and avoids users needing to manually copy files.

`engine/crates/bnto/src/tui/screens/settings.rs`, `engine/crates/bnto/src/tui/app_helpers/`

---

### Triage: Dry-run preview for file-processing recipes

**Priority: Triage.** `bnto dry-run` only shows shell commands/dependencies (useful for shell-command recipes). File-processing recipes (flatten-folders, file-rename, etc.) need a dry-run mode that shows "file X would become Y" without writing anything. Should work in both CLI (`bnto dry-run flatten-folders /path/to/dir`) and TUI (pre-execution preview screen).

`engine/crates/bnto/src/commands/dry_run.rs`, `engine/crates/bnto-engine/src/executor/`

### ~~Triage: Audit output node coupling to file writes~~ — DONE (PR #512)

**Delivered.** Output node modes refactored: `download/display/preview` → `write/overwrite/message/none`. `file-move` processor removed. `WriteOutcome` enum in `io.rs` dispatches mode-aware output. `resolve_output_mode()` in `pipeline.rs`. CLI + TUI bridges respect mode. PR 1 of 2 (`refactor/output-node-modes`).

### Triage: Audit node-to-node data passing architecture

**Priority: Triage.** Nodes should hand off rich context (file path, metadata, what they did) to the next node rather than relying on implicit assumptions. Currently processors like file-rename do filesystem work (fs::rename) that may belong in the pipeline runner or output writer — nodes are doing secret file management instead of their one job. Need a deep review of: (1) what context flows between nodes (FileData, filename, metadata), (2) whether processors are doing filesystem work that belongs at a different layer, (3) the boundary between "process this data" and "manage where files live on disk", (4) namespace/context available to nodes. Related to "Audit output node coupling" backlog item above.

`engine/crates/bnto-file/src/rename.rs`, `engine/crates/bnto-core/src/processor.rs`, `engine/crates/bnto-engine/src/pipeline/`

---

## Reference

| Document                                   | Purpose                                                                      |
| ------------------------------------------ | ---------------------------------------------------------------------------- |
| [PLAN-HISTORY.md](PLAN-HISTORY.md)         | Completed sprint history (Phase 0 through Sprint 13, Homepage)               |
| `.claude/strategy/engine-expansion.md`     | Engine expansion strategy — dependency system, ProcessContext, TUI, taxonomy |
| `.claude/strategy/tonkotsu-strategy.md`    | `tonkotsu` crate — rich terminal forms for ratatui, huh parity               |
| `.claude/strategy/tui-controls-bubbles.md` | Sprint 15 — Bubbles-inspired display/edit modes, FilePath control, picker UX |
| `.claude/strategy/engine-execution.md`     | Engine execution architecture — pipeline executor, progress events           |
| `.claude/strategy/bntos.md`                | Predefined Bnto registry — slugs, fixtures, SEO targets, tiers               |
| `.claude/strategy/core-principles.md`      | Trust commitments, key principles                                            |
| `.claude/rules/`                           | Auto-loaded rules (architecture, code-standards, engine-node-patterns, etc.) |
| `.claude/skills/`                          | Agent skills (pickup, project-manager, code-review, pre-commit)              |
