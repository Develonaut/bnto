# Bnto — Build Plan

**Last Updated:** March 14, 2026
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

- **Editor is functional and in beta.** Execution wired (RunButton → WASM → results), compartment redesign shipped, I/O visual hierarchy complete, editor UX polish complete (hover delete, exit animations, drag reorder, auto-behaviors), copy/labels cleaned up.
- **Active work:** Sprint 5 — finishing the editor for v1. Config panel controls (biggest UX gap), auto-download default, save/My Recipes, E2E verification.
- **Tabled:** Sprint 4B (Code Editor), Sprint 6 (Edit/Run Mode), Sprint 5B W2-4 (LayerPanel polish, processing node accents).
- **M1 delivered:** All 6 Tier 1 bntos + 2 Tier 1B multi-node compositions run 100% client-side via Rust→WASM
- **Cloud pipeline:** Go API on Railway + R2 file transit — M4 infrastructure ready
- **WASM engine:** 5 Rust crates, single cdylib, 1.6MB raw / 606KB gzipped
- **Auth:** `@convex-dev/auth`. Password auth, integration tests complete, E2E auth lifecycle verified (13/13 tests)
- **Infra:** GitHub Actions CI (Rust + TypeScript + CI Gate), automatic Convex production deploy on merge to main, Lighthouse CI on PRs, PostHog telemetry wired
- **Packages:** `@bnto/core`, `@bnto/auth`, `@bnto/backend`, `@bnto/nodes`, `@bnto/ui`, `@bnto/editor`

---

## What's Built (don't redo)

- [x] Monorepo: Turborepo + pnpm + Taskfile.dev + go.work
- [x] Go engine + API: 10 node types, CLI, HTTP API on Railway — archived, ready for M4
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
- [x] Cloud execution: R2 file transit, presigned URLs, Railway deployment — full pipeline verified
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

---

## Revenue & Monetization Context

Pricing, revenue projections, and "ready to charge" criteria live in [.claude/business/](business/) — see [pricing-strategy.md](business/pricing-strategy.md), [seo-monetization.md](business/seo-monetization.md), and [feature-funnel.md](business/feature-funnel.md).

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
| Sprint 7-8   | Desktop app                                  | Top-of-funnel. Word of mouth begins. Free forever — trust signal.                                                                |
| Sprint 9     | Stripe + Pro tier                            | **First revenue possible.** Pro: $8/month for persistence, collaboration, server-side AI, priority processing.                   |

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

---

## Active Sprint

### Sprint 5: Editor v1 (M2 Completion)

**Goal:** Ship the editor as a usable v1 product. Users can build recipes, configure nodes with proper controls (not raw text inputs), run them, save to their account. This is the M2 completion path.

**Control strategy:** See [config-controls.md](.claude/strategy/config-controls.md) for the full parameter→control matrix, missing controls list, and build checklist.

#### Wave 1 — Auto-Download Default

- [x] `@bnto/nodes` + `@bnto/core` — Enable auto-download by default on Output node. Flip `autoDownload` default from `false` to `true` in: (1) Zod schema default in `@bnto/nodes/schemas/output.ts`, (2) `DEFAULTS` object in `core/adapters/browser/deriveOutputConfig.ts`, (3) all 8 predefined recipes in `@bnto/nodes/recipes/*.ts`, (4) blank definition in `@bnto/nodes/createBlankDefinition.ts`

#### Wave 2 — Config Panel Controls (v1 blockers)

Build the missing UI controls that make the config panel usable. Raw text inputs for template expressions, comma-separated lists, and key-value pairs are the biggest v1 UX problem. See [config-controls.md](.claude/strategy/config-controls.md) § "Missing Controls" and "Node Parameter Matrix" for the full audit.

**Step 1 — Build UI primitives** in `@bnto/ui/interaction/`:

- [x] `@bnto/ui` — **Textarea**: Multiline text input. Reference: `shadcn-blocks/components/textarea/textarea-form-1.tsx`
- [x] `@bnto/ui` — **Combobox** (was TagPicker): Multi-select combobox with search, badges, and remove. Built on cmdk + Popover + Command primitive. For `z.array(z.string())` params (Input `extensions`). Reference: `shadcn-blocks/components/combobox/combobox-multi-select-1.tsx`
- [x] `@bnto/ui` — **KeyValueEditor**: Add/remove key→value pairs. For `z.record(z.string())` params (Spreadsheet `columns`, Transform `mappings`). Reference: `shadcn-blocks/components/input/input-special-1.tsx` (list pattern)

**Step 2 — Motorway showcase** (each new control gets a `ShowcaseSection` under the Controls tab):

- [x] `apps/web` — **Motorway Controls tab update**: Add showcase sections for Textarea, TagPicker, and KeyValueEditor. Column layout showing variants, states, edge cases. This is the living catalog — if it's not on Motorway, it doesn't exist.

**Step 3 — Wire into editor control registry**:

- [x] `@bnto/editor` — **TextareaControl**: Create control component, register as `textarea` in `CONTROL_REGISTRY`
- [x] `@bnto/editor` — **TagPickerControl**: Create control component, register as `tagPicker` in `CONTROL_REGISTRY`
- [x] `@bnto/editor` — **KeyValueEditorControl**: Create control component, register as `keyValue` in `CONTROL_REGISTRY`

**Step 4 — Update inference + schemas**:

- [x] `@bnto/nodes` — **inferFieldType updates**: Map `z.array(z.string())` → `tagPicker`, `z.record(z.string())` → `keyValue`. Add `textarea` hint support via `NodeParamMeta`
- [x] `@bnto/nodes` — **Schema metadata cleanup**: Add `hidden: true` to Input node `accept` param (derive from `extensions`). Add `hidden: true` to Parallel node `tasks` param. Add `hidden: true` (or read-only) to `operation` param on image, file-system, and spreadsheet nodes — operation is pre-set from the palette, no need to expose in config panel. Add human-readable labels to `z.enum()` options where missing (Loop `mode`, Input `mode`, Output `mode`). **Also DRY up recipe definitions:** Extract shared `defaultOutputNode()` and `defaultInputNode()` factory functions to eliminate repeated I/O node boilerplate across all 8 recipe files — output params (mode, zip, autoDownload, label) and input params (mode, accept, extensions, multiple) are copy-pasted everywhere

#### Wave 3 — Save + My Recipes

Convex persistence for custom recipes. Backend save mutation + core hooks already exist.

- [ ] **CLAIMED** `apps/web` — Save button in editor toolbar, tier limits (Free: 3 recipes, Pro: unlimited)
- [ ] **CLAIMED** `apps/web` — My Recipes integration (load saved recipes into editor)
- [ ] **CLAIMED** `apps/web` — Unsaved changes warning on navigation (`beforeunload` prompt when `isDirty` is true)

#### Wave 4 — E2E Verification

Existing editor E2E coverage is strong (33+ tests across 5 spec files). This wave fills gaps.

- [ ] `apps/web` — Verify existing E2E tests pass with new controls (TagPicker, KeyValueEditor render for correct params)
- [ ] `apps/web` — Keyboard shortcuts: Cmd-Z (undo), Cmd-Shift-Z (redo), Delete (remove), Cmd-Enter (run), Cmd-S (export)
- [ ] `apps/web` — Accessibility audit (focus management, screen reader labels on canvas nodes)

---

## Tabled Sprints

### Sprint 6: Edit Mode ↔ Run Mode — TABLED

**Tabled (March 2026).** Mini Motorways-inspired edit/run mode switch. Same canvas for editing and running — pause to edit, unpause to watch traffic flow. Requires Sprint 5 execution integration (done). Deferred until v1 editor ships. See `.claude/decisions/editor-ux-direction.md` for the full design.

### Sprint 5B Waves 2-4: LayerPanel Polish + Processing Node Accents — TABLED

**Tabled (March 2026).** Category color pips on processing nodes, selected state ring, LayerPanel I/O distinction + selected highlights. Wave 1 (I/O visual hierarchy) shipped. Remaining waves need more design iteration. Existing Pressable pressed state handles selection well enough for v1.

### Sprint 4B: Code Editor (CodeMirror 6) — TABLED

**Goal:** A schema-aware `.bnto.json` code editor for power users — the coding-oriented counterpart to the visual canvas. Users who prefer code get the same power as the visual canvas, with the speed and precision of text editing. Slash commands bring Notion-like ergonomics. The code editor is free (same as the visual editor).

**Status:** Unblocked but deferred until visual editor ships to production.

**Required reading:** Before picking up ANY task in Sprint 4B, read [code-editor.md](.claude/strategy/code-editor.md) — the design document covering tech choice rationale (CM6 over Monaco), architecture (headless-first + store sync), feature tiers, slash command implementation, JSON Schema strategy, CLI/TUI parallels, React integration pattern, theming, and performance considerations. Also read the persona at `.claude/skills/code-editor-expert/SKILL.md` for CM6-specific APIs, extension patterns, and gotchas.

**Architecture: headless-first + CM6.** The code editor shares Sprint 4's headless foundation (Wave 1 pure functions, Wave 2 editor store). CM6 extensions provide JSON-specific intelligence on top. Both editors are views of the same `Definition` in `useEditorStore`. See [code-editor.md § Architecture](.claude/strategy/code-editor.md) for the state flow diagram.

**Tech choice: CodeMirror 6, not Monaco.** ~40 KB gzipped vs ~2.4 MB (60x smaller). CSS variable theming (direct OKLCH integration). Native mobile support. Headless state (`EditorState` without DOM). See [code-editor.md § Tech Choice](.claude/strategy/code-editor.md) for the full comparison table and evidence (Sourcegraph, Replit, Chrome DevTools migrations).

**Key implementation patterns** (from design doc + persona):

- **React integration:** Custom `useCodeEditor` hook with `useRef`/`useEffect` — NOT `@uiw/react-codemirror`. CM6 author recommends imperative integration.
- **Theming:** CM6 `EditorView.theme()` with CSS variables (`var(--background)`, `var(--primary)`, etc.). Dark mode automatic. See [code-editor.md § Theming](.claude/strategy/code-editor.md).
- **Slash commands:** CM6 `CompletionSource` or `StateField` + `showTooltip` facet. Context-aware — only activates inside `"nodes"` arrays. See [code-editor.md § Slash Commands](.claude/strategy/code-editor.md).
- **Store sync:** `Annotation` pattern prevents sync loops between CM6 and Zustand. Debounced JSON parsing (200ms). See [code-editor.md § React Integration](.claude/strategy/code-editor.md).
- **JSON Schema:** Generated from `@bnto/nodes` types (build step), fed to `codemirror-json-schema` at runtime. See [code-editor.md § JSON Schema Strategy](.claude/strategy/code-editor.md).

**Persona ownership:**
| Wave | Lead Persona | Supporting | Rationale |
|------|-------------|------------|-----------|
| Wave 1 | — (build step, no persona needed) | — | JSON Schema generation from `@bnto/nodes` types — pure TypeScript |
| Wave 2 | `/code-editor-expert` | `/frontend-engineer` | CM6 foundation, theming, React integration. Frontend Engineer helps with component composition |
| Wave 3 | `/code-editor-expert` | — | Slash commands and command registry — pure CM6 extension work |
| Wave 4 | `/code-editor-expert` + `/frontend-engineer` | `/reactflow-expert` | Store sync, split view, command palette. ReactFlow Expert advises on store integration |
| Wave 5 | `/code-editor-expert` + `/frontend-engineer` | — | Breadcrumbs, polish, E2E tests |

**Rule:** For ANY work touching CodeMirror 6 APIs, editor extensions, slash commands, JSON Schema integration, or CM6 theming — invoke `/code-editor-expert`. This persona is THE authority on CM6 in this codebase.

**Dependencies:** Sprint 4 Wave 1 (pure functions) and Wave 2 (editor store) must complete first. The code editor consumes the shared store — it doesn't own it.

#### Wave 1 (parallel — JSON Schema generation)

Generate a JSON Schema from existing `@bnto/nodes` types. This schema drives CM6 validation, autocompletion, and hover tooltips. Generated, not hand-written — stays in sync with node definitions automatically. Pure TypeScript build step — no persona needed, but read [code-editor.md § JSON Schema Strategy](.claude/strategy/code-editor.md) before starting.

- [ ] `@bnto/nodes` — **JSON Schema generator script**: Build step that derives a JSON Schema from `ParameterSchema` objects (all 10 node types), `Definition` type structure, and `NODE_TYPE_INFO` metadata. Output: `packages/@bnto/nodes/src/generated/bnto.schema.json`. Schema includes per-node-type parameter constraints (`visibleWhen`, `requiredWhen`, enum values, min/max, defaults).
- [ ] `@bnto/nodes` — **Schema export**: Export the generated schema from the package entry point. Consumers import it as `import schema from "@bnto/nodes/schema"`.
- [ ] `@bnto/nodes` — **Unit tests for schema generation**: Verify schema validates known-good `.bnto.json` fixtures. Verify schema rejects malformed definitions. Verify per-node-type parameter constraints are present.

#### Wave 2 (parallel — CM6 foundation)

Editor component with JSON language, schema validation, autocompletion, hover tooltips, and warm theme. The core editing experience — everything else builds on this. **Invoke `/code-editor-expert`** — this persona owns all CM6 APIs, extension patterns, theming, and React integration. Also invoke `/frontend-engineer` for component composition.

- [ ] `apps/web` — **Install CM6 packages**: `@codemirror/state`, `@codemirror/view`, `@codemirror/lang-json`, `@codemirror/autocomplete`, `@codemirror/lint`, `@codemirror/commands`, `@codemirror/search`, `codemirror-json-schema`.
- [ ] `apps/web` — **`bntoTheme()` extension**: CM6 theme using CSS variables from `globals.css`. OKLCH tokens for background, foreground, primary (caret/cursor), accent (selection), muted (gutters), border, destructive (error diagnostics). Dark mode automatic via CSS variable resolution.
- [ ] `apps/web` — **`useCodeEditor(options)` hook**: Custom React hook (not `@uiw/react-codemirror`). Creates `EditorView` in `useEffect`, stores in `useRef`. Configures: `json()` language, `jsonSchema()` with bnto schema, `bntoTheme()`, standard keymap, history (undo/redo), bracket matching, code folding, line numbers. Cleanup on unmount.
- [ ] `apps/web` — **`CodeEditor` component**: Thin wrapper around `useCodeEditor`. Renders a `<div ref={containerRef} />` with proper sizing. Lazy-loaded via `next/dynamic({ ssr: false })` — CM6 needs DOM.
- [ ] `apps/web` — **Unit tests for CM6 extensions**: Theme applies correct CSS variables. Schema validation produces diagnostics for invalid JSON. Autocompletion suggests node type names and parameter names.

#### Wave 3 (parallel — slash commands + command registry)

Inline slash command menu for node template insertion — the bridge between "code editor" and "visual editor" ergonomics. **Invoke `/code-editor-expert`** — this persona owns slash command implementation (CM6 `CompletionSource` vs `StateField` + `showTooltip` approach), context-aware activation, and the command registry pattern.

- [ ] `apps/web` — **Command registry**: `EditorCommand` type with `id`, `label`, `description`, `category`, `icon`, `shortcut`, `slashTrigger`, `available`, `execute`. Registry populated from `NODE_TYPE_INFO` (one "Insert X Node" command per node type) plus editor commands (Format JSON, Validate, Run, Export). Single source of truth shared by slash menu and Cmd-K palette.
- [ ] `apps/web` — **`bntoSlashCommands()` extension**: CM6 `CompletionSource` (or `StateField` + `showTooltip` — evaluate which approach is better). Activates when user types `/` inside a `"nodes": [...]` array. Shows filterable list of node types with icons and descriptions. On selection, inserts a complete, valid node JSON block with generated ID, default params from schema, and cursor positioned at the first editable parameter.
- [ ] `apps/web` — **Node template generation**: Pure function `generateNodeTemplate(nodeType)` → formatted JSON string for a new node of the given type, with default parameter values from `ParameterSchema`. Used by both slash commands and command palette.
- [ ] `apps/web` — **Unit tests for slash commands**: Slash menu activates on `/` at valid position. Menu filters as user types. Selection inserts valid node JSON. Menu doesn't activate outside `"nodes"` array.

#### Wave 4 (parallel — store sync + command palette + split view)

Bidirectional sync between code editor and visual canvas. Cmd-K palette for app-level commands. Split view for simultaneous editing. **Invoke `/code-editor-expert` + `/frontend-engineer`.** Code Editor Expert owns the CM6 `Annotation` sync pattern and store integration. Frontend Engineer owns component composition and split view layout. `/reactflow-expert` advises on store integration with the visual canvas.

- [ ] `apps/web` — **Store sync extension**: CM6 `updateListener` + `Annotation` pattern. Code editor changes → parse JSON → validate → update `useEditorStore.definition`. Store changes → serialize to JSON → dispatch CM6 transaction with `externalUpdate` annotation (prevents sync loop). Debounced parsing (200ms) for performance.
- [ ] `apps/web` — **Command palette (Cmd-K)**: Uses `cmdk` (shadcn/ui `Command` component). Opens on Cmd-K anywhere in the editor. Lists all commands from the command registry. Keyboard navigable, filterable. Not CM6-specific — works across the entire app.
- [ ] `apps/web` — **Split view**: Side-by-side `BentoCanvas` + `CodeEditor`, both reading from `useEditorStore`. Changes in either sync through the store. Resizable split pane. Toggle between code-only, visual-only, and split modes via `EditorModeToggle`.
- [ ] `apps/web` — **Unit tests for sync**: Code edit → store updates. Store change → CM6 document updates. External annotation prevents sync loops. Invalid JSON doesn't crash store.

#### Wave 5 (sequential — breadcrumbs, polish, E2E)

Navigation aids and full end-to-end verification. **Invoke `/code-editor-expert` + `/frontend-engineer`.** Code Editor Expert owns breadcrumb implementation (CM6 `ViewPlugin` + Lezer tree walking) and template expression hints. Frontend Engineer owns E2E test composition.

- [ ] `apps/web` — **Breadcrumb panel**: JSON path breadcrumbs above the editor showing current cursor position: `root > nodes > [0] > parameters > quality`. CM6 `ViewPlugin` watches cursor, walks Lezer parse tree. Clicking a breadcrumb segment navigates the cursor to that position.
- [ ] `apps/web` — **Format on save**: Pretty-print JSON on Cmd-S. Preserves cursor position.
- [ ] `apps/web` — **Template expression hints**: Autocomplete for `{{.INPUT_DIR}}`, `{{.item}}`, `{{index . "node-id" "port"}}` inside string values. Custom `CompletionSource` that activates inside `{{...}}` delimiters.
- [ ] `apps/web` — **E2E tests**: Open code editor → JSON renders with syntax highlighting. Type invalid JSON → error diagnostics appear. Type `/` → slash menu shows node types. Select from slash menu → valid node template inserted. Edit in code editor → visual canvas updates (split view). Edit in visual canvas → code editor updates. Cmd-K → command palette opens. Breadcrumbs show correct path. Export produces valid `.bnto.json`.

---

## Phase 2: Desktop App (Local Execution)

**Goal:** Free desktop app. Same React frontend, local engine execution. Free forever, unlimited runs. No account needed. Trust signal and top-of-funnel growth driver.

**Desktop tech: Tauri (Rust-native).** M1 Rust evaluation passed — one codebase for browser WASM + desktop native + CLI.

### Sprint 7: Desktop Bootstrap

**Persona ownership:**
| Package | Persona |
|---------|---------|
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

### Sprint 8: Local Execution

**Persona ownership:** Same as Sprint 7 — `/frontend-engineer` (desktop UI), `/core-architect` (adapter), `/rust-expert` (engine).

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

**"Ready to charge" gate:** Before Sprint 9, confirm: real users running browser bntos, conversion hooks built and tested (Save, History, Premium), people return voluntarily, at least one server-side bnto (AI or shell) ready for Pro tier.

### Sprint 9: Stripe + Pro Tier (M5)

**Goal:** First revenue. Pro sells real value — not artificial limits on browser-native operations.

**What Pro includes:** $8/month or $69/year. Saved workflows, execution history (30-day retention), team sharing (up to 5 members), server-side premium bntos (AI, shell, video — M4), priority processing, API access.

**What stays free forever:** All browser-capable bntos, unlimited runs, desktop app. See ROADMAP.md trust commitments.

**Persona ownership:**
| Package | Persona |
|---------|---------|
| `apps/web` | `/frontend-engineer` |
| `@bnto/backend` | `/backend-engineer` |
| `archive/api-go` | `/go-engineer` |

#### Wave 1 (parallel — payments)

- [ ] `apps/web` — `/frontend-engineer` — Stripe integration (checkout session, webhook handler, subscription sync to Convex)
- [ ] `@bnto/backend` — `/backend-engineer` — `planTier` updated on successful Stripe webhook (free → pro)
- [ ] `apps/web` — `/frontend-engineer` — Upgrade page (`/upgrade`) — pricing, Pro benefits, Stripe checkout CTA
- [ ] `apps/web` — `/frontend-engineer` — Billing management page (current plan, cancel, manage via Stripe portal)

#### Wave 2 (parallel — Pro feature gates)

- [ ] `@bnto/backend` — `/backend-engineer` — Pro feature gates: 30-day history retention, team sharing (up to 5 members), priority processing queue
- [ ] `archive/api-go` — `/go-engineer` — Server-side execution quota enforcement (applies to premium server-side bntos only — AI, shell, video)
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
- **Option B (Explicit):** User builds iteration manually — `Input → Loop (forEach) → Image (compress inside loop) → Output`. More flexible, more transparent, matches current Go engine model.

**Proposed direction:** Smart by default (Option A) with an advanced toggle to switch to explicit looping for power users. This affects engine processing, definition schema, and editor UX. Needs a deep review session with full project context ([bntos.md](strategy/bntos.md), [business/mvp-roadmap.md](business/mvp-roadmap.md)) before implementation.

**Touches:** `@bnto/nodes` (definition schema), `engine/` (execution model), `@bnto/editor` (node placement + wiring), recipe definitions (compress-images etc.), `io-nodes.md` strategy doc.

---

## Backlog

### Codebase Audit: Go-Era Artifacts & Migration Debt

**Priority: Medium.** Audit the entire codebase to remove artifacts from the Go engine migration era. During the transition from Go→Rust, several patterns were duplicated or left as compatibility shims. Now that the Rust engine owns execution and `@bnto/nodes` owns definitions, these should be cleaned up.

**What to look for:**

1. **Duplicated execution logic** — `@bnto/core` may still have JS-side pipeline orchestration (`executePipeline.ts`) that duplicates what the Rust executor now handles. Verify the deprecated path is truly dead and remove it.
2. **Go-era parameter schemas** — `@bnto/nodes` schemas for `spreadsheet` and `file-system` still reference Go-era operations (`read`/`write` for spreadsheet, `path`/`content`/`source`/`dest` for file-system). These don't match the Rust processors (`clean`/`rename` and `find`/`replace`/`prefix`/`suffix`). Align the TS schemas with what the engine actually implements.
3. **Deprecated API methods** — `browserExecutionService.hasImplementation()` is an alias for `isCapable()`. Migrate consumers and remove the alias.
4. **Split comment patterns** — Comments that say "Go engine does X" or reference `archive/engine-go/` behavior as if it's current.
5. **Oversized Rust files** — `executor.rs` (2068 lines), node crate files (1000-2000 lines each) violate Bento Box. Split into focused modules.
6. **`processFile` API path** — The single-file `processFile()` worker API may be dead code now that `executePipeline()` handles everything. Verify and remove if so.

**Scope:** `packages/core/`, `packages/@bnto/nodes/`, `engine/crates/`, `apps/web/` (any remaining Go references in UI code).

**Tasks:**

- [ ] `packages/@bnto/nodes` — Align `spreadsheet` and `file-system` Zod schemas with Rust processor parameters (clean/rename, find/replace/prefix/suffix)
- [ ] `packages/core` — Remove deprecated `hasImplementation()` alias, migrate `useRecipeFlow.ts` to `isCapable()`
- [ ] `packages/core` — Verify `processFile` worker path is dead code and remove if so
- [ ] `packages/core` — Remove or deprecate `executePipeline.ts` if fully replaced by Rust executor
- [ ] `engine` — Split `executor.rs` into focused modules (executor, primitive execution, container execution)
- [ ] `engine` — Add comment density pass to executor.rs sections 400+ for consistency with other files
- [ ] Cross-cutting — Grep for "Go engine", "Go API", "archive/engine-go" references in non-archive code and remove stale ones

---

### UX: Compartment Node Visual Redesign — Phases 2-3 (Mini Motorways Buildings)

**Phase 1 delivered in Sprint 5 Wave 1** (icon registry + category color mapping). Phases 2-3 remain in backlog as polish.

**Phase 2: Elevation-driven execution states**

Replace the current flat status handling with elevation transitions that make compartments physically pop as they progress. The Card `.surface` system already provides springy elevation changes — we just need to map states correctly.

| State       | Elevation      | Visual effect                                        |
| ----------- | -------------- | ---------------------------------------------------- |
| `idle`      | `none` or `sm` | Flat/barely lifted — resting in the bento box        |
| `pending`   | `sm`           | Slight lift, muted appearance — waiting in queue     |
| `active`    | `md`           | Rising up — "being serviced" like a MM building      |
| `completed` | `lg`           | Full pop — satisfying spring bounce to max elevation |

The spring animation on Card elevation changes creates the Mini Motorways "building materializing" feel automatically. As the recipe runs, compartments pop up one by one in sequence — like buildings appearing on the map.

**Phase 3: Bento grid layout**

Replace the current horizontal strip (all nodes in a single row at 220px stride) with a proper bento box grid that uses varied compartment sizes. Different node types get different footprints:

| Tier          | Size     | Used for                                                     |
| ------------- | -------- | ------------------------------------------------------------ |
| **Standard**  | 140×140  | Most nodes (image, spreadsheet, transform, etc.)             |
| **Compact**   | 100×100  | Simple nodes (edit-fields with no parameters)                |
| **Wide**      | 200×140  | Nodes with more visual content (future inline controls)      |
| **Container** | 240×180+ | Group, loop, parallel — larger to suggest they hold children |

The grid layout algorithm should pack compartments like a real bento box — no uniform grid, but a visually balanced arrangement. Update `bentoSlots.ts` to support varied slot sizes.

**Future (not in scope):**

- Inline micro-controls on nodes (radial dials, parameter badges) — nice-to-have after core visual identity ships
- Interactive connection handles — design decision is no edges
- Per-node execution progress bars — elevation + status color is sufficient

**Tasks:**

- [ ] `packages/editor` — **Elevation state mapping**: Update `CompartmentNode.tsx` status → elevation mapping: idle=none/sm, pending=sm, active=md, completed=lg. Leverage existing Card spring animations
- [ ] `packages/editor` — **Bento grid layout**: Update `bentoSlots.ts` with varied slot sizes per node type tier (standard/compact/wide/container). Replace horizontal strip with proper 2D bento packing
- [ ] `packages/editor` — **Motorway showcase**: Update Motorway editor showcase to demonstrate the new visual treatment with all node types visible
- [ ] `apps/web` — **E2E verification**: Verify editor canvas renders correctly with new node visuals. Update screenshots if page-level layout changed

### UX: Expandable Container Nodes (Recipe/Group Drill-Down)

**Priority: Near-term.** Container nodes (groups, sub-recipes) should have an expand button that reveals their inner node structure as a vertical layout within the canvas. Currently containers are opaque — the user sees "Compress Image / Recipe" but can't see the loop → leaf structure inside without reading the `.bnto.json`.

**Behavior:**

- Container cards (any node with `isContainer: true` or `displayName`) show a small expand/collapse toggle
- Clicking expand opens the container inline, displaying child nodes in a vertical stack layout below the parent card
- Nested containers can be expanded recursively (group → loop → leaf)
- Collapsed is the default — users who just want to tweak surfaced params never need to expand
- Expanded state is visual-only (editor store), does not affect the Definition

**Design direction:** Think of it like a folder in a file tree — click to reveal contents, click again to collapse. The vertical layout avoids disrupting the horizontal bento grid. Child nodes render at a smaller scale or indented to show nesting depth.

**Dependencies:** Requires the definition tree to be stored in the editor (already done — `definition` field in EditorState). May benefit from the bento grid layout work (Phase 3 above) for proper space allocation.

**Tasks:**

- [ ] `packages/editor` — **Expand/collapse state**: Add `expandedNodeIds: Set<string>` to EditorState + toggle action
- [ ] `packages/editor` — **Expanded container renderer**: New component that renders child nodes vertically when a container is expanded. Reads children from `definition` tree via `findDefinitionById`
- [ ] `packages/editor` — **Expand toggle UI**: Add expand/collapse button to CompartmentNode for container types
- [ ] `packages/editor` — **Nested expansion**: Support recursive expand (expanded group shows its loop, which can also be expanded to show the leaf)
- [ ] `apps/web` — **E2E**: Verify expand/collapse works, screenshots if layout changes

---

### UX: Global Error Boundary with GitHub Issue Reporter

**Priority: Medium.** Add a global error boundary that catches unhandled React errors and presents a branded error dialog with enough context to file a GitHub issue. Currently there are zero error boundaries — any unhandled throw crashes the page with a white screen. No `error.tsx`, `global-error.tsx`, or React ErrorBoundary exists.

**Goal:** When an unhandled error occurs, the user sees a helpful dialog (not a white screen) with a "Report this issue" button that opens a pre-filled GitHub issue on `Develonaut/bnto`.

**Current state (as of research):**

- No error boundaries or error pages exist (only `not-found.tsx` for 404)
- PostHog captures product events but NOT unhandled exceptions
- Auth session loss is handled (`SessionProvider.onSessionLost` → redirect to `/signin`)
- Scattered `try/catch` in auth forms and recipe execution — no centralized error handling

**Architecture — Next.js App Router error files:**

Next.js App Router has built-in error boundary support via convention files. These are React Error Boundaries under the hood. The implementing agent should create:

1. **`app/global-error.tsx`** — Catches errors in the root layout itself. Must be `"use client"` and must render its own `<html>` and `<body>` tags (replaces the entire document). This is the last-resort catch-all.
2. **`app/(app)/error.tsx`** — Catches errors within the authenticated app shell (dashboard, settings, etc.). Can use the app's design system since the root layout is still intact.
3. **`app/[bnto]/error.tsx`** — Catches errors on recipe/tool pages. Same approach — branded error UI with report button.

**Error dialog UX requirements:**

- Show a branded, friendly error message (not a stack trace dump)
- "Report this issue" button that opens a GitHub issue via URL pre-fill
- "Try again" button that calls `reset()` (the Next.js error boundary reset function)
- "Go home" link as fallback navigation
- Use existing design system components (`Card`, `Button`, `Heading`, `Stack`) where available (not in `global-error.tsx` which replaces the document)

**GitHub issue pre-fill approach:**

URL pattern: `https://github.com/Develonaut/bnto/issues/new?labels[]=bug&title=...&body=...`

The body should include (as Markdown):

- **Error message** — `error.message` (first 200 chars)
- **Route** — `window.location.pathname`
- **Component stack** — from `error.digest` or React's `errorInfo.componentStack` (truncated to 5 frames)
- **Browser/OS** — `navigator.userAgent`
- **App version** — read from env var (set at build time, e.g., `NEXT_PUBLIC_APP_VERSION` or `process.env.npm_package_version`)
- **JS stack trace** — `error.stack` (first 5 frames, inside a `<details>` block to collapse it)
- **Timestamp** — `new Date().toISOString()`

**CRITICAL: URL length limit.** GitHub returns 414 for URLs over ~8,000 chars. The `body` must be truncated. Strategy: truncate stack traces to first 5 frames, cap total body at ~4,000 chars (leaves room for encoding overhead). Use `encodeURIComponent()` on all values.

**Helper function for building the issue URL:**

```typescript
// Pure function — no React dependency, testable in isolation
function buildGitHubIssueUrl(error: Error, route: string): string {
  const title = `[Bug] ${error.message.slice(0, 80)}`;
  const body = [
    `## Error\n\`${error.message.slice(0, 200)}\``,
    `## Route\n\`${route}\``,
    `## Environment`,
    `- **Browser:** \`${navigator.userAgent}\``,
    `- **Timestamp:** ${new Date().toISOString()}`,
    `- **Version:** \`${process.env.NEXT_PUBLIC_APP_VERSION ?? "unknown"}\``,
    error.stack
      ? `\n<details><summary>Stack trace</summary>\n\n\`\`\`\n${error.stack.split("\n").slice(0, 8).join("\n")}\n\`\`\`\n</details>`
      : "",
  ].join("\n\n");

  const params = new URLSearchParams({
    title,
    body: body.slice(0, 4000),
    "labels[]": "bug",
  });
  return `https://github.com/Develonaut/bnto/issues/new?${params}`;
}
```

**PostHog integration (optional enhancement):**

- Capture `app_error` event via `core.telemetry.capture()` with error message, route, and digest
- This gives the dev team server-side visibility even if users don't file issues
- Only if PostHog is already initialized — never block error UI on telemetry

**Testing strategy:**

- Unit test `buildGitHubIssueUrl()` — verify URL structure, encoding, truncation
- Unit test that the URL stays under 8,000 chars even with long stack traces
- E2E test: trigger an error (e.g., render a component that throws), verify the error dialog appears with "Report" and "Try again" buttons
- E2E test: verify "Try again" calls `reset()` and re-renders

**Files to create/modify:**

- `apps/web/app/global-error.tsx` — Root-level catch-all (standalone `<html>`)
- `apps/web/app/(app)/error.tsx` — App shell error boundary (uses design system)
- `apps/web/app/[bnto]/error.tsx` — Recipe page error boundary (uses design system)
- `apps/web/lib/buildGitHubIssueUrl.ts` — Pure function for issue URL construction
- `apps/web/components/ErrorReport.tsx` — Shared error dialog UI (Card + buttons + error details)

**Design system compliance:**

- Use `Card elevation="md"` for the error dialog container
- Use `Heading`, `Text`, `Button`, `Stack` for layout
- Use `font-mono` for error message and stack trace display
- Use `Animate.FadeIn` for the error dialog entrance
- Use `destructive` color for the error icon/accent
- The `global-error.tsx` file cannot use the design system (it replaces `<html>`) — use minimal inline styles matching the theme tokens

**Scope boundaries:**

- This is error REPORTING, not error RECOVERY. Don't add retry logic to individual components
- Don't add Sentry or a third-party error tracking service — keep it simple with GitHub issues + PostHog events
- Don't change existing `try/catch` patterns in auth forms or execution — those handle expected errors with user-friendly messages. This boundary catches UNEXPECTED errors only

**Tasks:**

- [ ] `apps/web` — Create `buildGitHubIssueUrl()` pure function in `lib/` with unit tests (URL construction, encoding, truncation, length limit)
- [ ] `apps/web` — Create `ErrorReport` component — branded error dialog with "Report this issue" (GitHub link), "Try again" (reset), and "Go home" (navigation)
- [ ] `apps/web` — Create `app/global-error.tsx` — root catch-all with minimal inline-styled error UI + GitHub issue link
- [ ] `apps/web` — Create `app/(app)/error.tsx` — app shell error boundary using `ErrorReport` component
- [ ] `apps/web` — Create `app/[bnto]/error.tsx` — recipe page error boundary using `ErrorReport` component
- [ ] `apps/web` — (Optional) Capture `app_error` PostHog event on boundary trigger via `core.telemetry.capture()`
- [ ] `apps/web` — Add `NEXT_PUBLIC_APP_VERSION` to build env (Vercel env var or `package.json` read)
- [ ] `apps/web` — E2E test: trigger error, verify dialog renders with Report/Try Again/Go Home buttons

### Infra: Tag-Based Release Pipeline (GitHub Actions + Vercel)

**Priority: Medium.** Automated release workflow: tag a commit on `main` → GitHub Action builds a Vercel preview → full test suite (unit + E2E) runs against the live preview URL → green = ready to promote to production. Currently deploys are fully manual (`vercel --prod` or MCP tool).

- [ ] `infra` — Create GitHub Actions workflow triggered by git tags (`v*` or `release-*`)
- [ ] `infra` — Workflow step: build Vercel preview deployment via CLI, capture preview URL
- [ ] `infra` — Workflow step: run Playwright E2E tests against the preview URL (`baseURL` override)
- [ ] `infra` — Workflow step: run unit/integration tests (`task ui:test`, `task wasm:test:unit`)
- [ ] `infra` — On all-green: notify (GitHub comment/Slack) with preview URL + "ready to promote" status
- [ ] `infra` — Optional: auto-promote to production if all checks pass, or require manual promotion via Vercel dashboard
- [ ] `infra` — Add `VERCEL_TOKEN`, `VERCEL_ORG_ID`, `VERCEL_PROJECT_ID` to GitHub repo secrets

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

### Chore: Remove Remaining Go References from Codebase

**Priority: Low.** Three files outside `@bnto/nodes` still reference "Go engine." Clean up in a small PR.

- [ ] `packages/core/src/__tests__/integration/execution.test.ts:22` — Remove "Go engine" reference
- [ ] `packages/core/src/__tests__/integration/transit-pipeline.test.ts:10` — Remove "Go engine" reference
- [ ] `packages/@bnto/backend/convex/schema.ts:52` — Remove "Go engine" comment (note: `goExecutionId` field still exists, may need schema migration)

### Chore: Go Engine Archival & Node Migration Reference

**Priority: High.** The archived Go engine (`archive/engine-go/`, ~33K LOC) and API server (`archive/api-go/`, ~2.5K LOC) are slated for deletion. Before removal, all 10 node type implementations have been documented in [go-engine-migration.md](strategy/go-engine-migration.md) as a migration reference.

**Migration reference doc:** `.claude/strategy/go-engine-migration.md` — complete implementation details, parameters, patterns, dependencies, and open decisions for all 10 Go node types.

**What's fully migrated (safe to delete):**

- `image` — compress, resize, convert (Rust `bnto-image`, 224 tests)
- `file-system` rename/move (Rust `bnto-file`, 32 tests)
- `spreadsheet` CSV clean + rename (Rust `bnto-csv`, 42 tests)

**What's partially migrated (gaps documented):**

- `file-system` — missing: read, write, copy, delete, mkdir, exists, list with glob
- `spreadsheet` — missing: Excel (.xlsx) read/write (`excelize/v2` equivalent)

**What's not migrated (documented for future):**

- Orchestration: `group`, `loop`, `parallel` — needed for multi-step recipes
- Data: `transform` (expr-lang), `edit-fields` (Go templates) — needed for Tier 2 recipes
- Server-only: `http-request`, `shell-command` — M4 Pro tier

**Tasks:**

- [x] `.claude/strategy/` — Create `go-engine-migration.md` with full node inventory, parameters, patterns, dependencies, and migration paths
- [ ] `archive/` — **Final review**: Walk through `go-engine-migration.md` with the team, confirm nothing is missing before deletion
- [ ] `archive/` — **Delete `archive/engine-go/`**: Remove Go engine source code. Update `go.work`, `.gitignore`, `Taskfile.yml`, `bnto.code-workspace` to remove Go engine references
- [ ] `archive/` — **Delete `archive/api-go/`**: Remove Go API server source code. Update Docker, Taskfile, and CI references. (Note: if M4 cloud uses Go, fork to a separate repo first)
- [ ] `.claude/` — **Update docs**: Remove Go engine references from CLAUDE.md, architecture.md, ROADMAP.md. Update "What's Built" section in PLAN.md
- [ ] `infra` — **Clean up Taskfile**: Remove `task build`, `task test`, `task vet`, `task api:*` commands that target the Go engine
- [ ] `infra` — **Clean up CI**: Remove Go-related checks from CI if any remain (Go checks already removed from CI Gate, but verify)

### Engine: Unmigrated Node Operations (Rust WASM)

**Priority: Medium.** Bring Go engine operations that have no Rust equivalent yet. Reference: [go-engine-migration.md](strategy/go-engine-migration.md).

**Tier 2 recipe blockers:**

- [ ] `engine` — **`bnto-image`: composite operation** — overlay/watermark. Needed for `/watermark-images` (Tier 2, 30K+ monthly searches). See Go `image.go` composite logic
- [ ] `engine` — **`bnto-image`: EXIF metadata strip** — needed for `/strip-exif` (Tier 2, 15K+ monthly searches). Go used `imaging` library strip
- [ ] `engine` — **`bnto-csv`: merge operation** — concat + deduplicate multiple CSVs. Needed for `/merge-csv` (Tier 2, 12K+ monthly searches)
- [ ] `engine` — **`bnto-csv`: CSV-to-JSON conversion** — needed for `/csv-to-json` (Tier 2, 25K+ monthly searches). May be a `transform` concern

**Orchestration (multi-step recipe support):**

- [ ] `@bnto/core` or `engine` — **Multi-step recipe orchestration**: Design how the browser adapter handles recipes with multiple processing nodes (group/loop pattern from Go). Currently the Web Worker processes one file through one node type. Multi-step requires either JS-side orchestration or WASM-side pipeline support. See `go-engine-migration.md` § Orchestration Nodes
- [ ] `engine` — **Expression evaluation in browser**: Choose a JS expression evaluator to replace `expr-lang/expr` for `transform` node and `loop` while/break conditions. Candidates: `expr-eval`, `filtrex`, custom safe evaluator

**Excel support:**

- [ ] `engine` — **`bnto-csv`: Excel (.xlsx) read/write** — Go used `excelize/v2`. Rust options: `calamine` (read) + `rust_xlsxwriter` (write). Lower priority than CSV operations

### Engine: Spreadsheet Node Template Resolution — M3/M4 (Go engine)

**Milestone: M3/M4.** Go engine work — not blocking M1 (browser execution uses Rust/JS, not Go). The `clean-csv` predefined Bnto fails in cloud execution. The `read-csv` node (type: `spreadsheet`) receives `<no value>` for its input file path template variable.

**Discovered via:** Integration E2E test. All image-based pipelines work — only the spreadsheet node path is broken.

- [ ] `engine` — Reproduce locally: `bnto run` with `clean-csv` fixture against a test CSV file
- [ ] `engine` — Debug template resolution in `spreadsheet` node's `Execute()`
- [ ] `engine` — Fix template variable resolution so `read-csv` receives the actual file path
- [ ] `engine` — Verify fix: E2E `clean-csv` test passes (`task e2e`)

### Engine: Loop Node Output Collection — M3/M4 (Go engine)

**Milestone: M3/M4.** Go engine work — not blocking M1. The `loop` node currently collects original items, not sub-node outputs.

**Impact:** The `rename-csv-columns` fixture is a read → write pass-through. True column remapping requires this fix.

- [ ] `engine` — Loop node collects sub-node outputs instead of (or in addition to) original items
- [ ] `engine` — Alternative: new array-level transform node that operates on all rows at once

### Engine: `pdf` Node Type — M3/M4 (Go engine)

**Milestone: M3/M4.** Go engine work. Required for the PDF to Images Bnto (Tier 2, 50K+ monthly searches).

- [ ] `engine` — Implement `pdf` node type (wrap `pdfcpu` Go library, or shell-command + ghostscript as interim)
- [ ] `engine` — Unit tests for PDF → image conversion
- [ ] `engine` — Integration fixture: `pdf-to-images.bnto.json`

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

### Infra: Domain Setup (bnto.io Custom Domains)

Web app domain (`bnto.io`) delivered in Sprint 2C. API domain (`api.bnto.io`) deferred to M4.

- [x] `infra` — Connect `bnto.io` to Vercel + Cloudflare DNS, verify auth redirects — Delivered in Sprint 2C
- [ ] `infra` — (M4) Add `api.bnto.io` CNAME → Railway, configure custom domain, update `GO_API_URL`

### Infra: Graduate SEO Validation from E2E to Unit Tests

**Priority: Medium.** Graduate SEO validation from slow E2E to unit tests (metadata, registry↔sitemap sync). Keep thin E2E for noindex/redirect/404. Lighthouse CI already delivered (Sprint 3 Wave 1 — GitHub Actions workflow + `task seo:audit`).

- [ ] `apps/web` — Move metadata validation to unit tests (`bntoRegistry.test.ts`)
- [x] `apps/web` — ~~Add Lighthouse CI with `seo: 90` threshold~~ — Delivered in Sprint 3 Wave 1: `.github/workflows/lighthouse.yml` + `lighthouserc.json` + `task seo:audit`
- [ ] `apps/web` — Slim E2E to redirects + 404 + noindex only

### Testing: Sprint 3 Deferred E2E Tests

**Deferred from Sprint 3 Wave 3 (March 2026).** Platform features are built and working. Test coverage deferred until editor MVP ships.

- [ ] `apps/web` — Playwright E2E: AuthGate conversion flow
- [ ] `apps/web` — Playwright E2E: browser-local execution history
- [ ] `@bnto/backend` — Unit tests for execution analytics queries

### Testing: Standardize E2E Selectors on data-testid

Current E2E tests mix CSS classes, `getByRole`, `getByText`, and `data-testid`. Standardize on `data-testid` for state detection and element targeting. Keep semantic selectors for accessibility assertions.

- [ ] `apps/web` — Audit E2E specs, add `data-testid` attributes, update selectors

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

### Showcase: Radial Light Source Controls

**Priority: Low (fun polish).** Replace linear slider on `/showcase` with radial + elevation controls for light source direction/height.

- [ ] `apps/web` — `RadialSlider` generic UI component (circular drag input, configurable labels)
- [ ] `apps/web` — Light elevation control → `--light-elevation` CSS variable
- [ ] `apps/web` — Wire into surface shadow system, replace `LightSourceSlider` on showcase

### UX: Expression Input — Pill Tokens & Variable Picker

**Priority: Medium.** Template expression fields (rename patterns, loop items, break conditions) are plain `<Input>` elements with placeholder hints. Users write `{{name}}-compressed.{{ext}}` with zero editor assistance. This is fine for Tier 1-2 recipes (structured controls handle everything), but becomes a usability cliff when `transform`, `http-request`, and `ai` nodes ship.

**Strategy doc:** [expression-input-ux.md](strategy/expression-input-ux.md) — full competitor analysis (Zapier, Make.com, n8n, Apple Shortcuts, Power Automate, Retool), recommended approach, engine changes, phased rollout.

**Phased delivery:**

**Phase 1 (current — no work needed):** Tier 1-2 recipes use structured controls exclusively. Template fields are hidden or pre-filled. Users never write expressions.

**Phase 2 (when Tier 3 nodes ship — transform, http-request):**

- [ ] `engine` — Add `template_variables: Option<Vec<TemplateVariable>>` to `ParameterDef` in `metadata.rs`. Each variable declares name, label, description, source, example value. Populate in processors that have template params (file-system rename pattern, loop items)
- [ ] `packages/@bnto/nodes` — Update codegen (`generate-from-catalog.ts`) to propagate `templateVariables` into `NodeSchemaDefinition` params
- [ ] `packages/editor` — **ExpressionInput component**: Rich text input that renders `{{var}}` as visual pill tokens. Backspace selects/deletes pills. Underlying value stays a template string
- [ ] `packages/editor` — **Variable picker popover**: Grouped by source (file metadata, upstream outputs, loop context). Search/filter. Inserts pill at cursor
- [ ] `packages/editor` — **SchemaField dispatch**: If `templateVariables` is set on a param, render `ExpressionInput` instead of `TextControl`
- [ ] `packages/editor` — **Fixed/Expression toggle**: Per-field toggle (n8n-style) that switches between structured control and expression input. Trailing icon on SchemaField
- [ ] `apps/web` — E2E: Verify pill token rendering, variable picker insertion, Fixed/Expression toggle

**Phase 3 (when ai nodes ship — Tier 4):**

- [ ] `packages/editor` — Expression validation feedback (red underline for unknown variables, type mismatches)
- [ ] `packages/editor` — Autocomplete for function names and variable paths (beyond pill insertion)
- [ ] `packages/editor` — Function reference tab in variable picker (document available template functions)

---

### Performance: WASM Bundle Size & Processing Benchmarks

**Deferred from Sprint 2B.** WASM bundle: 1.6MB raw / 606KB gzipped. ~20% above 500KB target. Not blocking M1.

- [ ] `engine` — Profile bundle size per crate, evaluate code splitting vs single bundle
- [ ] `apps/web` — Processing speed + memory benchmarks per node type

### Performance: Next.js Server Component Audit (Pre-Launch)

**Priority: Pre-launch.** Audit `"use client"` directives — push boundaries down to smallest leaf, convert parents to Server Components, lazy load modals/below-fold with `next/dynamic`.

**Known issues from dashboard page work (Sprint 3):**

- `app/(app)/my-recipes/page.tsx` uses `dynamic()` with `ssr: false` for all data-dependent components (UsageStats, WorkflowGrid, RecentExecutions). This is an anti-pattern — it means null render during SSR → loading fallback after hydration → skeleton → data (triple-jump). The page should be restructured: page.tsx as a Server Component composing small client leaves that each handle their own loading states. Only the Convex-dependent leaf components need `"use client"`.
- Skeleton dimensions were manually aligned to prevent layout shift but the root cause is the SSR gap from `ssr: false`. With proper Server Component structure, static parts (heading, tab list) render immediately in HTML, and only data-fetching leaves show skeletons.
- `AppShell.Content` needed `min-h-[80svh]` as a band-aid to prevent footer visibility during the SSR→hydration gap. This should become unnecessary once pages use proper Server Component composition.
- Same pattern likely exists on other `(app)` routes — audit all `dynamic({ ssr: false })` usage.

- [ ] `apps/web` — Inventory `"use client"` files, refactor candidates to Server Components
- [ ] `apps/web` — Restructure `my-recipes/page.tsx` — Server Component page with client leaf islands (eliminate `ssr: false` anti-pattern)
- [ ] `apps/web` — Audit all `dynamic({ ssr: false })` usage, replace with proper server/client composition
- [ ] `apps/web` — Eliminate barrel imports in client components, lazy load heavy components
- [ ] `apps/web` — Run Lighthouse / bundle analyzer before and after, confirm no regression

### Infra: Vercel Preview Deployment Verification

**Deferred from Sprint 2A Wave 5.** Verify auth flow end-to-end on Vercel preview deployment. Not blocking M1 browser execution.

- [ ] `apps/web` — Verify auth flow on Vercel preview deployment (cookie behavior, proxy redirects, sign-in/sign-out)

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

The Go engine supports recursive `Definition.Nodes`. The web app must preserve this composability. Guard rails (not new tasks — apply when building related features):

- Config panels must work at any nesting depth
- Execution progress must be recursive (group nodes show children's progress)
- JSON editor must represent recursive structure faithfully
- Visual editor (Sprint 4) must support drill-down into group nodes

### Bug: Input Node File Extension Filter Not Enforced

**Priority: High. Partially addressed by Sprint 5 Wave 2** (TagPicker control for extensions, hide `accept` param). The extension enforcement bug (file dialog not respecting `extensions`) still needs a separate fix.

The `extensions` parameter on the Input node (`@bnto/nodes/schemas/input.ts`) is not respected by the native file dialog — users can select files outside the allowed extensions (e.g., PDFs when only image extensions are configured). The `accept` attribute on the file input likely isn't being derived from the Input node's `extensions` param.

**Fix:** Trace how the Input node's `extensions` array flows to the actual `<input type="file" accept="...">` element. Ensure the file picker's `accept` attribute is built from the extensions list. Add regression tests so this doesn't break again.

**Tasks:**

- [ ] `packages/core` or `packages/editor` — `/frontend-engineer` — Trace the Input node `extensions` param to the file picker `accept` attribute. Fix the gap where extensions aren't being forwarded
- [ ] `packages/editor` or `apps/web` — `/frontend-engineer` — Add unit/integration tests verifying that Input node `extensions` are reflected in the file dialog filter
- [ ] `apps/web` — `/quality-engineer` — E2E test: configure Input node with image-only extensions, verify file picker restricts selection

**Note:** The `accept` (MIME types) param is also exposed but may not need to be surfaced to users — file extensions are the more intuitive control. Consider hiding `accept` from the config panel if `extensions` covers the use case. See related UX item below.

---

### UX: Input Node File Extension Control — Multi-Select — ABSORBED INTO SPRINT 5

**Absorbed into Sprint 5 Wave 2** (config panel controls). TagPicker UI primitive, `CONTROL_REGISTRY` wiring for `z.array(z.string())` → `tagPicker`, and hiding `accept` param are all Sprint 5 W2 tasks. See [config-controls.md](.claude/strategy/config-controls.md) for the full matrix.

---

### ~~Triage: Operation Select May Be Redundant in Config Panel~~ — ABSORBED INTO SPRINT 5 W2

**Absorbed into Sprint 5 Wave 2 Step 4** (schema metadata cleanup). Hide `operation` param on image, file-system, and spreadsheet nodes since it's pre-set from the palette.

### Home Page Marquee for Recipe Cards

**Priority: P3 — Post-Editor.** The home page hero has too many recipe cards pushing content below the fold. Use Magic UI's Marquee component to display recipe cards in a scrolling row, keeping the rest of the page above the fold. Reference: https://magicui.design/docs/components/marquee.md

### Full Codebase Quality Audit Post-Editor v1

**Priority: P3 — Post-Editor Gate.** After editor v1 ships, run a full codebase sweep: dead code removal (knip per package), code standards compliance, and a domain-by-domain audit where each persona skill verifies its area follows all rules. Cover all packages and apps. Good candidate for a dedicated sprint between editor v1 and M3 work.

### Triage: SelectTrigger missing press animation

**Priority: Triage.** The Select input trigger doesn't animate on click like Menu triggers do. SelectTrigger should have the same pressable spring effect as the Menu trigger component.

Files: `packages/ui/src/interaction/Select.tsx`, `packages/ui/src/interaction/Menu/MenuTrigger.tsx`

### Triage: PopupTrigger shared component

**Priority: Triage.** Menu, Select, and Combobox all trigger popups but have separate trigger styling. Create a shared PopupTrigger component that centralizes the pressable spring animation, surface treatment, and chevron icon behavior so all popup-triggering controls inherit consistent look and feel.

Files: `packages/ui/src/interaction/Menu/MenuTrigger.tsx`, `packages/ui/src/interaction/Select.tsx`, `packages/ui/src/interaction/Combobox.tsx`

### Triage: Remove sm/lg button sizes

**Priority: Triage.** Consider removing `sm` and `lg` size variants from Button, keeping only `md`. Emphasis and hierarchy would be controlled through elevation instead of size, making the system feel more consistent.

Files: `packages/ui/src/interaction/Button.tsx`, all consumers of `size="sm"` or `size="lg"`

---

## Reference

| Document                                     | Purpose                                                                                                           |
| -------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| `.claude/journeys/`                          | User journey test matrices — auth, engine, API, web app, editor                                                   |
| `.claude/strategy/bntos.md`                  | Predefined Bnto registry — slugs, fixtures, SEO targets, tiers                                                    |
| `.claude/strategy/editor-architecture.md`    | Shared editor layer — store, hooks, package strategy, switchable editors                                          |
| `.claude/strategy/editor-user-journey.md`    | Editor user journey — stages, flows, success criteria, phased delivery                                            |
| `.claude/strategy/visual-editor.md`          | Bento box visual editor — compartment design, grid layout, execution state                                        |
| `.claude/strategy/code-editor.md`            | Code editor design — CM6, slash commands, JSON Schema                                                             |
| `.claude/strategy/visual-editor.md`          | Bento box visual editor — compartment design, grid layout, execution state                                        |
| `.claude/strategy/go-engine-migration.md`    | Go engine node inventory — migration reference before archive deletion                                            |
| `.claude/strategy/cloud-desktop-strategy.md` | Architecture, cost analysis, cloud execution topology                                                             |
| `.claude/strategy/core-principles.md`        | Trust commitments, "For Claude Code" guidance                                                                     |
| `.claude/rules/`                             | Auto-loaded rules (architecture, code-standards, components, etc.)                                                |
| `.claude/skills/`                            | Agent skills (pickup, project-manager, code-review, pre-commit)                                                   |
| [.claude/business/](business/)               | Pricing strategy, revenue projections, SEO monetization, feature funnel, brand, personas, competitive positioning |
