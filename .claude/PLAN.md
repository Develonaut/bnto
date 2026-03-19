# Bnto — Build Plan

**Last Updated:** March 18, 2026 (groomed — Sprint 6 complete, Sprint 7 active, backlog cleaned: 34 archived items removed, recipe tags bug fixed)
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

- **M1 delivered (Feb 2026):** All 6 Tier 1 bntos + 2 Tier 1B multi-node compositions run 100% client-side via Rust→WASM
- **M2 delivered (March 2026):** Editor v1 shipped — schema-driven config controls, save/My Recipes, keyboard shortcuts, accessibility audit. Accounts, execution history, PostHog telemetry all live.
- **Sprint 6 (Quality & Cleanup) complete.** Error boundaries, dead code removal, Server Component audit, auto-save, Button simplification, triage batch — all done.
- **Tabled (deep backlog):** Code Editor (CM6), Edit/Run Mode, Sprint 5B W2-4 (LayerPanel polish, processing node accents).
- **Cloud infrastructure:** R2 file transit — ready for M4 (server technology TBD)
- **WASM engine:** 5 Rust crates, single cdylib, 1.6MB raw / 606KB gzipped
- **Auth:** `@convex-dev/auth`. Password auth, integration tests complete, E2E auth lifecycle verified (13/13 tests)
- **Infra:** GitHub Actions CI (Rust + TypeScript + CI Gate), tag-triggered release pipeline (CI gate → Vercel preview → E2E → Lighthouse → auto-deploy Vercel + Convex to production on stable tags → GitHub Release), PostHog telemetry wired
- **Packages:** `@bnto/core`, `@bnto/auth`, `@bnto/backend`, `@bnto/nodes`, `@bnto/ui`, `@bnto/editor`

---

## What's Built (don't redo)

- [x] Monorepo: Turborepo + pnpm + Taskfile.dev
- [x] @bnto/core: Layered singleton (clients → services → adapters), React Query + Convex adapter, 38+ hooks
- [x] @bnto/auth: `@convex-dev/auth` integration, password auth
- [x] @bnto/backend: Convex schema (users, workflows, executions, executionLogs), auth, crons, analytics fields
- [x] @bnto/nodes: Engine-agnostic node definitions, Zod schemas, recipes, validation (10 node types)
- [x] @bnto/ui: Extracted Motorway design system — primitives, layout, typography, feedback, surface, interaction, overlay, animation components
- [x] @bnto/editor: Extracted editor package — EditorCanvas, EditorToolbar, LayerPanel, ConfigPanel, CompartmentNode, NodePaletteMenu, adapters, hooks, store, actions
- [x] Web app: Auth flow, SEO infrastructure, middleware, landing pages (real content), privacy policy
- [x] Playwright E2E: 27+ screenshots, user journey tests, execution flow tests, site navigation (desktop + mobile)
- [x] Rust WASM engine: 5 crates, single cdylib, Web Worker wrapper, progress reporting, 44+ unit tests
- [x] Browser execution: All 6 Tier 1 bntos client-side via WASM, ZIP download, auto-download
- [x] Cloud execution infrastructure: R2 file transit, presigned URLs — ready for M4
- [x] Recipe page overhaul (Sprint 2D): RecipeShell, PhaseIndicator, FileCard, RecipeConfigSection, useRecipeFlow
- [x] Motorway design system: Grid, LinearProgress, ToolbarProgress, RadioGroup, NavButton, RadialSlider, surface system, Pressable + Surface composition
- [x] Per-instance browser execution stores: Factory pattern, `core.wasm.createExecution()`, no state leaks
- [x] Sprint 3 pre-work: Anonymous→password userId preservation, FIXME cleanup, Knip audit, naming audit, codebase standards review, schema analytics fields
- [x] GitHub Actions CI: Rust (fmt + clippy + unit + WASM) + TypeScript (build + lint + test) + CI Gate
- [x] convexQuery skip guards: All adapter functions use `"skip"` for falsy IDs (PR #23)
- [x] Format versioning + Zod node validation (Sprint 4G): `.bnto.json` format version constant, schema versioning, Zod parameter schemas for all 12 node types, schema-driven config panel with registry-based controls
- [x] Editor production route (Sprint 5 W1-W2): `/editor` route, `?from={slug}` recipe loading, compartment node redesign (icons + category colors), "Open in Editor" nav integration
- [x] Pipeline executor extraction (Sprint 4H): Runtime-agnostic `executePipeline()` in `@bnto/core`, `NodeRunner` contract, `processFiles()` removed from browser adapter, comprehensive TDD test suite
- [x] Editor API layer (Sprint 5D): `createEditor()` factory, 5 domain clients (nodes, definition, execution, history, panels), 5 services, React binding layer (`EditorProvider`, `useEditor`, domain hooks), full component migration, deprecated hooks deleted
- [x] Multi-node recipes (Tier 1B): optimize-images-for-web, generate-thumbnails — first multi-node predefined recipes with 3-operation pipelines inside forEach loops
- [x] Slider presets + select labels: Quality→compression rename, slider preset system, select option labels
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

---

## Revenue & Monetization Context

Pricing, revenue projections, and "ready to charge" criteria live in private business docs (`BNTO_PRIVATE_DOCS_PATH` in `.env.local`) — see `pricing-strategy.md`, `seo-monetization.md`, and `feature-funnel.md`.

**Monetization model (updated Feb 2026):** Browser execution is free unlimited. Pro sells real value — persistence, collaboration, premium compute. See ROADMAP.md for the full model.

| Sprint       | What Ships                                   | Revenue Implication                                                                                                              |
| ------------ | -------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| Sprint 2B    | Browser execution (M1 MVP)                   | **All Tier 1 bntos run client-side.** Zero backend cost. Files never leave user's machine.                                       |
| Sprint 2C    | Launch readiness (content + domain)          | **bnto.io live and indexable.** Real content on every page. SEO crawling begins. First real users possible.                      |
| Sprint 2D    | Recipe page UX overhaul                      | **COMPLETE.** Progressive phase-driven flow. Motorway design language on every tool page.                                        |
| Sprint H     | Housekeeping                                 | **COMPLETE.** FileUpload rewrite, Rust test audit, EXIF coverage, Pressable, CI, ESLint.                                         |
| Sprint 3     | Platform features (accounts, history)        | Accounts exist. Conversion hooks scaffolded (Save, History). Usage analytics instrumented.                                       |
| Sprint 4     | Recipe editor (headless + visual)            | Power users self-identify. Create/customize recipes = highest-intent Pro signal. Free editor fosters community recipe ecosystem. |
| Sprint 4D-4G | Package extraction + versioning + validation | Clean architecture. Zod schemas. Packages ready for desktop (M3).                                                                |
| Sprint 5     | Editor v1 (config controls, save, polish)    | **M2 completion.** Editor gives users a reason to create accounts. Save custom recipes = highest-intent Pro signal.              |
| Sprint 8-9   | Desktop app                                  | Top-of-funnel. Word of mouth begins. Free forever — trust signal.                                                                |
| Sprint 10    | Stripe + Pro tier                            | **First revenue possible.** Pro: $8/month for persistence, collaboration, server-side AI, priority processing.                   |

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

Format versioning activated across the stack. Zod schemas replaced hand-rolled `ParameterSchema` DSL for all 12 node types. Schema-driven config panel with `CONTROL_REGISTRY` map dispatching Zod-inferred `FieldControl` types to `@bnto/ui` controls. 3 waves: format version constants + schema version field, Zod migration + validation function, schema-driven `SchemaForm` + `SchemaField` components (PRs #114-#116).

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

**M2 is delivered. Sprint 6 (Quality & Cleanup) is complete.** Direction decided: **Tier 2 (Explore & Discovery Infrastructure)** → then **Tier 3 (Near-Term Recipes)**. Unify how recipes/nodes are listed before expanding the recipe catalog. See `bntos.md` for the full tier breakdown.

**Next up:** Sprint 7 (Explore & Discovery, Tier 2) → Sprint 8 (Near-Term Recipes, Tier 3).

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

## Active Sprint

### Sprint 7: Explore & Discovery Infrastructure (Tier 2)

**Goal:** Unify how recipes and nodes are listed across all surfaces, then build a dedicated Explore page. When this sprint is done, adding a recipe to `@bnto/nodes` automatically appears on every surface (home, Explore page, editor palette, sitemap, README). This is a prerequisite for Tier 3 recipe expansion.

**Problem:** Currently 5+ surfaces list recipes/nodes using different data sources and transforms:

- Home: `RecipeGrid` → `BNTO_REGISTRY` (8 recipes, web-specific SEO wrapper)
- Navbar: `RecipesMenu` → `navData.ts` `buildRecipeCategories()` (6 Tier 1 recipes, categorized)
- Editor palette: `useNodePalette` → `NODE_TYPE_INFO` + `CATEGORIES` + `PROCESSORS` (12 node types)
- Editor open dialog: `RecipePickerGrid` → `RECIPES` from `@bnto/nodes` (all predefined)
- Tool pages + sitemap: `bntoRegistry.ts` → `generateStaticParams`
- README: Hardcoded recipe list — will drift as recipes grow
- Editor URL: `?from={slug}` (predefined) vs `?recipe={id}` (saved) — two params for the same concept

**Persona ownership:**

| Package       | Persona                                 |
| ------------- | --------------------------------------- |
| `@bnto/core`  | `/core-architect`                       |
| `@bnto/nodes` | `/core-architect`                       |
| `apps/web`    | `/frontend-engineer` + `/nextjs-expert` |

#### Wave 1 (parallel — audit + cleanup + URL unification)

- [x] `@bnto/nodes` + `apps/web` — **Audit all listing surfaces**: Map every component/hook that lists recipes or nodes. Document data source, transform, filtering, and output shape for each. Identify divergences (missing recipes, different categories, stale hardcoded lists). Produce a comparison table. _(Results: 15 surfaces audited, README.md stale (6/8 recipes), all dynamic surfaces trace to `@bnto/nodes` RECIPES. See `strategy/unified-recipe-model.md`)_
- [x] `@bnto/core` — **Design unified recipe/node query API**: Propose how `@bnto/core` exposes a single query that all surfaces consume. Consider: should this be a core client (`core.catalog` or `core.explore`), or a query-only API? What filtering/grouping capabilities does it need? Write a brief design doc or add to `core-api.md`. _(Decision: a Recipe IS a Definition. Eliminate both `Recipe` wrapper types, delete `RecipeDefinition` duplicate. Persist `Definition` objects directly. Publishing metadata in web registry, persistence in thin store envelope. `core.catalog` client for unified surface access. Full design in `strategy/unified-recipe-model.md`)_
- [ ] `apps/web` — **Unify editor URL slug pattern**: Replace dual `?from={slug}` / `?recipe={id}` params with a single `?recipe={identifier}` param. The editor page resolves the identifier to either a predefined recipe (by slug) or a saved recipe (by ID). Centralise URL construction in `lib/routes.ts` (e.g. `editorUrl(id)`). Update all consumers: RecipeGrid, RecipeCardShowcase, Open dialog, nav links.
- [ ] `apps/web` — **Consolidate Recipe types**: Superseded by unified recipe model — see `strategy/unified-recipe-model.md`. Implementation moves to Wave 2 as part of the type migration.

#### Wave 2 (parallel — unified recipe model: type migration)

Design doc: `strategy/unified-recipe-model.md`

- [ ] `@bnto/nodes` — **Refactor predefined recipes to `Definition`**: Delete `Recipe`, `AcceptSpec`, `SEOSpec` types from `recipe.ts`. Update all 8 recipe files to export `Definition` directly (remove slug/seo/accept/features/category wrapper). Update `RECIPES` to `readonly Definition[]`. Rename `getRecipeBySlug()` → `getDefinitionBySlug()`. Add `deriveAcceptSpec(definition)` and `deriveCategory(definition)` pure functions. Update all downstream imports.
- [ ] `@bnto/core` — **Delete `RecipeDefinition`, simplify persistence**: Delete the `RecipeDefinition`/`Position`/`Metadata`/`Port`/`Edge`/`FieldsConfig` duplicate types from `types/recipe.ts`. Import `Definition` from `@bnto/nodes`. Replace `Recipe` with `SavedRecipe` (thin persistence envelope: `{ definition: Definition; savedAt; syncedAt; cloudId? }`). Update `recipesStore`, `recipeClient`, `recipeService`, transforms, hooks. Keep `RecipeListItem` as a projection derived from `Definition`.
- [ ] `apps/web` — **Replace `BntoEntry` with `PublishedRecipe`**: Refactor `bntoRegistry.ts` — new `PublishedRecipe` type pairs a `Definition` with publishing metadata (slug, seo, features). Derive `accept` from input node via `deriveAcceptSpec()`. Derive `category` via `deriveCategory()`. Update all consumers (RecipeMarquee, RecipeGrid, tool pages, sitemap, navData, BntoJsonLd). Delete dead `RecipeGrid` component in `components/blocks/` if confirmed unused.
- [ ] `@bnto/core` — **Build `core.catalog` client**: New domain client on the `core` singleton. Read-only access to predefined Definitions and node type info. Methods: `getRecipes()`, `getRecipeBySlug()`, `getNodeTypes()`, `getCategories()`, `getProcessors()`. Filtering helpers: `getRecipesByCategory()`, `getBrowserNodeTypes()`. React hooks via `useMemo` (static data). All surfaces import from `core.catalog` instead of `@bnto/nodes` directly.

#### Wave 3 (parallel — surface migration + Explore page)

- [ ] `apps/web` — **Migrate all surfaces to `core.catalog`**: Update home RecipeMarquee, navData.ts, RecipeCardShowcase, tool pages, sitemap, llms.txt, llms-full.txt to consume `core.catalog` instead of direct `@bnto/nodes`/`BNTO_REGISTRY` imports.
- [ ] `packages/editor` — **Migrate editor surfaces to `core.catalog`**: Update `useNodePalette` and `RecipePickerGrid` (open dialog) to consume `core.catalog` instead of direct `@bnto/nodes` imports.
- [ ] `apps/web` — **Build `/explore` page**: Full-page searchable/filterable recipe & node browser. Categories, search, metadata cards. Server component page with client interactive leaves. Uses `core.catalog`.
- [ ] `apps/web` — **Migrate navbar Explore**: Replace dropdown with a link to `/explore`. Keep a compact "quick access" subset if desired, but primary action is navigating to the Explore page.

#### Wave 4 (sequential — verify + auto-generation)

- [ ] `apps/web` — **SEO verification**: Ensure `generateStaticParams`, `generateMetadata`, sitemap, and `llms.txt` all derive from `core.catalog`. Adding a Definition to `@bnto/nodes` = it appears everywhere.
- [ ] `apps/web` — **E2E tests**: Verify Explore page renders, search/filter works, recipe cards link to tool pages. Verify editor palette and open dialog still show correct items. Page-level screenshots for `/explore`.
- [ ] Repo root — **Auto-generate README recipe list**: The predefined recipe table in `README.md` should be generated from `@bnto/nodes` RECIPES registry (like `llms.txt`). Add a script or codegen step so the README stays current as recipes grow.

---

## Tabled Sprints

### Edit Mode ↔ Run Mode — TABLED

**Tabled (March 2026).** Mini Motorways-inspired edit/run mode switch. Same canvas for editing and running — pause to edit, unpause to watch traffic flow. Requires Sprint 5 execution integration (done). Deferred until v1 editor ships. See `.claude/decisions/editor-ux-direction.md` for the full design.

### Sprint 5B Waves 2-4: LayerPanel Polish + Processing Node Accents — TABLED

**Tabled (March 2026).** Category color pips on processing nodes, selected state ring, LayerPanel I/O distinction + selected highlights. Wave 1 (I/O visual hierarchy) shipped. Remaining waves need more design iteration. Existing Pressable pressed state handles selection well enough for v1.

---

## Phase 2: Desktop App (Local Execution)

**Goal:** Free desktop app. Same React frontend, local engine execution. Free forever, unlimited runs. No account needed. Trust signal and top-of-funnel growth driver.

**Desktop tech: Tauri (Rust-native).** M1 Rust evaluation passed — one codebase for browser WASM + desktop native + CLI.

**Sprint numbering:** Desktop Bootstrap = Sprint 8, Local Execution = Sprint 9.

### Sprint 8: Desktop Bootstrap

**Persona ownership:**
| Package | Persona |
| -------------- | -------------------- |
| `apps/desktop` | `/frontend-engineer` |
| `@bnto/core` | `/core-architect` |
| `engine` | `/rust-expert` |

#### Wave 1 (parallel — setup)

- [ ] `apps/desktop` — `/frontend-engineer` — Bootstrap Tauri desktop project
- [ ] `@bnto/core` — `/core-architect` — Implement desktop adapter (Tauri IPC bindings)
- [ ] `engine` — `/rust-expert` — Expose engine functions for desktop bindings (RunWorkflow, ValidateWorkflow, etc.)

#### Wave 2 (parallel — integration)

- [ ] `apps/desktop` — `/frontend-engineer` — Wire up native ↔ React bindings
- [ ] `@bnto/core` — `/core-architect` — Runtime detection routes to desktop adapter in native webview
- [ ] `apps/desktop` — `/frontend-engineer` — Local file browser for selecting .bnto.json files

#### Wave 3 (sequential — verify)

- [ ] `apps/desktop` — `/frontend-engineer` — Verify workflow list, edit, and save work via native bindings
- [ ] `apps/desktop` — `/frontend-engineer` — Verify runtime detection correctly identifies desktop environment

---

### Sprint 9: Local Execution

**Persona ownership:** Same as Sprint 8 — `/frontend-engineer` (desktop UI), `/core-architect` (adapter), `/rust-expert` (engine).

#### Wave 1 (parallel — execution)

- [ ] `apps/desktop` — `/frontend-engineer` — Execute workflows via Tauri bindings (all node types)
- [ ] `@bnto/core` — `/core-architect` — Execution progress streaming via Tauri adapter
- [ ] `apps/web` — `/frontend-engineer` — Execution progress component (reusable — node status, duration, logs)

#### Wave 2 (parallel — features)

- [ ] `apps/desktop` — `/frontend-engineer` — Execution results view (output data, logs, duration)
- [ ] `apps/desktop` — `/rust-expert` — shell-command node support (full local execution, no restrictions)
- [ ] `apps/desktop` — `/frontend-engineer` — Error handling and cancellation support

#### Wave 3 (sequential — build + distribute)

- [ ] `apps/desktop` — `/frontend-engineer` — Integration tests for local execution
- [ ] `apps/desktop` — `/frontend-engineer` — macOS build (.app bundle, code signing)
- [ ] `apps/desktop` — `/frontend-engineer` — Windows build (.exe)
- [ ] `apps/desktop` — `/frontend-engineer` — Linux build (AppImage)

---

## Phase 3: Monetization + Polish

**Goal:** Wire up payments, enforce quotas, make the product feel complete.

**"Ready to charge" gate:** Before Sprint 10, confirm: real users running browser bntos, conversion hooks built and tested (Save, History, Premium), people return voluntarily, at least one server-side bnto (AI or shell) ready for Pro tier.

### Sprint 10: Stripe + Pro Tier (M5)

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

### Editor: Smart I/O — Implicit vs Explicit Looping

**Status:** Needs design decision (review with full project context)

When a recipe has multi-file input and a processing node (e.g., Image compress), should the editor:

- **Option A (Smart/implicit):** Automatically iterate over inputs — user adds `Input → Image (compress) → Output`, engine handles the loop. Simple, fewer nodes, covers 90% of cases.
- **Option B (Explicit):** User builds iteration manually — `Input → Loop (forEach) → Image (compress inside loop) → Output`. More flexible, more transparent.

**Proposed direction:** Smart by default (Option A) with an advanced toggle to switch to explicit looping for power users. This affects engine processing, definition schema, and editor UX. Needs a deep review session with full project context ([bntos.md](strategy/bntos.md), `mvp-roadmap.md` in private business docs) before implementation.

**Touches:** `@bnto/nodes` (definition schema), `engine/` (execution model), `@bnto/editor` (node placement + wiring), recipe definitions (compress-images etc.), `io-nodes.md` strategy doc.

---

## Backlog

### UX: Unified Popup/FloatingSurface Primitive

**Priority: Medium.** Dialog.Content, Menu.Content, and AccountGate all repeat the same floating surface pattern: `Card elevation="lg"` + `Animate.ScaleIn from={0.6} easing="spring-bouncier"` + pointer-events/z-index management. Extract a shared composition primitive so consumers compose it instead of duplicating the Card/animation/z-index logic.

- [ ] `apps/web` — Frontend engineer investigation: audit Dialog, Menu, AccountGate for shared patterns (animation, elevation, overlay, dismiss)
- [ ] `apps/web` — Design the primitive API — how does it compose with Radix primitives that need `asChild`? Should it handle overlays or just the floating card?
- [ ] `apps/web` — Implement `Popup` (or `FloatingSurface`) primitive in `components/ui/`
- [ ] `apps/web` — Migrate Dialog.Content, Menu.Content, and AccountGate to use the shared primitive

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

**Priority: Medium.** Bring Go engine operations that have no Rust equivalent yet. Reference: [go-engine-migration.md](strategy/go-engine-migration.md).

**Tier 3 recipe blockers:**

- [ ] `engine` — **`bnto-image`: composite operation** — overlay/watermark. Needed for `/watermark-images` (Tier 3, 30K+ monthly searches). See Go `image.go` composite logic
- [ ] `engine` — **`bnto-image`: EXIF metadata strip** — needed for `/strip-exif` (Tier 3, 15K+ monthly searches). Go used `imaging` library strip
- [ ] `engine` — **`bnto-csv`: merge operation** — concat + deduplicate multiple CSVs. Needed for `/merge-csv` (Tier 3, 12K+ monthly searches)
- [ ] `engine` — **`bnto-csv`: CSV-to-JSON conversion** — needed for `/csv-to-json` (Tier 3, 25K+ monthly searches). May be a `transform` concern

**Orchestration (multi-step recipe support):**

- [ ] `@bnto/core` or `engine` — **Multi-step recipe orchestration**: Design how the browser adapter handles recipes with multiple processing nodes (group/loop pattern from Go). Currently the Web Worker processes one file through one node type. Multi-step requires either JS-side orchestration or WASM-side pipeline support. See `go-engine-migration.md` § Orchestration Nodes
- [ ] `engine` — **Expression evaluation in browser**: Choose a JS expression evaluator to replace `expr-lang/expr` for `transform` node and `loop` while/break conditions. Candidates: `expr-eval`, `filtrex`, custom safe evaluator

**Excel support:**

- [ ] `engine` — **`bnto-csv`: Excel (.xlsx) read/write** — Go used `excelize/v2`. Rust options: `calamine` (read) + `rust_xlsxwriter` (write). Lower priority than CSV operations

### Engine: `pdf` Browser Node — Future (Tier 3)

**Priority: Medium.** PDF to Images Bnto (Tier 3, 50K+ monthly searches). Browser-side via pdf.js + Canvas (JS), not Go engine. Rewrite of the Go `pdfcpu` approach for client-side execution.

- [ ] `engine` or `apps/web` — Implement browser-side PDF → image conversion (pdf.js + Canvas)
- [ ] `engine` — Unit tests for PDF → image conversion
- [ ] `@bnto/nodes` — Add `pdf` node type definition, recipe fixture `pdf-to-images.bnto.json`

### Infra: Clean Up Convex Dev Environment (Better Auth Remnants)

Convex dev (`zealous-canary-422`) has stale Better Auth records and test artifacts. Write a one-off cleanup mutation.

- [ ] `@bnto/backend` — Audit tables, write cleanup mutation (orphaned auth records, test users, stale executions)
- [ ] `@bnto/backend` — Run against dev, verify table health
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

- [ ] `apps/web` — Move metadata validation to unit tests (`bntoRegistry.test.ts`)
- [ ] `apps/web` — Slim E2E to redirects + 404 + noindex only

### Testing: Sprint 3 Deferred E2E Tests

**Deferred from Sprint 3 Wave 3 (March 2026).** Platform features are built and working. Test coverage deferred until editor MVP ships.

- [ ] `apps/web` — Playwright E2E: AuthGate conversion flow
- [ ] `apps/web` — Playwright E2E: browser-local execution history
- [ ] `@bnto/backend` — Unit tests for execution analytics queries

### Testing: Concurrent Quota Race Condition — M4/M5 (server-side quotas)

**Milestone: M4/M5 (Sprint 9+).** Quota enforcement only applies to server-side bntos. Browser bntos are free unlimited. This race condition matters when server-side execution has limits.

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

**M2 (Sprint 3) for hook UX, M5 (Sprint 9) for Stripe.** Value-driven conversion hooks (Save, History, Premium Bntos, Team) — no "limit reached" messaging for browser bntos.

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
| `.claude/strategy/go-engine-migration.md`                        | Go engine node inventory — migration reference before archive deletion                                            |
| `.claude/strategy/cloud-desktop-strategy.md`                     | Architecture, cost analysis, cloud execution topology                                                             |
| `.claude/strategy/core-principles.md`                            | Trust commitments, "For Claude Code" guidance                                                                     |
| `.claude/rules/`                                                 | Auto-loaded rules (architecture, code-standards, components, etc.)                                                |
| `.claude/skills/`                                                | Agent skills (pickup, project-manager, code-review, pre-commit)                                                   |
| Private business docs (`BNTO_PRIVATE_DOCS_PATH` in `.env.local`) | Pricing strategy, revenue projections, SEO monetization, feature funnel, brand, personas, competitive positioning |
