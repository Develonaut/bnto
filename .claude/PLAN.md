# Bnto — Build Plan

**Last Updated:** February 28, 2026
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

**Co-location decision (Feb 2026):** UI components and editor features live in `apps/web` for now. No separate `@bnto/ui` or `@bnto/editor` packages until there's a real second consumer (desktop app). Engine, core API, and data layer logic stays in `@bnto/core`. When the UI package is extracted, it will be published as `@bnto/ui` (npm) under the name **Motorway** — the Mini Motorways-inspired design system (surface, elevation, pressable, spring animations, warm palette).

---

## Current State

- **Active:** Sprint 3 (Platform Features, M2) — Wave 1 complete, Wave 2 ready
- **Next:** Sprint 4 + 4B (Recipe Editors) — visual + code editors, parallel alongside Sprint 3
- **M1 delivered:** All 6 Tier 1 bntos run 100% client-side via Rust→WASM
- **Cloud pipeline:** Go API on Railway + R2 file transit — M4 infrastructure ready
- **WASM engine:** 5 Rust crates, single cdylib, 1.6MB raw / 606KB gzipped
- **Auth:** `@convex-dev/auth`. Anonymous sessions, integration tests complete
- **Packages:** `@bnto/core`, `@bnto/auth`, `@bnto/backend`, `@bnto/nodes`

---

## What's Built (don't redo)

- [x] Monorepo: Turborepo + pnpm + Taskfile.dev + go.work
- [x] Go engine + API: 10 node types, CLI, HTTP API on Railway — archived, ready for M4
- [x] @bnto/core: Layered singleton (clients → services → adapters), React Query + Convex adapter, 38+ hooks
- [x] @bnto/auth: `@convex-dev/auth` integration, anonymous sessions, conversion flow
- [x] @bnto/backend: Convex schema (users, workflows, executions, executionLogs), auth, crons, analytics fields
- [x] @bnto/nodes: Engine-agnostic node definitions, schemas, recipes, validation (10 node types)
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

---

## Revenue & Monetization Context

Pricing, revenue projections, and "ready to charge" criteria live in Notion ("SEO & Monetization Strategy").

**Monetization model (updated Feb 2026):** Browser execution is free unlimited. Pro sells real value — persistence, collaboration, premium compute. See ROADMAP.md for the full model.

| Sprint | What Ships | Revenue Implication |
|--------|-----------|---------------------|
| Sprint 2B | Browser execution (M1 MVP) | **All Tier 1 bntos run client-side.** Zero backend cost. Files never leave user's machine. |
| Sprint 2C | Launch readiness (content + domain) | **bnto.io live and indexable.** Real content on every page. SEO crawling begins. First real users possible. |
| Sprint 2D | Recipe page UX overhaul | **COMPLETE.** Progressive phase-driven flow. Motorway design language on every tool page. |
| Sprint H | Housekeeping | **COMPLETE.** FileUpload rewrite, Rust test audit, EXIF coverage, Pressable, CI, ESLint. |
| Sprint 3 | Platform features (accounts, history) | Accounts exist. Conversion hooks scaffolded (Save, History). Usage analytics instrumented. |
| Sprint 4 | Recipe editor (headless + visual) | Power users self-identify. Create/customize recipes = highest-intent Pro signal. Free editor fosters community recipe ecosystem. |
| Sprint 5-6 | Desktop app | Top-of-funnel. Word of mouth begins. Free forever — trust signal. |
| Sprint 7 | Stripe + Pro tier | **First revenue possible.** Pro: $8/month for persistence, collaboration, server-side AI, priority processing. |

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

---

### Sprint 2D: Recipe Page UX Overhaul — COMPLETE
Progressive phase-driven flow (Files → Configure → Results) with Motorway design language. RecipeShell, PhaseIndicator, FileCard, RecipeConfigSection, useRecipeFlow, per-instance execution stores. 27+ screenshots regenerated. All 4 waves complete.

---

### Sprint H: Housekeeping — COMPLETE
Tech debt cleanup: FileUpload→react-dropzone, core.browser→core.wasm rename, shared ESLint config, Pressable component, React import sweep, GitHub Actions CI (PR #10), Rust test audit, EXIF orientation coverage. All tasks delivered.

---

### Sprint 3: Platform Features (M2)
**Goal:** Accounts earn their keep. Users who sign up get persistence, history, and a reason to stay. Conversion hooks are natural — Save, History, Server Nodes — not artificial run caps. See [pricing-model.md](strategy/pricing-model.md) for the full free vs premium framework.

**Persona ownership:**
| Package | Persona |
|---------|---------|
| `@bnto/backend` | `/backend-engineer` |
| `@bnto/core` | `/core-architect` |
| `apps/web` | `/frontend-engineer` |
| `infra` | No specific persona — general |

#### Pre-work — COMPLETE
Anonymous→password userId fix, FIXME cleanup, privacy policy rewrite, README review, Knip dead code audit (14 files, 11 deps), naming audit, codebase standards review (149 violations), schema analytics fields, site navigation E2E tests.

#### Wave 1 (parallel — core hooks + UI components + infra decisions)

- [x] `@bnto/core` — `/core-architect` — `useExecutionHistory()` hook (paginated, per-user)
- [x] `@bnto/core` — `/core-architect` — `useUsageAnalytics()` hook (total runs, most-used bntos, last activity)
- [x] `apps/web` — `/frontend-engineer` — RecipeCard component (name, node count, last run status, last updated)
- [x] `apps/web` — `/frontend-engineer` — StatusBadge component (pending, running, completed, failed)
- [x] `apps/web` — `/frontend-engineer` — EmptyState component (no workflows yet)
- [x] `infra` — **Analytics layer decision:** PostHog selected. Decision doc at `.claude/decisions/analytics.md`. Privacy policy updated to remove premature "no tracking" promises. Copy across FAQ, pricing, hero updated to be honest, not aspirational.
- [x] `@bnto/core` — `/core-architect` — **PostHog telemetry integration:** `core.telemetry` namespace (client → adapter), `TelemetryProvider` with config injection, E2E test hook via `window.__bnto_telemetry__`. Production-only env vars (Vercel). 2 E2E tests.
- [x] `infra` — **SEO validation tooling:** Lighthouse CI in GitHub Actions (advisory, all 10 public routes). `task seo:audit` for local audits. Google Search Console verified via Cloudflare DNS. `@lhci/cli` installed as dev dep.

#### Wave 2 (parallel — dashboard + conversion hooks)

- [ ] `apps/web` — `/frontend-engineer` — Dashboard page: saved workflows, recent executions, usage analytics
- [ ] `apps/web` — `/frontend-engineer` — Execution history page (list of past runs with status, re-run capability)
- [ ] `apps/web` — `/frontend-engineer` — **Save prompt** (conversion hook): "Want to keep this workflow? Sign up to save it." — appears after successful browser execution for anonymous users
- [ ] `apps/web` — `/frontend-engineer` — **History prompt** (conversion hook): "Sign up to access your execution history and re-run past workflows."
- [ ] `apps/web` — `/frontend-engineer` — **Browser auth behavior verification:** Token expiry, sign-out invalidation (moved from Sprint 2A Wave 5)
- [ ] `apps/web` — `/frontend-engineer` — Pricing page update: Pro sells persistence, collaboration, premium compute — not run limits
- [ ] `apps/web` — `/frontend-engineer` — **Data fetching & skeleton audit:** Scan all existing components in `apps/web/` for violations of the co-located query pattern, prop drilling, mismatched skeletons, missing skeletons, separate `*Skeleton.tsx` files (for simple cases), transforms outside `select`, and loading wrapper anti-patterns. Fix violations in-place. Reference: [data-fetching-strategy.md](strategy/data-fetching-strategy.md), [skeletons.md](rules/skeletons.md)

#### Wave 3 (sequential — test)

- [ ] `apps/web` — `/frontend-engineer` — Playwright E2E: save prompt appears after anonymous execution
- [ ] `apps/web` — `/frontend-engineer` — Playwright E2E: execution history page shows past runs for authenticated users
- [ ] `@bnto/backend` — `/backend-engineer` — Unit tests for execution analytics queries

---

### Sprint 4: Recipe Editor (Headless-First)
**Goal:** Users can create recipes from a blank canvas or customize existing ones — add/remove/configure nodes, run, and export as `.bnto.json`. The editor is free (pricing-model.md: "recipe editor is free"). Power users who create custom recipes are the highest-intent Pro upgrade candidates.

**Architecture: headless-first.** The editor is built as layers. Logic lives in pure functions, a state machine, and hooks — no visual dependency. The bento box visual (compartment cards on a grid) is one skin; the code editor (CodeMirror 6) is another. Both are views of the same `Definition` in the shared store. Users can switch between them on the fly. See [editor-architecture.md](.claude/strategy/editor-architecture.md) for the shared layer design and [visual-editor.md](.claude/strategy/visual-editor.md) for the bento box visual editor.

```
@bnto/nodes (types, schemas, validation)      ← already built
         ↓
Pure functions (definition CRUD, adapters)     ← Wave 1
         ↓
Editor store (Zustand — headless operations)   ← Wave 2
         ↓
React hooks (reactive bindings)                ← Wave 2
         ↓
Dumb components (BentoCanvas / CodeEditor)     ← Wave 3+
```

**Two entry points, same state:** `createBlankDefinition()` (empty bento box) or `loadRecipe(slug)` (pre-assembled recipe from `@bnto/nodes`). Both produce the same `EditorState` shape — same operations, same output, same visual.

**Prior art:** Atomiton's `createFieldsFromSchema` pattern. Define node parameter schemas once (`@bnto/nodes/schemas/`), auto-derive config panel UI. ~70-80% of fields need zero UI code. Already built in `@bnto/nodes` — schemas exist for all 10 node types with `visibleWhen`, `requiredWhen`, enum values, min/max, and defaults.

**What this is NOT:** Save to Convex (Sprint 3 prerequisite), execution history, workflow versioning, container node nesting (group/loop as visual sub-canvases), or the JSON/code editor (Sprint 4B — CodeMirror 6, shares the headless store but is a distinct coding-oriented experience with its own persona). Those layer on naturally once the headless foundation exists.

**Persona ownership:**
| Wave | Lead Persona | Supporting | Rationale |
|------|-------------|------------|-----------|
| Wave 1 | — (pure functions, no persona needed) | — | `@bnto/nodes` pure functions — framework-agnostic, no React or ReactFlow dependency |
| Wave 2 | `/reactflow-expert` | — | Zustand store wraps ReactFlow's change/apply pattern. Definition ↔ Flow adapters are the core seam. ReactFlow Expert owns all graph state management and adapter design |
| Wave 3 | `/reactflow-expert` + `/frontend-engineer` | — | ReactFlow Expert owns canvas interaction, connection validation, drag-and-drop. Frontend Engineer owns component composition (RecipeEditor, EditorToolbar, NodeConfigPanel, NodePalette), theming (Motorway tokens), and animation (Animate.* API) |
| Wave 4 | `/reactflow-expert` + `/frontend-engineer` | — | ReactFlow Expert maps execution state to node visual state on canvas. Frontend Engineer handles progress UI patterns and E2E test composition |

**Rule:** For ANY work touching ReactFlow APIs, graph state, canvas interaction, or the Definition ↔ Flow adapter layer — invoke `/reactflow-expert`. This persona is THE authority on `@xyflow/react` in this codebase. When visual skin work begins (Wave 3+), invoke BOTH `/reactflow-expert` AND `/frontend-engineer` together.

#### Wave 1 (parallel — headless definition operations)

Pure functions that manipulate `Definition` trees. No React, no store, no UI. Fully testable in isolation. These are the atomic operations the editor performs.

- [ ] `@bnto/nodes` — **`createBlankDefinition()`**: Returns a minimal valid `Definition` — root group node with one input port and one output port, no children. The "blank canvas" entry point.
- [ ] `@bnto/nodes` — **`addNode(definition, nodeType, position?)`**: Inserts a new child node into the root group with default parameters from the schema. Auto-generates unique ID, creates default ports from `NODE_TYPE_INFO`. Returns new `Definition` (immutable — never mutate).
- [ ] `@bnto/nodes` — **`removeNode(definition, nodeId)`**: Removes a node and all edges connected to it. Returns new `Definition`.
- [ ] `@bnto/nodes` — **`updateNodeParams(definition, nodeId, params)`**: Merges new parameter values into a node's `parameters` object. Validates against `NodeSchema` (type checks, required fields, enum values, min/max). Returns new `Definition` or validation errors.
- [ ] `@bnto/nodes` — **`moveNode(definition, nodeId, position)`**: Updates a node's `position`. Returns new `Definition`.
- [ ] `@bnto/nodes` — **`definitionToRecipe(definition, metadata?)`**: Wraps a `Definition` into a `Recipe` with slug, name, description, accept spec. For export.
- [ ] `@bnto/nodes` — **Unit tests for all CRUD operations**: Every function tested with all 10 node types. Edge cases: remove node (clean removal), update params with invalid values (validation errors), blank definition is valid, move node to new position, nested container operations.

#### Wave 2 (parallel — editor store + React hooks)

Zustand store that wraps the pure functions into a reactive state machine. Hooks provide the React binding layer. Still headless — no visual components. **`/reactflow-expert` leads** — owns the Definition ↔ Bento adapter design and Zustand store architecture.

- [ ] `apps/web` — **`useEditorStore` (Zustand)**: Editor state: `definition` (current `Definition`), `selectedNodeId`, `isDirty`, `validationErrors[]`, `executionState` (per-node status map). Actions: `loadRecipe(slug)`, `createBlank()`, `addNode(type)`, `removeNode(id)`, `selectNode(id)`, `updateParams(nodeId, params)`, `moveNode(...)`, `resetDirty()`. All actions delegate to Wave 1 pure functions. Undo/redo via history stack (store snapshots).
- [ ] `apps/web` — **`useEditorNode(nodeId)` hook**: Returns node data + schema + visible params (conditional visibility resolved). Subscribes to store slice — re-renders only when this node changes.
- [ ] `apps/web` — **`useNodePalette()` hook**: Returns available node types from `NODE_TYPE_INFO`, grouped by category, with `browserCapable` flags. Filters server-only nodes based on context (browser editor = browser-capable only).
- [ ] `apps/web` — **`useEditorExport()` hook**: Returns `{ exportAsRecipe, download }` — wraps current definition as a `Recipe` or triggers browser `.bnto.json` file download. Validates definition before export. Pure serialization — no visual dependency.
- [ ] `apps/web` — **Definition ↔ Bento adapters**: `definitionToBento(definition)` → `{ nodes: CompartmentNodeType[] }`. `bentoToDefinition(nodes)` → `Definition`. Pure functions that bridge the headless model to the visual layer. Map node types to compartment variants, positions, and sizes. No edges — execution order derived from compartment position. Unit tested — round-trip: `definition → bento → definition` produces equivalent output.
- [ ] `apps/web` — **Unit tests for store + hooks**: Store operations tested via Vitest (no rendering). Hook tests via `renderHook`. Adapter round-trip tests. Undo/redo verification.

#### Wave 3 (parallel — visual canvas integration)

Wire the headless store to the bento box canvas. The compartment grid becomes a live, interactive editor. **`/reactflow-expert` + `/frontend-engineer` co-lead.** ReactFlow Expert owns canvas interaction and node positioning. Frontend Engineer owns component composition, theming, and animation.

- [ ] `apps/web` — **`RecipeEditor` component**: Composes `EditorToolbar` + `BentoCanvas` + `NodeConfigPanel`. Reads from `useEditorStore`. Two entry modes: `<RecipeEditor slug="compress-images" />` (loads predefined) or `<RecipeEditor />` (blank canvas). Includes `EditorModeToggle` to switch between visual (bento) and code (CM6) editors.
- [ ] `apps/web` — **`EditorToolbar` component**: Action bar above canvas — recipe selector dropdown (all Tier 1 recipes + "Blank"), Add Node button (opens palette), Remove Selected button, Run button, Reset/Replay button, Export `.bnto.json` button, Undo/Redo buttons. Reads/dispatches to `useEditorStore`.
- [ ] `apps/web` — **`NodePalette` component**: Slide-out panel listing available node types from `useNodePalette()`. Click-to-add (adds compartment at next available slot). Grouped by category. Browser-capable badge. Server-only nodes shown grayed with "Pro" badge (visible but not addable in browser context — definitions always available per pricing model).
- [ ] `apps/web` — **`NodeConfigPanel` component**: Side panel that renders when a compartment is selected. Uses `useEditorNode(selectedNodeId)` to get schema + current params. Auto-generates form fields from `ParameterSchema` (Atomiton pattern): string → text input, number → number input with min/max, boolean → toggle, enum → select dropdown. `visibleWhen` and `requiredWhen` handled reactively. Parameter changes dispatch `updateParams` to store.
- [ ] `apps/web` — **Enable canvas interaction**: Upgrade `BentoCanvas` to accept an `interactive` prop. When `true`: `nodesDraggable={true}`, `elementsSelectable={true}`. Node drag updates position via `moveNode`. Selection dispatches `selectNode`. No edge connections — execution order is positional. When `false`: current read-only showcase behavior (backward compatible).
- [ ] `apps/web` — **Motorway debug section**: Replace the hardcoded `BentoBoxShowcase` in `/motorway` with `<RecipeEditor />`. The Motorway page becomes the editor playground — load recipes, add/remove compartments, configure parameters, run, export.

#### Wave 4 (parallel — execution + polish)

The editor runs recipes and shows execution state on the canvas. Compartments visually reflect processing progress — each lights up as it processes, then settles as the next activates. **`/reactflow-expert` + `/frontend-engineer` co-lead.**

- [ ] `apps/web` — **Execution integration**: Wire Run button to browser WASM execution path. When running: compartments show execution state (idle → pending → active → completed) via status-driven elevation changes and variant color shifts. Progress callbacks from `browserExecutionService` update `executionState` in editor store.
- [ ] `apps/web` — **Export `.bnto.json`**: Download button in toolbar that serializes current editor state to a valid `.bnto.json` file via `useEditorExport().download()`. Validates before export. Users can take their recipe anywhere — CLI, desktop, share with others.
- [ ] `@bnto/backend` — **Tag editor users**: When a user opens the editor, set `hasUsedEditor: true` on their user record. Highest-intent Pro upgrade candidates. Query: `ctx.db.query("users").withIndex("by_hasUsedEditor")`.
- [ ] `apps/web` — **E2E tests**: Load recipe → canvas renders matching compartments. Add node → compartment appears in next slot. Remove node → compartment removed, remaining reflow. Configure params → node updates. Export → valid `.bnto.json` file. Run → execution progress shown on compartments. Blank canvas → add nodes → build a recipe from scratch.

---

### Sprint 4B: Code Editor (CodeMirror 6)

**Goal:** A schema-aware `.bnto.json` code editor for power users — the coding-oriented counterpart to the visual canvas. Users who prefer code get the same power as the visual canvas, with the speed and precision of text editing. Slash commands bring Notion-like ergonomics. The code editor is free (same as the visual editor).

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

### Sprint 5: Desktop Bootstrap

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

### Sprint 6: Local Execution

**Persona ownership:** Same as Sprint 5 — `/frontend-engineer` (desktop UI), `/core-architect` (adapter), `/rust-expert` (engine).

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

**"Ready to charge" gate:** Before Sprint 7, confirm: real users running browser bntos, conversion hooks built and tested (Save, History, Premium), people return voluntarily, at least one server-side bnto (AI or shell) ready for Pro tier.

### Sprint 7: Stripe + Pro Tier (M5)

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

### ~~Sprint 8: Visual Editor + History~~

**ABSORBED into Sprint 4 (Recipe Editor).** Sprint 4 now covers the full visual editor: headless definition CRUD, Zustand store, ReactFlow canvas, node palette, property editor, and execution state visualization. The headless-first architecture means all visual editor work builds on the same pure-function foundation.

**Remaining items not yet in Sprint 4:**
- [ ] `apps/web` — Execution history with full per-node logs and re-run support (depends on Sprint 3 accounts/history)
- [ ] `apps/web` — Workflow versioning and duplication (depends on Sprint 3 save infrastructure)
- [ ] `apps/web` — Container node visual nesting (group/loop as collapsible sub-canvases — future enhancement)
- [ ] `apps/web` — Drag-and-drop from node palette to canvas position (Sprint 4 Wave 3 uses click-to-add; drag-and-drop is a polish pass)
- [x] `apps/web` — JSON/Code editor → **Promoted to Sprint 4B** (CodeMirror 6, 5 waves, own persona `/code-editor-expert`). See Sprint 4B above.

---

## Immediate Backlog

### Infra: Convex Production Deployment Pipeline

**Priority: High.** Convex production (`gregarious-donkey-712`) was never deployed — discovered when anonymous auth started failing on bnto.io with 400 errors. `npx convex deploy --yes` and `npx @convex-dev/auth` had to be run manually. There is no automatic mechanism to deploy Convex functions to production when code merges to `main`.

- [x] `infra` — Investigate options: GitHub Actions step on merge to main, Vercel build hook (`--cmd`), or manual deploy gate
- [x] `infra` — Implement chosen mechanism so Convex prod stays in sync with main
- [x] `.claude/rules/pre-commit.md` — Add Convex deploy reminder to the push/PR workflow if manual

### Infra: PostHog Events Not Appearing in Dashboard

**Priority: High.** Recipe telemetry events (`files_added`, `recipe_run_started`, `recipe_run_completed`, etc.) are wired and verified in E2E tests via `window.__bnto_telemetry__`, but events may not be reaching the PostHog dashboard when using bnto.io in production.

- [x] `apps/web` — Verify `NEXT_PUBLIC_POSTHOG_KEY` and `NEXT_PUBLIC_POSTHOG_HOST` are set correctly on Vercel production
- [x] `apps/web` — Check if PostHog `init()` is actually called in production (DNT check, env var presence)
- [x] `apps/web` — Inspect network tab on bnto.io for PostHog requests (`/i/v0/e/` or `/decide/`)
- [x] `apps/web` — Verify events appear in PostHog Live Events view after a real compress-images run

---

## Backlog

### UX: Compositional BouncyStagger Audit

**Priority: High.** Apply `BouncyStagger` compositionally (per-section opt-in) instead of wrapping entire `AppShell.Content` — the shell-level wrap caused a ~20px layout jump.
- [ ] `apps/web` — Audit all pages using `AppShell.Content` — identify sections that benefit from staggered entrance
- [ ] `apps/web` — Add `BouncyStagger` to card grids (home page BntoGallery, recipe card lists)
- [ ] `apps/web` — Add `BouncyStagger` to file card lists in RecipeShell (already done in phase flow)
- [ ] `apps/web` — Verify no layout shift on any page after compositional application
- [ ] `apps/web` — Update Motorway showcase (`PhaseFlowShowcase`) to demonstrate the compositional pattern

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

| Bucket | Prefix | Auto-delete after |
|---|---|---|
| `bnto-transit` + `bnto-transit-dev` | `uploads/` | 1 hour |
| `bnto-transit` + `bnto-transit-dev` | `executions/` | 24 hours |

- [ ] `infra` — Configure R2 lifecycle rules in Cloudflare dashboard (prod + dev buckets)

### Infra: Domain Setup (bnto.io Custom Domains)

Web app domain (`bnto.io`) promoted to Sprint 2C Wave 1. API domain (`api.bnto.io`) deferred to M4.

- [ ] `infra` — Connect `bnto.io` to Vercel + Cloudflare DNS, verify auth redirects
- [ ] `infra` — (M4) Add `api.bnto.io` CNAME → Railway, configure custom domain, update `GO_API_URL`

### Infra: Graduate SEO Validation from E2E to Unit Tests + Lighthouse CI

**Priority: Medium.** Graduate SEO validation from slow E2E to unit tests (metadata, registry↔sitemap sync) + Lighthouse CI (`seo: 90`). Keep thin E2E for noindex/redirect/404.

- [ ] `apps/web` — Move metadata validation to unit tests (`bntoRegistry.test.ts`)
- [ ] `apps/web` — Add Lighthouse CI with `seo: 90` threshold
- [ ] `apps/web` — Slim E2E to redirects + 404 + noindex only

### Testing: Standardize E2E Selectors on data-testid

Current E2E tests mix CSS classes, `getByRole`, `getByText`, and `data-testid`. Standardize on `data-testid` for state detection and element targeting. Keep semantic selectors for accessibility assertions.

- [ ] `apps/web` — Audit E2E specs, add `data-testid` attributes, update selectors

### Testing: Concurrent Quota Race Condition — M4/M5 (server-side quotas)

**Milestone: M4/M5.** Quota enforcement only applies to server-side bntos. Browser bntos are free unlimited. This race condition matters when server-side execution has limits.

- [ ] `@bnto/core` — Integration test: fire 2+ concurrent `startPredefined` calls for a user at limit-1 runs, verify at most 1 succeeds
- [ ] `@bnto/backend` — If race confirmed, investigate Convex transaction isolation guarantees or atomic increment patterns

### UX: Per-File Format Override for Convert Image Format

**Priority: Medium.** Per-file format override on `convert-image-format` FileCards. Touches UI (inline Select), state (per-file config map in Zustand), and engine (per-file config passthrough).

- [ ] `apps/web` — Per-file format override state + inline Select on FileCard
- [ ] `@bnto/core` — Update `browserExecute` for per-file config overrides
- [ ] `engine` — Verify Rust WASM supports per-file format params

### Testing: Monthly Run Reset Cycle — M4/M5 (server-side quotas)

**Milestone: M4/M5.** Run reset logic applies to server-side quota tracking. Browser bntos have no quotas.

- [ ] `@bnto/backend` — Unit test: seed user with `runsResetAt` in the past, call the reset mutation, verify `runsUsed` resets to 0 and `runsResetAt` advances to next month
- [ ] `@bnto/core` — Integration test (if feasible): verify reset behavior against real Convex dev

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

### UI: Extract Motorway Design System (`@bnto/ui`)

**Trigger: Desktop app (M3).** Extract `apps/web/components/ui/` → `packages/ui/` as `@bnto/ui` (branded **Motorway**). Zero domain knowledge, purely generic design system. Triggered when desktop creates a second consumer.

- [ ] `packages/ui` — Bootstrap package, move primitives + utility layer + CSS tokens
- [ ] `apps/web` — Update imports to `@bnto/ui`
- [ ] `apps/desktop` — Wire `@bnto/ui` as dependency

### Showcase: Radial Light Source Controls

**Priority: Low (fun polish).** Replace linear slider on `/showcase` with radial + elevation controls for light source direction/height.

- [ ] `apps/web` — `RadialSlider` generic UI component (circular drag input, configurable labels)
- [ ] `apps/web` — Light elevation control → `--light-elevation` CSS variable
- [ ] `apps/web` — Wire into surface shadow system, replace `LightSourceSlider` on showcase

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

**M2 (Sprint 3) for hook UX, M5 (Sprint 7) for Stripe.** Value-driven conversion hooks (Save, History, Premium Bntos, Team) — no "limit reached" messaging for browser bntos.

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


---

## Reference

| Document | Purpose |
|----------|---------|
| `.claude/journeys/` | User journey test matrices — auth, engine, API, web app |
| `.claude/strategy/bntos.md` | Predefined Bnto registry — slugs, fixtures, SEO targets, tiers |
| `.claude/strategy/editor-architecture.md` | Shared editor layer — store, hooks, package strategy, switchable editors |
| `.claude/strategy/visual-editor.md` | Bento box visual editor — compartment design, grid layout, execution state |
| `.claude/strategy/code-editor.md` | Code editor design — CM6, slash commands, JSON Schema |
| `.claude/strategy/conveyor-belt.md` | Conveyor belt showcase — Motorway page R&D (not the editor) |
| `.claude/strategy/cloud-desktop-strategy.md` | Architecture, cost analysis, cloud execution topology |
| `.claude/strategy/core-principles.md` | Trust commitments, "For Claude Code" guidance |
| `.claude/rules/` | Auto-loaded rules (architecture, code-standards, components, etc.) |
| `.claude/skills/` | Agent skills (pickup, project-manager, code-review, pre-commit) |
| Notion: "SEO & Monetization Strategy" | Pricing, revenue projections, quota limits |
