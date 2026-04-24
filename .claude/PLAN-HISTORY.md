# Bnto — Completed Sprint History

**Purpose:** Detailed history of completed sprints, moved from PLAN.md to keep the active plan focused. See [PLAN.md](PLAN.md) for current state, active sprints, and backlog.

---

## Phase 0: Foundation — COMPLETE

Monorepo restructuring, engine solidification with TDD (>90% coverage on all 10 node types), integration test fixtures, CLI smoke tests, Go API server, Convex setup, web app shell, @bnto/core hooks.

## Sprint 1: Infrastructure Migration — COMPLETE

Moved from Railway/Convex Auth to Vercel/Better Auth. Auth provider, Convex schema, Vercel deployment, proxy middleware, sign-in/sign-up/sign-out pages, route protection. Wave 4 (auth verification) was skipped — gaps caught and resolved in Sprint 2A.

## Sprint 2: Predefined Bntos + Cloud Execution — Waves 1-4 COMPLETE

6 Tier 1 fixtures, SEO URL routing, bnto registry, tool page UI (file drop, per-bnto config), R2 file transit, Railway deployment, env config (R2/Convex/Vercel/Railway), execution UI (RunButton, ExecutionProgress, ExecutionResults), predefined execution path. Wave 5 (pipeline verification) blocked by auth — deferred to Sprint 2A Wave 5.

## Sprint 2A: Auth Fix — COMPLETE

Migrated to `@convex-dev/auth` (eliminates JWT race condition). Anonymous sessions, proxy middleware, integration tests (A1-A7, C1-C3, S1-S3). Core integration test harness (ConvexHttpClient factory). Execution + upload/download integration tests. Playwright E2E pipeline verification. Auth evaluation documented in git history.

## Sprint 2.5: Codebase Polish — COMPLETE

Node.js subpath imports (`#components/*`, `#lib/*`), camelCase file rename (hooks, utils, lib), PascalCase component rename, dot-notation primitive wrappers, Button audit/migration, Button pseudo-state fix, Button animations (Mini Motorways motion language). Font review (DM Sans → Geist evaluation) deferred to backlog.

## Sprint 2B: Browser Execution (M1 MVP) — COMPLETE

All 6 Tier 1 bntos running 100% client-side via Rust→WASM. `@bnto/nodes` package (engine-agnostic definitions), Rust workspace with 5 crates, Web Worker wrapper, browser adapter in `@bnto/core`, BntoPageShell browser routing, ZIP download for multi-file results. Rust evaluation checkpoint PASSED. WASM bundle: 1.6MB raw / 606KB gzipped. 44+ Rust unit tests, WASM integration tests, Playwright E2E with screenshot assertions for all 6 bntos. **M1 milestone delivered.**

## Sprint 2C: Launch Readiness — COMPLETE

bnto.io live and indexable. All Mainline template content replaced with real bnto content (home, pricing, FAQ, privacy, footer, navbar). Messaging audit (no false claims). CSS animation refactor (JS → CSS-driven). Site navigation E2E tests. 15/15 static pages generate cleanly.

## Sprint 2D: Recipe Page UX Overhaul — COMPLETE

Progressive phase-driven flow (Files → Configure → Results) with Motorway design language. RecipeShell, PhaseIndicator, FileCard, RecipeConfigSection, useRecipeFlow, per-instance execution stores. 27+ screenshots regenerated. All 4 waves complete.

## Sprint H: Housekeeping — COMPLETE

Tech debt cleanup: FileUpload→react-dropzone, core.browser→core.wasm rename, shared ESLint config, Pressable component, React import sweep, GitHub Actions CI (PR #10), Rust test audit, EXIF orientation coverage. All tasks delivered.

## Sprint 3A: Remove Anonymous User System — COMPLETE

Eliminated anonymous Convex session system across 5 waves (backend schema, core hooks, web components, auth E2E, docs cleanup). Auth is now binary: signed in or not. 13/13 auth E2E tests passing. All anonymous references removed from schema, code, and docs.

## Sprint 3: Platform Features (M2) — COMPLETE (Wave 3 tabled)

Accounts earn their keep: execution history (IndexedDB for unauth, Convex for auth), `/my-recipes` dashboard, PostHog telemetry, Lighthouse CI, save prompt conversion hook, pricing page, browser auth verification, execution history migration on signup. Wave 3 (3 E2E test tasks) tabled — see backlog "Testing: Sprint 3 Deferred E2E Tests."

## Sprint 4: Recipe Editor (Headless-First) — COMPLETE

Headless-first editor: Wave 1 (`@bnto/nodes` pure functions — CRUD, adapters, tests), Wave 2 (Zustand store, ReactFlow adapters, hooks), Wave 3 (Motorway MVP — BentoCanvas, EditorToolbar, NodePalette, NodeConfigPanel, RecipeEditor). Architecture: `@bnto/nodes` → pure functions → Zustand store → React hooks → visual skin. Two entry points: `createBlankDefinition()` or `loadRecipe(slug)`.

## Sprint 4C: Input & Output Nodes — COMPLETE

Self-describing recipes via `input` and `output` node types (PR #102). 4 waves: Wave 1 (`@bnto/nodes` — I/O types, schemas, recipe updates, 22 tests), Wave 2 (`@bnto/core` adapter reads I/O nodes, editor store singleton constraints), Wave 3 (generic InputRenderer/OutputRenderer, I/O compartment rendering), Wave 4 (RecipeShell migration, per-slug I/O code deleted, E2E verified).

## Sprint 4D: Extract `@bnto/ui` (Motorway Design System) — COMPLETE

Moved all UI primitives, design tokens, and shared components from `apps/web/components/` to `packages/ui/` as `@bnto/ui`. Zero domain knowledge — pure visual building blocks. 3 waves: package scaffold + primitives, shared components, rewire + verify (PR #103).

## Sprint 4E: Extract `@bnto/editor` — COMPLETE

Moved all editor components from `apps/web/components/editor/` to `packages/editor/` as `@bnto/editor`. Editor depends on `@bnto/ui` + `@bnto/core` + `@bnto/nodes`. 2 waves: package scaffold + move, rewire + verify. 90 editor tests + 66 web tests pass.

## Sprint 4F: Code Standards Review — COMPLETE

Audited all active code against updated `code-standards.md` (March 2026 tightened limits). 3 waves: per-package file size + structure audit (all 6 packages), cross-cutting DRY + Object.assign + Server Component audit, Zustand store ownership audit. Every file conforms.

## Sprint 4G: Versioning & Node Validation — COMPLETE

Format versioning activated across the stack. Zod schemas replaced hand-rolled `ParameterSchema` DSL for all 15 node types. Schema-driven config panel with `CONTROL_REGISTRY` map dispatching Zod-inferred `FieldControl` types to `@bnto/ui` controls. 3 waves (PRs #114-#116).

## Sprint 4H: Pipeline Executor Extraction — COMPLETE

Runtime-agnostic `executePipeline()` extracted to `@bnto/core`. `NodeRunner` contract, `processFiles()` removed from browser adapter. Comprehensive TDD test suite (pure Node.js, no browser). 4 waves: types + tests, implementation, adapter cleanup, export + E2E verification.

## Sprint 5D: Editor API Layer — COMPLETE

`createEditor()` factory with `client → service → store` abstraction mirroring `@bnto/core`. 5 domain clients (nodes, definition, execution, history, panels), 5 services, React binding layer (`EditorProvider`, `useEditor`, domain hooks), full component migration, deprecated hooks deleted. 5 waves.

## Editor Beta Launch — COMPLETE

Feature flag removed, beta badges on nav/CTAs, dismissible banner on `/editor` with localStorage persistence. E2E verified (PR #173).

## Sprint 5A: Editor UX — COMPLETE

Node interaction + empty state + config polish. 5 waves: hover delete overlay + PlaceholderSlot + isIoNode flag, exit animations, config panel identity echo, LayerPanel drag-to-reorder, empty canvas auto-behaviors, E2E verification.

## Sprint 5B Wave 1: I/O Node Visual Hierarchy — COMPLETE

Size differentiation (100×100 vs 120×120), muted color for I/O nodes, elevation distinction (sm vs md), Pressable behavior split (I/O not pressable-to-configure). Unit tests for all visual distinctions.

## Sprint 5C: Editor Copy + Nav Labels — COMPLETE

Renamed nav "Create" → "New Recipe", recipe page CTA "Customize in Editor" → "Open in Editor". Grep-verified no remaining old copy.

## Sprint 5: Editor v1 (M2 Completion) — COMPLETE

Editor shipped as usable v1: auto-download default, config panel controls, schema metadata cleanup, save to account + My Recipes integration + unsaved changes warning, keyboard shortcuts, accessibility audit. All 4 waves complete. **M2 milestone delivered.**

---

## Sprint 6: Quality & Cleanup — COMPLETE

**Goal:** Lock in quality after M2. Clean up dead code, add error boundaries, audit performance, resolve triage items. No new features — stabilize what's built before expanding.

**Persona ownership:**

| Package                | Persona                                 |
| ---------------------- | --------------------------------------- |
| `apps/web`             | `/frontend-engineer` + `/nextjs-expert` |
| `packages/core`        | `/core-architect`                       |
| `packages/@bnto/nodes` | `/core-architect`                       |
| `engine`               | `/rust-expert`                          |

### Wave 1 (parallel — error boundaries + dead code)

- [x] `apps/web` — **Global error boundary**: Create `buildGitHubIssueUrl()` pure function + `ErrorReport` component + `global-error.tsx` + `(app)/error.tsx` + `[bnto]/error.tsx`. Unit tests for URL construction. PostHog `app_error` telemetry on boundary trigger.
- [x] `packages/core` — **Dead code removal**: Verified — `processFile` already removed in Sprint 4H, `hasImplementation()` already removed, `executePipeline` is active (JS↔WASM adapter, not redundant). No action needed.
- [x] `packages/@bnto/nodes` — **Align stale schemas**: Verified — schemas are auto-generated from Rust engine catalog via `task nodes:generate`. Hand-written wrappers only add `hidden: true` on operation field. No Go-era operations remain.

### Wave 2 (parallel — Go archive + Rust cleanup)

- [x] `archive/` — **Delete Go engine**: Deleted `archive/engine-go/`. Removed `go.work`. Updated `.gitignore`, `Taskfile.yml`, `bnto.code-workspace`, `README.md`, `CLAUDE.md`.
- [x] `archive/` — **Delete Go API**: Deleted `archive/api-go/`. Deleted `Dockerfile.api`. Updated `.dockerignore`. Updated test fixture references in `transit-helpers.ts`.
- [x] `infra` — **Clean up Taskfile + CI**: Removed all Go tasks from Taskfile. Updated `build:all`/`test:all` to Rust + TS only. Removed `dev:all`. No Go-related CI checks found.
- [x] `engine` — **Split `executor.rs`**: Split `executor/mod.rs` (523 lines) into three focused modules: `mod.rs` (299 lines — public API, dispatch, shared types), `primitive.rs` (184 lines — leaf node execution), `container.rs` (230 lines — loop/group/parallel containers + sub-pipeline). Comment density pass for consistency. All 437 tests pass.

### Wave 3 (parallel — performance + stale references)

- [x] `apps/web` — **Server Component audit**: Removed `"use client"` from 15 pure presentational UI components. Pushed editor page client boundary down. Extracted recipe page static header. Lazy-loaded config components. Moved currentUser fetch to self-fetching SessionMarker leaf. No `ssr: false` anti-patterns found.
- [x] `apps/web` — **Lighthouse audit**: Run `/lighthouse-audit --local` across all public pages. Fix failing a11y, SEO, best-practices assertions.
- [x] Cross-cutting — **Go reference sweep**: Grep for "Go engine", "Go API", "archive/engine-go" in non-archive code. Remove stale references. Update CLAUDE.md, architecture.md, ROADMAP.md.
- [x] `.claude/` — **Docs cleanup**: Update "What's Built" in PLAN.md, remove Go engine from CLAUDE.md Repository Structure, update architecture.md data flow diagram.

### Wave 4 (parallel — triage batch)

- [x] `apps/web` — **Simplify My Recipes page**: Remove stat cards and history section. Show saved recipes grid or empty state.
- [x] `packages/ui` — **SelectTrigger press animation**: Add pressable spring effect matching Menu trigger.
- [x] `packages/ui` — **PopupTrigger shared component**: Unify Menu, Select, Combobox trigger styling.
- [x] `packages/editor` — **File menu icons**: Add icons to "Open" and "Export" menu items.
- [x] `packages/editor` — **Raw useStore audit**: Migrate raw `useStore(storeApi, ...)` calls to domain hook factories.
- [x] `apps/web` — **Fix reducedMotion type errors**: Fix `reducedMotion` type errors in E2E spec `test.use()` calls.
- [x] `apps/web` — **Remove redundant default props**: Audit for components passing props matching defaults.
- [x] `apps/web` — **Home page marquee**: Replace static RecipeGrid with scrolling Marquee component.
- [x] `packages/editor` — **File menu transform origin**: Fix popover/menu animation direction.
- [x] `packages/editor` — **I/O node mode labels**: Display current mode on Input/Output compartment nodes.
- [x] `packages/editor` — **Pre-populate extension TagPicker**: Ship Input node file extension TagPicker with static list.
- [x] `apps/web` — **Kbd component + shortcuts dialog**: Create `<Kbd>` primitive for shortcut hints. Add `Cmd+/` keyboard shortcuts dialog.

### Wave 5 (parallel — final quality + triage cleanup)

- [x] `apps/web` — **Replace competitor comparison with bnto-first benchmarks**: Rewrite "How It Works" BragLayout to showcase bnto's own capabilities instead of competitor comparison.
- [x] `apps/web` — **Delete button on My Recipe cards**: Add delete action to saved recipe cards on `/my-recipes`.
- [x] `packages/editor` + `@bnto/core` — **Auto-save recipes**: Replace manual Save with transparent persistence. Download/Export remains manual. Debounced auto-save to localStorage (PRs #204, #205, #212).
- [x] `engine` — **Thin Rust comment density**: Reduce inline comment noise. Update CLAUDE.md Rust standards section.
- [x] Cross-cutting — **Inline handler audit**: Extract inline `onClick` handlers to named functions.
- [x] Cross-cutting — **CSS-first interaction audit**: Identify JS `useState`/ternary className patterns that CSS pseudo-classes or `data-*` attributes could handle.
- [x] Cross-cutting — **Test naming unification**: Audit all test suites for naming consistency.
- [x] `apps/web` — **Standardize E2E selectors on data-testid**: Audit and replace fragile selectors (PRs #203, #208).
- [x] `apps/web` + `packages/editor` — **Local recipe persistence for unauthenticated users**: Open `/my-recipes` to unauthenticated users with localStorage-backed recipe list. Add upsell messaging.

### Wave 6 (parallel — Button simplification + polish)

- [x] `packages/ui` — **Simplify Button behavioral props — CSS-first with data attributes**: Remove `pressed` and `hovered` JS props. Replace with CSS-driven data-state attributes. Remove `sm` and `lg` size variants — keep only `md`.
- [x] `apps/web` — **Theme menu lighting direction control**: RadialSlider, Zustand store with localStorage persistence, `--light-angle` CSS variable driving surface shadow system.

---

## Sprint 7: Explore & Discovery Infrastructure (Tier 2) — COMPLETE

**Goal:** Unify how recipes and nodes are listed across all surfaces, then build a dedicated Explore page. When this sprint is done, adding a recipe to `@bnto/nodes` automatically appears on every surface.

**Persona ownership:**

| Package       | Persona                                 |
| ------------- | --------------------------------------- |
| `@bnto/core`  | `/core-architect`                       |
| `@bnto/nodes` | `/core-architect`                       |
| `apps/web`    | `/frontend-engineer` + `/nextjs-expert` |

### Wave 1 (parallel — audit + cleanup + URL unification)

- [x] `@bnto/nodes` + `apps/web` — **Audit all listing surfaces**: Map every component/hook that lists recipes or nodes. 15 surfaces audited, README.md stale (6/8 recipes).
- [x] `@bnto/core` — **Design unified recipe/node query API**: Recipe IS a Definition. Eliminate wrapper types, delete `RecipeDefinition` duplicate. `core.catalog` client for unified access.
- [x] `apps/web` — **Unify editor URL pattern**: Eliminated `?from={slug}`. "Open in Editor" clones template, navigates by ID (PR #228).
- [x] `apps/web` — **Consolidate Recipe types**: Unified recipe model — `Recipe` layered type, `UserRecipe extends Recipe`, `BntoEntry` derived (PR #226).

### Wave 2 (parallel — unified recipe model: type migration)

- [x] `@bnto/nodes` — **Layered Recipe type**: `Recipe` wraps `Definition` with display metadata. UUID ids. `deriveCategory()` (PR #226).
- [x] `@bnto/core` — **Delete `RecipeDefinition`, simplify persistence**: `UserRecipe extends Recipe`, transforms updated (PR #226).
- [x] `apps/web` — **Refactor `bntoRegistry.ts`**: `BntoEntry` derived from `Recipe`. SEO derived from `recipe.name` (PR #226).
- [x] `@bnto/core` — **Build `core.registry` client**: 6th domain. Zustand store with `populate()`. Client + React hooks (PR #227).

### Wave 3 (parallel — surface migration + Explore page)

- [x] `apps/web` — **Migrate runtime surfaces to `core.registry`**: RecipeMarquee, RecipeCardShowcase consume hooks (PR #229).
- [x] `packages/editor` — **Migrate editor surfaces to `core.registry`**: `useNodePalette` and `RecipePickerGrid` (PR #229).
- [x] `apps/web` — **Build `/explore` page**: Full-page searchable/filterable recipe & node browser. Server component with client leaves (PR #281).
- [x] `apps/web` — **Migrate navbar Explore**: Replace dropdown with link to `/explore`.

### Wave 4 (sequential — verify + auto-generation)

- [x] `apps/web` — **SEO verification**: All surfaces derive from `core.catalog`.
- [x] `apps/web` — **E2E tests**: Explore page renders, search/filter works, recipe cards link.
- [x] Repo root — **Auto-generate README recipe list**: Generated from `@bnto/nodes` RECIPES registry.

---

## Sprint 8: Tier 3 Near-Term Recipes — COMPLETE

**Goal:** Expand the recipe catalog with high-SEO-value recipes running 100% client-side. Tier 3 recipes target high-volume search queries (watermark: 30K/mo, strip-exif: 15K/mo, merge-csv: 12K/mo, csv-to-json: 25K/mo).

**Persona ownership:**

| Package       | Persona              |
| ------------- | -------------------- |
| `engine`      | `/rust-expert`       |
| `@bnto/nodes` | `/core-architect`    |
| `apps/web`    | `/frontend-engineer` |

### Wave 1 (parallel — engine operations)

- [x] `engine` — **`bnto-image`: image-overlay/watermark operation** (PRs #308, #309)
- [x] `engine` — **`bnto-image`: EXIF metadata strip**
- [x] `engine` — **`bnto-csv`: merge operation**
- [x] `engine` — **`bnto-csv`: CSV-to-JSON conversion**

### Wave 2 (parallel — recipes + codegen)

- [x] All recipe fixtures + codegen + golden tests for watermark-images, strip-exif, merge-csv, csv-to-json (PRs #294, #296, #308, #309)

### Wave 3 (parallel — SEO pages + E2E)

- [x] `apps/web` — SEO pages auto-propagate via `getAllRecipes()`. NAV_DESCRIPTIONS added
- [x] `apps/web` — E2E tests: 11 tests total with programmatic assertions
- [x] `apps/web` — Lighthouse audit: All 18 public pages pass thresholds

---

## Sprint 8.5: Simplify Config, Reconnect Editor Lightweight — COMPLETE

**Goal:** Three phases. Disconnect editor (done), make recipe config schema-driven (any recipe gets controls for free), then reconnect the editor as a lightweight open+export tool with no persistence.

### Sprint 8.5a: Disconnect Editor + Slash Dead Code — COMPLETE

Pure deletion + reference cleanup. Deleted editor route, nav buttons, "Open in Editor" links, core persistence code (recipesStore, mergeCloudRecipes, useRecipeSync, fileTransfer), My Recipes components. Preserved: `core.recipes.run()` execution path, `recipeService.ts`, `recipeAdapter.ts`.

### Sprint 8.5b: Favorites + My Recipes — TABLED

Tabled (March 2026). User preferences/engagement features deferred to post-MVP.

### Sprint 8.5c: Schema-Driven Recipe Config — COMPLETE

Replaced ~600 LOC of hardcoded per-recipe config with dynamic schema-driven config. DynamicRecipeConfig component reads recipe definition, walks processing nodes, renders SchemaForm per node. Config state became `Record<nodeId, Record<string, unknown>>`. All old per-recipe config files deleted (PRs #302, #303).

### Sprint 8.5d: Reconnect Editor (Open + Export Only) — COMPLETE

Brought back `/editor` route as lightweight open+export tool. No persistence — sessionStorage only. `core.recipes` re-added as 7th domain. `recipesStore` (Zustand + sessionStorage), `recipeClient`, `UserRecipe` (simplified), `fileTransfer`, hooks. Nav items, "Open in Editor", routes/config/copy all restored. Beta dialog with import/export messaging. 7 E2E spec files (PR #305).

---

## Phase 2: Engine Expansion (CLI-First)

### Sprint 9: Engine Expansion — COMPLETE

**Persona ownership:**

| Package  | Persona        |
| -------- | -------------- |
| `engine` | `/rust-expert` |

#### Wave 1 (parallel — dependency system + ProcessContext)

- [x] `engine/crates/bnto-core` — `requires: Vec<Dependency>` on `NodeMetadata` (PR #315)
- [x] `engine/crates/bnto-core` — `ProcessContext` trait: `NoopContext` for browser, `NativeContext` for CLI (PR #318)
- [x] `engine/crates/bnto-engine` — Dependency checker with clear error + install hints (PR #320)
- [x] `engine/crates/bnto` — `bnto doctor` command (PR #320)

#### Wave 2 (parallel — video node type)

- [x] `engine/crates/bnto-video` — New crate: `video-download` processor wrapping yt-dlp (PRs #321-#329)
- [x] Registered in `bnto-engine`, golden tests, recipe fixture
- [x] Codegen: video category + node type propagated through TypeScript (PR #336)

#### Wave 3 (parallel — CLI polish)

- [x] `bnto list`, `bnto info <recipe>`, enhanced `bnto run` with progress bars
- [x] README updated to pitch CLI usage front and center

---

## Sprint 10: TUI — COMPLETE

**Goal:** `bnto tui` launches an interactive terminal UI — recipe browser, file picker, execution progress, results summary. Same engine, richer interface.

**Strategy doc:** [tui-strategy.md](strategy/tui-strategy.md)
**Architecture:** Elm Architecture (TEA) — pure `update()` functions, 5 screen systems.
**Framework:** `ratatui` + `crossterm`

### Wave 1 (parallel — shell + theme + browser)

- [x] TUI app shell: `ratatui` + `crossterm`, screen router, event loop, theme, `bnto tui` subcommand (~5 tests)
- [x] Recipe browser screen: list recipes grouped by category, search, `j/k` navigation (~10 tests)
- [x] Shared widgets: help bar, search input, status line (~4 tests)

### Wave 2 (parallel — detail + picker)

- [x] Recipe detail screen: schema-to-control mapping (tui-slider, Input, Toggle, Select), param editing (~8 tests)
- [x] File picker screen: multi-select, directory browsing, extension filtering (33 tests)

### Wave 3 (parallel — execution + results)

- [x] Execution screen: per-file and per-node status, elapsed timer, cancel, auto-transition (10 tests)
- [x] Results screen: output file list, compression savings, re-run (9 tests)

### Wave 4 (sequential — integration + docs)

- [x] Detail "confirm and proceed" action
- [x] End-to-end wiring: browser → detail → picker → execution → results → browser
- [x] Screen transition integration tests
- [x] CLI integration tests
- [x] Documentation + README

**Sprint 10 totals: 6 screens, 278 tests, 32 Rust files**

---

## Sprint 11: Engine-Owned Node Schema + TUI Schema-Driven Config — COMPLETE

**Plan doc:** [.claude/plans/inherited-watching-hennessy.md](./plans/inherited-watching-hennessy.md)

**Goal:** Make the Rust engine the single source of truth for node config field schemas AND `.bnto.json` document types. `@bnto/nodes` collapses to a barrel over engine-generated code. Both web and TUI consume the same `control` field. Deletes ~930 LOC of hand-written TypeScript.

### Wave 1 — Engine owns schema (sequential)

- [x] Extend `ParameterDef` + `ParameterType` shape (PR 1): add `group`, `suffix`, `control`, `accept`, `presets`, `inverted`, `Array`/`Record` variants
- [x] Add `ParameterDef` metadata for 7 IO/container/data node types (PR 2)
- [x] Add document-shape Rust types (PR 3): `Definition`, `Edge`, `Port`, `Metadata`, `Recipe`, `AcceptSpec` with `ts-rs` derives
- [x] Codegen overhaul + delete ~930 LOC (PR 3): `inferFieldType.ts` absorbed, all 20 node types generated, document-shape types via `ts-rs`

### Wave 2 — Consumers (parallel)

- [x] Web verification (PR 4): editor config panel, Motorway form showcase, SchemaForm render identically
- [x] Enrich `ParamEntry` with full metadata (PR 5): `constraints`, `description`, `suffix`
- [x] TUI controls module (PR 5): `boolean.rs`, `enum_select.rs`, `number.rs`. ~50 new tests
- [x] TUI visibility + custom recipes + scrolling (PR 6)

### Wave 3 — Ship (sequential)

- [x] End-to-end integration tests (PR 7): 12 integration tests loading real recipes
- [x] Update strategy docs + README + PLAN.md

---

## Sprint 12B: Recipe-Level Dependencies + Shell Command — COMPLETE

**Goal:** Close the dependency gap for connector-as-recipe architecture. Recipe JSON gains a `requires` field. `shell-command` processor. `download-video` migration from `bnto-video` crate to `shell-command`.

**Strategy doc:** [recipe-deps-strategy.md](strategy/recipe-deps-strategy.md)

### Wave 1 — Recipe-level requires (sequential)

- [x] Add `Deserialize` to `Dependency` + `requires` to `PipelineDefinition` (~6 tests)
- [x] Merge recipe-level deps in `collect_pipeline_dependencies()` (~6 tests)

### Wave 2 — Shell command processor (sequential)

- [x] `shell-command` processor in new `bnto-shell` crate. Security boundary (shell denylist, path validation, env var sanitization). 36 tests (21 security + 15 functional)

### Wave 3 — Download-video migration (sequential)

- [x] Convert `download-video` recipe to `shell-command` + recipe-level `requires: [yt-dlp, ffmpeg]`. Deleted `bnto-video` crate
- [x] Update CLI integration + codegen. `CategoryName` type, `requires` round-trip, `Dependency` type export

**Sprint 12B totals: ~4 PRs, ~30 tests, ~500-800 LOC**

### Sprint 12B Follow-up: Recipe Fields (node-level) — COMPLETE

**Goal:** User-facing field declarations on nodes — typed controls that map to `{{fields.*}}` template syntax in parameters. Node-level fields are the building blocks; recipe-level is deferred.

**Strategy doc:** [recipe-fields.md](strategy/recipe-fields.md)

- [x] `FieldDef` enum (string/number/boolean/enum variants) in `field_def.rs`
- [x] `fields` on `PipelineNode` (node-level) — moved from recipe root to individual nodes
- [x] `{{fields.*}}` double-brace template resolution in `resolve.rs`
- [x] Shell-command placeholders migrated to `{{output_dir}}`, `{{url}}`, `{{input}}`
- [x] TUI discovery: `detail_loader.rs` walks nodes for field-based params
- [x] `download-video.bnto.json` uses node-level fields (format, videoCodec, audioCodec)
- [x] TypeScript codegen updated

---

## Sprint 12A: Data Persistence + Home + Library — COMPLETE

**Goal:** Establish the storage foundation and core TUI user journey. XDG-compliant data persistence, Home screen, My Library, `bnto` default to TUI.

**Strategy docs:** [tui-data-persistence.md](strategy/tui-data-persistence.md), [tui-user-journey.md](strategy/tui-user-journey.md)

### Wave 1 — Storage foundation (sequential)

- [x] `BntoPaths` struct + resolution: XDG-compliant, `BNTO_HOME`/`BNTO_CONFIG_DIR` overrides (~10 tests)
- [x] Atomic writes + TOML config: `tempfile::NamedTempFile` + `persist()`, schema versioning (~10 tests)

### Wave 2 — Migration + error handling (sequential)

- [x] Config migration from old layout: JSON→TOML, telemetry merge (~8 tests)
- [x] Surface save errors + wire `BntoPaths` (~7 tests)

### Wave 3 — Home screen + navigation (parallel)

- [x] Home screen (main menu): My Library, Recipes, New Recipe, Settings (~5 tests)
- [x] App router update: `Screen::Home` and `Screen::Library`, Home as default (~8 tests)

### Wave 4 — My Library + CLI default (parallel)

- [x] My Library screen: load `.bnto.json` files, search/filter, rename/delete (34 unit + 10 app tests)
- [x] "Add to Library" + CLI default: copy recipe JSON to `recipes_dir`, collision detection, `bnto` launches TUI

**Sprint 12A totals: ~8 PRs, ~65 tests, ~1500-2000 LOC**

---

## Sprint 11.5: `bnto-form` — TUI Form Widget Crate — COMPLETE

**Goal:** Standalone, open-source ratatui form crate replacing hand-built detail screen controls. TEA-native, pure-function architecture, zero bnto dependency.

**Strategy doc:** [bnto-form-strategy.md](strategy/bnto-form-strategy.md)

### Wave 1 — Core types + TextInput (sequential)

- [x] Crate scaffold + core types: `Field`, `FieldKind`, `FieldState`, `FormModel`, `FormMessage` (~10 tests)
- [x] TextInput control: grapheme-aware cursor, placeholder, char limit (~15 tests)

### Wave 2 — Select + Confirm + Number (parallel)

- [x] Select field: compact cycling, expanded vertical list with filter (~20 tests)
- [x] Confirm field: side-by-side Yes/No, shortcuts (~8 tests)
- [x] Number field + vendored slider rendering (~12 tests)

### Wave 3 — Validation + Theme + Form API (sequential)

- [x] Validation system: `ValidatorFn`, built-in validators, inline errors (~15 tests)
- [x] Theme + form-level API: `FormTheme`, `render_form()`, scroll, reset (~15 tests)

### Wave 4 — bnto integration (sequential)

- [x] Replace detail screen controls: bridge `ParamEntry` → `bnto_form::Field`, wire into detail screen (~10 tests)

**Sprint 11.5 totals: ~6 PRs, ~105 tests, ~2000-2500 LOC**

---

## Sprint 12: TUI List Editor — COMPLETE

**Goal:** Transform the TUI from a read-only runner into a recipe editor. List editor handles 90% of editing needs.

**Strategy doc:** [recipe-editors.md](strategy/recipe-editors.md)
**Implementation plan:** [editor-implementation-plan.md](strategy/editor-implementation-plan.md)

### Wave 1 — Editor State Model + Recipe I/O (sequential)

- [x] Editor state model: `EditorModel`, `EditorNode`, `EditorSnapshot`, `EditorSource`. Undo/redo (~15 tests)
- [x] Recipe file I/O: load/save `.bnto.json`, roundtrip fidelity (~8 tests)

### Wave 2 — List Editor Screen (parallel)

- [x] Editor screen shell + navigation: `EditorMessage` + `update()` + `view()`, expand/collapse, dirty guard (~10 tests)
- [x] Node add/remove: picker overlay, delete with confirmation, undo (~12 tests)
- [x] Node reorder: `Shift+j`/`Shift+k`, cursor follows (~6 tests)

### Wave 3 — Inline Config + Schema Controls (sequential)

- [x] Inline parameter editing: Sprint 11 type-aware controls, `visible_when` filtering (~10 tests)

### Wave 4 — Save + Entry Points (parallel)

- [x] Save workflow: save to disk, confirm overwrite, `Ctrl+s`, dirty flag clear (~5 tests)
- [x] Entry points + app integration: `--new`, file arg, browser `e` key, detail `e` key (~5 tests)

**Sprint 12 totals: ~8 PRs, ~75 tests, ~1500-2000 LOC**

---

## Sprint 13: TUI Wizard — COMPLETE

**Goal:** Guided recipe creation for first-time users. "What do you want to do?" → category → operation → config → done.

**Strategy doc:** [recipe-editors.md](strategy/recipe-editors.md) (§ Wizard Editor)

### Wave 1 — Wizard Flow (sequential)

- [x] Wizard state model: step progression (Category → Operation → Config → Complete), engine metadata lists (~10 tests)
- [x] Wizard screen + rendering: step-by-step prompts, category grid, operation list, config controls (~8 tests)

### Wave 2 — Wizard-to-Editor Handoff (sequential)

- [x] Auto-name + handoff to List editor: recipe name generation, model population, screen transition (~5 tests)

**Sprint 13 totals: ~3 PRs, ~25 tests**

---

## Homepage & Site Polish — COMPLETE (April 2026)

Homepage redesigned from recipe gallery to developer-facing landing page. Pieces 1-9 delivered: copy polish, nav restructure, hero animations, explore page animations, "What's in the box" redesign with mascots, recipe showcase marquee, "Build Your Own" code editor section, "Open Kitchen" section, footer refresh. Kawaii sushi mascots integrated (5 purchased from Catalyst Labs).

**Strategy docs:** [homepage-sprint-plan.md](strategy/homepage-sprint-plan.md), [brand-messaging-audit.md](strategy/brand-messaging-audit.md), [landing-page-inspiration.md](strategy/landing-page-inspiration.md)

- [x] Piece 1: Copy polish (hero, section dividers, pitch points, footer tagline)
- [x] Piece 2: Nav restructure (rename, ExploreDropdown, CTA + GitHub star)
- [x] Piece 3: Hero section animations (SlideUp, FadeIn, ScaleIn, Stagger, InView)
- [x] Piece 4: Explore page spring animations
- [x] Pieces 5-9: Section redesigns, recipe showcase, build your own, open kitchen, footer refresh

---

## Editor: Smart Iteration — DELIVERED (March 2026)

Added `settings.iteration: "auto" | "explicit"` to the Definition. When `"auto"`, the engine wraps contiguous per-file processor sequences in implicit per-file loops. Both modes produce byte-identical output (proven via 20 golden tests). Recipe settings panel in ConfigPanel.

**What shipped:**

- Rust: `PipelineSettings`, `IterationMode`, `InputCardinality` types + `run_auto_iteration()` executor + JSON Schema
- TypeScript: types propagated through `@bnto/nodes` → `@bnto/registry` → `@bnto/core`
- 10 flat recipe fixtures + 10 golden equivalence tests
- Recipe Settings Panel (iteration mode toggle) in ConfigPanel
- `rfNodesToDefinition` preserves `settings` on export
