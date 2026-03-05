# Bnto — Build Plan

**Last Updated:** March 4, 2026
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

**Co-location decision (Feb 2026):** UI components and editor features live in `apps/web` for now. No separate `@bnto/ui` or `@bnto/editor` packages until there's a real second consumer (desktop app). Engine, core API, and data layer logic stays in `@bnto/core`. When the UI package is extracted, it will be published as `@bnto/ui` (npm) under the name **Motorway** — the Mini Motorways-inspired design system (surface, elevation, pressable, spring animations, warm palette).

---

## Current State

- **FOCUS: Self-describing recipes + runnable editor.** Sprint 4 (visual editor) complete. Sprint 4C (I/O Nodes) is next — makes recipes self-describing so custom editor creations can actually run. See [io-nodes.md](.claude/strategy/io-nodes.md) for the architecture reference.
- **Next up:** Sprint 4C Wave 1 (`@bnto/nodes` — I/O node types, schemas, recipe updates). Unblocked now.
- **Parallel track:** Sprint 4D (UI Package Extraction) — extract `apps/web/components/ui/` to `packages/ui` (`@bnto/ui` / Motorway). Prerequisite for editor package extraction. Can run in parallel with Sprint 4C.
- **RF store refactor complete:** PRs #86-#94 merged. Sprint 4C Waves 1-2 are both unblocked.
- **Sequence:** Sprint 4C (I/O Nodes) → Sprint 4D (UI Package) → Sprint 4E (Editor Package) → hook up I/O nodes with predefined recipes in editor.
- **Tabled:** Sprint 3 remaining (3 E2E test tasks) — platform features are built and working, test coverage deferred to backlog.
- **Tabled:** Sprint 4B (Code Editor) — unblocked but deferred until visual editor + I/O nodes ship.
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

### Sprint 3A: Remove Anonymous User System

**Goal:** Eliminate the anonymous Convex session system. The model becomes binary: you're signed in or you're not. No invisible Convex users, no anonymous→real upgrade flow. Browser execution is free unlimited without any server-side session.

**Why now:** The anonymous system added complexity to every layer (schema, auth, core hooks, UI, tests) without serving any current product goal. Removing it before Sprint 3 Wave 2 (dashboard, conversion hooks) prevents building new features on top of dead infrastructure.

**What stays:** `@convex-dev/auth` password provider, session cookies, proxy route protection, sign-in/sign-up/sign-out flows. The auth system itself is fine — we're only removing the anonymous session layer on top of it.

**Persona ownership:**
| Package | Persona |
|---------|---------|
| `@bnto/backend` | `/backend-engineer` |
| `@bnto/core` | `/core-architect` |
| `@bnto/auth` | `/backend-engineer` |
| `apps/web` | `/frontend-engineer` |
| `.claude/` docs | No specific persona |

#### Wave 1 (backend — schema + auth simplification)

Strip anonymous plumbing from the data layer. This is the foundation — everything else depends on schema being clean.

- [x] `@bnto/backend` — Remove anonymous auth provider from `convex/auth.ts`. Keep only password provider. Simplify `convex/_helpers/user_lifecycle.ts`
- [x] `@bnto/backend` — Schema cleanup: remove `isAnonymous`, quota fields, `by_anonymous` index from users table
- [x] `@bnto/backend` — Simplify `convex/execution_events.ts` to require userId on all execution events
- [x] `@bnto/backend` — Delete anonymous test files. Update remaining tests to remove anonymous scenarios
- [x] `@bnto/backend` — **Validation:** `task ui:test` passes for `@bnto/backend`. All remaining tests green

#### Wave 2 (core — hooks + types + adapters)

Remove anonymous hooks and types from `@bnto/core`. This is the transport-agnostic API layer — clean it so the web app has a simple auth surface.

- [x] `@bnto/core` — Delete anonymous/quota hooks. Remove exports from `reactCore.ts`
- [x] `@bnto/core` — Simplify `useSignUp.ts` and `useAuth.ts`. Auth state is binary
- [x] `@bnto/core` — Clean types and transforms: remove `isAnonymous` and quota fields
- [x] `@bnto/core` — Delete anonymous integration tests. Update remaining tests
- [x] `@bnto/core` — **Validation:** `task ui:test` passes for `@bnto/core`. All remaining tests green

#### Wave 3 (web — components + auth flow)

Remove anonymous UI patterns from the web app. Simplify auth page, remove gate components, clean up providers.

- [x] `apps/web` — Delete `UpgradePrompt.tsx`. **AccountGate retained** — it's the primary conversion component for enticing unauthenticated users to create accounts. Used on `/my-recipes` and available for any gated surface
- [x] `apps/web` — Simplify providers, NavUser, SignInForm: remove anonymous session handling
- [x] `apps/web` — Simplify recipe flow: browser execution is always allowed, no gates
- [x] `apps/web` — **Validation:** `task ui:build` passes. No TypeScript errors from removed types/hooks

#### Wave 4 (auth E2E — verify the simplified system)

Comprehensive E2E tests proving the simplified auth model works end-to-end. Every user-facing flow tested.

- [x] `apps/web` — Delete `anonymous-conversion.spec.ts` E2E test file
- [x] `apps/web` — **E2E: New user journey** — fresh visitor → /signin defaults to signup mode → fill form → create account → lands on home → user menu shows email → sign out → stays on /signin (no bounce)
- [x] `apps/web` — **E2E: Returning user journey** — /signin shows "Welcome back" → sign in → lands on home → user menu shows email → can access protected routes
- [x] `apps/web` — **E2E: Sign-out round-trip** — sign up → home → sign out → /signin → sign back in → home → user menu shows same email
- [x] `apps/web` — **E2E: Route protection** — unauthenticated user hits /executions → redirected to /signin. Authenticated user hits /signin → redirected to /. Sign out → protected routes blocked again
- [x] `apps/web` — **E2E: Browser execution without account** — visit recipe page (e.g. /compress-images) with no account → drop files → run → execution completes → download works. No sign-up prompt blocking the flow
- [x] `apps/web` — **E2E: Form toggle** — signup ↔ signin toggle works. Invalid credentials show error. Duplicate email on signup signs in existing user
- [x] `apps/web` — **Validation:** All E2E tests pass (13/13 auth, screenshots regenerated). Test account cleanup via global teardown

#### Wave 5 (docs + cleanup)

Update all documentation and strategy files to reflect the simplified auth model. Remove references to anonymous users, quotas, and conversion funnels that no longer exist.

- [x] `.claude/` — Update `PLAN.md`, `pricing-model.md`, `auth-routing.md`, `environment-variables.md` to remove anonymous references
- [x] `.claude/` — Update journey docs: `journeys/auth.md` (remove anonymous conversion rows)
- [x] `.claude/` — Update persona skills that reference anonymous patterns: `backend-engineer`, `security-engineer`, `security-review`
- [x] `@bnto/backend` — Production schema cleanup: mutation to delete orphaned anonymous user records, then deploy strict schema removing the optional fields
- [x] **Validation:** `task check` passes (full quality gate). Grep verification confirms no dead references

---

### Sprint 3: Platform Features (M2)
**Goal:** Accounts earn their keep. Users who sign up get persistence, history, and a reason to stay. Conversion hooks are natural — Save, History, Server Nodes — not artificial run caps. See [pricing-model.md](strategy/pricing-model.md) for the full free vs premium framework.

**Prerequisite:** Sprint 3A (anonymous user removal) must be complete. The anonymous system is gone — auth is binary (signed in or not). Conversion prompts are value-driven.

**Persona ownership:**
| Package | Persona |
|---------|---------|
| `@bnto/backend` | `/backend-engineer` |
| `@bnto/core` | `/core-architect` |
| `apps/web` | `/frontend-engineer` |
| `infra` | No specific persona — general |

#### Pre-work — COMPLETE
~~Anonymous→password userId fix~~, FIXME cleanup, privacy policy rewrite, README review, Knip dead code audit (14 files, 11 deps), naming audit, codebase standards review (149 violations), schema analytics fields, site navigation E2E tests.

#### Wave 1 (parallel — core hooks + UI components + infra decisions)

- [x] `@bnto/core` — `/core-architect` — `useExecutionHistory()` hook (paginated, per-user)
- [x] `@bnto/core` — `/core-architect` — `useUsageAnalytics()` hook (total runs, most-used bntos, last activity)
- [x] `apps/web` — `/frontend-engineer` — RecipeCard component (name, node count, last run status, last updated)
- [x] `apps/web` — `/frontend-engineer` — StatusBadge component (pending, running, completed, failed)
- [x] `apps/web` — `/frontend-engineer` — EmptyState component (no workflows yet)
- [x] `infra` — **Analytics layer decision:** PostHog selected. Decision doc at `.claude/decisions/analytics.md`. Privacy policy updated to remove premature "no tracking" promises. Copy across FAQ, pricing, hero updated to be honest, not aspirational.
- [x] `@bnto/core` — `/core-architect` — **PostHog telemetry integration:** `core.telemetry` namespace (client → adapter), `TelemetryProvider` with config injection, E2E test hook via `window.__bnto_telemetry__`. Production-only env vars (Vercel). 2 E2E tests.
- [x] `infra` — **SEO validation tooling:** Lighthouse CI in GitHub Actions (advisory, all 10 public routes). `task seo:audit` for local audits. Google Search Console verified via Cloudflare DNS. `@lhci/cli` installed as dev dep.

#### Wave 2 (parallel — dashboard + auth behavior)

- [x] `apps/web` — `/frontend-engineer` — Dashboard page (`/my-recipes`): saved workflows, recent executions, usage stats, Recent/Saved tabs, sign-up conversion prompt for unauthenticated users
- [x] `@bnto/core` + `apps/web` — `/core-architect` + `/frontend-engineer` — **Browser-local execution history:** IndexedDB adapter in `@bnto/core` for unauthenticated users. Core routes internally — `core.executions.useHistory()` returns data regardless of auth state, web app never knows if it came from Convex or IndexedDB. 10-entry cap, oldest rotated out. Stores: slug, timestamp, status, duration. Foundation for AccountGate conversion. *(Merged in #70)*
- [x] `apps/web` — `/frontend-engineer` — **Execution history tab in `/my-recipes`** (no standalone `/executions` route). List of past runs with status. Re-run for authenticated users only. Three-tier access per Feature Funnel (Notion): unauth=read-only browser-local, free=7-day server-synced with re-run, pro=30-day. Consumes `core.executions.useHistory()` — doesn't know about storage backend. *(PR #74)*
- [x] `apps/web` — `/frontend-engineer` — **Save prompt** (conversion hook): After successful browser execution for unauthenticated users, surface inline save-focused prompt — "Want to save this recipe? Sign up — it's free." Natural value moment, not a blocking gate. *(PR #74)*
- [x] `apps/web` — `/frontend-engineer` — **Browser auth behavior verification:** Token expiry, sign-out invalidation, cookie-based default mode (moved from Sprint 2A Wave 5) — PR #75
- [x] `apps/web` — `/frontend-engineer` — Pricing page update: Free vs Pro side-by-side comparison (persistence, collaboration, premium compute)
- [x] `apps/web` — `/frontend-engineer` — **Data fetching & skeleton audit:** Scan all existing components in `apps/web/` for violations of the co-located query pattern, prop drilling, mismatched skeletons, missing skeletons, separate `*Skeleton.tsx` files (for simple cases), transforms outside `select`, and loading wrapper anti-patterns. Fix violations in-place. Reference: [data-fetching-strategy.md](strategy/data-fetching-strategy.md), [skeletons.md](rules/skeletons.md) *(PR #74 — no violations found, 5 ssr:false usages all justified)*

#### Wave 3 (sequential — test) — TABLED (March 2026)

**Moved to backlog.** Platform features are built and working. Test coverage deferred — editor MVP is the priority. See backlog section "Testing: Sprint 3 Deferred E2E Tests."

- [ ] ~~`apps/web` — Playwright E2E: AuthGate conversion flow~~
- [ ] ~~`apps/web` — Playwright E2E: browser-local execution history~~
- [ ] ~~`@bnto/backend` — Unit tests for execution analytics queries~~

#### Wave 4 (sequential — conversion data migration)

- [x] `@bnto/core` + `@bnto/backend` — `/core-architect` + `/backend-engineer` — **Execution history bug investigation + fix:** (1) Authenticated users may not be writing executions to Convex (only IndexedDB). Verify both write paths work. (2) Re-run button not rendering in execution history despite slug being present — investigate rendering/data issue. (3) **Local→Convex execution history migration on signup:** When a user signs up, read their browser-local execution history (IndexedDB) and batch-write it to Convex. Deduplicate by execution ID. Clear local history after successful migration. The `core.executions` layer should handle this transparently — detect auth state change (unauth→auth) and trigger migration automatically. No user action required. *(PR #78)*
- [x] `apps/web` — `/frontend-engineer` — Playwright E2E: **anonymous→signup conversion preserves history.** Full journey: (1) run a recipe as unauthenticated user, (2) verify execution appears in browser-local history, (3) sign up, (4) verify the same execution now appears in Convex-backed history on `/my-recipes`, (5) verify browser-local history is cleared. This is the C1→C2 conversion journey from `journeys/auth.md`. *(PR #78)*

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
| Wave 3 | `/reactflow-expert` + `/frontend-engineer` | — | ReactFlow Expert owns canvas interaction, connection validation. Frontend Engineer owns component composition (RecipeEditor, EditorToolbar, NodeConfigPanel, NodePalette), theming (Motorway tokens), and animation (Animate.* API) |

**Rule:** For ANY work touching ReactFlow APIs, graph state, canvas interaction, or the Definition ↔ Flow adapter layer — invoke `/reactflow-expert`. This persona is THE authority on `@xyflow/react` in this codebase. When visual skin work begins (Wave 3), invoke BOTH `/reactflow-expert` AND `/frontend-engineer` together.

#### Wave 1 (parallel — headless definition operations)

Pure functions that manipulate `Definition` trees. No React, no store, no UI. Fully testable in isolation. These are the atomic operations the editor performs.

- [x] `@bnto/nodes` — **`createBlankDefinition()`**: Returns a minimal valid `Definition` — root group node with one input port and one output port, no children. The "blank canvas" entry point.
- [x] `@bnto/nodes` — **`addNode(definition, nodeType, position?)`**: Inserts a new child node into the root group with default parameters from the schema. Auto-generates unique ID, creates default ports from `NODE_TYPE_INFO`. Returns new `Definition` (immutable — never mutate).
- [x] `@bnto/nodes` — **`removeNode(definition, nodeId)`**: Removes a node and all edges connected to it. Returns new `Definition`.
- [x] `@bnto/nodes` — **`updateNodeParams(definition, nodeId, params)`**: Merges new parameter values into a node's `parameters` object. Validates against `NodeSchema` (type checks, required fields, enum values, min/max). Returns new `Definition` or validation errors.
- [x] `@bnto/nodes` — **`moveNode(definition, nodeId, position)`**: Updates a node's `position`. Returns new `Definition`.
- [x] `@bnto/nodes` — **`definitionToRecipe(definition, metadata?)`**: Wraps a `Definition` into a `Recipe` with slug, name, description, accept spec. For export.
- [x] `@bnto/nodes` — **Unit tests for all CRUD operations**: Every function tested with all 10 node types. Edge cases: remove node (clean removal), update params with invalid values (validation errors), blank definition is valid, move node to new position, nested container operations.

#### Wave 2 (parallel — editor store + React hooks)

Zustand store that wraps the pure functions into a reactive state machine. Hooks provide the React binding layer. Still headless — no visual components. **`/reactflow-expert` leads** — owns the Definition ↔ Bento adapter design and Zustand store architecture.

- [x] `apps/web` — **`useEditorStore` (Zustand)**: Editor state: `definition` (current `Definition`), `selectedNodeId`, `isDirty`, `validationErrors[]`, `executionState` (per-node status map). Actions: `loadRecipe(slug)`, `createBlank()`, `addNode(type)`, `removeNode(id)`, `selectNode(id)`, `updateParams(nodeId, params)`, `moveNode(...)`, `resetDirty()`. All actions delegate to Wave 1 pure functions. Undo/redo via history stack (store snapshots).
- [x] `apps/web` — **`useEditorNode(nodeId)` hook**: Returns node data + schema + visible params (conditional visibility resolved). Subscribes to store slice — re-renders only when this node changes.
- [x] `apps/web` — **`useNodePalette()` hook**: Returns available node types from `NODE_TYPE_INFO`, grouped by category, with `browserCapable` flags. Filters server-only nodes based on context (browser editor = browser-capable only).
- [x] `apps/web` — **`useEditorExport()` hook**: Returns `{ exportAsRecipe, download }` — wraps current definition as a `Recipe` or triggers browser `.bnto.json` file download. Validates definition before export. Pure serialization — no visual dependency.
- [x] `apps/web` — **Definition ↔ Bento adapters**: `definitionToBento(definition)` → `{ nodes: CompartmentNodeType[] }`. `bentoToDefinition(nodes)` → `Definition`. Pure functions that bridge the headless model to the visual layer. Map node types to compartment variants, positions, and sizes. No edges — execution order derived from compartment position. Unit tested — round-trip: `definition → bento → definition` produces equivalent output.
- [x] `apps/web` — **Unit tests for store + hooks**: Store operations tested via Vitest (no rendering). Hook tests via `renderHook`. Adapter round-trip tests. Undo/redo verification.

#### Wave 3 (Lean MVP Editor — the priority)

**Goal: Ship the creation loop.** Users can load a recipe (or start blank), add/remove/configure nodes, run it, and export as `.bnto.json`. Functional, not polished. This is the product differentiator — what makes bnto more than another TinyPNG.

**Development home: Motorway `/editor` section.** All editor work lives in a dedicated "Editor" tab/section within the Motorway dev page (`app/(dev)/motorway/`). Iterate here until the editor is ready for production integration into a real route. The Motorway page imports real production components (never mock/fake versions).

**`/reactflow-expert` + `/frontend-engineer` co-lead.** ReactFlow Expert owns canvas interaction and node positioning. Frontend Engineer owns component composition and theming.

**Execution order:** (1) Motorway Editor section with canvas → (2) Toolbar + Palette + ConfigPanel → (3) RecipeEditor composition with run + export.

- [x] `apps/web` — **Motorway showcase component sharing**: `StationNode`, `ConveyorEdge`, `ConveyorCanvas`, `CompartmentNode`, and `BentoCanvas` extracted to shared location. Both Motorway showcase and production editor import from the same source.
- [x] `apps/web` — **Enable canvas interaction**: `BentoCanvas` accepts `interactive` prop for draggable/selectable/pannable mode with `onNodesChange` for controlled state. Backward compatible.
- [x] `apps/web` — **Motorway Editor section**: Editor tab in the Motorway dev page. Wraps `EditorProvider` + interactive `BentoCanvas` wired to `useEditorStore`. Recipe selector to load predefined recipes or create blank.
- [x] `apps/web` — **`EditorToolbar` component**: Action bar — recipe selector dropdown, Add Node, Remove Selected, Run, Export `.bnto.json`, Undo/Redo. Reads/dispatches to `useEditorStore`.
- [x] `apps/web` — **`NodePalette` component**: Slide-out panel listing available node types from `useNodePalette()`. Click-to-add. Grouped by category. Browser-capable badge.
- [x] `apps/web` — **`NodeConfigPanel` component**: Side panel that renders when a compartment is selected. Uses `useEditorNode(selectedNodeId)` to get schema + current params. Auto-generates form fields from `ParameterSchema` (Atomiton pattern). `visibleWhen` and `requiredWhen` handled reactively.
- [x] `apps/web` — **`RecipeEditor` component**: Composes `EditorToolbar` + `BentoCanvas` + `NodeConfigPanel`. Two entry modes: `<RecipeEditor slug="compress-images" />` (loads predefined) or `<RecipeEditor />` (blank canvas).

**Deferred from old Wave 3-4 (not MVP):**
- Execution state visualization on compartments (status-driven elevation/color) — nice polish, not needed for MVP
- Tag editor users in Convex (`hasUsedEditor`) — conversion tracking, not needed until save/accounts matter
- E2E tests — come after the editor works
- Undo/redo buttons in toolbar — store supports it, UI can wait
- `EditorModeToggle` placeholder — Sprint 4B concern

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

### Sprint 4C: Input & Output Nodes

**Goal:** Make recipes self-describing. Add `input` and `output` as first-class node types so the `.bnto.json` definition declares what a recipe needs and what it produces — no hardcoded UI per slug. This is the bridge between "predefined recipes with bespoke UI" and "custom recipes anyone can run."

**Required reading:** [io-nodes.md](.claude/strategy/io-nodes.md) — the full architecture reference. Covers the problem, node schemas, migration strategy, engine impact, UI impact, and open questions.

**Why now:** The editor (Sprint 4) lets users create recipes. But custom recipes can't run because the UI doesn't know what input to collect or how to present output. I/O nodes close this gap. They also eliminate per-slug UI hardcoding (6 config components + switch router + type map) and enable future community recipe sharing.

**Scope:** Two new node types in `@bnto/nodes`, updated predefined recipes, generic I/O renderers in `apps/web`, and wiring through `@bnto/core`'s browser adapter. No Rust engine changes — I/O nodes are declarations consumed by the environment, not the engine.

**Persona ownership:**
| Wave | Lead Persona | Supporting | Rationale |
|------|-------------|------------|-----------|
| Wave 1 | — (pure TS, no persona needed) | — | `@bnto/nodes` type definitions, schemas, recipe updates — framework-agnostic |
| Wave 2 | `/core-architect` | — | Browser adapter reads I/O nodes to configure execution. Core owns the bridge |
| Wave 3 | `/frontend-engineer` | `/reactflow-expert` | Generic I/O renderers replace hardcoded components. ReactFlow Expert advises on editor compartment rendering |
| Wave 4 | `/frontend-engineer` | — | Migration + E2E verification |

**Dependencies:** Sprint 4 Wave 1 (`@bnto/nodes` pure functions) must be complete — it is. Sprint 4 Wave 3 (editor components) should be complete for editor integration — it is. Sprint 4B (Code Editor) is NOT a dependency.

**Prerequisite:** RF store refactor complete (PRs #86-#94 merged). All waves are unblocked.

#### Wave 1 (parallel — node type definitions + schemas + recipe updates) `@bnto/nodes`

Define the two new node types, their parameter schemas, and update all 6 predefined recipes to use them. Pure TypeScript — no React, no UI, no engine changes.

**Internal ordering:** Tasks 1-5 (registry, schemas, helpers) can run in parallel. Tasks 6-8 (recipe updates, blank definition, tests) depend on 1-5 being complete.

- [ ] `@bnto/nodes` — **Add `input` and `output` to node type registry**: Add to `NODE_TYPES` constant, `NodeTypeName` union, `NODE_TYPE_INFO` metadata. New category: `"io"`. Both `browserCapable: true`, `isContainer: false`. See [io-nodes.md § Proposal](strategy/io-nodes.md#proposal-two-new-node-types) for the full type definitions.
- [ ] `@bnto/nodes` — **Input node schema** (`schemas/input.ts`): Parameters: `mode` (enum: `file-upload`, `text`, `url`), `accept` (array of MIME types), `extensions` (array), `label` (string), `multiple` (boolean, default true), `maxFileSize` (number), `maxFiles` (number), `placeholder` (string). `accept`/`extensions`/`label` visible when mode is `file-upload`. `placeholder` visible when mode is `text` or `url`.
- [ ] `@bnto/nodes` — **Output node schema** (`schemas/output.ts`): Parameters: `mode` (enum: `download`, `display`, `preview`), `filename` (string template), `zip` (boolean, default true), `label` (string), `autoDownload` (boolean, default false). `filename`/`zip`/`autoDownload` visible when mode is `download`.
- [ ] `@bnto/nodes` — **`deriveAcceptSpec(definition)`**: Pure function that finds the input node in a definition and extracts `accept`, `extensions`, `label` into an `AcceptSpec`. Returns `undefined` if no input node found. Used by `definitionToRecipe()` to populate `Recipe.accept` from the definition instead of duplicating it.
- [ ] `@bnto/nodes` — **`getInputNode(definition)` / `getOutputNode(definition)`**: Pure helper functions that find the input/output node in a definition's node tree. Return `Definition | undefined`. Used by adapters and renderers.
- [ ] `@bnto/nodes` — **Update all 6 predefined recipes**: Add input and output nodes to each recipe definition. Remove `file-system "list"` nodes that read `{{.INPUT_DIR}}/*` — the input node replaces them. Remove `{{.OUTPUT_DIR}}` from process node parameters — the output node declares delivery. Keep `Recipe.accept` populated via `deriveAcceptSpec()` for backward compatibility. See [io-nodes.md § Migration](strategy/io-nodes.md#migration-strategy) for the per-recipe mapping. *(Depends on tasks 1-5)*
- [ ] `@bnto/nodes` — **Update `createBlankDefinition()`**: Blank definitions start with an input node (mode: `file-upload`, accept: `*`) and an output node (mode: `download`). Users configure them from there. *(Depends on tasks 1-5)*
- [ ] `@bnto/nodes` — **Unit tests**: Validate input/output schemas. `deriveAcceptSpec()` extracts correct values. `getInputNode()`/`getOutputNode()` find nodes in flat and nested definitions. Updated recipes pass validation. `createBlankDefinition()` includes I/O nodes. Round-trip: `definition → recipe → definition` preserves I/O nodes. *(Depends on all above)*

#### Wave 2 (parallel — core adapter + editor store updates) `@bnto/core` + `apps/web`

Wire the browser adapter to read I/O nodes from the definition instead of relying on `Recipe.accept` and hardcoded output handling. Update the editor store to treat I/O nodes as persistent (always present, not deletable).

- [ ] `@bnto/core` — **Browser adapter reads input node**: The adapter receives the full `Definition` via the execution context. On run: `getInputNode(definition)` → if found, use its `accept`/`extensions` for file validation. If not found, fall back to `Recipe.accept`. The input node is the single source of truth for what files a recipe accepts.
- [ ] `@bnto/core` — **Browser adapter reads output node**: After execution, `getOutputNode(definition)` → if found, use its `mode`/`zip`/`autoDownload`/`label` for result delivery. If not found, current behavior (download all, zip if multiple). The output node is the single source of truth for how results are presented.
- [ ] `apps/web` — **Editor store: I/O node protection**: `useEditorStore.removeNode()` prevents deletion of input/output nodes (they're structural). `addNode()` prevents adding a second input or output node. I/O nodes are always the first and last in position order.
- [ ] `apps/web` — **Definition ↔ Bento adapters**: `definitionToBento()` maps input/output nodes to distinct compartment variants (visual differentiation in the grid — different color/icon). `bentoToDefinition()` preserves I/O node position constraints.

#### Wave 3 (parallel — generic UI renderers) `apps/web`

Build generic `InputRenderer` and `OutputRenderer` components that read the I/O node config and render the appropriate widget. These replace the hardcoded per-slug I/O.

- [ ] `apps/web` — **`InputRenderer` component**: Reads the input node from the current recipe definition. For `mode: "file-upload"`: renders `FileUpload.Dropzone` with `accept` and `extensions` from the node's parameters. For `mode: "text"` / `mode: "url"`: renders placeholder UI with "coming soon" message (forward compatibility). Props: `definition`, `onFilesChange`, `files`.
- [ ] `apps/web` — **`OutputRenderer` component**: Reads the output node from the current recipe definition. For `mode: "download"`: renders the `FileCard` grid + ZIP download using `label`, `zip`, and `autoDownload` from the node's parameters. For `mode: "display"` / `mode: "preview"`: renders placeholder UI. Props: `definition`, `results`, `onDownload`.
- [ ] `apps/web` — **Editor: I/O compartment rendering**: Input and output nodes render as distinct compartments in `BentoCanvas` — different variant color, appropriate icon (upload icon for input, download icon for output), always pinned to first/last position. `NodeConfigPanel` renders I/O node parameters from their schemas (mode selector, accept config, etc.).
- [ ] `apps/web` — **Unit tests for renderers**: `InputRenderer` renders dropzone for file-upload mode. `OutputRenderer` renders file cards for download mode. Both handle missing I/O nodes gracefully (fallback).

#### Wave 4 (sequential — migration + verification) `apps/web`

Migrate the recipe page to use generic renderers. Verify all 6 predefined recipes work with the new I/O nodes. Clean up hardcoded components.

- [ ] `apps/web` — **Migrate `RecipeShell`**: Replace hardcoded `FileUpload.Dropzone` with `InputRenderer`. Replace hardcoded `BrowserExecutionResults` with `OutputRenderer`. `RecipeShell` reads I/O config from the definition, not from slug-keyed maps.
- [ ] `apps/web` — **Migrate `useRecipeFlow`**: The hook reads input node config to determine accepted file types (for validation) and output node config to determine delivery behavior. Remove `BntoConfigMap` type and `DEFAULT_CONFIGS` — processing parameters stay in per-recipe config components for now, but I/O is driven by nodes.
- [ ] `apps/web` — **Deprecate per-slug I/O code**: Mark `RecipeConfigSection` switch router, `BntoConfigMap` type, and `DEFAULT_CONFIGS` as deprecated. They still handle processing parameters (quality, width, format) until NodeConfigPanel replaces them. Only the I/O portions are removed.
- [ ] `apps/web` — **E2E verification**: All 6 predefined recipes still work end-to-end — file upload, processing, results, download. Screenshot regression for recipe pages. Editor: create blank recipe → input/output nodes present → add process node → configure → run → results appear via OutputRenderer. Export `.bnto.json` → I/O nodes present in output.
- [ ] `apps/web` — **Validation**: `task ui:build` passes. `task ui:test` passes. `task e2e` passes. No TypeScript errors from updated types.

---

### Sprint 4D: UI Package Extraction (`@bnto/ui` / Motorway)

**Goal:** Extract `apps/web/components/ui/` into `packages/ui` (`@bnto/ui`) so the editor package (`@bnto/editor`, Sprint 4E) can import shared UI primitives without depending on the web app. This is the "second consumer" that justifies the extraction.

**Can run in parallel with Sprint 4C** (I/O Nodes). No shared files — 4C touches `@bnto/nodes` + `@bnto/core` + recipe pages, 4D touches `components/ui/` and package infrastructure.

**What moves:** Everything in `apps/web/components/ui/` (~53 files, ~62 total with subdirectories). This includes primitives (Button, Card, Dialog, etc.), composition components (Animate, BentoGrid, Grid), layout (Container, Row, Stack, AppShell), and utilities (createCn, cn, icons).

**What stays in `apps/web`:**
- `components/blocks/` — app-specific compositions (Navbar, Footer, RecipeCard, AuthGate, etc.)
- `components/editor/` — editor components (future `@bnto/editor`)
- `components/ThemeProvider.tsx`, `useTheme.ts` — app-level theme wiring
- `app/globals.css` — CSS token definitions (consumed by `@bnto/ui` via Tailwind `@source`)

**Persona ownership:**
| Wave | Lead Persona | Rationale |
|------|-------------|-----------|
| Wave 1 | `/frontend-engineer` | Package scaffolding, build config, exports |
| Wave 2 | `/frontend-engineer` | Move files, update all imports across `apps/web` |
| Wave 3 | `/frontend-engineer` | Verify build, tests, E2E, no regressions |

#### Wave 1 (sequential — package setup)

- [ ] `packages/ui` — **Scaffold `@bnto/ui` package**: `package.json` (name: `@bnto/ui`), `tsconfig.json`, barrel `index.ts`. Match build config pattern from `@bnto/core`. Add to `pnpm-workspace.yaml` if needed. Add `@bnto/ui` as dependency of `apps/web`.
- [ ] `packages/ui` — **Tailwind source directive**: Add `@source` in `apps/web/app/globals.css` pointing to `@bnto/ui` so Tailwind scans the extracted package for class names. See [gotchas.md](rules/gotchas.md#tailwind-v4--monorepo).
- [ ] `packages/ui` — **Export strategy**: Decide barrel vs direct imports. Primitives are leaf components — barrel re-export is fine for server components, but client components should be importable directly to avoid bundle bloat. Document the pattern.

#### Wave 2 (sequential — migration)

- [ ] `packages/ui` — **Move `components/ui/` files**: Move all files from `apps/web/components/ui/` to `packages/ui/src/`. Preserve directory structure (e.g., `FileUpload/` folder stays as-is).
- [ ] `apps/web` — **Update all imports**: Find-and-replace `@/components/ui/` → `@bnto/ui` (or the agreed import pattern) across all files in `apps/web/`. This includes `components/blocks/`, `components/editor/`, all page files, and any `_components/` folders.
- [ ] `packages/ui` — **Resolve internal dependencies**: `createCn.ts` imports from `tailwind-variants`, `cn.ts` imports from `clsx`/`tailwind-merge`. Ensure these are dependencies of `@bnto/ui`, not just `apps/web`. Move any shared type files that UI components depend on.
- [ ] `packages/ui` — **Handle theme tokens**: UI components reference CSS variables (`--background`, `--primary`, etc.) via Tailwind utilities. These are defined in `apps/web/app/globals.css`. Verify that `@bnto/ui` components work correctly when the CSS is provided by the consuming app (no bundled CSS in the package).

#### Wave 3 (sequential — verify)

- [ ] `apps/web` — **Build verification**: `task ui:build` passes. No TypeScript errors from updated imports. No missing module errors.
- [ ] `apps/web` — **Test verification**: `task ui:test` passes. `task e2e` passes. No visual regressions.
- [ ] `apps/web` — **Stale reference scan**: Grep for any remaining `@/components/ui/` imports — should be zero. Grep for any remaining `../ui/` relative imports that should now use `@bnto/ui`.
- [ ] `packages/ui` — **Package build**: `@bnto/ui` builds independently (`pnpm --filter @bnto/ui build`). Exports resolve correctly when imported from `apps/web`.

---

### Sprint 4E: Editor Package Extraction (`@bnto/editor`) — Planned

**Goal:** Extract `apps/web/components/editor/` into `packages/editor` (`@bnto/editor`). Depends on Sprint 4D (`@bnto/ui` must be a package first so `@bnto/editor` can import from it).

**Status:** Not yet scoped in detail. Will be planned after Sprint 4D ships. Key consideration: editor components import from both `@bnto/ui` (primitives) and `@bnto/core` (execution, store hooks). The extraction must preserve these dependency directions without creating circular imports.

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

### UX: Compartment Node Visual Redesign (Mini Motorways Buildings)

**Priority: High.** The current `CompartmentNode` renders as a flat colored card with centered text — all nodes look identical except for color. There's no visual personality, no sense of what each node *does*, and no connection to the Mini Motorways "building" metaphor. Nodes should be immediately identifiable at a glance, like buildings on a Mini Motorways map.

**Current state:** `CompartmentNode.tsx` renders a `.surface` Card with `label` + `sublabel` text. Variant colors exist (primary, secondary, accent, muted, success, warning) but are arbitrarily assigned. All nodes are the same 120×120 size. No icons. No category-driven visual identity.

**Goal:** Each compartment feels like a distinct building in a bento box. You can identify what a node does without reading its label. The canvas reads like a well-packed bento — varied compartment sizes, category-driven colors, icons as visual anchors. Execution state drives elevation changes with satisfying springy pops as compartments progress.

**Design principles:**
- **No edges/connections** — execution order = compartment position. The bento box metaphor is about spatial arrangement, not wiring
- **Elevation-driven execution state** — compartments physically rise and settle as they execute. The `.surface` Card system already supports this via the `elevation` prop with spring animations
- **Icons are the building silhouette** — large, centered, immediately communicates the node's purpose
- **Category = neighborhood** — nodes of the same category share a color, making the canvas scannable

**Phase 1: Icon registry + category color mapping**

Add a prominent icon to each node type and map categories to consistent variant colors. This is the biggest visual bang for effort — immediately makes every node recognizable.

Icon mapping (Lucide icons):

| Node Type | Icon | Visual metaphor |
|---|---|---|
| `image` | `ImageIcon` | Universal image symbol |
| `spreadsheet` | `Table` | Rows and columns |
| `file-system` | `FolderOpen` | File operations |
| `transform` | `Shuffle` | Data flowing between shapes |
| `edit-fields` | `PenLine` | Editing values |
| `http-request` | `Globe` | Network/external call |
| `shell-command` | `TerminalSquare` | Command line |
| `group` | `Braces` | Container/grouping |
| `loop` | `RefreshCw` | Iteration/cycling |
| `parallel` | `Columns3` | Concurrent lanes |

Category → color mapping:

| Category | Variant | Rationale |
|---|---|---|
| `image` | `primary` (terracotta) | Hero category, warm and prominent |
| `spreadsheet` | `secondary` (teal) | Cool counterpoint, data-oriented |
| `file` | `accent` (golden) | Foundation operations |
| `data` | `muted` (warm off-white) | Background/subtle operations |
| `network` | `secondary` (teal) | External connections |
| `control` | `warning` (warm orange) | Orchestration, meta-level |
| `system` | `muted` (warm off-white) | Power-user, understated |

Node anatomy (Phase 1):
```
┌─────────────────────┐
│                     │
│     [icon 32px]     │  ← Large category icon, muted foreground
│                     │
│      Image          │  ← Label (font-display, semibold, sm)
│      image          │  ← Sublabel (font-mono, xs, muted)
│                     │
└─────────────────────┘
```

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

### Chore: Codebase File Size & Structure Audit

**Priority: High.** Code standards have been tightened (March 2026): files target 50-100 lines (hard cap 250), functions get their own file if reused or more than a few lines, components with more than 2-3 sub-components break into folder + barrel. The existing codebase predates these tighter limits and needs a sweep.

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

**Milestone: M4/M5.** Quota enforcement only applies to server-side bntos. Browser bntos are free unlimited. This race condition matters when server-side execution has limits.

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
| `.claude/strategy/go-engine-migration.md` | Go engine node inventory — migration reference before archive deletion |
| `.claude/strategy/cloud-desktop-strategy.md` | Architecture, cost analysis, cloud execution topology |
| `.claude/strategy/core-principles.md` | Trust commitments, "For Claude Code" guidance |
| `.claude/rules/` | Auto-loaded rules (architecture, code-standards, components, etc.) |
| `.claude/skills/` | Agent skills (pickup, project-manager, code-review, pre-commit) |
| Notion: "SEO & Monetization Strategy" | Pricing, revenue projections, quota limits |
