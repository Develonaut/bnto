# Bnto — Build Plan

**Last Updated:** March 5, 2026
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

- **FOCUS: Package extraction + code standards review before editor.** Sprint 4C (I/O Nodes) complete — all tasks merged (PR #102). Recipes are self-describing.
- **Next up:** (1) Sprint 4D — extract `@bnto/ui` (Motorway design system), (2) Sprint 4E — extract `@bnto/editor`, (3) Sprint 4F — code standards audit across all packages. Clean, well-structured code before leaning into the complex editor work.
- **Then:** Sprint 5 — editor to production (compartment redesign, `/editor` route, execution, save, E2E). This is the M2 completion path.
- **Uncommitted editor work:** 7 files modified on `main` (CanvasToolbar, FileMenu, NodeConfigPanel, EditorCanvas, EditorOverlay, useEditorCanvas, icons) — needs branch/PR before next sprint.
- **Tabled:** Sprint 4B (Code Editor) — unblocked but deferred until visual editor ships to production.
- **Tabled:** Sprint 3 remaining (3 E2E test tasks) — platform features are built and working, test coverage deferred to backlog.
- **Tabled:** `/my-recipes` dashboard — hidden from nav (March 2026). Brings no value without the editor. Will resurface when users have recipes worth saving.
- **Tabled:** Save button on recipe toolbar — removed (March 2026). No save infrastructure to connect to yet. Will return with editor + accounts.
- **M1 delivered:** All 6 Tier 1 bntos run 100% client-side via Rust→WASM
- **Cloud pipeline:** Go API on Railway + R2 file transit — M4 infrastructure ready
- **WASM engine:** 5 Rust crates, single cdylib, 1.6MB raw / 606KB gzipped
- **Auth:** `@convex-dev/auth`. Password auth, integration tests complete, E2E auth lifecycle verified (13/13 tests)
- **Infra:** GitHub Actions CI (Rust + TypeScript + CI Gate), automatic Convex production deploy on merge to main, Lighthouse CI on PRs, PostHog telemetry wired
- **Packages:** `@bnto/core`, `@bnto/auth`, `@bnto/backend`, `@bnto/nodes`

---

## What's Built (don't redo)

- [x] Monorepo: Turborepo + pnpm + Taskfile.dev + go.work
- [x] Go engine + API: 10 node types, CLI, HTTP API on Railway — archived, ready for M4
- [x] @bnto/core: Layered singleton (clients → services → adapters), React Query + Convex adapter, 38+ hooks
- [x] @bnto/auth: `@convex-dev/auth` integration, password auth
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
| Sprint 4D-4E | Package extraction (`@bnto/ui`, `@bnto/editor`) | Clean architecture. Packages ready for desktop (M3). |
| Sprint 4F | Code standards review | Technical hygiene. Every file conforms to updated standards. |
| Sprint 5 | Editor to production | **M2 completion.** Editor gives users a reason to create accounts. Save custom recipes = highest-intent Pro signal. |
| Sprint 6-7 | Desktop app | Top-of-funnel. Word of mouth begins. Free forever — trust signal. |
| Sprint 8 | Stripe + Pro tier | **First revenue possible.** Pro: $8/month for persistence, collaboration, server-side AI, priority processing. |

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

### Sprint 3A: Remove Anonymous User System — COMPLETE
Eliminated anonymous Convex session system across 5 waves (backend schema, core hooks, web components, auth E2E, docs cleanup). Auth is now binary: signed in or not. 13/13 auth E2E tests passing. All anonymous references removed from schema, code, and docs.

### Sprint 3: Platform Features (M2) — COMPLETE (Wave 3 tabled)
Accounts earn their keep: execution history (IndexedDB for unauth, Convex for auth), `/my-recipes` dashboard, PostHog telemetry, Lighthouse CI, save prompt conversion hook, pricing page, browser auth verification, execution history migration on signup. Wave 3 (3 E2E test tasks) tabled — see backlog "Testing: Sprint 3 Deferred E2E Tests."

---

### Sprint 4: Recipe Editor (Headless-First) — COMPLETE
Headless-first editor: Wave 1 (`@bnto/nodes` pure functions — CRUD, adapters, tests), Wave 2 (Zustand store, ReactFlow adapters, hooks), Wave 3 (Motorway MVP — BentoCanvas, EditorToolbar, NodePalette, NodeConfigPanel, RecipeEditor). Architecture: `@bnto/nodes` → pure functions → Zustand store → React hooks → visual skin. Two entry points: `createBlankDefinition()` or `loadRecipe(slug)`. See [editor-architecture.md](.claude/strategy/editor-architecture.md), [visual-editor.md](.claude/strategy/visual-editor.md).

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

### Sprint 4C: Input & Output Nodes — COMPLETE
Self-describing recipes via `input` and `output` node types (PR #102). 4 waves: Wave 1 (`@bnto/nodes` — I/O types, schemas, recipe updates, 22 tests), Wave 2 (`@bnto/core` adapter reads I/O nodes, editor store singleton constraints), Wave 3 (generic InputRenderer/OutputRenderer, I/O compartment rendering), Wave 4 (RecipeShell migration, per-slug I/O code deleted, E2E verified). See [io-nodes.md](.claude/strategy/io-nodes.md).

---

### Sprint 4D: Extract `@bnto/ui` (Motorway Design System)

**Goal:** Move all UI primitives, design tokens, and shared components from `apps/web/components/` to `packages/ui/` as `@bnto/ui`. Zero domain knowledge — pure visual building blocks. This establishes the package boundary before the editor ships, making the complex editor code easier to follow for both humans and agents.

**Persona ownership:** `/frontend-engineer` leads all waves. `/nextjs-expert` advises on import restructuring.

#### Wave 1 (parallel — package scaffold + primitives)

- [ ] `packages/ui` — **Bootstrap package**: Create `packages/ui/` with `package.json` (`@bnto/ui`), `tsconfig.json`, barrel export. Zero runtime deps except React + Tailwind
- [ ] `packages/ui` — **Move primitives**: Move `apps/web/components/primitives/` → `packages/ui/src/primitives/`. These are thin shadcn/Radix wrappers with zero domain knowledge
- [ ] `packages/ui` — **Move utility layer**: Move `cn.ts`, `create-cn.ts` → `packages/ui/src/utils/`. These are the styling foundation
- [ ] `packages/ui` — **Move CSS tokens**: Move design token definitions from `globals.css` into a consumable format. The `@theme inline` block and token variables stay in the app's CSS but reference `@bnto/ui` token values

#### Wave 2 (parallel — shared components)

- [ ] `packages/ui` — **Move layout components**: `Stack`, `Row`, `Grid`, `Container`, `PageLayout`, `BentoGrid` → `packages/ui/src/layout/`
- [ ] `packages/ui` — **Move typography components**: `Heading`, `Text`, `Badge`, `Label` → `packages/ui/src/typography/`
- [ ] `packages/ui` — **Move feedback components**: `Skeleton`, `LinearProgress`, `ToolbarProgress`, `Spinner` → `packages/ui/src/feedback/`
- [ ] `packages/ui` — **Move surface components**: `Card` (with surface/pressable system), `Separator` → `packages/ui/src/surface/`
- [ ] `packages/ui` — **Move interaction components**: `Button`, `Switch`, `Select`, `Slider`, `RadialSlider`, `RadioGroup`, `Checkbox` → `packages/ui/src/interaction/`
- [ ] `packages/ui` — **Move overlay components**: `Dialog`, `Sheet`, `Popover`, `DropdownMenu`, `Tooltip` → `packages/ui/src/overlay/`
- [ ] `packages/ui` — **Move animation components**: `Animate.*` composition API → `packages/ui/src/animation/`

#### Wave 3 (sequential — rewire + verify)

- [ ] `apps/web` — **Update all imports**: Replace `@/components/ui/` and `@/components/primitives/` imports with `@bnto/ui` across the entire web app
- [ ] `apps/web` — **Tailwind v4 source directive**: Add `@source "../../node_modules/@bnto/ui"` to `globals.css` so Tailwind scans the extracted package
- [ ] `apps/web` — **Verify**: `task ui:build`, `task ui:test`, `task e2e` all pass. No behavior changes — only import paths changed

---

### Sprint 4E: Extract `@bnto/editor`

**Goal:** Move all editor components from `apps/web/components/editor/` to `packages/editor/` as `@bnto/editor`. Editor depends on `@bnto/ui` + `@bnto/core` + `@bnto/nodes`. This separates the complex editor system from the web app shell.

**Prerequisite:** Sprint 4D complete (`@bnto/ui` extracted).

**Persona ownership:** `/frontend-engineer` + `/reactflow-expert` lead.

#### Wave 1 (parallel — package scaffold + move)

- [ ] `packages/editor` — **Bootstrap package**: Create `packages/editor/` with `package.json` (`@bnto/editor`), `tsconfig.json`. Dependencies: `@bnto/ui`, `@bnto/core`, `@bnto/nodes`, `@xyflow/react`
- [ ] `packages/editor` — **Move editor components**: Move `apps/web/components/editor/` → `packages/editor/src/`. This includes `RecipeEditor/`, `EditorPanel/`, `CanvasToolbar`, `NodeConfigPanel`, `NodePalette`, `CompartmentNode`, adapters, hooks, store, actions
- [ ] `packages/editor` — **Move editor store**: Ensure `useEditorStore` and all editor Zustand state lives in `@bnto/editor`

#### Wave 2 (sequential — rewire + verify)

- [ ] `apps/web` — **Update all editor imports**: Replace `@/components/editor/` imports with `@bnto/editor` across the web app
- [ ] `apps/web` — **Verify**: `task ui:build`, `task ui:test`, `task e2e` all pass. No behavior changes
- [ ] `apps/web` — **Tailwind source directive**: Add `@source "../../node_modules/@bnto/editor"` to `globals.css` if editor has its own Tailwind classes

---

### Sprint 4F: Code Standards Review

**Goal:** Audit all active code against updated `code-standards.md` (March 2026 tightened limits). Every file conforms to size limits, naming conventions, and composition patterns. Clean slate before the editor production sprint.

**Prerequisite:** Sprint 4E complete (packages extracted, imports stable).

**Persona ownership:** `/frontend-engineer` leads. Each package audit is an independent task.

#### Wave 1 (parallel — per-package audit)

- [ ] `packages/ui` — **File size + structure audit**: Scan for files over 100 lines. Split oversized files. One export per file. Folder + barrel where earned. Verify dot-notation compliance
- [ ] `packages/editor` — **File size + structure audit**: Same scan. Editor files are likely the biggest offenders — split aggressively. Verify actions pattern (pure functions for state mutations, thin hook wrappers)
- [ ] `packages/core` — **File size + structure audit**: Oversized adapters, services, clients. One function per file in utils. Verify `select` rule on all `useQuery` calls
- [ ] `packages/@bnto/backend` — **File size + structure audit**: Convex function files, helpers, validators
- [ ] `packages/@bnto/nodes` — **File size + structure audit**: Schema files, registry, helpers
- [ ] `apps/web` — **File size + structure audit**: Pages, route components, lib/. Focus on what remains after UI/editor extraction

#### Wave 2 (sequential — cross-cutting + verify)

- [ ] `all packages` — **Cross-package DRY audit**: Identify duplicated utilities across packages (`cn`, `createCn`, type guards, format helpers). Consolidate into `@bnto/ui` (styling utils) or `@bnto/core` (logic utils) as appropriate
- [ ] `all packages` — **Dot-notation compliance sweep**: Every multi-part component uses dot-notation. Migrate any remaining flat imports
- [ ] `all packages` — **Verify**: `task ui:build`, `task ui:test`, `task e2e` all pass after all restructuring

---

### Sprint 5: Editor to Production (M2 Completion)

**Goal:** Ship the editor as a real product surface. Users can access `/editor`, build recipes from scratch or customize predefined ones, run them, see results, and save to their account. This is the M2 completion path — the editor gives users a reason to create accounts.

**Prerequisite:** Sprint 4F (Code Standards Review) complete. Packages extracted (`@bnto/ui`, `@bnto/editor`), code clean. Uncommitted editor work on `main` must be branched/PR'd first.

**Persona ownership:**
| Wave | Lead Persona | Supporting | Rationale |
|------|-------------|------------|-----------|
| Wave 1 | `/frontend-engineer` | `/reactflow-expert` | Node visual identity, compartment redesign |
| Wave 2 | `/frontend-engineer` | — | Production route, entry points, navigation |
| Wave 3 | `/frontend-engineer` | `/reactflow-expert` | Execution integration, elevation-driven progress |
| Wave 4 | `/backend-engineer` + `/core-architect` + `/frontend-engineer` | — | Save infrastructure, My Recipes integration |
| Wave 5 | `/quality-engineer` + `/frontend-engineer` | — | E2E tests, keyboard shortcuts, polish |

#### Wave 1 (parallel — Compartment Node Visual Redesign Phase 1)

Make every node immediately identifiable at a glance. Icon registry + category color mapping. This is the highest-impact visual improvement — transforms the editor from prototype to product.

- [ ] `@bnto/editor` — **Icon registry**: Create `adapters/nodeIcons.ts` — maps `NodeTypeName` to Lucide icon component. Pure data, one file. Icons: image=`ImageIcon`, spreadsheet=`Table`, file-system=`FolderOpen`, transform=`Shuffle`, edit-fields=`PenLine`, http-request=`Globe`, shell-command=`TerminalSquare`, group=`Braces`, loop=`RefreshCw`, parallel=`Columns3`, input=`Upload`, output=`Download`
- [ ] `@bnto/editor` — **Category color registry**: Create `adapters/nodeColors.ts` — maps `NodeCategory` to `CompartmentVariant`. Pure data, one file. image=primary, spreadsheet=secondary, file=accent, data=muted, network=secondary, control=warning, system=muted, io=info
- [ ] `@bnto/editor` — **CompartmentNode redesign**: Update `CompartmentNode.tsx` — add large icon (32px) above label, restructure from centered-text to icon-above-text layout. Import from icon/color registries. Category-driven variant color
- [ ] `@bnto/editor` — **Adapter integration**: Update `definitionToBento` adapter to use icon/color registries when converting Definition to BentoNode (set variant from category, set size from tier)

#### Wave 2 (parallel — Production Route + Entry)

Create the `/editor` route and wire entry points from recipe pages and navigation.

- [ ] `apps/web` — `/frontend-engineer` — Create `/editor` route with AccountGate (sign-in prompt for unauthenticated users)
- [ ] `apps/web` — `/frontend-engineer` — `?from={slug}` query param loads predefined recipe from `@bnto/nodes` registry
- [ ] `apps/web` — `/frontend-engineer` — Auto-scaffold Input + Output compartments for blank canvas (default when no `?from=`)
- [ ] `apps/web` — `/frontend-engineer` — Add `/editor` to app navigation (authenticated users only)
- [ ] `apps/web` — `/frontend-engineer` — "Open in Editor" bridge button on recipe pages → `/editor?from={slug}`

#### Wave 3 (sequential — Execution Integration)

Wire the Run button to browser WASM execution. Elevation-driven progress on compartments. Results routed to Output node.

- [ ] `@bnto/editor` — `/frontend-engineer` — Wire Run button → `core.executions.createExecution()` → browser WASM engine
- [ ] `@bnto/editor` — `/reactflow-expert` — Elevation-driven progress: compartments pop as nodes execute (idle → active → completed). Leverage existing Card spring animations
- [ ] `@bnto/editor` — `/frontend-engineer` — Results routed to Output node config panel (download list)
- [ ] `@bnto/editor` — `/frontend-engineer` — Auto-download toggle on Output node
- [ ] `@bnto/editor` — `/frontend-engineer` — Reset/re-run flow (clear results, re-execute)
- [ ] `@bnto/editor` — `/frontend-engineer` — Error states on individual compartments (node failure → destructive variant)

#### Wave 4 (parallel — Save + Bridge)

Convex persistence for custom recipes. My Recipes integration. This is the M2 conversion moment — users create recipes, want to save them, create accounts.

- [ ] `@bnto/backend` — `/backend-engineer` — Recipe save mutation (Convex schema: recipes table with userId, definition, metadata)
- [ ] `@bnto/core` — `/core-architect` — `core.recipes.save()` and `core.recipes.useMyRecipes()` hooks
- [ ] `apps/web` — `/frontend-engineer` — Save button in editor toolbar, tier limits (Free: 3 recipes, Pro: unlimited)
- [ ] `apps/web` — `/frontend-engineer` — My Recipes integration (load saved recipes into editor)
- [ ] `apps/web` — `/frontend-engineer` — Dirty state tracking + unsaved changes warning on navigation

#### Wave 5 (sequential — E2E + Polish)

End-to-end verification and keyboard shortcuts. See [journeys/editor.md](.claude/journeys/editor.md) for the full test matrix.

- [ ] `apps/web` — `/quality-engineer` — E2E test suite for editor entry + build + execute + export flows
- [ ] `apps/web` — `/quality-engineer` — Predefined recipe parity tests (all 6 recipes via `?from={slug}`)
- [ ] `apps/web` — `/frontend-engineer` — Keyboard shortcuts: Cmd-Z (undo), Cmd-Shift-Z (redo), Delete (remove), Cmd-Enter (run), Cmd-S (export)
- [ ] `apps/web` — `/quality-engineer` — Round-trip fidelity test (export → re-import → deep equality)
- [ ] `apps/web` — `/frontend-engineer` — Accessibility audit (focus management, screen reader labels on canvas nodes)

---

## Phase 2: Desktop App (Local Execution)

**Goal:** Free desktop app. Same React frontend, local engine execution. Free forever, unlimited runs. No account needed. Trust signal and top-of-funnel growth driver.

**Desktop tech: Tauri (Rust-native).** M1 Rust evaluation passed — one codebase for browser WASM + desktop native + CLI.

### Sprint 6: Desktop Bootstrap

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

### Sprint 7: Local Execution

**Persona ownership:** Same as Sprint 6 — `/frontend-engineer` (desktop UI), `/core-architect` (adapter), `/rust-expert` (engine).

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

**"Ready to charge" gate:** Before Sprint 8, confirm: real users running browser bntos, conversion hooks built and tested (Save, History, Premium), people return voluntarily, at least one server-side bnto (AI or shell) ready for Pro tier.

### Sprint 8: Stripe + Pro Tier (M5)

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

### ~~Old Sprint 8: Visual Editor + History~~

**ABSORBED into Sprint 4 (Recipe Editor).** Sprint 4 now covers the full visual editor: headless definition CRUD, Zustand store, ReactFlow canvas, node palette, property editor, and execution state visualization. The headless-first architecture means all visual editor work builds on the same pure-function foundation.

**Remaining items not yet in Sprint 4:**
- [ ] `apps/web` — Execution history with full per-node logs and re-run support (depends on Sprint 3 accounts/history)
- [ ] `apps/web` — Workflow versioning and duplication (depends on Sprint 3 save infrastructure)
- [ ] `apps/web` — Container node visual nesting (group/loop as collapsible sub-canvases — future enhancement)
- [ ] `apps/web` — Drag-and-drop from node palette to canvas position (Sprint 4 Wave 3 uses click-to-add; drag-and-drop is a polish pass)
- [x] `apps/web` — JSON/Code editor → **Promoted to Sprint 4B** (CodeMirror 6, 5 waves, own persona `/code-editor-expert`). See Sprint 4B above.

---

## Immediate Backlog

*All immediate items complete — Convex deploy pipeline and PostHog events resolved. See completed sprints.*

---

## Backlog

### UX: Compartment Node Visual Redesign — Phases 2-3 (Mini Motorways Buildings)

**Phase 1 promoted to Sprint 5 Wave 1** (icon registry + category color mapping). Phases 2-3 remain in backlog as polish.

**Phase 2: Elevation-driven execution states**

Replace the current flat status handling with elevation transitions that make compartments physically pop as they progress. The Card `.surface` system already provides springy elevation changes — we just need to map states correctly.

| State | Elevation | Visual effect |
|---|---|---|
| `idle` | `none` or `sm` | Flat/barely lifted — resting in the bento box |
| `pending` | `sm` | Slight lift, muted appearance — waiting in queue |
| `active` | `md` | Rising up — "being serviced" like a MM building |
| `completed` | `lg` | Full pop — satisfying spring bounce to max elevation |

The spring animation on Card elevation changes creates the Mini Motorways "building materializing" feel automatically. As the recipe runs, compartments pop up one by one in sequence — like buildings appearing on the map.

**Phase 3: Bento grid layout**

Replace the current horizontal strip (all nodes in a single row at 220px stride) with a proper bento box grid that uses varied compartment sizes. Different node types get different footprints:

| Tier | Size | Used for |
|---|---|---|
| **Standard** | 140×140 | Most nodes (image, spreadsheet, transform, etc.) |
| **Compact** | 100×100 | Simple nodes (edit-fields with no parameters) |
| **Wide** | 200×140 | Nodes with more visual content (future inline controls) |
| **Container** | 240×180+ | Group, loop, parallel — larger to suggest they hold children |

The grid layout algorithm should pack compartments like a real bento box — no uniform grid, but a visually balanced arrangement. Update `bentoSlots.ts` to support varied slot sizes.

**Future (not in scope):**
- Inline micro-controls on nodes (radial dials, parameter badges) — nice-to-have after core visual identity ships
- Interactive connection handles — design decision is no edges
- Per-node execution progress bars — elevation + status color is sufficient

**Tasks:**
- [ ] `apps/web` — **Icon registry**: Create `editor/adapters/nodeIcons.ts` — maps `NodeTypeName → LucideIcon`. Pure data, one file
- [ ] `apps/web` — **Category color registry**: Create `editor/adapters/nodeColors.ts` — maps `NodeCategory → CompartmentVariant`. Pure data, one file
- [ ] `apps/web` — **CompartmentNode redesign**: Update `CompartmentNode.tsx` — add icon rendering above label, restructure layout from centered-text to icon-above-text. Import from icon/color registries
- [ ] `apps/web` — **Elevation state mapping**: Update `CompartmentNode.tsx` status → elevation mapping: idle=none/sm, pending=sm, active=md, completed=lg. Leverage existing Card spring animations
- [ ] `apps/web` — **Bento grid layout**: Update `bentoSlots.ts` with varied slot sizes per node type tier (standard/compact/wide/container). Replace horizontal strip with proper 2D bento packing
- [ ] `apps/web` — **Adapter integration**: Update `definitionToBento` adapter to use icon/color registries when converting Definition → BentoNode (set variant from category, set size from tier)
- [ ] `apps/web` — **Motorway showcase**: Update Motorway editor showcase to demonstrate the new visual treatment with all node types visible
- [ ] `apps/web` — **E2E verification**: Verify editor canvas renders correctly with new node visuals. Update screenshots if page-level layout changed

### ~~UX: Editor User Journey — Full Implementation~~ — PROMOTED TO SPRINT 5

**Absorbed into Sprint 5: Editor to Production (March 2026).** All waves promoted. Wave 2 (I/O Nodes) delivered by Sprint 4C (PR #102). See Sprint 5 for the active task list.

**Strategy doc:** [editor-user-journey.md](.claude/strategy/editor-user-journey.md)
**E2E test matrix:** [journeys/editor.md](.claude/journeys/editor.md)

**Success criteria (carried to Sprint 5):**
1. Task completion — build compress-images from scratch, run it, download results, < 5 min
2. Round-trip fidelity — export `.bnto.json` → re-import → identical state
3. Predefined recipe parity — editor is a superset of recipe pages

### ~~Chore: Codebase File Size & Structure Audit~~ — PROMOTED TO SPRINT 4F

**Promoted (March 2026).** Core audit scope absorbed into Sprint 4F (Code Standards Review). The `@bnto/utils` package idea below remains in backlog — Sprint 4F consolidates shared utils into `@bnto/ui` (styling) or `@bnto/core` (logic) instead of creating a separate package.

**Goal:** Every file in the active codebase (`apps/web/`, `packages/core/`, `packages/@bnto/`) conforms to the updated size limits in `code-standards.md`. Directory structure reads like a table of contents — each file named after what it does, co-located tests where applicable.

**Approach:** Audit by package, one PR per package. Don't change behavior — only restructure. Tests must pass before and after.

**Cross-package DRY rule:** Any utility function used by more than one package (`cn`, `createCn`, type guards, format helpers, etc.) must move to a shared `@bnto/utils` package. Duplicated logic across packages is a signal — consolidate into `@bnto/utils` with one function per file, domain-grouped folders.

**Tasks:**
- [ ] `packages/@bnto/utils` — **Create shared utils package**: Scaffold `@bnto/utils` with `package.json`, `tsconfig.json`, barrel export. Zero runtime deps. This is the home for cross-package pure functions
- [ ] `packages/@bnto/utils` — **Identify and consolidate cross-package utils**: Scan all packages for duplicated or shared utilities (`cn`, `createCn`, type guards, formatters, validators). Move to `@bnto/utils`, update imports across consumers
- [ ] `apps/web` — **Scan for oversized files**: Find all `.ts`/`.tsx` files over 100 lines. Triage each: split into folder + barrel, extract functions to own files, or justify as-is (hard cap 250)
- [ ] `apps/web` — **Component decomposition audit**: Find components with more than 2-3 sub-components defined in one file. Break into folder structure (`FeatureRoot.tsx`, sub-parts, `index.ts` barrel)
- [ ] `apps/web` — **Utils/lib audit**: Find any multi-function files in `lib/`, `utils/`, or similar. Each exported function gets its own camelCase file. Group in domain folders. Move cross-package utils to `@bnto/utils`
- [ ] `packages/core` — **Same audit**: Oversized files, multi-function modules, sub-component cohabitation. Split and barrel. Move shared utils to `@bnto/utils`
- [ ] `packages/@bnto/backend` — **Same audit**: Convex function files, helpers, validators. One function per file where practical
- [ ] `packages/@bnto/nodes` — **Same audit**: Schema files, registry, helpers
- [ ] **Verify**: `task ui:build`, `task ui:test`, `task e2e` all pass after restructuring. No behavior changes — only file organization

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
    error.stack ? `\n<details><summary>Stack trace</summary>\n\n\`\`\`\n${error.stack.split("\n").slice(0, 8).join("\n")}\n\`\`\`\n</details>` : "",
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

### UX: Compositional BouncyStagger Audit

**Priority: High.** Apply `BouncyStagger` compositionally (per-section opt-in) instead of wrapping entire `AppShell.Content` — the shell-level wrap caused a ~20px layout jump.
- [x] `apps/web` — Audit all pages using `AppShell.Content` — identify sections that benefit from staggered entrance
- [x] `apps/web` — Add `BouncyStagger` to card grids (home page BntoGallery, recipe card lists)
- [x] `apps/web` — Add `BouncyStagger` to file card lists in RecipeShell (already done in phase flow)
- [x] `apps/web` — Verify no layout shift on any page after compositional application
- [x] `apps/web` — Update Motorway showcase (`PhaseFlowShowcase`) to demonstrate the compositional pattern

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
- [ ] `.claude/` — **Update docs**: Remove Go engine references from CLAUDE.md, architecture.md, monorepo-structure.md, ROADMAP.md. Update "What's Built" section in PLAN.md
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

| Bucket | Prefix | Auto-delete after |
|---|---|---|
| `bnto-transit` + `bnto-transit-dev` | `uploads/` | 1 hour |
| `bnto-transit` + `bnto-transit-dev` | `executions/` | 24 hours |

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

**Milestone: M4/M5 (Sprint 8+).** Quota enforcement only applies to server-side bntos. Browser bntos are free unlimited. This race condition matters when server-side execution has limits.

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

### ~~UI: Extract Motorway Design System~~ — PROMOTED TO SPRINT 4D/4E

**Promoted (March 2026).** Package extraction moved from backlog to active sprints (4D + 4E). Extraction happens before editor production sprint to establish clean package boundaries. See Sprint 4D (`@bnto/ui`) and Sprint 4E (`@bnto/editor`) for the full task lists.

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
| `.claude/journeys/` | User journey test matrices — auth, engine, API, web app, editor |
| `.claude/strategy/bntos.md` | Predefined Bnto registry — slugs, fixtures, SEO targets, tiers |
| `.claude/strategy/editor-architecture.md` | Shared editor layer — store, hooks, package strategy, switchable editors |
| `.claude/strategy/editor-user-journey.md` | Editor user journey — stages, flows, success criteria, phased delivery |
| `.claude/strategy/visual-editor.md` | Bento box visual editor — compartment design, grid layout, execution state |
| `.claude/strategy/code-editor.md` | Code editor design — CM6, slash commands, JSON Schema |
| `.claude/strategy/conveyor-belt.md` | Conveyor belt showcase — Motorway page R&D (not the editor) |
| `.claude/strategy/go-engine-migration.md` | Go engine node inventory — migration reference before archive deletion |
| `.claude/strategy/cloud-desktop-strategy.md` | Architecture, cost analysis, cloud execution topology |
| `.claude/strategy/core-principles.md` | Trust commitments, "For Claude Code" guidance |
| `.claude/rules/` | Auto-loaded rules (architecture, code-standards, components, etc.) |
| `.claude/skills/` | Agent skills (pickup, project-manager, code-review, pre-commit) |
| Notion: "SEO & Monetization Strategy" | Pricing, revenue projections, quota limits |
