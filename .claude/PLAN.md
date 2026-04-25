# Bnto — Build Plan

**Last Updated:** April 25, 2026 (Groom: Sprint 15 complete, Sprint 16 defined — recipe expansion + huh parity + engine infra)
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

**CLI is the product.** `cargo install bnto` gets you 18 recipes. The web is a landing page.

- **v0.12.0 released (April 2026):** 18 recipes, video-download node (yt-dlp), dependency system, ProcessContext, `bnto list/info/run/doctor/install/dry-run` commands, TUI execution progress, vector operations (SVG). Published to crates.io
- **Engine (Rust):** Library crates (bnto-core, bnto-image, bnto-csv, bnto-file, bnto-shell, bnto-engine), WASM entry point (bnto-wasm), CLI binary (bnto). CLI is the primary consumer, browser (WASM) is secondary
- **M1-M2 delivered:** Browser execution (WASM), editor v1, accounts, execution history — all shipped but web is now maintenance mode
- **CLI/TUI-first pivot (April 2026):** Web reduced to landing page. Editor frozen. Auth stripped. Frontend/premium work on hold. Focus: engine, CLI, TUI, infra
- **TUI delivered (Sprint 10):** `bnto tui` via ratatui + crossterm — 7 screens (home, browser/recipes, detail, picker, execution, results, settings), 400+ tests
- **TUI schema-driven config (Sprint 11):** Type-aware parameter controls (boolean toggles, enum selects, number sliders, validation), engine-owned node schema, ~930 LOC hand-written TS deleted
- **Recipe-level deps + shell-command (Sprint 12B):** `PipelineDefinition.requires`, `shell-command` processor with security boundary, `download-video` migrated from dedicated crate, `bnto-video` deleted. Recipe fields (`{{fields.*}}`) delivered
- **Data persistence + Home + Library (Sprint 12A):** XDG-compliant storage (`BntoPaths`), atomic writes, TOML config, Home screen, My Library, `bnto` = TUI default
- **`bnto-form` crate (Sprint 11.5):** Standalone ratatui form widget library (TextInput, Select, Confirm, Number), TEA-native, zero bnto dependency. ~105 tests
- **TUI List Editor (Sprint 12):** Full recipe editing — add/remove/reorder nodes, inline param editing, undo/redo, save workflow, multiple entry points. ~75 tests
- **TUI Wizard (Sprint 13):** Guided recipe creation — category → operation → config → done. Hands off to List editor
- **TUI controls polish (Sprints 14-15):** Bubbles-inspired display/edit modes, FilePath field type, TextArea, fuzzy Select filter, picker search/metadata/breadcrumbs, vim keybindings. `bnto-form` now at huh parity for shipped controls
- **Next:** Sprint 16 — recipe expansion (file nodes, image crop/rotate), bnto-form huh parity (FullScreenEdit, MultiSelect, field grouping, Note), engine infra (template expressions, version constraints, migration tool), design spikes
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

### Sprint 11.5: `bnto-form` — TUI Form Widget Crate — COMPLETE

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

**Sprints 14-15 complete.** Engine hardening (PRs #448-#454) and TUI controls polish (PRs #455-#457) both shipped. `bnto-form` now has Display/Edit mode, FilePath, TextArea, fuzzy Select — matching Charm Bubbles quality.

**Next: Sprint 16 — Recipe Expansion + huh Parity.** Three streams: (1) bring `bnto-form` to full huh parity (FullScreenEdit, MultiSelect, field grouping, Note field), (2) expand file operations (BRU-style composable nodes), (3) add image crop/rotate for iLovePNG parity. Plus engine infrastructure (template expressions, version constraints, migration tool). Grows recipe count from 18→22+.

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
| `bnto-form/src/form.rs`     | ~523       | Form widget — extract widget-specific logic                               |
| `bnto/src/tui/keys.rs`      | ~668       | Key dispatch — idiomatic match arms, evaluate extraction                  |

Note: TEA `update()` match blocks and `handle_*_key()` are idiomatic Rust (per MEMORY.md) — splitting them would be worse. But `app.rs` at 1305 prod lines has room to extract screen-specific update handlers into separate modules. `metadata.rs` and `form.rs` are not TEA patterns and should be broken up.

- [x] `engine/crates/bnto` — **Audit + extract `app.rs`**: Extracted 24 handler functions into `app_helpers/` module directory (6 submodules: navigation, editor, wizard, home_detail, library, settings). `app.rs` reduced from 1305 → 300 prod lines
- [x] `engine/crates/bnto-core` — **Break up `metadata.rs`**: Extracted `node_types.rs` (252 prod) and `parameters.rs` (107 prod). `metadata.rs` reduced to 85 prod lines (re-export hub)
- [x] `engine/crates/bnto-form` — **Break up `form.rs`**: Extracted `controls/dispatch.rs` (301 prod — TEA dispatch, idiomatic). `form.rs` reduced to 142 prod lines
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

**Goal:** Make `bnto-form` controls and the file picker feel as polished as [Charm Bubbles](https://github.com/charmbracelet/bubbles). Form fields get display/edit modes. File selection becomes a form control. Picker gets search, metadata, breadcrumbs.

**Strategy doc:** [tui-controls-bubbles.md](strategy/tui-controls-bubbles.md)

**Persona ownership:**

| Package                   | Persona        |
| ------------------------- | -------------- |
| `engine/crates/bnto-form` | `/rust-expert` |
| `engine/crates/bnto`      | `/rust-expert` |

#### Wave 1 — Form Control Interaction Model (sequential)

- [x] `engine/crates/bnto-form` — **Display/Edit mode for form fields**: Each field renders a compact display line (label + value). Enter opens edit mode (full control). Enter/Esc returns to display. RED tests: display rendering, mode transitions, value preservation (~6 tests)
- [x] `engine/crates/bnto-form` — **FilePath field type**: New `FieldKind::FilePath` renders as path string in display mode. Edit mode opens inline directory browser (picker logic extracted into bnto-form). RED tests: display, browser nav, file selection, ext filter (~8 tests)

#### Wave 2 — Picker Polish (parallel with Wave 1 completion)

- [x] `engine/crates/bnto` — **Picker search/filter**: Inline text search filters entries by filename (case-insensitive). Backspace clears. Shows match count. RED tests: filter narrows, clear restores, empty state (~5 tests)
- [x] `engine/crates/bnto` — **Picker file metadata columns**: Aligned perms + human-readable sizes. Toggle with `p`. Symlink `->` indicator. RED tests: size format, perms display, symlink indicator (~4 tests)
- [x] `engine/crates/bnto` — **Picker breadcrumb path**: Styled path segments replacing plain directory string. Current dir highlighted. RED tests: breadcrumb rendering, segment styling (~3 tests)

#### Wave 3 — Form Control Refinements (depends on Wave 1)

- [x] `engine/crates/bnto-form` — **Select with fuzzy filter**: Typing filters options by fuzzy substring. Cycling preserved when no filter. RED tests: fuzzy match, cycling fallback, clear filter (~4 tests)
- [x] `engine/crates/bnto-form` — **TextArea field type**: Multi-line input. Display shows first line + count. Edit shows scrollable editor. RED tests: multi-line, scroll, display truncation (~5 tests)

**Sprint 15 totals: ~7 PRs, ~35 tests**

---

## Sprint 16: Recipe Expansion + huh Parity — NEXT

**Goal:** Rename `bnto-form` → `tonkotsu` and make it the Rust equivalent of Charm's [huh](https://github.com/charmbracelet/huh) library. Expand recipe catalog with new image and file operations. Strengthen engine infrastructure for future recipes. Grows recipe count from 18→22+.

**Strategy docs:** [bnto-form-strategy.md](strategy/bnto-form-strategy.md) (§ huh Parity), [file-node-ecosystem.md](strategy/file-node-ecosystem.md) (Phases 1-2), [tui-controls-bubbles.md](strategy/tui-controls-bubbles.md)

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

- [ ] `engine/crates/bnto-form` → `engine/crates/tonkotsu` — **Rename crate to tonkotsu**: Rename directory (`git mv`), update `Cargo.toml` (package name, bin name → `tonkotsu-demo`), update workspace `Cargo.toml` member, update `bnto/Cargo.toml` dependency, update all `use bnto_form::` → `use tonkotsu::` imports (~15 source files), update `Taskfile.yml` (`form:demo` task), update strategy docs + PLAN.md + CLAUDE.md references. Verify `task wasm:lint && task cli:test` pass clean. (~0 tests — pure rename, existing tests validate)

#### Wave 1 — tonkotsu: huh Parity (parallel, depends on Wave 0)

Bring `tonkotsu` (formerly `bnto-form`) to full feature parity with Charm's huh library. See [bnto-form-strategy.md § huh Parity](strategy/bnto-form-strategy.md) for the gap analysis.

- [ ] `engine/crates/tonkotsu` — **FullScreenEdit form mode**: Third `FormMode` variant. Display mode identical to DisplayEdit (compact one-liners). Edit mode hides all other fields, renders dedicated panel with label header + full control + helper footer. Becomes default demo mode. RED tests: display rendering, edit panel visibility, mode transitions, all field types, panel framing, helper text (~8 tests)
- [ ] `engine/crates/tonkotsu` — **MultiSelect field type**: New `FieldKind::MultiSelect`. Display: `"Tags: image, vector (2 selected)"`. Edit: checkboxes with Space to toggle, Enter to confirm. Wrapping navigation. RED tests: toggle selection, display formatting, confirm/cancel, empty selection (~5 tests)
- [ ] `engine/crates/tonkotsu` — **Field grouping**: `FieldGroup` wraps fields into named sections. In FullScreenEdit, groups render as navigable pages (next/prev). In DisplayEdit, groups render as visual sections with headers. RED tests: group rendering, page navigation, field-to-group mapping (~5 tests)
- [ ] `engine/crates/tonkotsu` — **Note field type**: Read-only `FieldKind::Note` for informational text between fields. Not editable, not focusable. Display: styled text block. RED tests: renders text, skipped by focus navigation, not editable (~3 tests)

#### Wave 2 — File Node Expansion (parallel with Wave 1)

BRU-style composable file operations. See [file-node-ecosystem.md](strategy/file-node-ecosystem.md) Phase 1 (enhance file-rename) and Phase 2 (file-sanitize).

- [ ] `engine/crates/bnto-file` — **Enhance file-rename: counter + extension params**: Add `counter_start` (integer, default 1), `counter_pad` (integer, default 0), `extension` (string) params. New `{{counter}}` template variable (auto-incrementing, respects start/pad). RED tests: counter formatting, zero-pad widths, extension replacement, counter across files (~5 tests)
- [ ] `engine/crates/bnto-file` — **File-sanitize processor**: New `file-sanitize` processor. Params: `mode` (slugify/strip/normalize), `separator` (default `-`), `max_length` (default 0 = no limit). Pure string manipulation, browser+CLI. RED tests: each mode, unicode normalization, max length truncation, separator replacement (~5 tests)
- [ ] `engine/crates/bnto-engine` + `engine/recipes/` — **File recipes + codegen**: `number-files.bnto.json` (file-rename with counter), `sanitize-filenames.bnto.json` (file-sanitize). Register processors. Golden tests. Codegen updates. SEO slugs: `/number-files`, `/sanitize-filenames`. RED tests: recipe execution, golden output verification (~4 tests)

#### Wave 3 — Engine Infrastructure (parallel with Wave 2)

Template expressions, version constraints, and migration tooling. Strengthens the engine for future recipe complexity.

- [ ] `engine/crates/bnto-core` — **Template expression expansion**: Extend `{{fields.*}}` template system with `{{env.*}}` (environment variables), `{{ctx.*}}` (execution context — temp dir, working dir, platform), `{{node.<id>.*}}` (inter-node output references). RED tests: each namespace resolution, missing var handling, nested references (~6 tests)
- [ ] `engine/crates/bnto-core` — **Version constraint enforcement**: Parse `<binary> --version` output, validate against `Dependency.version` semver constraint. Fail pipeline before execution if version doesn't satisfy. RED tests: semver parsing, constraint matching, version extraction from output (~5 tests)
- [ ] `engine/crates/bnto` — **`bnto migrate` CLI command**: Migrate `.bnto.json` files across breaking parameter changes (e.g., `compression`→`quality`). Detect version, apply sequential transforms, report changes. RED tests: version detection, migration transforms, idempotent re-run (~5 tests)

#### Wave 4 — Design Spikes (parallel with Wave 3, strategy docs not code)

Write strategy docs to unblock future sprints. No code — research, mockups, and architecture decisions.

- [ ] `.claude/strategy/execution-progress-ux.md` — **Rich execution progress UX design spike**: Competitive audit (Claude Code, cargo, docker, Bubbles), Unicode indicator inventory (spinners, progress bars, frames), metrics design (elapsed, throughput, ETA, file count), layout mockups (CLI single-line vs TUI multi-line), architecture review (engine events vs rendering), phased scope recommendation
- [ ] `.claude/strategy/recipe-secrets.md` — **Secret/env variable management design spike**: How recipes reference secrets without embedding in `.bnto.json`. Resolution per target (CLI reads env/dotfiles, server reads vault, browser prompts). Integration with `{{env.*}}` template namespace (Wave 4). Threat model for secret exposure

#### Wave 5 — Image Recipe Expansion (depends on Wave 2 codegen pattern)

iLovePNG parity — crop and rotate. Uses existing `image` crate primitives (zero new dependencies).

- [ ] `engine/crates/bnto-image` — **Crop image processor**: New `image-crop` processor. Params: `x`, `y`, `width`, `height`, `anchor` (center/top-left/top-right/bottom-left/bottom-right). Auto EXIF orientation via existing `decode_with_orientation()`. RED tests: crop dimensions, bounds validation, anchor positioning, EXIF handling (~6 tests)
- [ ] `engine/crates/bnto-image` — **Rotate image processor**: New `image-rotate` processor. Params: `degrees` (enum: 90/180/270), `flip_horizontal` (bool), `flip_vertical` (bool). Uses existing `image::imageops::rotate*()` + `flip_*()`. RED tests: each rotation angle, flip combinations, rotation+flip compound (~5 tests)
- [ ] `engine/crates/bnto-engine` + `engine/recipes/` — **Crop/Rotate recipes + codegen**: `crop-images.bnto.json`, `rotate-images.bnto.json`. Register processors. Golden tests. Codegen updates. SEO slugs: `/crop-images`, `/rotate-images`. RED tests: recipe execution, golden output verification (~4 tests)

**Sprint 16 totals: ~14 PRs, ~72 tests, 4 new recipes (18→22), 2 strategy docs, 1 crate rename**

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

### Sprint 15: `bnto-editor` Crate Extraction — BACKLOG

**Goal:** Extract the shared editor state model from TUI into standalone `bnto-editor` crate. Reusable for desktop (Tauri) and third-party integrations.

**Implementation plan:** [editor-implementation-plan.md](strategy/editor-implementation-plan.md) (§ Phase 4)
**Depends on:** Sprint 12-14 (editor state model proven in production)

**Persona ownership:**

| Package                     | Persona        |
| --------------------------- | -------------- |
| `engine/crates/bnto-editor` | `/rust-expert` |
| `engine/crates/bnto`        | `/rust-expert` |

#### Wave 1 — Extract (sequential)

- [ ] `engine/crates/bnto-editor` — **Extract `bnto-editor` crate**: Move `EditorModel`, `EditorNode`, `EditorSnapshot`, `EditorCommand`, recipe I/O, wizard state model, validation from `bnto/src/tui/screens/` to `engine/crates/bnto-editor/`. TUI becomes a consumer (editor state + TUI rendering). All existing editor unit tests move to crate. RED tests: `EditorModel` is `Send + Sync`, `EditorCommand::apply` is pure (~5 new tests)

**Sprint 15 totals: ~1 PR, migration + ~5 new tests**

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
2. ~~**Version constraint enforcement**~~ — **→ Sprint 16 Wave 4**
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

### ~~Triage: iLovePNG recipe parity~~ → Sprint 16 Wave 3

**Promoted.** Crop + Rotate promoted to Sprint 16 Wave 3. Remaining iLovePNG candidates (blur face, upscale, HTML to image, meme generator) require ML or headless browser — deep backlog.

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

**Phase 3 — EPS → SVG (CLI-only shell-out): Priority: Low.**

- [ ] `engine/crates/bnto-vector` — EPS/AI→SVG processor via Inkscape/Ghostscript shell-out
- [ ] `#[cfg(feature = "native")]` only — no browser support
- [ ] `bnto doctor` checks for Inkscape/Ghostscript availability
- [ ] `engine/recipes/` — `convert-eps-to-svg.bnto.json`
- [ ] Codegen + golden tests + test count updates
- [ ] **Delivers:** `/convert-eps-to-svg` recipe page (CLI-only)

### Backlog: File Node Ecosystem — BRU-Style Composable File Operations

**Priority: Backlog (after vector work).** Expand the `file` category from 1 recipe to 6-8 with composable node processors inspired by Bulk Rename Utility. Enhance `file-rename` (counter, extension params), add new nodes (`file-collect`, `file-copy`, `file-filter`, `file-sanitize`, `file-metadata`). Each node unlocks standalone recipes and custom compositions. Full strategy: [file-node-ecosystem.md](.claude/strategy/file-node-ecosystem.md)

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

1. **Recipe trust levels + first-run consent** — Distinguish built-in/local/community recipes. Prompt before executing `shell-command` nodes from untrusted sources. Cache approval per recipe hash. (P0 for community recipes, not needed while all recipes are built-in)
2. ~~**`bnto inspect <recipe>` command**~~ — **Done.** Shipped as `bnto dry-run` in Sprint 14 Wave 1 (PR #451)
3. **Path traversal prevention in shell-command args** — Sandbox file path arguments to execution working directory. Reject absolute paths and `..` traversal. Canonicalize + verify
4. ~~**Fix `extra_args` whitespace splitting in yt-dlp adapter**~~ — **Resolved.** `bnto-video` crate deleted; `download-video` migrated to shell-command recipe with `{{fields.*}}` templates. The old whitespace-splitting code path no longer exists
5. ~~**TOCTOU fix in NativeContext::temp_file()**~~ — **Done.** Fixed in Sprint 14 Wave 1 (PR #450)
6. **Network capability classification** — Classify recipes as local-only vs network-capable. Warn on network binaries (`curl`, `wget`, `nc`, `ssh`). Future: outbound domain allowlist
7. **Recipe signatures** — Sign built-in recipes. Unsigned community recipes trigger warnings. Foundation for verified registry

### Triage: FullScreenEdit FormMode (default)

**Priority: Triage.** Add a third `FormMode::FullScreenEdit` that renders only the focused field's control on a dedicated sub-screen when editing (like Bubble Tea's `huh` library). User wants this as the default form mode instead of `DisplayEdit` which expands inline. `engine/crates/bnto-form`. Related: `.claude/strategy/tui-controls-bubbles.md`

### Triage: Rich execution progress UX for CLI/TUI — NEEDS DESIGN SPIKE

**Priority: Triage (blocked on strategy doc).** Make recipe execution feel alive with animated Unicode progress indicators, spinners, throughput counters, and elapsed time — inspired by Claude Code's progress display (e.g. `✳ Quantumizing… (3m 7s · ↓ 4.6k tokens)`). Covers both `bnto run` (CLI) and TUI execution screen: animated Unicode spinner characters, progress bars with percentage, per-file status indicators, streaming byte/throughput metrics.

**Before this can be promoted to a sprint, write a strategy doc** (`strategy/execution-progress-ux.md`) covering:

- [ ] **Competitive audit**: Survey progress UX in Claude Code, `cargo install` progress, `docker pull`, Charm Bubbles spinner/progress, `npx`/`pnpm` install indicators. Screenshot or describe each
- [ ] **Unicode indicator inventory**: Which spinner characters (braille, box-drawing, geometric shapes, emoji), animation frame sequences, and progress bar styles to support
- [ ] **Metrics design**: What metrics to surface per context — elapsed time, throughput (bytes/s, files/s), ETA, file count progress (3/12), current file name, current node name
- [ ] **Layout design**: ASCII mockups for CLI (`bnto run`) single-line progress vs TUI execution screen multi-line layout. How does it degrade on narrow terminals?
- [ ] **Architecture**: Where do progress events originate (engine `bnto-core` events vs CLI rendering), what new event types are needed beyond existing `PipelineStarted`/`NodeStarted`/`FileProgress`/`NodeCompleted`/`PipelineCompleted`
- [ ] **Scope definition**: What ships in the first PR vs follow-up. Recommend phased: spinner+elapsed first, then throughput metrics, then per-file progress bars

`engine/crates/bnto`, `engine/crates/bnto-core`

---

## Reference

| Document                                   | Purpose                                                                      |
| ------------------------------------------ | ---------------------------------------------------------------------------- |
| [PLAN-HISTORY.md](PLAN-HISTORY.md)         | Completed sprint history (Phase 0 through Sprint 13, Homepage)               |
| `.claude/strategy/engine-expansion.md`     | Engine expansion strategy — dependency system, ProcessContext, TUI, taxonomy |
| `.claude/strategy/bnto-form-strategy.md`   | `bnto-form` crate — huh-inspired ratatui form widgets, ecosystem research    |
| `.claude/strategy/tui-controls-bubbles.md` | Sprint 15 — Bubbles-inspired display/edit modes, FilePath control, picker UX |
| `.claude/strategy/engine-execution.md`     | Engine execution architecture — pipeline executor, progress events           |
| `.claude/strategy/bntos.md`                | Predefined Bnto registry — slugs, fixtures, SEO targets, tiers               |
| `.claude/strategy/core-principles.md`      | Trust commitments, key principles                                            |
| `.claude/rules/`                           | Auto-loaded rules (architecture, code-standards, engine-node-patterns, etc.) |
| `.claude/skills/`                          | Agent skills (pickup, project-manager, code-review, pre-commit)              |
