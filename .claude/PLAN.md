# Bnto — Build Plan

**Last Updated:** March 6, 2026
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

- **FOCUS: Editor to production.** Sprint 5 Waves 1-2 complete (compartment redesign, `/editor` route, nav integration). Sprint 4G complete (format versioning, Zod schemas, schema-driven config panel).
- **Active work — execution order:**
  1. **Sprint 5A Wave 1** — finish exit animation (isIoNode, hover delete, placeholder DONE — exit animation remains)
  2. ~~**Sprint 4H**~~ **COMPLETE** ‖ **Sprint 5B** — 5B: visual hierarchy (unblocked, no shared dependencies)
  3. **Sprint 5 Wave 3** — execution wiring (Run → `executePipeline` → WASM → elevation). **Unblocked** — Sprint 4H complete.
  4. **Sprint 5A Waves 2–5** — config panel identity, LayerPanel reorder, auto-behaviors, E2E
  5. **Sprint 5C** — copy + nav label cleanup (~30 min)
  6. **Sprint 6** — Edit Mode ↔ Run Mode (Mini Motorways pattern)
  7. **Sprint 5 Waves 4–5** — save infrastructure, My Recipes, final E2E
- **Tabled:** Sprint 4B (Code Editor) — unblocked but deferred until visual editor ships to production.
- **Tabled:** Sprint 3 remaining (3 E2E test tasks) — platform features are built and working, test coverage deferred to backlog.
- **Tabled:** `/my-recipes` dashboard — hidden from nav (March 2026). Brings no value without the editor. Will resurface when users have recipes worth saving.
- **Tabled:** Save button on recipe toolbar — removed (March 2026). No save infrastructure to connect to yet. Will return with editor + accounts.
- **M1 delivered:** All 6 Tier 1 bntos run 100% client-side via Rust→WASM
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

---

## Revenue & Monetization Context

Pricing, revenue projections, and "ready to charge" criteria live in Notion ("SEO & Monetization Strategy").

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
| Sprint 5     | Editor to production                         | **M2 completion.** Editor gives users a reason to create accounts. Save custom recipes = highest-intent Pro signal.              |
| Sprint 5A-5C | Editor UX polish                             | Prototype → product. Visual hierarchy, interaction patterns, copy.                                                               |
| Sprint 6     | Edit/Run mode                                | Mini Motorways feel. Same canvas for editing and running.                                                                        |
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

---

## Active Sprints

### Sprint 5: Editor to Production (M2 Completion)

**Goal:** Ship the editor as a real product surface. Users can access `/editor`, build recipes from scratch or customize predefined ones, run them, see results, and save to their account. This is the M2 completion path — the editor gives users a reason to create accounts.

**Persona ownership:**
| Wave | Lead Persona | Supporting | Rationale |
|------|-------------|------------|-----------|
| Wave 1 | `/frontend-engineer` | `/reactflow-expert` | Node visual identity, compartment redesign |
| Wave 2 | `/frontend-engineer` | — | Production route, entry points, navigation |
| Wave 3 | `/frontend-engineer` | `/reactflow-expert` | Execution integration, elevation-driven progress |
| Wave 4 | `/backend-engineer` + `/core-architect` + `/frontend-engineer` | — | Save infrastructure, My Recipes integration |
| Wave 5 | `/quality-engineer` + `/frontend-engineer` | — | E2E tests, keyboard shortcuts, polish |

#### Wave 1 (parallel — Compartment Node Visual Redesign Phase 1) — COMPLETE

- [x] `@bnto/editor` — **Icon registry**: `adapters/nodeIcons.ts` — maps `NodeTypeName` to Lucide icon component
- [x] `@bnto/editor` — **Category color registry**: `adapters/nodeColors.ts` — maps `NodeCategory` to `CompartmentVariant`
- [x] `@bnto/editor` — **CompartmentNode redesign**: Icon-above-text layout, category-driven variant color
- [x] `@bnto/editor` — **Adapter integration**: `definitionToBento` uses icon/color registries

#### Wave 2 (parallel — Production Route + Entry) — COMPLETE

- [x] `apps/web` — `/editor` route (public, no auth gate)
- [x] `apps/web` — `?from={slug}` query param loads predefined recipe
- [x] `apps/web` — Auto-scaffold Input + Output compartments for blank canvas
- [x] `apps/web` — Add `/editor` to app navigation
- [x] `apps/web` — "Open in Editor" bridge button on recipe pages

#### Wave 3 (sequential — Execution Integration)

> **Execution order note:** Do Sprint 5A Wave 1 first (isIoNode flag + hover delete + placeholder). Then come back here for execution wiring.

> ⚠️ **Architecture rule:** Do NOT call `BntoWorker.processFiles()` directly for multi-file execution. That method is a temporary placeholder in the wrong layer. The editor MUST wire execution through a runtime-agnostic `executePipeline()` function (see `decisions/implicit-iteration.md`). If `executePipeline()` doesn't exist yet in `@bnto/core`, create it as part of this wave before wiring the Run button. The browser worker is injected as `runNode` — the loop logic stays in the executor, not in the adapter.

Wire the Run button to browser WASM execution. Elevation-driven progress on compartments. Results routed to Output node.

- ~~`@bnto/core` — Extract pipeline executor~~ — **Completed by Sprint 4H.** `executePipeline()` exists and is exported from `@bnto/core` before this wave starts.
- [ ] `@bnto/editor` — `/frontend-engineer` — Wire Run button → `executePipeline()` → browser WASM engine. Import `executePipeline` and `NodeRunner` from `@bnto/core`. Build `NodeRunner` from `core.executions` browser adapter's `processFile`. Do NOT call `BntoWorker.processFiles()` directly.
- [ ] `@bnto/editor` — `/reactflow-expert` — Elevation-driven progress: compartments pop as nodes execute (idle → active → completed). Leverage existing Card spring animations
- [ ] `@bnto/editor` — `/frontend-engineer` — Results routed to Output node config panel (download list)
- [ ] `@bnto/editor` — `/frontend-engineer` — Auto-download toggle on Output node
- [ ] `@bnto/editor` — `/frontend-engineer` — Reset/re-run flow (clear results, re-execute)
- [ ] `@bnto/editor` — `/frontend-engineer` — Error states on individual compartments (node failure → destructive variant)

**Execution boundary contract:** The execution path MUST call `validateDefinition()` (which includes per-node parameter validation via Zod) before invoking the WASM engine. If validation fails → reject with errors, surface in the UI, no engine invocation.

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

### Sprint 5A: Editor UX — Node Interaction + Empty State + Config Polish

**Goal:** Close the felt gap between prototype and product. Controls live where they belong: on node (delete), in LayerPanel (reorder), in config panel (configure). Empty canvas entry gives an immediate, clear signal of what to do next.

**Prerequisite:** Sprint 5 Waves 1–2 complete (route + entry points exist, compartment nodes live).

**Persona ownership:** `/frontend-engineer` leads all waves.

**Design constraints (non-negotiable):**

- Keep `Pressable/Card` as base node primitive — NOT Surface
- Whole card surface is the click target (select + open config)
- No edges, no arrow-reorder buttons on nodes
- I/O nodes protected (no delete, no removal from toolbar)

> ⚠️ **Animation rule — read before touching `CompartmentNode.tsx`:** The `<ScaleIn from={0.7} easing="spring-bouncy">` wrapper on every `CompartmentNode` is the Mini Motorways building pop-in. It is the single most important animation in the editor and a core part of the brand identity. Do not remove it, replace it with `FadeIn`, or flatten the spring curve. See `strategy/design-language.md` § "Editor Animation Language" for full rationale. If you refactor `CompartmentNode.tsx` for any reason, verify `ScaleIn` survives.

#### Wave 1 — Node Hover Overlay + Placeholder Slot

> **Partially complete.** Three of four implementation tasks are done (PR #120). The exit animation remains — it requires adding `motion/react` to `@bnto/editor` dependencies first.

- [x] `packages/editor` — **`CompartmentNode` hover overlay**: `DeleteOverlay` component with `group-hover:opacity-100`, `stopPropagation`, destructive variant. Hidden when `data.isIoNode === true`.
- [x] `packages/editor` — **`PlaceholderSlot` component**: Dashed muted card with centered Plus button. Opens palette via `useEditorPanels`. Disappears when a non-I/O node exists.
- [x] `packages/editor` — **`isIoNode` flag in adapter**: `createCompartmentNode.ts:72` and `definitionToBento.ts:48` both set `isIoNode: isIoNodeType(type)`. Tests at `createCompartmentNode.test.ts:85-97` and `definitionToBento.test.ts:167-183`.
- [ ] **CLAIMED** `packages/editor` — **Add `react-animate-presence` + `tailwindcss-motion` dependencies**: Lightweight CSS-first exit animations. `react-animate-presence` (~1-2 KB) delays DOM removal until exit animation completes. `tailwindcss-motion` provides exit utility classes. No need for full `motion/react` (18 KB).
- [ ] **CLAIMED** `packages/editor` — **Node exit animation**: Use `react-animate-presence` to wrap compartment nodes so deleted nodes exit with a spring scale-down (reverse of the entrance). Exit classes: `motion-scale-out-75 motion-opacity-out-0` with spring easing. Nodes must not disappear instantly — the exit spring mirrors the entrance spring and is equally important to the feel.
- [ ] **CLAIMED** `packages/editor` — **Unit tests for remaining work**: Exit animation triggers on node removal. `PlaceholderSlot` renders when only I/O nodes present (existing: `placeholderVisibility.test.ts`). Hover overlay does not render for I/O nodes (existing: implicitly tested via `isIoNode` adapter tests — add explicit `CompartmentNode` render test).

#### Wave 2 — Config Panel Identity Echo

> **Do this after Sprint 5 Wave 3 (execution wiring).**

Make the config panel feel like it belongs to the node that was clicked. One look at the panel header and the user knows which compartment they're configuring.

- [ ] `packages/editor` — **Config panel node icon**: Update `ConfigPanelRoot.tsx` — import `ICON_COMPONENTS` from `adapters/nodeIcons`. Render the node's icon (24px, `text-muted-foreground`) to the left of the heading in `PanelHeader`. Icon sourced from `typeInfo.iconKey`. If no icon exists for the type, render nothing.
- [ ] `packages/editor` — **Config panel empty state improvement**: When no node is selected (`!configNodeId`), replace "Select a node to configure" with: a `PlusIcon` (muted, 24px), a `Text` heading "Nothing selected", and a `Text size="xs" color="muted"` body "Click a compartment to configure it, or add a new one."
- [ ] `packages/editor` — **SchemaForm field grouping**: Add optional `group?: string` key to `NodeParamMeta` in `@bnto/nodes`. When consecutive visible params share the same `group` value, `SchemaForm` renders a `PanelDivider` with the group label above them. Start with Loop node: group `{ mode, items }` as "Iteration" and `{ breakCondition }` as "Control". Purely a rendering concern — no store changes.
- [ ] `packages/editor` — **Unit tests**: Config panel header renders node icon when a node is selected. Config panel shows improved empty state when no node is selected. `SchemaForm` renders group dividers between param groups.

#### Wave 3 — LayerPanel Drag-to-Reorder

Make the LayerPanel the canonical surface for recipe structure management. Drag-to-reorder in the list gives users control over execution order without any canvas arrow buttons.

- [ ] `packages/editor` — **Add `@dnd-kit/sortable` dependency**: Run `pnpm --filter @bnto/editor add @dnd-kit/core @dnd-kit/sortable @dnd-kit/utilities`. Required for drag-to-reorder in the LayerPanel. Not currently installed.
- [ ] `packages/editor` — **`NodeList` drag-to-reorder**: Add drag handles to `NodeItem` in `NodeList`. Use `@dnd-kit/sortable`. Each `NodeItem` gets a `GripVerticalIcon` drag handle on the left, visible always. I/O nodes at top/bottom are locked — cannot be reordered (render without drag handle, add `data-locked` for styling). Dragging updates position via a new `reorderNode(fromIndex, toIndex)` store action.
- [ ] `packages/editor` — **`reorderNode` store action + pure function**: Create `actions/reorderNode.ts` — pure function `reorderNode(state, fromIndex, toIndex): Partial<EditorState>`. Moves node in `nodes` array while preserving I/O nodes at fixed positions. Captures undo snapshot. Updates `isDirty`. Thin wrapper hook `useReorderNode` follows the three-layer pattern.
- [ ] `packages/editor` — **Adapter sync**: After reorder, `rfNodesToDefinition` already reads `nodes` array order as execution order — no changes needed. Add unit test to verify order preserved on export.
- [ ] `packages/editor` — **Unit tests**: `reorderNode` moves nodes correctly. I/O nodes cannot be moved to non-terminal positions. Undo restores previous order. `NodeList` renders drag handles for non-I/O nodes only.

#### Wave 4 — Empty Canvas Entry + Auto-behaviors

The first frame of the editor experience should tell the user what to do. Auto-behaviors reduce friction for new users without adding complexity for returning ones.

- [ ] `packages/editor` — **Auto-open palette on blank canvas**: In `EditorCanvasRoot`, after the store initializes with a blank definition, call `openNodePalette()` once if `nodes.length === 2` (only I/O nodes). Use `useEffect` with one RAF (`requestAnimationFrame`) delay. Only fires once per session via `useRef` flag.
- [ ] `packages/editor` — **Auto-select Input node on blank canvas**: After the canvas renders on blank canvas entry, auto-select the `input` node and open the config panel. Shows the file dropzone immediately. Same `useRef` once-per-session guard.
- [ ] `packages/editor` — **Run button in `EditorToolbar`**: Add Run button between the node navigation group and undo group. `variant="primary"` (terracotta), `elevation="sm"`, `size="icon"` with `PlayIcon`. Disabled when definition has no processing nodes or execution is in progress. Fires `handleRun` (stub for now — wired in Sprint 5 Wave 3).
- [ ] `packages/editor` — **Unit tests**: Palette auto-opens once on blank canvas mount. Does not fire on subsequent renders or after a node is added. Input node auto-selected on blank canvas. Run button disabled when no processing nodes exist.

#### Wave 5 — Verify + E2E

Verify all UX changes hold together as a system. No regressions on existing editor tests.

- [ ] `packages/editor` — **Verify unit tests**: `task ui:test` passes across all `packages/editor` tests. No regressions in existing store, action, adapter, or hook tests.
- [ ] `apps/web` — **E2E: blank canvas entry**: Navigate to `/editor`. Palette auto-opens. Input node selected, config panel shows "Input" header with Upload icon. Click a node type — `PlaceholderSlot` disappears, real node appears with spring pop-in animation.
- [ ] `apps/web` — **E2E: node hover delete**: Hover a non-I/O compartment — delete × appears. Click × — node removed with spring exit animation, without selecting it first. Hover Input or Output compartment — no × appears.
- [ ] `apps/web` — **E2E: LayerPanel reorder**: Open LayerPanel. Drag a processing node up/down — order updates on canvas. I/O nodes cannot be dragged. Undo restores previous order.
- [ ] `apps/web` — **E2E: config panel identity**: Select a node. Config panel header shows the node's icon and label. Deselect (click canvas background) — config panel shows improved empty state.
- [ ] `apps/web` — **Screenshot update**: Regenerate editor E2E screenshots after all visual changes. `task e2e` green.

---

### Sprint 5B: Node Visual Identity — Hierarchy, Selection, and I/O Distinction

**Goal:** Make the canvas immediately readable at a glance. Right now all three node types render identically — same card color, same size, same elevation. This sprint establishes a clear three-tier visual language: I/O nodes (grounded structural anchors) → processing nodes (lifted, configurable steps) → selected node (highlighted, active focus).

**Decision doc:** `.claude/decisions/editor-ux-direction.md` — visual hierarchy section.

**Prerequisite:** Sprint 5A Wave 1 complete (`isIoNode` flag exists in adapter).

**Persona ownership:** `/frontend-engineer` leads all waves.

**Design constraints (non-negotiable):**

- Keep `Pressable asChild` wrapping `Card` as the base for all node types — including I/O. Don't introduce a separate primitive.
- I/O nodes are NOT pressable-to-configure. They can be selected for metadata but don't open a configurable config panel. Reflect this visually.
- No color for color's sake. Every visual change communicates information (type, state, or category).
- Selected ring must be visible at a glance — not just an elevation change.

> ⚠️ **Animation rule:** `<ScaleIn from={0.7} easing="spring-bouncy">` must remain on ALL node types including I/O after this sprint's visual changes. The pop-in is non-negotiable. See `strategy/design-language.md` § "Node Entrance: The Building Pop-In".

**The three-tier system:**

| Tier          | Nodes         | Elevation (rest) | Elevation (selected)       | Color                | Shape              |
| ------------- | ------------- | ---------------- | -------------------------- | -------------------- | ------------------ |
| Structural    | input, output | `sm`             | `md`                       | `muted` (warm cream) | square — `90×90`   |
| Processing    | all others    | `md`             | `lg`                       | `card` (white)       | square — `120×120` |
| Selected ring | any selected  | —                | + `ring-2 ring-primary/60` | —                    | —                  |

#### Wave 1 — I/O Node Distinction

Differentiate Input and Output from processing nodes through elevation, color, and shape — not just icon. These are the walls of the bento box, not a compartment inside it.

- [x] `packages/editor` — **I/O node sizing**: Update `definitionToBento` adapter — set `width: 160, height: 90` for nodes where `isIoNodeType(nodeType)` is true. Processing nodes keep `120×120`. Different aspect ratio signals "different kind of thing" before the label is read.
- [x] `packages/editor` — **I/O node color + elevation**: In `CompartmentNode.tsx`, **replace** the current uniform `elevation={selected ? "lg" : "sm"}` with conditional logic: when `data.isIoNode` is true, render `Card color="muted" elevation={selected ? "md" : "sm"}`. When false (processing), render `Card elevation={selected ? "lg" : "md"}`. This is a conscious replacement of the existing scheme, not an addition. Muted cards read as "part of the canvas" — warm cream that doesn't compete with the steps in between.
- [x] `packages/editor` — **I/O node Pressable behavior**: Remove `toggle` and `active` props from the `Pressable` wrapper for I/O nodes. They can still receive clicks for selection feedback but shouldn't feel like pressable configuration buttons.
- [x] `packages/editor` — **Unit tests**: I/O nodes render with `color="muted"`, `elevation="sm"`, and `160×90` dimensions. Processing nodes render with `elevation="md"` and `120×120`. Adapter sets `isIoNode` correctly for all 12 node types.

#### Wave 2 — Processing Node Category Accents + Selected State

Make the selected state obvious and give processing nodes a subtle category signal without overwhelming the calm Motorway aesthetic.

- [ ] `packages/editor` — **Selected ring**: Add `ring-2 ring-primary/60 ring-offset-2 ring-offset-background rounded-xl` to the outer `group relative` wrapper when `selected` is true, via `cn()`. Creates a visible terracotta outline that reads as "active focus" at a glance.
- [ ] `packages/editor` — **Elevation on selection**: `elevation={selected ? "lg" : "md"}` for processing nodes. `elevation={selected ? "md" : "sm"}` for I/O nodes. Ring + elevation change together make selection unmistakable.
- [ ] `packages/editor` — **Category color pip**: Add `border-l-[3px]` in category color to processing nodes. Create `adapters/categoryBorderColor.ts` mapping `CompartmentVariant` → Tailwind border class (e.g. `primary` → `border-primary/70`). I/O nodes get no pip.
- [ ] `packages/editor` — **Unit tests**: Selected processing node has ring class. Unselected has no ring. Category pip border renders for processing nodes. I/O nodes have no pip. Elevation changes correctly for both types on selection.

#### Wave 3 — LayerPanel Visual Parity

The LayerPanel list should visually echo the canvas hierarchy — I/O nodes look distinct in the list too.

- [ ] `packages/editor` — **`NodeItem` I/O distinction**: Update `NodeItem.tsx` — when `node.data.isIoNode` is true, render dot indicator in `text-muted-foreground`. Add `text-xs text-muted-foreground` "I/O" sublabel to reinforce structural vs. configurable distinction.
- [ ] `packages/editor` — **`NodeItem` selected state**: When a node is selected in the store, the `NodeItem` row gets `bg-muted/50` background highlight.
- [ ] `packages/editor` — **Unit tests**: I/O `NodeItem` renders muted dot and "I/O" sublabel. Selected `NodeItem` applies background highlight. Processing `NodeItem` uses category color dot.

#### Wave 4 — Verify + Screenshot

- [ ] `packages/editor` — **Verify unit tests**: All 5A + 5B tests pass. No regressions.
- [ ] `apps/web` — **E2E: visual hierarchy**: Navigate to `/editor?from=compress-images`. Input and Output cards are visibly different from the Image processing node (different size, muted tone). Click the Image node — ring appears, elevation lifts. Click Input node — different selection treatment (no ring, subtle lift). LayerPanel echoes the visual distinction.
- [ ] `apps/web` — **Screenshot update**: Regenerate editor E2E screenshots. `task e2e` green.

---

### Sprint 5C: Editor Copy + Nav Label Cleanup

**Goal:** Nail the copy and labels across editor entry points. Small changes, high signal — language shapes how users understand what they're looking at.

**Prerequisite:** Sprint 5 Wave 2 (production route) complete.

**Tasks:**

- [ ] `apps/web` — **Rename nav "Create" → "New Recipe"**: Update `DesktopNav.tsx:15` and `MobileNavMenu.tsx:100` (both say "Create"). Label: "New Recipe". Route: `/editor` (unchanged). Decision: "Create" is vague. "New Recipe" matches the product mental model and pairs with the recipe pages.
- [ ] `apps/web` — **Update recipe page CTA copy**: Change "Customize in Editor" → "Open in Editor" in `OpenInEditorLink.tsx:12`. Currently reads "Customize in Editor". "Customize" implies minor tweaks; "Open" implies full access and pairs with "New Recipe" in the nav.
- [ ] `apps/web` — **Verify**: All nav + recipe page CTA copy consistent. No remaining "Create" or "Customize in Editor" references. Grep for both strings across `apps/web/`.

> **Future copy consideration (post-Sprint 5 Wave 3):** Once execution is wired in the editor, revisit the recipe page CTA. "Open in Editor" is functional but "Make it yours →" or "Build your own version" signals creative ownership more than "Open" and may convert better. Worth A/B testing once traffic exists. Do not change this now — the editor needs to actually run recipes before a possessive CTA is honest.

---

### Sprint 6: Edit Mode ↔ Run Mode (Mini Motorways Pattern)

**Goal:** Make the editor feel like Mini Motorways — pause to edit the road network, unpause to watch traffic flow. The same canvas surface serves both editing and running. No separate screens.

**Prerequisite:** Sprint 5 Wave 3 (execution wired). The mode switch requires execution to exist — don't build this before execution works.

**Decision doc:** `.claude/decisions/editor-ux-direction.md` — "Vision: Edit Mode ↔ Run Mode" section. Read this before picking up any task in this sprint.

**The two modes:**

- **Edit mode (current state):** Canvas grid visible. Nodes are pressable/selectable. Config panel slides in on click. LayerPanel open. Toolbar shows full node management controls. Run button triggers mode switch.
- **Run mode:** Canvas grid fades or hides. Nodes are static (no click interaction). Config panel + LayerPanel close. Toolbar collapses to just Stop button. Nodes animate through the elevation sequence (idle → pending → active → completed). Output node shows results or download prompt.
- **Transition:** Run button → run mode. Stop button → edit mode, canvas restored to exact state before run.

**Why this matters:** The power-user loop is edit → run → tweak → run again. Forcing a screen change breaks that loop. The Mini Motorways analogy is exact: pausing to lay roads, unpausing to watch traffic.

#### Wave 1 — Editor Mode State

- [ ] `packages/editor` — **`editorMode` in store**: Add `editorMode: "edit" | "run"` to `EditorState`. Add `setEditorMode(mode)` action (pure function + hook wrapper). Default: `"edit"`. Run button dispatches `setEditorMode("run")`. Stop button dispatches `setEditorMode("edit")`.
- [ ] `packages/editor` — **Mode-aware toolbar**: `EditorToolbar` reads `editorMode`. In `"edit"` mode: full controls (add, navigate, delete, undo/redo, reset, Run button). In `"run"` mode: toolbar collapses to just a Stop button (`variant="destructive"`, `PlayIcon` replaced with `SquareIcon`). Animate the collapse with `Animate.FadeIn`.
- [ ] `packages/editor` — **Mode-aware panels**: `EditorConfigPanel` and `EditorLayerPanel` both read `editorMode`. In `"run"` mode: panels slide out and stay closed (cannot be opened by clicking nodes). In `"edit"` mode: panels behave normally.
- [ ] `packages/editor` — **Unit tests**: Store transitions between `edit` and `run` correctly. Toolbar renders stop-only in run mode. Panels are non-interactive in run mode.

#### Wave 2 — Canvas Grid Transition

- [ ] `packages/editor` — **Grid fade on run mode**: `BentoCanvas` reads `editorMode`. In `"run"` mode: fade the ReactFlow background grid (CSS `opacity` transition, ~300ms). In `"edit"` mode: grid visible. The grid disappearing signals "this is no longer a configuration surface."
- [ ] `packages/editor` — **Node interaction lock in run mode**: `CompartmentNode` reads `editorMode` from store (via context or prop). In `"run"` mode: disable `Pressable` (remove `onClick` handler or add `pointer-events-none` wrapper). Nodes should not be selectable during execution — they're displaying state, not accepting input.
- [ ] `packages/editor` — **Bridge `executionState` to node `data.status`**: The store has `executionState: Record<string, NodeExecutionStatus>` and `setExecutionState()`, but nothing currently syncs this into each `BentoNode.data.status` during re-renders. Add a derived state selector or `onNodesChange` integration that maps `executionState[nodeId]` → `node.data.status` so `CompartmentNode` receives live status updates. Without this bridge, the elevation sequence has no input.
- [ ] `packages/editor` — **Elevation sequence integration**: Wire the now-live `data.status` field (`idle | pending | active | completed`) to elevation during run mode. This is the visual "traffic flowing" moment. `CompartmentNode` reads `data.status` (bridged above) and maps it to `Card elevation` during run mode execution.
- [ ] `packages/editor` — **Unit tests**: Canvas grid has reduced opacity in run mode. Nodes are non-interactive in run mode. Elevation changes with status in run mode.

#### Wave 3 — Results at Output Node + E2E

- [ ] `packages/editor` — **Output node run-mode state**: When execution completes, the `output` node shows a completion state — icon changes to `CheckCircle`, sublabel shows file count (e.g., "3 files ready"), elevation holds at `lg`. Clicking the completed Output node (only in run mode, after completion) triggers download. This is the delivery moment.
- [ ] `packages/editor` — **Return to edit mode after download**: After download triggers, show a brief "Done" state then auto-transition back to edit mode (2s delay or on Stop button press). Canvas grid reappears, panels re-open, toolbar restores. User is back in the same canvas state they left.
- [ ] `apps/web` — **E2E: mode switch flow**: Open `/editor?from=compress-images`. Add files to Input node. Click Run — toolbar collapses to Stop, grid fades, panels close. Nodes animate through elevation sequence. Output node shows completion. Click output node — download triggers. Canvas returns to edit mode.
- [ ] `apps/web` — **Screenshot update**: Capture both edit mode and run mode states. `task e2e` green.

---

### Sprint 4H: Pipeline Executor Extraction — TDD-First Architecture Correction

**Goal:** Extract all orchestration logic out of runtime adapters into a single, runtime-agnostic `executePipeline()` function. Prove it correct with comprehensive tests that run in pure Node.js — no browser, no WASM, no Worker. Once the tests are green, the cleanup follows. Any runtime placed on top inherits correct behavior automatically.

**The principle:** If we can make it run and work flawlessly at the engine/core level with tests, we can put any app layer in front of it and know we're good.

**Why this order (tests before cleanup):** Tests define the contract. Writing them first forces precise thinking about what `executePipeline` must do — then the implementation and the adapter cleanup follow naturally. This is also what protects the refactor: existing tests stay green throughout, new tests prove the new layer is correct before anything depends on it.

**Why now, not later:** Sprint 5 Wave 3 wires the editor to the execution engine. If this isn't fixed first, the editor builds on the wrong foundation. This is the prerequisite — not optional.

**Decision doc:** `.claude/decisions/implicit-iteration.md` — full audit, architecture diagram, rules, exact file list. Read before picking up any task.

**Task runner:** `pnpm --filter @bnto/core test` (runs vitest in Node.js, `environment: "node"` per `vitest.config.ts`). These tests require zero browser setup.

**Persona ownership:** `/core-architect` leads all waves.

**Files changing:**

| File                                                         | Change                                                                                                              |
| ------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------- |
| `packages/core/src/engine/types.ts`                          | **NEW** — `NodeRunner`, `FileInput`, `FileResult`, `RecipeDefinition`, `PipelineProgressCallback`, `PipelineResult` |
| `packages/core/src/engine/executePipeline.ts`                | **NEW** — runtime-agnostic pipeline executor                                                                        |
| `packages/core/src/engine/executePipeline.test.ts`           | **NEW** — comprehensive unit tests, mock `NodeRunner`, pure Node.js                                                 |
| `packages/core/src/types/browser.ts`                         | Remove `processFiles()` from `BrowserEngine` interface                                                              |
| `packages/core/src/adapters/browser/BntoWorker.ts`           | Remove `processFiles()` method                                                                                      |
| `packages/core/src/adapters/browser/toBrowserEngine.ts`      | Remove `processFiles` pass-through                                                                                  |
| `packages/core/src/adapters/browser/BntoWorker.test.ts`      | Remove `processFiles` tests — iteration proven by `executePipeline.test.ts`                                         |
| `packages/core/src/services/browserExecutionService.ts`      | Replace `engine.processFiles()` with `executePipeline(singleNodeRecipe, files, runNode)`                            |
| `packages/core/src/services/browserExecutionService.test.ts` | Replace `processFiles` mocks with `processFile` mocks                                                               |
| `packages/core/src/index.ts`                                 | Export `executePipeline`, `NodeRunner`, core engine types                                                           |

---

#### Wave 1 — Define types and write tests first

> **TDD gate:** Tests must be written and failing correctly before implementation starts. A failing test that correctly describes behavior is a passing wave.

No browser APIs. No WASM. No Worker. Pure TypeScript types and vitest.

- [x] `@bnto/core` — `/core-architect` — **`engine/types.ts`**: Define the engine layer vocabulary. `NodeRunner` (single-file contract every runtime implements — `(file, nodeType, params, onProgress?) => Promise<FileResult>`), `FileInput`, `FileResult`, `PipelineDefinition` (ordered node list, node type, params, I/O markers), `PipelineProgressCallback` (`(nodeIndex, fileIndex, totalFiles, percent, message) => void`), `PipelineResult` (all output files, per-node metadata, total duration). Zero imports from browser, WASM, or Worker code.

  > **Naming: `PipelineDefinition`, NOT `RecipeDefinition`.** `RecipeDefinition` already exists in `packages/core/src/types/recipe.ts` (the Convex-backed recipe shape with `id`, `type`, `version`, `ports`, `edges`, recursive `nodes`). The pipeline executor's input is a simpler ordered node list for execution — different type, different purpose. Use `PipelineDefinition` to avoid ambiguity.

- [x] `@bnto/core` — `/core-architect` — **`engine/executePipeline.test.ts`**: Write the full test suite against the not-yet-implemented `executePipeline`. Use `vi.fn()` as `NodeRunner` — no real engine needed. Tests must cover:
  - **Single processing node:** `runNode` called once per file; results collected in input order
  - **Multi-node recipe:** outputs of node N passed as inputs to node N+1; `runNode` called `nodes × files` times
  - **I/O node skipping:** `input` and `output` type nodes do not call `runNode` — they are structural markers only
  - **Call count assertion:** `expect(runNode).toHaveBeenCalledTimes(processingNodes * files.length)` — proves the loop is right
  - **Node failure propagates:** if `runNode` rejects, `executePipeline` rejects with the error; no silent swallowing
  - **Empty file array:** resolves with empty results, no errors, no `runNode` calls
  - **Single file:** behaves identically to multi-file with one file
  - **Progress aggregation:** `onProgress` receives `(nodeIndex, fileIndex, totalFiles, percent, message)` in the correct sequence
  - **Node order guarantee:** `runNode` calls happen in recipe order, not I/O node order
  - **`runNode` receives correct args:** filename, nodeType, params match `PipelineDefinition` per-node
  - **Result structure:** `PipelineResult.files` contains all output files; `PipelineResult.durationMs` is a positive number

  Model: follow the depth of `executionInstanceStore.test.ts` — happy path, error cases, edge cases, boundary behavior documented in test names.

  > **Type naming reminder:** The test file imports `PipelineDefinition` (not `RecipeDefinition`) from `../engine/types`. See Wave 1 naming note.

---

#### Wave 2 — Implement `executePipeline` until tests go green

> **TDD gate:** Wave 2 is complete when `pnpm --filter @bnto/core test` reports all `executePipeline.test.ts` tests passing. No partial credit.

- [x] `@bnto/core` — `/core-architect` — **`engine/executePipeline.ts`**: Implement `executePipeline(definition, files, runNode, onProgress)`. Walk `definition.order`, skip nodes where `node.type === "input" || node.type === "output"`, iterate all current files through each processing node sequentially, chain outputs, aggregate progress, return `PipelineResult`. No browser APIs. No WASM. No dynamic imports. The only I/O this function does is call `runNode`. The function accepts `PipelineDefinition` (not `RecipeDefinition` — see Wave 1 naming note).
- [x] `@bnto/core` — `/core-architect` — **Run `pnpm --filter @bnto/core test`**: All `executePipeline.test.ts` tests green. Existing tests (`BntoWorker.test.ts`, `browserExecutionService.test.ts`, `executionInstanceStore.test.ts`, etc.) still pass — no regressions.

---

#### Wave 3 — Strip `processFiles` from browser adapter

> **TDD gate:** All tests must stay green after every file change. Run `pnpm --filter @bnto/core test` after each file. Do not move to the next file if tests are red.

With the executor proven correct by tests, remove the iteration logic from the places it doesn't belong.

- [x] `@bnto/core` — `/core-architect` — **`types/browser.ts`**: Remove `processFiles()` from `BrowserEngine` interface. `processFile` stays. The engine interface is now single-file only. Run tests — green.
- [x] `@bnto/core` — `/core-architect` — **`adapters/browser/BntoWorker.ts`**: Remove `processFiles()` method. The worker wrapper exposes `processFile` only. Run tests — green.
- [x] `@bnto/core` — `/core-architect` — **`adapters/browser/toBrowserEngine.ts`**: Remove `processFiles` pass-through. Only `processFile` adapted. Run tests — green.
- [x] `@bnto/core` — `/core-architect` — **`adapters/browser/BntoWorker.test.ts`**: Remove the `processFiles` test at line 302 (`"processFiles after terminate throws a clear error"`). This is the only `processFiles`-specific test — other terminate tests and all `processFile` tests stay. The `processFiles` method itself is removed from `BntoWorker.ts` in a prior task. Run tests — green.
- [x] `@bnto/core` — `/core-architect` — **`services/browserExecutionService.ts`**: Replace the `engine.processFiles()` call in `execute()` with `executePipeline()`. Build a single-node `PipelineDefinition` from the slug + params. Inject `(file, nodeType, params, onProgress) => engine.processFile(file, nodeType, params, onProgress)` as `NodeRunner`. Run tests — green.

  > **Progress callback signature change:** The current `execute()` wraps the engine's `(fileIndex, percent, message)` into `BrowserFileProgressInput`. After switching to `executePipeline`, the pipeline's `PipelineProgressCallback` provides `(nodeIndex, fileIndex, totalFiles, percent, message)` — it adds `nodeIndex`. The service must adapt: map `nodeIndex` from the pipeline callback into the existing `BrowserFileProgressInput` shape (ignore `nodeIndex` for now since the service builds a single-node pipeline). When multi-node editor execution arrives (Sprint 5 Wave 3), the progress shape will need to expand.

- [x] `@bnto/core` — `/core-architect` — **`services/browserExecutionService.test.ts`**: Replace all `processFiles` mock references with `processFile` mock. The service no longer calls `engine.processFiles()`. The mock engine's `processFiles` method is removed from `createMockEngine()`. Update progress tests: the `onProgress` callback now comes from `executePipeline` internals, not from the engine's `processFiles`. Verify `processFile` receives correct per-file calls. Run tests — green.

---

#### Wave 4 — Export, document, E2E

> **Final gate:** `pnpm --filter @bnto/core test` all green. All 6 recipe pages work. Sprint 5 Wave 3 is unblocked.

- [x] `@bnto/core` — `/core-architect` — **`index.ts`**: Export `executePipeline`, `NodeRunner`, `FileInput`, `FileResult`, `PipelineDefinition`, `PipelineResult` from `@bnto/core`. Sprint 5 Wave 3 and the future CLI import from here — not from internal paths.
- [x] `@bnto/core` — `/core-architect` — **JSDoc on `executePipeline`**: Document the layer contract. What `NodeRunner` is responsible for. Why no browser APIs belong here. The explicit loop node override point (future). This comment is the first thing the next engineer reads before touching execution.
- [x] `apps/web` — `/quality-engineer` — **E2E smoke test**: All 6 recipe pages process files correctly end-to-end. `compress-images`, `resize-images`, `convert-image-format`, `clean-csv`, `rename-csv-columns`, `rename-files`. Multi-file upload → processing → download works. Behavior is identical to before the refactor — zero user-visible change, correct architecture underneath.

---

### Audit: Session Summary Validation

**Goal:** Re-validate the plan against the session summary to confirm all decisions, architecture rules, and sprint definitions are correctly captured and no gaps remain.

**When:** After Sprint 4H Wave 4 completes and before Sprint 5 Wave 3 starts. This is the checkpoint — the pipeline executor is proven correct, the plan is groomed, and execution wiring is about to begin.

**Reference document:** `/Users/ryan/Downloads/bnto-session-summary.md`

- [ ] `/project-manager` — **Re-run codebase audit**: Read the session summary in full. For each decision documented in the summary, verify it is correctly reflected in PLAN.md, `decisions/implicit-iteration.md`, `strategy/design-language.md`, and `decisions/editor-ux-direction.md`. Report any remaining gaps.
- [ ] `/project-manager` — **Verify Sprint 4H deliverables**: Confirm `packages/core/src/engine/` directory exists. Confirm `executePipeline.ts`, `executePipeline.test.ts`, and `types.ts` exist. Confirm all tests pass (`pnpm --filter @bnto/core test`). Confirm `executePipeline` and `NodeRunner` are exported from `@bnto/core` index. Confirm `processFiles()` is removed from `BrowserEngine` interface, `BntoWorker`, and `toBrowserEngine`.
- [ ] `/project-manager` — **Verify animation identity**: Confirm `ScaleIn from={0.7} easing="spring-bouncy"` still exists on `CompartmentNode.tsx`. Confirm `AnimatePresence` exit animation is wired. Confirm `strategy/design-language.md` has the "Editor Animation Language" section with PROTECTED headers.
- [ ] `/project-manager` — **Verify execution order**: Confirm the "Active work — execution order" in PLAN.md Current State matches the actual dependency graph. Confirm Sprint 5 Wave 3 is unblocked (4H complete). Confirm Sprint 6 prerequisites are met (execution wired).
- [ ] `/project-manager` — **Report**: Produce a pass/fail summary. If all pass, Sprint 5 Wave 3 is greenlit. If any fail, document what needs fixing before execution wiring starts.

---

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

**Status:** Needs design decision (review with full project context + Notion workspace)

When a recipe has multi-file input and a processing node (e.g., Image compress), should the editor:

- **Option A (Smart/implicit):** Automatically iterate over inputs — user adds `Input → Image (compress) → Output`, engine handles the loop. Simple, fewer nodes, covers 90% of cases.
- **Option B (Explicit):** User builds iteration manually — `Input → Loop (forEach) → Image (compress inside loop) → Output`. More flexible, more transparent, matches current Go engine model.

**Proposed direction:** Smart by default (Option A) with an advanced toggle to switch to explicit looping for power users. This affects engine processing, definition schema, and editor UX. Needs a deep review session with Notion context (Bnto Directory, MVP Scope, recipe specs) before implementation.

**Touches:** `@bnto/nodes` (definition schema), `engine/` (execution model), `@bnto/editor` (node placement + wiring), recipe definitions (compress-images etc.), `io-nodes.md` strategy doc.

---

## Backlog

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

---

## Reference

| Document                                     | Purpose                                                                    |
| -------------------------------------------- | -------------------------------------------------------------------------- |
| `.claude/journeys/`                          | User journey test matrices — auth, engine, API, web app, editor            |
| `.claude/strategy/bntos.md`                  | Predefined Bnto registry — slugs, fixtures, SEO targets, tiers             |
| `.claude/strategy/editor-architecture.md`    | Shared editor layer — store, hooks, package strategy, switchable editors   |
| `.claude/strategy/editor-user-journey.md`    | Editor user journey — stages, flows, success criteria, phased delivery     |
| `.claude/strategy/visual-editor.md`          | Bento box visual editor — compartment design, grid layout, execution state |
| `.claude/strategy/code-editor.md`            | Code editor design — CM6, slash commands, JSON Schema                      |
| `.claude/strategy/conveyor-belt.md`          | Conveyor belt showcase — Motorway page R&D (not the editor)                |
| `.claude/strategy/go-engine-migration.md`    | Go engine node inventory — migration reference before archive deletion     |
| `.claude/strategy/cloud-desktop-strategy.md` | Architecture, cost analysis, cloud execution topology                      |
| `.claude/strategy/core-principles.md`        | Trust commitments, "For Claude Code" guidance                              |
| `.claude/rules/`                             | Auto-loaded rules (architecture, code-standards, components, etc.)         |
| `.claude/skills/`                            | Agent skills (pickup, project-manager, code-review, pre-commit)            |
| Notion: "SEO & Monetization Strategy"        | Pricing, revenue projections, quota limits                                 |
