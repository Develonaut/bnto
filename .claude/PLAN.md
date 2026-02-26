# Bnto — Build Plan

**Last Updated:** February 26, 2026
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

**Co-location decision (Feb 2026):** UI components and editor features live in `apps/web` for now. No separate `@bnto/ui` or `@bnto/editor` packages until there's a real second consumer (desktop app). Engine, core API, and data layer logic stays in `@bnto/core`. When the UI package is extracted, it will be published as `@bnto/ui` (npm) under the name **Motorway** — the Mini Motorways-inspired design system (depth, pressable, spring animations, warm palette).

---

## Current State

- **Active:** Sprint 2C (Launch Readiness) — make bnto.io presentable for real users and crawlers
- **Next:** Sprint 2D (Recipe Page UX Overhaul) — transform tool pages from flat config-first stacks into progressive phase-driven flow
- **M1 delivered (Feb 2026):** All 6 Tier 1 bntos run 100% client-side via Rust→WASM. Uniform Rust engine — no JS fallback. Files never leave the user's machine. Rust evaluation checkpoint PASSED.
- **Cloud pipeline:** Complete. Go API on Railway, R2 file transit, Convex real-time subscriptions — all verified end-to-end with integration E2E tests. This is M4 infrastructure delivered ahead of schedule.
- **WASM engine:** 6 Rust crates (`bnto-image`, `bnto-csv`, `bnto-file`, `bnto-core`, `bnto-wasm`). Single cdylib entry point. 1.6MB raw / 606KB gzipped. Web Worker wrapper with progress reporting.
- **Auth:** Migrated to `@convex-dev/auth`. Anonymous sessions create real `users` rows. Integration tests complete. `AUTH_SECRET` env var required in Convex deployments.
- **Go engine:** Complete. CLI with 10 node types (>90% coverage), HTTP API on Railway, BntoService shared API layer. Paused for web — browser execution is M1 priority. Ready for M3 (desktop) and M4 (premium server-side).
- **Web app:** Next.js on Vercel. Auth, SEO tool pages, execution UI built. Landing pages still use Mainline template placeholder content — Sprint 2C replaces all of it.
- **Launch blocker:** bnto.io domain not yet connected to Vercel. ~25 files of Mainline template content (hero, features, testimonials, pricing, about, contact) reference a project management tool, not bnto. Must be replaced before real users or crawlers see the site.
- **Packages:** `@bnto/core` (layered singleton), `@bnto/auth` (`@convex-dev/auth` wrappers), `@bnto/backend` (Convex schema + functions), `@bnto/nodes` (engine-agnostic definitions). UI co-located in `apps/web/components/`.

---

## What's Built (don't redo)

- [x] Monorepo: Turborepo + pnpm + Taskfile.dev + go.work
- [x] Go engine: 10 node types, orchestration, validation, storage, secrets, path resolution
- [x] Go API server: HTTP handlers wrapping BntoService (apps/api/), deployed to Railway
- [x] Contract tests: Go JSON responses match @bnto/core TypeScript types
- [x] @bnto/core: Layered singleton (clients → services → adapters), React Query + Convex adapter, 38 hooks
- [x] @bnto/auth: `@convex-dev/auth` integration (migrated from Better Auth — see decisions/auth-evaluation.md)
- [x] @bnto/backend: Convex schema (users, workflows, executions, executionLogs), auth, crons, run counter fields
- [x] Web app: Landing pages (Mainline template — **being replaced in Sprint 2C** with real bnto content)
- [x] Web app: Auth flow (sign-in, sign-up, sign-out with signal cookie, proxy route protection)
- [x] Web app: SEO infrastructure (bnto-registry.ts, [bnto]/page.tsx, generateStaticParams, generateMetadata, BntoJsonLd, sitemap.ts, llms.txt, robots.txt)
- [x] Web app: Middleware (canonical URL normalization, auth routing, bnto slug pass-through)
- [x] All 6 Tier 1 fixtures in engine/tests/fixtures/workflows/
- [x] Playwright E2E: user journeys, visual parity tests, execution flow tests (9 tests, 8 screenshots)
- [x] Execution UI: RunButton, ExecutionProgress (real-time), ExecutionResults (R2 download)
- [x] Predefined execution path: BntoPageShell → useRunPredefined → Convex startPredefined → Railway Go API
- [x] Download infrastructure: @bnto/core download client/service/adapter/hook chain
- [x] R2 infrastructure: buckets (dev + prod), presigned URLs, CORS, env vars across Convex/Vercel/Railway
- [x] Auth integration tests: anonymous execution (A1-A7), conversion flow (C1-C3), auth lifecycle (S1-S3) — see journeys/auth.md
- [x] Codebase polish: Node.js subpath imports, PascalCase components, camelCase hooks, dot-notation primitives, layout/typography primitives, CSS animation system

---

## Revenue & Monetization Context

Pricing, revenue projections, and "ready to charge" criteria live in Notion ("SEO & Monetization Strategy").

**Monetization model (updated Feb 2026):** Browser execution is free unlimited. Pro sells real value — persistence, collaboration, premium compute. See ROADMAP.md for the full model.

| Sprint | What Ships | Revenue Implication |
|--------|-----------|---------------------|
| Sprint 2B | Browser execution (M1 MVP) | **All Tier 1 bntos run client-side.** Zero backend cost. Files never leave user's machine. |
| Sprint 2C | Launch readiness (content + domain) | **bnto.io live and indexable.** Real content on every page. SEO crawling begins. First real users possible. |
| Sprint 2D | Recipe page UX overhaul | **Core product experience matches marketing quality.** Progressive phase-driven flow. Motorway design language on every tool page. |
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

### Sprint 2A: Auth Fix — Waves 1-3 COMPLETE
Evaluated `@convex-dev/auth` vs fixing Better Auth. Chose `@convex-dev/auth` (eliminates JWT race condition). Full migration: auth provider, anonymous sessions with real `users` rows, proxy middleware, AppGate removal. Integration tests: anonymous execution (A1-A7), conversion flow (C1-C3), auth lifecycle (S1-S3). See `decisions/auth-evaluation.md`.

### Sprint 2.5: Codebase Polish — COMPLETE
Node.js subpath imports (`#components/*`, `#lib/*`), camelCase file rename (hooks, utils, lib), PascalCase component rename, dot-notation primitive wrappers, Button audit/migration, Button pseudo-state fix, Button animations (Mini Motorways motion language). Font review (DM Sans → Geist evaluation) deferred to backlog.

### Sprint 2B: Browser Execution (M1 MVP) — COMPLETE
All 6 Tier 1 bntos running 100% client-side via Rust→WASM. `@bnto/nodes` package (engine-agnostic definitions), Rust workspace with 5 crates, Web Worker wrapper, browser adapter in `@bnto/core`, BntoPageShell browser routing, ZIP download for multi-file results. Rust evaluation checkpoint PASSED. WASM bundle: 1.6MB raw / 606KB gzipped. 44+ Rust unit tests, WASM integration tests, Playwright E2E with screenshot assertions for all 6 bntos. **M1 milestone delivered.**

---

## Phase 1: Web App + Cloud Execution (continued)

### Sprint 2A: Auth Fix — Waves 4-5 (ACTIVE)

#### Wave 4 (sequential — core integration tests against real Convex dev)

**Why this layer exists:** `convex-test` (Wave 3) validates logic in-memory. Playwright E2E (Wave 5) validates user journeys through a browser. This layer sits between — it calls real Convex dev functions through `@bnto/core`'s imperative API without browser overhead. It catches: wrong env vars, missing indexes, auth provider misconfiguration, schema migration issues, R2 connectivity.

**Auth approach:** `ConvexHttpClient` calls `@convex-dev/auth`'s `signIn` action directly — returns a JWT token. Set it with `client.setAuth(token)` and all subsequent calls are authenticated. No React, no browser. See `decisions/core-integration-testing.md`.

**Test infrastructure:** Requires `task dev:all` (Convex dev + Go API via tunnel + R2 dev bucket).

- [x] `@bnto/core` — **Test harness setup:** `ConvexHttpClient` factory with anonymous + password auth, test lifecycle helpers (cleanup), Vitest integration config (separate from unit tests, longer timeouts). File: `packages/core/src/__tests__/integration/setup.ts`
- [x] `@bnto/core` — **Auth integration tests:** Anonymous sign-in, authenticated client calls, unauthenticated rejection, password sign-up/sign-in, anonymous → password upgrade (userId NOT preserved via ConvexHttpClient — browser E2E needed). Files: `auth-lifecycle.test.ts` (S1-S3, 16 tests), `conversion-flow.test.ts` (C1-C3, 14 tests). Also fixed: `@convex-dev/auth` v0.0.90 response parsing in `setup.ts`, broken `convex.config.ts` component import, `auth.config.ts` provider config, deprecated Vitest `poolOptions`
- [x] `@bnto/core` — **Execution integration tests:** `core.executions.startPredefined()` against real Convex dev — creates execution record, increments runsUsed, enforces quota. Verify status transitions (pending → running → completed/failed) via polling. Also fixed: unified run limit system (env-driven via `_helpers/run_limits.ts`, single source of truth for anonymous=3, free=25). File: `execution.test.ts` (17 tests)
- [x] `@bnto/core` — **Upload/download integration tests:** `core.uploads.generateUrls()` returns valid R2 presigned URLs, upload succeeds, after execution `core.downloads.getDownloadUrls()` returns valid download URLs. Full transit: upload → execute → download against R2 dev bucket. **Coverage gap:** The full file pipeline (browser → R2 → Go engine → R2 → browser) is untested end-to-end at the integration layer. This is the highest-priority gap in our test coverage.

#### Wave 5 (sequential — verify full pipeline in browser)

**Note on Sprint 2 stash:** `git stash@{0}` contains Sprint 2 Wave 5 work (integration test spec, playwright config, data-session attributes). This was written against the old Better Auth system — **review and adapt, don't blindly pop.** The test structure may be reusable but auth wiring is obsolete.

- [x] `apps/web` — Playwright E2E integration tests: full pipeline (upload → R2 → Go engine → R2 → download) using shared engine test fixtures. Separate `playwright.integration.config.ts` targets `task dev:all` on port 4000. Progress-aware test helpers added (`waitForExecutionStatus`, `waitForPhase`, `captureTransientPhase`, `captureUploadProgress`). Data attributes added to UploadProgress, ExecutionProgress, ExecutionResults for observability.
- [x] `apps/web` — **CRITICAL: Anonymous → password userId preservation (C1-C2).** ConvexHttpClient integration tests proved userId is NOT preserved (new user created on upgrade). Browser E2E tests confirmed the same. **RESOLVED in Sprint 3 Wave 1** via `PasswordWithAnonymousUpgrade` wrapper in `auth.ts` — wraps Password's authorize to extract anonymous userId from JWT in action context, passes through profile to `createOrUpdateUser`. All 3 E2E tests pass (C1-C3).
- [ ] `apps/web` — **Browser auth behavior verification:** Token expiry handling, sign-out session invalidation (JWT is stateless — browser relies on cookie clearing + proxy redirect). ConvexHttpClient can't test this — Playwright E2E required. **Moved to Sprint 3 (M2: Platform Features) — not blocking M1 browser execution.**
- [x] `apps/web` — **Monetization checkpoint:** Confirm execution events log `userId`, `bntoSlug`, `timestamp`, `durationMs` to Convex. **VERIFIED:** All 4 fields captured in `executionEvents` table. Run quota fields on user doc. `enforceQuota()` checks before execution. **Note:** Run quota enforcement applies to server-side bntos (M4). Browser bntos are free unlimited per ROADMAP.md.
- [ ] `apps/web` — Verify auth flow end-to-end on Vercel preview deployment — **Moved to backlog (not blocking M1)**

> **SEO checkpoint:** Before closing Sprint 2, verify in browser devtools that each `/[bnto]` URL returns correct `<title>` and `<meta description>` in the page source (not client-rendered).

> **Monetization checkpoint:** Confirm execution events are being written to Convex with `userId`, `bntoSlug`, `timestamp`, and `durationMs`. Sprint 3 builds the usage dashboard on top of this data.

---

### Sprint 2.5: Codebase Polish — COMPLETE

#### Wave 3 (parallel — button polish) — COMPLETE

- [x] `apps/web` — Fix Button pseudo-state bug: after active/click, button returns to hover state instead of default resting state. CSS `:active` → `:hover` transition fixed in depth/pressable system
- [x] `apps/web` — Button animations per Mini Motorways motion language (see `animation.md`): entrance spring for button appearance, smooth ease-out for press/release transitions, `motion-safe:` guards

**Remaining polish moved to backlog (UI: Button Polish, Font Review).** Build verification and E2E screenshots for button changes are included in Sprint 2B E2E suite.

---

### Sprint 2B: Browser Execution (M1 MVP)
**Goal:** All 6 Tier 1 bntos run 100% client-side in the browser. Zero backend for core experience. "Your files never leave your computer." This is the M1 milestone deliverable.

**Strategic context:** Browser execution costs us $0 and gives users unlimited free runs. Cloud execution (already built in Sprint 2/2A) becomes premium infrastructure for M4 (AI, shell, video). See ROADMAP.md for the full browser-first strategy.

#### Wave 1 (parallel — `@bnto/nodes` foundation)

The engine-agnostic package that any execution target (Rust WASM, JS, Go) consumes. Build this first — it's the safety net regardless of engine choice.

- [x] `packages/@bnto/nodes` — Create package: node type definitions (what each node does, input/output types)
- [x] `packages/@bnto/nodes` — Migrate recipe definitions from `engine/pkg/menu/recipes/` to TypeScript (6 Tier 1 recipes)
- [x] `packages/@bnto/nodes` — Define input/output schemas per node type (drives config panel UI — Atomiton `createFieldsFromSchema` pattern)
- [x] `packages/@bnto/nodes` — Workflow validation functions (works in browser, CLI, desktop)
- [x] `packages/@bnto/nodes` — Unit tests for schemas and validation

#### Wave 2 (sequential — first Rust WASM node + evaluation checkpoint)

Build one node in Rust, compile to WASM, run in a Web Worker. This is the M1 evaluation — by the end of this wave, we know if Rust works for us.

- [x] `engine-wasm/` — Set up Rust workspace with `wasm-pack` + `wasm-bindgen` targeting web
- [x] `engine-wasm/` — Build `compress-images` node in Rust (`image` crate with JPEG/PNG/WebP codecs, 44 unit tests, WASM bridge ready for unified entry point)
- [x] `apps/web` — Web Worker wrapper with progress reporting via `postMessage`
- [x] `packages/core` — Browser adapter: detect browser runtime, route execution to WASM Web Worker
- [x] `apps/web` — Wire BntoPageShell to use browser adapter for `/compress-images`
- [x] `apps/web` — E2E test: compress-images runs entirely client-side (no backend required)

**EVALUATION CHECKPOINT — PASSED (Feb 2026)**

| Question | Result |
|----------|--------|
| Development velocity | **PASS** — Each node built faster than the last. Patterns established early. |
| WASM boundary | **PASS** — ArrayBuffer transfers work cleanly. Web Worker wrapper handles File/Blob. |
| Bundle size | **ACCEPTABLE** — 606KB gzipped for all 6 nodes in single bundle. Above 500KB target by ~20%, but reasonable for 6 full nodes. |
| Ecosystem coverage | **PASS** — `image`, `csv`, `serde`, `regex` crates cover all Tier 1 needs. |
| Developer experience | **PASS** — wasm-pack builds fast, errors are debuggable, Rust tooling solid. |

**Decision: Rust is the uniform browser engine.** All 6 Tier 1 bntos built in Rust, including `rename-files` (originally planned as JS). Continued to Wave 3a (remaining Rust nodes).

#### Wave 2c (parallel — browser execution testing hardening)

Harden the browser execution stack with layered test coverage. Goal: "it just works" confidence from Rust unit tests through full E2E user journeys. See `journeys/browser-execution.md` for the full testing strategy.

**Rust edge cases** (truncated, corrupt, zero-byte inputs):
- [x] `engine-wasm/` — Rust unit tests: truncated file (< 10 bytes), corrupt magic bytes, zero-byte file
- [x] `engine-wasm/` — Rust unit tests: quality param bounds (0, 1, 100, 101), 1x1 pixel image

**WASM boundary coverage:**
- [x] `engine-wasm/` — WASM integration tests: all codec combinations (JPEG→JPEG, PNG→PNG, WebP→WebP)
- [x] `engine-wasm/` — WASM integration tests: progress callback fires with increasing percentages
- [x] `engine-wasm/` — WASM integration tests: large file (1MB+) doesn't OOM, output always <= input

**E2E 4-phase tests** (BEFORE → PROGRESS → FINISH → VERIFY for each bnto):
- [x] `apps/web` — E2E: compress JPEG with 4-phase screenshots + download verification
- [x] `apps/web` — E2E: compress PNG with 4-phase screenshots + download verification (PNG magic bytes)
- [x] `apps/web` — E2E: compress WebP with 4-phase screenshots + download verification (RIFF/WEBP header)
- [x] `apps/web` — E2E: unsupported file (.txt) shows user-friendly error, no crash
- [x] `apps/web` — E2E: corrupt image shows error card, Run Again available
- [x] `apps/web` — E2E: quality slider comparison (q=50 vs q=90, verify q=50 < q=90)
- [x] `apps/web` — E2E: batch 5+ files with progress, all outputs valid

**TS service/worker coverage:**
- [x] `packages/core` — browserExecutionService: engine init failure → clean error state
- [x] `packages/core` — browserExecutionReducer: batch progress fileIndex increments correctly
- [x] `apps/web` — BntoWorker: process after terminate → clear error

#### Wave 3a: Remaining Rust WASM nodes (uniform Rust engine — checkpoint passed)

- [x] `engine-wasm/` — `resize-images` node (Lanczos3/CatmullRom filters via `image` crate)
- [x] `engine-wasm/` — `convert-image-format` node (decode any → encode any via `image` crate)
- [x] `engine-wasm/` — `clean-csv` node (`csv` + `serde` crates, new `bnto-csv` crate)
- [x] `engine-wasm/` — `rename-csv-columns` node (`csv` + `serde` crates, header rewrite)
- [x] `engine-wasm/` — `rename-files` node (pattern matching + regex via Rust)
- [x] `apps/web` — Web Worker wrappers for all new WASM nodes
- [x] `apps/web` — E2E tests: all 6 bntos run client-side

#### Wave 4 (sequential — integration + polish) — COMPLETE

- [x] `apps/web` — BntoPageShell routes through browser adapter for Tier 1 bntos (done in Wave 2)
- [x] `apps/web` — Zip download for multi-file browser results (fflate, createZipBlob, E2E verified)

**Sprint 2B COMPLETE.** M1 (Browser Execution) delivered. All 6 Tier 1 bntos run client-side. WASM bundle: 1.6MB raw / 606KB gzipped. Performance benchmarks moved to backlog — not blocking M1 delivery.

---

### Sprint 2C: Launch Readiness
**Goal:** bnto.io is live, real, and indexable. Every page a crawler or user sees has genuine bnto content. All Mainline template placeholder content is gone. The site is presentable for SEO indexing and first real users.

**Context:** M1 (browser execution) is delivered — all 6 Tier 1 bntos work. But the web app still shows ~25 files of Mainline template content (hero about "modern product teams", fake testimonials from "Mercury Finance", pricing for "500 issues", investor photos, etc.). This must be replaced before bnto.io goes live. The tool pages (`/compress-images`, `/clean-csv`, etc.) are already real — the marketing wrapper around them is not.

**Design reference:** `/Users/ryan/Code/shadcn-blocks/blocks/` — extensive library of production-quality block components. Browse relevant categories (hero, feature, pricing, faq, footer, navbar, logos, stats, cta, trust-strip) for patterns and inspiration. Adapt layout and composition patterns to fit our design system — don't copy styling blindly.

#### Wave 1 (parallel — metadata + navigation + domain)

- [x] `apps/web` — **Root metadata overhaul:** Replace all "Mainline" metadata in `layout.tsx` — title ("bnto — Compress, Clean & Convert Files Free"), description, keywords (compress images, clean csv, rename files, convert format, free online tools), OG tags, Twitter card. Authors = "bnto". Remove shadcnblocks.com references.
- [x] `apps/web` — **Navbar simplification:** Strip Mainline nav items (Features dropdown, About, Pricing, FAQ, Contact). Keep: Logo, tool page links or "Tools" anchor, GitHub button (pointed at real bnto repo — we're MIT open source), theme toggle, Sign In. Fix dead `/login` link → `/signin`. Reference `shadcn-blocks/blocks/navbar/navbar8.tsx` for structure — clean, fixed top, responsive mobile sheet.
- [x] `apps/web` — **Footer rewrite:** Replace template footer. Real links: Tools (all 6 bnto slugs), GitHub repo, MIT license note. Remove fake social links (`@ausrobdev`). Simple and honest — no fake company sections. Browse `shadcn-blocks/blocks/footer/` for patterns.
- [x] `infra` — **Connect bnto.io to Vercel:** Add CNAME/A records in Cloudflare DNS. Configure custom domain in Vercel dashboard. Verify HTTPS + deployment. Update `robots.txt` sitemap URL if domain changes from `bnto.dev`.

#### Wave 2 (parallel — clean false claims + delete template garbage)

**Messaging rule for this entire sprint:** "Browser tools" are free. Not "everything." Every claim about "free" must be scoped to browser tools. Never say "free forever" as a blanket. Never say "no upsells." Never promise unshipped features (desktop, CLI, AI tools, Pro tier pricing). See `.claude/plans/partitioned-tumbling-wigderson.md` Part 2 for the full messaging framework.

- [x] `apps/web` — **HomeTimeline.tsx — Remove 3 unshipped sections:** Delete "Recipes" section (CLI terminal animation + "visual editor coming soon"), "Coming soon" section (6 fake tools with names/icons), and "Desktop" section (4 feature cards for nonexistent app). Keep only "How it works" and "Open source" sections. Remove `CodeLayout`, dead `INNER_RING`/`OUTER_RING` data for unshipped platforms (Desktop/CLI/Cloud) and tools (AI/Zip/JSON). Delete unused imports (`Terminal`, `AnimatedSpan`, `TypingAnimation` if orphaned).
- [x] `apps/web` — **HomeTimeline.tsx — Fix BragCard claim:** Change `"Free forever — no upsells"` label to `"Free browser tools — no limits"`. The old copy directly contradicts the planned Pro tier.
- [x] `apps/web` — **BntoGallery.tsx — Fix pitch point:** Change `"Desktop app coming soon — same tools, offline"` to `"Batch processing — drop multiple files at once"`. Desktop is not shipped.
- [x] `apps/web` — **FAQ.tsx — Fix 3 answers:** (1) Remove "More formats are on the way." from file types answer. (2) Rewrite "What is Pro?" → "Will bnto always be free?" / "All browser tools are free, unlimited, forever. We plan to add a paid tier in the future for features that need server-side processing — like AI-powered tools. Browser tools will never have limits or require payment." (3) Rewrite "Do I need an account?" → "No. Drop your files and use any tool immediately. No account, no signup, no email required."
- [x] `apps/web` — **copy.ts — Fix 2 constants:** `LICENSE_LINE` → "MIT Licensed. Browser tools free forever." `TRUST_LINE` → "Free, instant, private. Processed in your browser."
- [x] `apps/web` — **Navbar.tsx — Remove Motorway button** from desktop and mobile nav.
- [x] `apps/web` — **Move /motorway to /dev/motorway:** Create `app/(dev)/dev/motorway/page.tsx`, move content from `app/(app)/motorway/page.tsx`, delete old file. The `(dev)` route group keeps it accessible for internal use at `/dev/motorway` without cluttering the `(app)` shell.
- [x] `apps/web` — **not-found.tsx — Verify GitHub URL** matches `GITHUB_URL` from `copy.ts` (`github.com/Develonaut/bnto`).
- [x] `apps/web` — **Delete orphaned component files:** `Terminal.tsx` and `OrbitingCircles.tsx` (only used by deleted layouts).

#### Wave 3 (parallel — write real content for home page + pricing)

- [x] `apps/web` — **HomeTimeline.tsx — Rewrite Section A** ("How it works"): subtitle "How it works", title "Your browser does the work.\nNot a server.", body about browser-side processing vs server upload. Layout: keep BragLayout (stat cards + comparison bars). Competitive differentiator vs TinyPNG/CloudConvert.
- [x] `apps/web` — **HomeTimeline.tsx — Rewrite Section B** ("No catch"): subtitle "No catch", title "Free tools that stay free.\nOpen source you can verify.", body about no signup/watermarks/caps. CTA: "View on GitHub" → GITHUB_URL. Layout: new TrustLayout — Card with crossed-out anti-patterns + "MIT Licensed · Open Source" badge.
- [x] `apps/web` — **HomeTimeline.tsx — Replace orbit visualization** with TrustLayout card (crossed-out anti-patterns + GitHub CTA).
- [x] `apps/web` — **Pricing.tsx — Complete rewrite:** Single "Free" tier card. Heading: "Simple pricing." Subheading: "Every browser tool is free. No limits, no signup, no catch." 8 feature bullets. CTA: "Start using bnto" → `/`. Below card: future premium note. Delete all "Mainline" template content. Server component.
- [x] `apps/web` — **PricingTable.tsx — Deleted:** Removed template comparison table file entirely.
- [x] `apps/web` — **pricing/page.tsx — Add metadata:** title "Pricing", description "All browser tools free, unlimited, forever. No signup required."
- [x] `apps/web` — **Navbar.tsx — Add "Pricing" link** to desktop nav and mobile sheet.

#### Wave 4 (sequential — polish, navigation, verify)

- [x] `apps/web` — **Footer.tsx — Add Company column:** Pricing → `/pricing`, Privacy → `/privacy`, GitHub → `GITHUB_URL`.
- [x] `apps/web` — **sitemap.ts — Add missing pages:** `/pricing` (priority 0.6), `/faq` (priority 0.5), `/privacy` (priority 0.3).
- [x] `apps/web` — **Messaging audit (grep):** Verified: "free forever" only appears as "Browser tools free forever", "coming soon" zero results, "desktop" in marketing copy zero, "$8" or "Pro ($" zero, "Mainline" zero, "no upsells" zero.
- [x] `apps/web` — **E2E: home page renders correctly.** Dropped — E2E is strictly for user journeys (auth, recipe execution). Home/pricing page rendering is verified via `task ui:build` + SEO metadata E2E tests.
- [x] `apps/web` — **E2E: pricing page.** Dropped — same rationale. Pricing page verified via build + manual QA.
- [x] `apps/web` — **E2E: no template content anywhere.** Grep built output for "Mainline", "shadcnblocks", "Mercury", "free daily catered lunch" — zero matches. Only "Example Site" remains in privacy.mdx (flagged below).
- [x] `apps/web` — **SEO verification:** `task ui:build` passes clean (15/15 static pages). All pages generate: `/`, `/pricing`, `/faq`, `/privacy`, `/signin`, all 6 tool slugs, `/dev/motorway`, `sitemap.xml`. SEO metadata tests (`seo-metadata.spec.ts`) cover all Tier 1 slugs.
- [x] `apps/web` — **Privacy policy audit:** Privacy policy (`privacy.mdx`) still says "Example Site" (3 occurrences), "www.example.com", "example@example.com", "California, United States", last updated "April 07, 2021". **Follow-up task added to Sprint 3 Wave 1.**
- [x] `apps/web` — **Convert JS animations to CSS/Tailwind:** `ComparisonBar` refactored to pure CSS — removed `"use client"`, `useState`, `useEffect`, `setTimeout`. Now uses `data-active` attribute + CSS `transition` + `--bar-width` variable. `AnimatedCounter` refactored to use direct DOM writes (`ref.textContent`) instead of `useState` — eliminates ~72 React re-renders per animation. RAF kept (CSS can't display animated integers). `AnimatedBar` and `AnimatedRing` don't exist in the codebase (already removed).

---

### Sprint 2D: Recipe Page UX Overhaul
**Goal:** Transform `[bnto]` tool pages from flat config-first stacks into a progressive phase-driven flow matching the Motorway design language. Users experience: drop files -> rich preview -> configure (sensible defaults) -> run -> results. Every recipe page feels as polished as the home page.

**Context:** M1 (browser execution) is delivered and Sprint 2C made the marketing pages presentable. But the actual tool pages — the core product experience — still feel like dev prototypes. All 6 recipes share the same pattern (file upload -> configure -> run -> download), so a unified UX pattern benefits all of them.

#### Wave 1 (parallel — foundation: hook extraction + new UI components)

Three independent pieces the new shell will compose. No cross-dependencies.

- [ ] `apps/web` — **Extract `useRecipeFlow` hook from BntoPageShell.** Move all state management (files, config, browser/cloud execution, phase derivation, handleRun, handleReset) into `app/[bnto]/_hooks/useRecipeFlow.ts`. The hook accepts `{ entry: BntoEntry }` and returns the full state + actions currently scattered across BntoPageShell. After extraction, `BntoPageShell` imports the hook and renders identically (drop-in, no visual change). Verify `task ui:build` passes. This task is pure extraction — no rendering changes.

- [ ] `apps/web` — **Create `PhaseIndicator` component.** New file: `app/[bnto]/_components/PhaseIndicator.tsx`. Three phases: "Files" (1), "Configure" (2), "Results" (3). Props: `activePhase: 1 | 2 | 3`, `hasConfig?: boolean` (skip phase 2 label when false). Visual: horizontal row of circles connected by lines — active = `bg-primary`, completed = checkmark, upcoming = `bg-muted` outline. Responsive: circles + labels on desktop, circles only on mobile. Under 100 lines.

- [ ] `apps/web` — **Create `FileCard` component.** New file: `app/[bnto]/_components/FileCard.tsx`. Rich file preview: image thumbnail (via `URL.createObjectURL` with cleanup), file type icon (for non-images), file name (truncated), file size (formatted), type badge ("JPEG", "CSV"), delete button. Wraps in `Card` with `depth="sm"`. Props: `file: File`, `onRemove: () => void`, `disabled?: boolean`. Uses `Animate.ScaleIn` entrance. Under 120 lines.

#### Wave 2 (parallel — new shell composition + config/results wrappers)

Build the new `RecipeShell` and supporting wrappers. Depends on Wave 1 deliverables.

- [ ] `apps/web` — **Create `RecipeShell` composition component.** New file: `app/[bnto]/_components/RecipeShell.tsx`. Replaces BntoPageShell as the page orchestrator. Uses `useRecipeFlow` hook (Wave 1). Composition: `Container` > `Heading` + `Text` + `PhaseIndicator` + dropzone in `Card depth="md"` + file card `Grid` with `Animate.Stagger` + config section + `RunButton` + results section. All data-testid attributes preserved for E2E. Under 150 lines (pure composition, no logic). NOT wired into page.tsx yet.

- [ ] `apps/web` — **Create `RecipeConfigSection` wrapper.** New file: `app/[bnto]/_components/RecipeConfigSection.tsx`. Replaces `BntoConfigPanel` with Motorway styling. Wraps per-recipe config components in `Card` with `depth="sm"` + collapsible `Accordion` (default open). The slug-to-component routing (switch statement) moves here from BntoConfigPanel. Returns null for slugs with no config. Uses `Animate.FadeIn` entrance. Under 80 lines.

- [ ] `apps/web` — **Create `RecipeResultsSection` wrapper.** New file: `app/[bnto]/_components/RecipeResultsSection.tsx`. Consolidates the 4 conditional result blocks (browser progress, browser results, cloud progress, cloud results, error card) from BntoPageShell into one composition component. Wraps each in `Animate.SlideUp`. Under 100 lines. No changes to result component internals.

#### Wave 3 (sequential — wire up, migrate, E2E overhaul)

Connect new shell to page, delete old shell, regenerate all screenshots.

- [ ] `apps/web` — **Wire `RecipeShell` into page.tsx, delete old shell.** Replace `<BntoPageShell>` with `<RecipeShell>` in `app/[bnto]/page.tsx` (same dynamic import pattern with `ssr: false`). Delete `BntoPageShell.tsx` and `BntoConfigPanel.tsx`. Update imports. Verify `task ui:build` passes.

- [ ] `apps/web` — **Regenerate all browser execution E2E screenshots.** The 10 spec files in `e2e/journeys/browser/` have screenshot baselines from the old layout. Delete all `__screenshots__/` dirs under `e2e/journeys/browser/`. Run `task e2e` with `--update-snapshots`. Fix any broken selectors from the layout change. Visually verify every new screenshot with the Read tool. Do NOT change test logic — only fix selectors and regenerate screenshots.

- [ ] `apps/web` — **Regenerate site-navigation E2E screenshots for tool pages.** Delete stale tool page screenshots from `e2e/pages/`. Run tests with `--update-snapshots`. Visually verify. Covers both desktop and mobile viewport variants.

#### Wave 4 (parallel — polish: responsive, accessibility, animation)

Visual refinement pass. Ensure the new layout meets the Motorway quality bar.

- [ ] `apps/web` — **Responsive polish.** Verify mobile (375px), tablet (768px), desktop (1280px) layouts. Mobile: single column, 2-column file grid, config below. Desktop: 3-4 column file grid. Adjust `Grid` cols, Container size, gap props. No horizontal overflow on mobile.

- [ ] `apps/web` — **Keyboard accessibility audit.** Tab order follows visual flow (drop zone -> file cards -> config -> run). File card delete buttons have `aria-label`. PhaseIndicator has appropriate ARIA. Accordion keyboard nav works (Radix). Fix any gaps.

- [ ] `apps/web` — **Animation polish.** Verify Motorway motion language: (1) Page load: heading `FadeIn`, dropzone `SlideUp`. (2) Files added: card grid `Stagger` + `ScaleIn from={0.85}`. (3) Config: `SlideUp` with delay. (4) Results: `SlideUp`. All guarded with `motion-safe:`. Test passes with `reducedMotion: "reduce"`.

---

### Sprint 3: Platform Features (M2)
**Goal:** Accounts earn their keep. Users who sign up get persistence, history, and a reason to stay. Conversion hooks are natural — Save, History, Server Nodes — not artificial run caps. See [pricing-model.md](strategy/pricing-model.md) for the full free vs premium framework.

#### Wave 1 (parallel — account value + analytics schema)

- [x] `@bnto/backend` — **BLOCKER: Fix anonymous → password userId preservation.** Fixed via `PasswordWithAnonymousUpgrade` wrapper in `auth.ts`. The wrapper intercepts Password's `authorize` function (runs in the signIn action, which HAS auth context), extracts the current userId from the JWT, injects it into the profile object as `_anonymousUserId`, and the `createOrUpdateUser` callback reads it to upgrade the anonymous user in-place. This bypasses the library's limitation where internal mutations don't have auth context. All 3 E2E tests pass (C1-C3). userId is preserved through conversion.
- [x] `apps/web` — **Unfixme anonymous conversion E2E tests.** Tests un-fixme'd, all 3 pass: C1 (anonymous session created), C1-C2 (userId preserved through sign-up), C3 (profile shows correct email/name after conversion). Screenshots generated and verified. JSDoc updated to reflect the working fix.
- [x] `apps/web` — **Address all FIXME comments across the codebase.** 9 FIXMEs (excluding the 2 in `anonymous-conversion.spec.ts` covered above). Grouped by theme: **(1) Nav architecture refactor** — `Navbar.tsx` (3 FIXMEs: extract magic numbers to constants, replace `pendingHref` useState smell, make navbar hiding composable via layout instead of JS `if (hidden) return null`), `MobileNavMenu.tsx` (2 FIXMEs: extract shared compositional parts between desktop/mobile nav, create shared auth section component), `NavUser.tsx` (1 FIXME: refactor into composable compound component with dot-notation parts — trigger, menu, items). **(2) Middleware** — `middleware.ts` (1 FIXME: research Next.js best practice for middleware naming — `MiddlewareNotFoundError.ts` deprecated, may be `proxy.ts` now). **(3) Providers** — `providers/index.tsx` (1 FIXME: `pathnameRef` pattern is a code smell — research recommended Next.js + `@convex-dev/auth` approach for pathname tracking in providers). Research shadcn-blocks repo for nav patterns. After fixing, grep for remaining FIXME/HACK/XXX and confirm zero results.
- [x] `apps/web` — **Privacy policy rewrite:** Replace template `privacy.mdx` with real bnto privacy policy. Update company name (bnto), contact email, URL (bnto.io), jurisdiction, last updated date. Emphasize browser-only processing — files never leave the user's machine. Remove Flash Cookies section (obsolete). Remove Third-party Social Media Service section if not applicable.
- [x] `apps/web` — **README review before launch:** Rewrote README.md to reflect current state: Rust WASM browser engine, 6 Tier 1 tools, bnto.io as primary entry point, accurate repo structure, correct dev commands. Removed all stale Go CLI/Wails/deleted-package references.
- [x] `monorepo` — **Knip dead code audit:** 14 dead files deleted (~766 lines), 11 unused deps removed, 9 catalog entries removed, 15 unused icon re-exports removed. Created knip.json config. Build + tests pass (447 tests).
- [x] `monorepo` — **File & component naming audit:** 4 violations fixed: AnimatedThemeToggle export mismatch, provider.tsx→BntoCoreProvider.tsx, theme-store.ts→themeStore.ts, utils.ts→cn.ts (30 import sites updated). Build passes clean.
- [x] `monorepo` — **Full codebase coding standards review (multi-agent):** 5 parallel agents audited all packages against code-standards.md, architecture.md, components.md, and theming.md. 149 violations found (33 HIGH, 59 MEDIUM, 57 LOW) across Core, Frontend, Backend, Rust Engine, and Auth+Nodes domains. Key fixes: dot-notation migration for Popover/primitives, Raw*Doc types in @bnto/core (decouples transforms from backend), target-agnostic ProgressReporter in Rust engine (removes js-sys from bnto-core), ParameterSchema union types in @bnto/nodes, Convex function cleanup (explicit return types, error wrapping). All TS builds pass (6/6), all Rust tests pass (297 unit tests), all TS tests pass (447+ tests). Pre-existing lint issues in FileUpload.tsx (react-hooks/immutability) noted but not introduced by this task.
- [x] `@bnto/backend` — `planTier` field on user schema (free, pro). Usage analytics fields: `totalRuns`, `lastRunAt`
- [x] `@bnto/backend` — Execution analytics: aggregate queries for per-user history (by slug, by date range)
- [ ] `@bnto/core` — `useExecutionHistory()` hook (paginated, per-user)
- [ ] `@bnto/core` — `useUsageAnalytics()` hook (total runs, most-used bntos, last activity)
- [ ] `apps/web` — WorkflowCard component (name, description, node count, last run status)
- [ ] `apps/web` — StatusBadge component (pending, running, completed, failed)
- [ ] `apps/web` — EmptyState component (no workflows yet)
- [x] `apps/web` — **Site navigation journey tests (E2E):** Playwright user journey that navigates every public route on the site (home, all 6 tool slugs, pricing, FAQ, privacy, signin) with screenshot assertions at each stop. Verifies nav links work, no broken routes, and captures visual state as a regression baseline. Include mobile viewport variant.
- [ ] `infra` — **Analytics layer decision:** Evaluate and select analytics tooling for user behavior and usage tracking. Candidates: Plausible (privacy-first, no cookies), PostHog (product analytics, self-hostable), Vercel Analytics (built-in), or custom Convex events. Decision criteria: privacy alignment (browser-first, no third-party tracking claims in privacy policy), cost, self-hostable option, event tracking depth (tool usage, conversion funnels, retention). Document decision in `.claude/decisions/`.
- [ ] `infra` — **SEO validation tooling:** Set up Lighthouse CI in GitHub Actions for automated performance/SEO scoring on every PR. Configure Google Search Console for bnto.io (verify ownership, submit sitemap, monitor indexing). Add `task seo:audit` command that runs Lighthouse locally against all public routes and reports Core Web Vitals scores. Target: all pages green on Performance, Accessibility, Best Practices, SEO.
- [ ] `apps/web` — **Move /dev/motorway back to /motorway:** Reverse the Sprint 2C move. The design system demo page should live at `/motorway`, not `/dev/motorway`. Move `app/(dev)/dev/motorway/page.tsx` to `app/(app)/motorway/page.tsx`, delete `(dev)` route group, update Footer link.
- [ ] `engine` — **Rust engine test location consistency audit (multi-agent):** Audit all Rust crates under `engine/crates/` for inconsistent test placement. Some tests live next to code (`#[cfg(test)]` blocks in source files), others are in a separate `tests/` folder. Pick one convention and enforce it across all crates. This is a professionalism concern — the engine code should look intentional and consistent. Recommend: unit tests inline (`#[cfg(test)]`), integration/WASM tests in `tests/` directory.
- [ ] `engine` — **EXIF orientation test coverage verification:** Verify that Rust engine-level tests exist for the photo flipping/rotation bug (EXIF orientation handling). Check `bnto-image` crate for tests that cover orientation-corrected dimensions for JPEG images with EXIF rotation metadata (orientation tags 1-8). If missing, add unit tests at the engine level — not just E2E tests via Playwright.
- [ ] `apps/web` — **Replace home-rolled FileUpload with react-dropzone:** `components/ui/FileUpload.tsx` is 1400 lines of custom store, reducer, drag/drop, paste, file validation, and progress tracking — reimplementing what `react-dropzone` (v15, already installed) provides. Also causes 3 `react-hooks/immutability` lint errors from the custom store pattern. Only consumer: `FileDropZone.tsx`. Rewrite as a thin wrapper around `react-dropzone`, keep compound component API, target ~200-300 lines. Must pass all 6 bnto tool page E2E tests after.

#### Wave 2 (parallel — dashboard + conversion hooks)

- [ ] `apps/web` — Dashboard page: saved workflows, recent executions, usage analytics
- [ ] `apps/web` — Execution history page (list of past runs with status, re-run capability)
- [ ] `apps/web` — **Save prompt** (conversion hook): "Want to keep this workflow? Sign up to save it." — appears after successful browser execution for anonymous users
- [ ] `apps/web` — **History prompt** (conversion hook): "Sign up to access your execution history and re-run past workflows."
- [ ] `apps/web` — **Browser auth behavior verification:** Token expiry, sign-out invalidation (moved from Sprint 2A Wave 5)
- [ ] `apps/web` — Pricing page update: Pro sells persistence, collaboration, premium compute — not run limits

#### Wave 3 (sequential — test)

- [ ] `apps/web` — Playwright E2E: save prompt appears after anonymous execution
- [ ] `apps/web` — Playwright E2E: execution history page shows past runs for authenticated users
- [ ] `@bnto/backend` — Unit tests for execution analytics queries

---

### Sprint 4: Recipe Editor (Headless-First)
**Goal:** Users can create recipes from a blank canvas or customize existing ones — add/remove/configure nodes, connect them, run, and export as `.bnto.json`. The editor is free (pricing-model.md: "recipe editor is free"). Power users who create custom recipes are the highest-intent Pro upgrade candidates.

**Architecture: headless-first.** The editor is built as layers. Logic lives in pure functions, a state machine, and hooks — no visual dependency. The conveyor belt visual (sushi, depth cards, belts) is a themed skin applied on top. This means the editor can be reskinned, embedded, or offered as a headless library in the future.

```
@bnto/nodes (types, schemas, validation)      ← already built
         ↓
Pure functions (definition CRUD, adapters)     ← Wave 1
         ↓
Editor store (Zustand — headless operations)   ← Wave 2
         ↓
React hooks (reactive bindings)                ← Wave 2
         ↓
Visual skin (ConveyorCanvas / JSON panel)      ← Wave 3+
```

**Two entry points, same state:** `createBlankRecipe()` (empty canvas with one input + one output node) or `loadRecipe(slug)` (pre-assembled recipe from `@bnto/nodes`). Both produce the same `EditorState` shape — same operations, same output, same visual.

**Prior art:** Atomiton's `createFieldsFromSchema` pattern. Define node parameter schemas once (`@bnto/nodes/schemas/`), auto-derive config panel UI. ~70-80% of fields need zero UI code. Already built in `@bnto/nodes` — schemas exist for all 10 node types with `visibleWhen`, `requiredWhen`, enum values, min/max, and defaults.

**What this is NOT:** Save to Convex (Sprint 3 prerequisite), execution history, workflow versioning, container node nesting (group/loop as visual sub-canvases), or the JSON/code editor (Sprint 4B — CodeMirror 6, shares headless primitives but is a distinct coding-oriented experience with its own persona). Those layer on naturally once the headless foundation exists.

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
- [ ] `@bnto/nodes` — **`addEdge(definition, source, target, sourceHandle?, targetHandle?)`**: Creates a connection between two nodes. Validates: no self-loops, no duplicate edges, source/target exist. Returns new `Definition` or validation error.
- [ ] `@bnto/nodes` — **`removeEdge(definition, edgeId)`**: Removes an edge. Returns new `Definition`.
- [ ] `@bnto/nodes` — **`moveNode(definition, nodeId, position)`**: Updates a node's `position`. Returns new `Definition`.
- [ ] `@bnto/nodes` — **`definitionToRecipe(definition, metadata?)`**: Wraps a `Definition` into a `Recipe` with slug, name, description, accept spec. For export.
- [ ] `@bnto/nodes` — **Unit tests for all CRUD operations**: Every function tested with all 10 node types. Edge cases: remove node with connections (edges cascade-deleted), add edge to non-existent node (error), update params with invalid values (validation errors), blank definition is valid, nested container operations.

#### Wave 2 (parallel — editor store + React hooks)

Zustand store that wraps the pure functions into a reactive state machine. Hooks provide the React binding layer. Still headless — no visual components. **`/reactflow-expert` leads** — owns the Definition ↔ Flow adapter design and Zustand store architecture following ReactFlow's change/apply pattern.

- [ ] `apps/web` — **`useEditorStore` (Zustand)**: Editor state: `definition` (current `Definition`), `selectedNodeId`, `isDirty`, `validationErrors[]`, `executionState` (per-node status map). Actions: `loadRecipe(slug)`, `createBlank()`, `addNode(type)`, `removeNode(id)`, `selectNode(id)`, `updateParams(nodeId, params)`, `addEdge(...)`, `removeEdge(...)`, `moveNode(...)`, `resetDirty()`. All actions delegate to Wave 1 pure functions. Undo/redo via history stack (store snapshots).
- [ ] `apps/web` — **`useEditorNode(nodeId)` hook**: Returns node data + schema + visible params (conditional visibility resolved). Subscribes to store slice — re-renders only when this node changes.
- [ ] `apps/web` — **`useEditorEdges()` hook**: Returns all edges with resolved variant colors (from source node type). Subscribes to edge slice only.
- [ ] `apps/web` — **`useNodePalette()` hook**: Returns available node types from `NODE_TYPE_INFO`, grouped by category, with `browserCapable` flags. Filters server-only nodes based on context (browser editor = browser-capable only).
- [ ] `apps/web` — **`useEditorExport()` hook**: Returns `{ exportAsRecipe, download }` — wraps current definition as a `Recipe` or triggers browser `.bnto.json` file download. Validates definition before export. Pure serialization — no visual dependency.
- [ ] `apps/web` — **Definition ↔ ReactFlow adapters**: `definitionToFlow(definition)` → `{ nodes: StationNodeType[], edges: ConveyorEdgeType[] }`. `flowToDefinition(nodes, edges)` → `Definition`. Pure functions that bridge the headless model to the visual layer. Map node types to station variants, positions, and port handles. Unit tested — round-trip: `definition → flow → definition` produces equivalent output.
- [ ] `apps/web` — **Unit tests for store + hooks**: Store operations tested via Vitest (no rendering). Hook tests via `renderHook`. Adapter round-trip tests. Undo/redo verification.

#### Wave 3 (parallel — visual canvas integration)

Wire the headless store to the existing ConveyorCanvas. The conveyor belt becomes a live, interactive editor. **`/reactflow-expert` + `/frontend-engineer` co-lead.** ReactFlow Expert owns canvas interaction, connection validation, drag-and-drop. Frontend Engineer owns component composition, theming, and animation.

- [ ] `apps/web` — **`RecipeEditor` component**: Composes `EditorToolbar` + `ConveyorCanvas` + `NodeConfigPanel`. Reads from `useEditorStore`. Two entry modes: `<RecipeEditor slug="compress-images" />` (loads predefined) or `<RecipeEditor />` (blank canvas).
- [ ] `apps/web` — **`EditorToolbar` component**: Action bar above canvas — recipe selector dropdown (all Tier 1 recipes + "Blank"), Add Node button (opens palette), Remove Selected button, Run button, Reset/Replay button, Export `.bnto.json` button, Undo/Redo buttons. Reads/dispatches to `useEditorStore`.
- [ ] `apps/web` — **`NodePalette` component**: Slide-out panel listing available node types from `useNodePalette()`. Click-to-add (adds node at auto-positioned location). Grouped by category. Browser-capable badge. Server-only nodes shown grayed with "Pro" badge (visible but not addable in browser context — definitions always available per pricing model).
- [ ] `apps/web` — **`NodeConfigPanel` component**: Side panel that renders when a node is selected. Uses `useEditorNode(selectedNodeId)` to get schema + current params. Auto-generates form fields from `ParameterSchema` (Atomiton pattern): string → text input, number → number input with min/max, boolean → toggle, enum → select dropdown. `visibleWhen` and `requiredWhen` handled reactively. Parameter changes dispatch `updateParams` to store.
- [ ] `apps/web` — **Enable canvas interaction**: Upgrade `ConveyorCanvas` to accept an `interactive` prop. When `true`: `nodesDraggable={true}`, `nodesConnectable={true}`, `elementsSelectable={true}`. Node drag updates position via `moveNode`. New edge connections dispatch `addEdge`. Selection dispatches `selectNode`. When `false`: current read-only showcase behavior (backward compatible).
- [ ] `apps/web` — **Motorway debug section**: Replace the hardcoded `ConveyorShowcase` in `/dev/motorway` with `<RecipeEditor />`. The Motorway page becomes the editor playground — load recipes, add/remove nodes, connect them, configure parameters, run, export.

#### Wave 4 (parallel — execution + polish)

The editor runs recipes and shows execution state on the canvas. The conveyor belt visual becomes meaningful — it's not just pretty, it's showing real processing. **`/reactflow-expert` + `/frontend-engineer` co-lead.**

- [ ] `apps/web` — **Execution integration**: Wire Run button to browser WASM execution path. When running: station nodes show execution state (idle → running → completed/failed) via variant shifts and belt animation speed changes. Progress callbacks from `browserExecutionService` update `executionState` in editor store. Conveyor pieces flow faster during processing, pause on completion.
- [ ] `apps/web` — **Export `.bnto.json`**: Download button in toolbar that serializes current editor state to a valid `.bnto.json` file via `useEditorExport().download()`. Validates before export. Users can take their recipe anywhere — CLI, desktop, share with others.
- [ ] `@bnto/backend` — **Tag editor users**: When a user opens the editor, set `hasUsedEditor: true` on their user record. Highest-intent Pro upgrade candidates. Query: `ctx.db.query("users").withIndex("by_hasUsedEditor")`.
- [ ] `apps/web` — **E2E tests**: Load recipe → canvas renders matching stations. Add node → appears. Connect nodes → belt renders. Remove node → removed with edges. Configure params → node updates. Export → valid `.bnto.json` file. Run → execution progress shown on canvas. Blank canvas → add nodes → build a recipe from scratch.

---

### Sprint 4B: Code Editor (CodeMirror 6)

**Goal:** A schema-aware `.bnto.json` code editor for power users — the coding-oriented counterpart to the visual canvas. Users who prefer code get the same power as the visual canvas, with the speed and precision of text editing. Slash commands bring Notion-like ergonomics. The code editor is free (same as the visual editor).

**Required reading:** Before picking up ANY task in Sprint 4B, read [code-editor.md](.claude/strategy/code-editor.md) — the design document covering tech choice rationale (CM6 over Monaco), architecture (headless-first + store sync), feature tiers, slash command implementation, JSON Schema strategy, CLI/TUI parallels, React integration pattern, theming, and performance considerations. Also read the persona at `.claude/skills/personas/code-editor-expert/SKILL.md` for CM6-specific APIs, extension patterns, and gotchas.

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
- [ ] `apps/web` — **Split view**: Side-by-side `ConveyorCanvas` + `CodeEditor`, both reading from `useEditorStore`. Changes in either sync through the store. Resizable split pane. Toggle between code-only, visual-only, and split modes.
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

**Desktop tech depends on M1 outcome:**
- If Rust WASM succeeded → **Tauri** (Rust-native). One codebase for browser + desktop + CLI.
- If we bailed to JS → **Wails v2** (Go-native). Go engine powers desktop + CLI. JS adapters stay for browser.

### Sprint 5: Desktop Bootstrap

#### Wave 1 (parallel — setup)

- [ ] `apps/desktop` — Bootstrap desktop project (Tauri or Wails, per M1 outcome)
- [ ] `@bnto/core` — Implement desktop adapter (Tauri IPC bindings or Wails Go bindings)
- [ ] `engine` — Expose engine functions for desktop bindings (RunWorkflow, ValidateWorkflow, etc.)

#### Wave 2 (parallel — integration)

- [ ] `apps/desktop` — Wire up native ↔ React bindings
- [ ] `@bnto/core` — Runtime detection routes to desktop adapter in native webview
- [ ] `apps/desktop` — Local file browser for selecting .bnto.json files

#### Wave 3 (sequential — verify)

- [ ] `apps/desktop` — Verify workflow list, edit, and save work via native bindings
- [ ] `apps/desktop` — Verify runtime detection correctly identifies desktop environment

---

### Sprint 6: Local Execution

#### Wave 1 (parallel — execution)

- [ ] `apps/desktop` — Execute workflows via Wails Go bindings (all 10 node types)
- [ ] `@bnto/core` — Execution progress streaming via Wails adapter
- [ ] `apps/web` — Execution progress component (reusable — node status, duration, logs)

#### Wave 2 (parallel — features)

- [ ] `apps/desktop` — Execution results view (output data, logs, duration)
- [ ] `apps/desktop` — shell-command node support (full local execution, no restrictions)
- [ ] `apps/desktop` — Error handling and cancellation support

#### Wave 3 (sequential — build + distribute)

- [ ] `apps/desktop` — Integration tests for local execution
- [ ] `apps/desktop` — macOS build (.app bundle, code signing)
- [ ] `apps/desktop` — Windows build (.exe)
- [ ] `apps/desktop` — Linux build (AppImage)

---

## Phase 3: Monetization + Polish

**Goal:** Wire up payments, enforce quotas, make the product feel complete.

**"Ready to charge" gate:** Before Sprint 7, confirm: real users running browser bntos, conversion hooks built and tested (Save, History, Premium), people return voluntarily, at least one server-side bnto (AI or shell) ready for Pro tier.

### Sprint 7: Stripe + Pro Tier (M5)

**Goal:** First revenue. Pro sells real value — not artificial limits on browser-native operations.

**What Pro includes:** $8/month or $69/year. Saved workflows, execution history (30-day retention), team sharing (up to 5 members), server-side premium bntos (AI, shell, video — M4), priority processing, API access.

**What stays free forever:** All browser-capable bntos, unlimited runs, desktop app. See ROADMAP.md trust commitments.

#### Wave 1 (parallel — payments)

- [ ] `apps/web` — Stripe integration (checkout session, webhook handler, subscription sync to Convex)
- [ ] `@bnto/backend` — `planTier` updated on successful Stripe webhook (free → pro)
- [ ] `apps/web` — Upgrade page (`/upgrade`) — pricing, Pro benefits, Stripe checkout CTA
- [ ] `apps/web` — Billing management page (current plan, cancel, manage via Stripe portal)

#### Wave 2 (parallel — Pro feature gates)

- [ ] `@bnto/backend` — Pro feature gates: 30-day history retention, team sharing (up to 5 members), priority processing queue
- [ ] `apps/api` — Server-side execution quota enforcement (applies to premium server-side bntos only — AI, shell, video)
- [ ] `apps/web` — File size enforcement at R2 presigned URL generation for server-side recipes (Pro-only, size limits TBD based on usage data)

#### Wave 3 (sequential — test)

- [ ] `apps/web` — Playwright E2E: free user sees Pro conversion hooks (save, history, premium bntos)
- [ ] `apps/web` — Playwright E2E: Pro user has access to saved workflows and execution history

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

## Parallel Track: ADO Dashboard

Real-world dogfooding. Runs alongside any phase.

### Phase A: `ado` Node Type
- [ ] `engine` — Implement `ado` node (WIQL queries, work items, test runs, build status)
- [ ] `engine` — Unit tests with mock HTTP server
- [ ] `engine` — Integration fixture `.bnto.json`
- [ ] `engine` — Register in DefaultRegistry()

### Phase B: `aggregate` Node Type
- [ ] `engine` — Implement `aggregate` node (groupBy, count, sum, average, percentage, sortBy)
- [ ] `engine` — Unit tests, integration test chaining with other node types

### Phase C: `report` Node Type
- [ ] `engine` — Implement `report` node (terminal, markdown, json output)
- [ ] `engine` — Unit tests for each format

### Phase D: Dashboard Templates
- [ ] `engine` — 3-5 example dashboard `.bnto.json` fixtures in `engine/examples/`

---

## Backlog

### Infra: Shared Test Fixtures Package (`@bnto/test-fixtures`)

**Priority: Low (nice-to-have).** The `test-fixtures/` directory at repo root already serves the primary need — shared images (JPEG, PNG, WebP at small/medium/large sizes) consumed by both Go engine (`go:embed`) and Rust WASM (`include_bytes!()`). E2E tests reference engine fixtures directly. A formal TS package would add helpers and consolidate ad-hoc test files, but isn't blocking anything.

- [ ] `packages/@bnto/test-fixtures` — Create package wrapping `test-fixtures/` with TS helpers to load by name
- [ ] `packages/@bnto/test-fixtures` — Add sample CSVs (clean, dirty, large) and rename test files
- [ ] `apps/web` — Update E2E tests to import from shared package instead of ad-hoc paths

### Engine: Spreadsheet Node Template Resolution — M3/M4 (Go engine)

**Milestone: M3/M4.** Go engine work — not blocking M1 (browser execution uses Rust/JS, not Go). The `clean-csv` predefined Bnto fails in cloud execution. The `read-csv` node (type: `spreadsheet`) receives `<no value>` for its input file path template variable.

**Discovered via:** Integration E2E test. All image-based pipelines work — only the spreadsheet node path is broken.

- [ ] `engine` — Reproduce locally: `bnto run` with `clean-csv` fixture against a test CSV file
- [ ] `engine` — Debug template resolution in `spreadsheet` node's `Execute()`
- [ ] `engine` — Fix template variable resolution so `read-csv` receives the actual file path
- [ ] `engine` — Verify fix: E2E `clean-csv` test passes (`task e2e`)

### ~~Web: BntoPageShell Decomposition~~

**RESOLVED:** BntoPageShell was rewired for the browser adapter in Sprint 2B Wave 2. The component routes Tier 1 bntos through the browser adapter and handles ZIP download for multi-file results. No further decomposition needed — the component is within size limits.

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

The Convex dev deployment (`zealous-canary-422`) still contains stale data from the old Better Auth system and accumulated integration test artifacts. The `auth*` tables (`authAccounts`, `authRateLimits`, `authRefreshTokens`, `authSessions`, `authVerificationCodes`, `authVerifiers`) are now managed by `@convex-dev/auth`, but old Better Auth records and orphaned test users are still present.

**What to clean:**
- Old user records from Better Auth era (pre-migration)
- Orphaned auth sessions/tokens/verifiers that reference non-existent users
- Accumulated integration test users (`test-*@test.bnto.dev`)
- Stale execution records and execution events from test runs
- Any workflows created by test accounts

**Approach:** Write a one-off Convex mutation that identifies and deletes stale records. Run against dev first, then production if needed. Back up data before deletion.

- [ ] `@bnto/backend` — Audit Convex dev tables: identify Better Auth remnants vs current `@convex-dev/auth` records
- [ ] `@bnto/backend` — Write cleanup mutation: delete orphaned auth records, test users, and stale test data
- [ ] `@bnto/backend` — Run cleanup against dev deployment, verify table health
- [ ] `@bnto/backend` — (If needed) Run cleanup against production deployment

### Infra: Configure R2 Lifecycle Rules — M4 (cloud execution)

**Milestone: M4.** R2 is only used for cloud (server-side) execution. Not needed for M1 browser execution.

| Bucket | Prefix | Auto-delete after |
|---|---|---|
| `bnto-transit` + `bnto-transit-dev` | `uploads/` | 1 hour |
| `bnto-transit` + `bnto-transit-dev` | `executions/` | 24 hours |

- [ ] `infra` — Configure R2 lifecycle rules in Cloudflare dashboard (prod + dev buckets)

### Infra: Domain Setup (bnto.io Custom Domains)

**Web app domain promoted to Sprint 2C Wave 1.** API domain (`api.bnto.io`) deferred to M4 (premium server-side bntos need the Go API). Web app domain is the launch blocker.

- [ ] `infra` — Connect `bnto.io` to Vercel, update Cloudflare DNS — **promoted to Sprint 2C Wave 1**
- [ ] `infra` — Add `api.bnto.io` CNAME in Cloudflare DNS → Railway — **M4 (not needed for browser-only launch)**
- [ ] `infra` — Configure custom domain in Railway dashboard — **M4**
- [ ] `infra` — Update `GO_API_URL` in Convex prod to `https://api.bnto.io` — **M4**
- [ ] `infra` — Verify API health check at `https://api.bnto.io/health` — **M4**
- [ ] `infra` — Verify auth redirects work on `bnto.io`

### Infra: Graduate SEO Validation from E2E to Unit Tests + Lighthouse CI

**Priority: Medium.** Current SEO validation lives entirely in Playwright E2E tests (`e2e/pages/seo-metadata.spec.ts`). This works but is slow and misses performance/mobile concerns. Graduate to a layered approach:

1. **Move metadata validation to unit tests** (Vitest, fast) — title/description length (50-60 / 120-160 chars), registry↔sitemap sync, slug format. Most already exists in `bntoRegistry.test.ts`.
2. **Keep thin E2E layer** — just noindex check, 404 behavior, canonical redirects (need a running server).
3. **Add Lighthouse CI** on merge to main — `seo: 90` threshold catches mobile-friendliness, viewport, tap targets, CWV. Use `playwright-lighthouse` or `lighthouse-ci`.
4. **Google Search Console** — manual, weekly initially then monthly. Only real source of truth for actual indexing.

- [ ] `apps/web` — Add unit tests: title length (50-60 chars), description length (120-160 chars) to `bntoRegistry.test.ts`
- [ ] `apps/web` — Add unit test: sitemap entries match registry (import both, compare sets)
- [ ] `apps/web` — Add E2E test: no accidental `noindex` on Tier 1 pages
- [ ] `apps/web` — Add E2E test: `/sitemap.xml` is valid and contains all Tier 1 slugs
- [ ] `apps/web` — Add Lighthouse CI with `seo: 90` threshold on `/compress-images`
- [ ] `apps/web` — Migrate remaining SEO assertions from `seo-metadata.spec.ts` to unit tests, slim E2E to redirects + 404 + noindex only

### ~~Testing: Suppress or Handle `[useAnonymousSession] signIn failed` in E2E~~

**RESOLVED:** E2E tests now always run against the full dev stack (Next.js + Convex on port 4000). The "no backend" mode was removed — `useAnonymousSession` always has a backend to connect to.

### Testing: Standardize E2E Selectors on data-testid

Current E2E tests mix CSS classes, `getByRole`, `getByText`, and `data-testid`. Standardize on `data-testid` for state detection and element targeting. Keep semantic selectors for accessibility assertions.

- [ ] `apps/web` — Audit E2E specs, add `data-testid` attributes, update selectors

### Testing: Concurrent Quota Race Condition — M4/M5 (server-side quotas)

**Milestone: M4/M5.** Quota enforcement only applies to server-side bntos. Browser bntos are free unlimited. This race condition matters when server-side execution has limits.

- [ ] `@bnto/core` — Integration test: fire 2+ concurrent `startPredefined` calls for a user at limit-1 runs, verify at most 1 succeeds
- [ ] `@bnto/backend` — If race confirmed, investigate Convex transaction isolation guarantees or atomic increment patterns

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

Referral links to boost user acquisition. With browser-first, the referral reward shifts from "bonus runs" (old model) to Pro trial or extended history.

**Open questions:** What's the reward? Options: (a) 7-day Pro trial for both, (b) extended execution history (90 days instead of 30), (c) early access to new bntos. Needs validation after launch.

- [ ] `@bnto/backend` — Schema: `referrals` table (referrerId, referredUserId, reward, createdAt), `referralCode` field on users
- [ ] `@bnto/backend` — Mutation: `applyReferral` — validates code, applies reward to both users
- [ ] `@bnto/core` — Referral service/hooks: `useReferralCode()`, `useApplyReferral()`
- [ ] `apps/web` — Referral link generation UI in settings/profile
- [ ] `apps/web` — Landing page referral code capture (via URL param `?ref=CODE`)

### UI: Extract Motorway Design System (`@bnto/ui`)

**Trigger: Desktop app (M3).** When the desktop app creates a real second consumer for UI components, extract `apps/web/components/ui/` into `packages/ui/` as `@bnto/ui`. The official name is **Motorway** — the Mini Motorways-inspired design system.

**What moves:**
- `apps/web/components/ui/` → `packages/ui/src/` (all primitives and component wrappers)
- `apps/web/components/ui/create-cn.ts`, `cn.ts` → `packages/ui/src/` (utility layer)
- CSS tokens and the `.depth` / `.pressable` / spring animation system from `globals.css` → `packages/ui/styles/`
- Theme provider, animated toggle, and dark mode utilities

**What stays in `apps/web`:**
- Domain/business components (BntoPageShell, WorkflowCard, etc.)
- Page compositions and route-level components
- App-specific providers and wiring

**Package scope:**
- Published as `@bnto/ui` on npm, branded as Motorway
- Zero domain knowledge — purely generic, reusable design system
- Depth system (3D card elevation), pressable interactions, spring animations, warm palette tokens
- Could be used independently by anyone who wants the Mini Motorways aesthetic

**Prerequisites:**
- Desktop app bootstrap (Sprint 5) creates the second consumer
- Stable component API — no major churn expected

- [ ] `packages/ui` — Bootstrap `@bnto/ui` package (tsconfig, package.json with "motorway" description, exports)
- [ ] `packages/ui` — Move primitives from `apps/web/components/ui/primitives/`
- [ ] `packages/ui` — Move component wrappers (Button, Card, Dialog, Tabs, etc.)
- [ ] `packages/ui` — Move utility layer (`cn`, `createCn`)
- [ ] `packages/ui` — Extract CSS tokens and animation system into distributable stylesheet
- [ ] `apps/web` — Update all imports from `@/components/ui/` to `@bnto/ui`
- [ ] `apps/desktop` — Wire `@bnto/ui` as dependency
- [ ] `apps/web` — Add `@source` directive for Tailwind to scan `@bnto/ui` (see gotchas.md)

### Showcase: Radial Light Source Controls

**Priority: Low (fun polish).** Replace the linear slider on `/showcase` with more expressive light source controls that better illustrate the depth system's relationship to light direction.

**Two controls:**
1. **Radial slider** — generic UI primitive (`components/ui/RadialSlider`). Circular drag input where a thumb orbits a ring. `atan2()` maps pointer position to value (0–360 or any range). Configurable labels prop — the showcase uses compass cardinal directions (N/NE/E/SE/S/SW/W/NW) but the component itself is generic and reusable.
2. **Elevation slider** — top-to-bottom arc or vertical slider controlling light source height/elevation. Could drive shadow length (higher sun = shorter shadows, lower sun = longer shadows). Would need a new `--light-elevation` CSS variable and corresponding depth shadow scaling.

Both controls feed into the same CSS custom property system that drives the depth shadows on the page.

- [ ] `apps/web` — `RadialSlider` generic UI component (value, onChange, labels, size, thumb icon)
- [ ] `apps/web` — Light elevation control (vertical/arc → `--light-elevation`)
- [ ] `apps/web` — Wire elevation into depth shadow length scaling in `globals.css`
- [ ] `apps/web` — Replace `LightSourceSlider` on showcase page with RadialSlider + compass labels

### ~~UI: Font Family Review~~ — DONE

**Completed Feb 2026.** DM Sans replaced with Geist for display/headings. Final font stack: **Geist (display, font-black weight 900) + Inter (body) + Geist Mono (code)**. Mini Motorways-inspired: Swiss-style precision meets warm palette. Committed in `86cad62`.

- [x] `apps/web` — Swapped display font to Geist, updated Heading component to font-black (900)
- [x] `apps/web` — Typography showcase updated with actual font labels

### Performance: WASM Bundle Size & Processing Benchmarks

**Deferred from Sprint 2B Wave 4.** Current WASM bundle: 1.6MB raw / 606KB gzipped (all 6 nodes in single cdylib). Above the original 500KB target by ~20%. Not blocking M1 but worth profiling.

- [ ] `engine` — Profile bundle: which crates contribute most to size? (`twiggy` or `wasm-opt --print-code-section-sizes`)
- [ ] `engine` — Evaluate code splitting (lazy-load node crates) vs single bundle tradeoff
- [ ] `apps/web` — Processing speed benchmarks: time per node type for representative file sizes
- [ ] `apps/web` — Memory usage profiling: peak heap during batch processing

### Performance: Next.js Server Component Audit (Pre-Launch)

**Priority: Pre-launch (near end of MVP).** Before shipping, audit the entire `apps/web` tree to ensure we're not overusing `"use client"` and are getting the full benefit of Next.js Server Components. The rules in `performance.md` are clear — push client boundaries down to the smallest leaf that needs interactivity — but during fast iteration it's easy to mark whole pages or layouts as client components when only a child needs `useState` or an event handler.

**What to audit:**
- Every `"use client"` directive — is it on the smallest possible leaf, or can the boundary be pushed down?
- Pages and layouts that are client components but could be Server Components with a client island pattern
- Client-side data fetching that could be server-side (SSR/SSG)
- Barrel imports in client components that pull in unnecessary bundle weight
- Components marked client just because they import a client dependency transitively
- Heavy components that should be lazy loaded (`next/dynamic`)

**Success criteria:** Every `"use client"` directive has a clear justification (needs hooks, event handlers, or browser APIs). No page-level or layout-level `"use client"` without a strong reason. Bundle size measurably smaller or unchanged (no regression).

- [ ] `apps/web` — Inventory all `"use client"` files, categorize as justified vs candidate for Server Component conversion
- [ ] `apps/web` — Refactor candidates: push `"use client"` down to leaf components, convert parents to Server Components
- [ ] `apps/web` — Verify no barrel imports in client components (import specific files directly)
- [ ] `apps/web` — Lazy load below-fold and modal components with `next/dynamic`
- [ ] `apps/web` — Run Lighthouse / Next.js bundle analyzer before and after, confirm no regression

### Infra: Vercel Preview Deployment Verification

**Deferred from Sprint 2A Wave 5.** Verify auth flow end-to-end on Vercel preview deployment. Not blocking M1 browser execution.

- [ ] `apps/web` — Verify auth flow on Vercel preview deployment (cookie behavior, proxy redirects, sign-in/sign-out)

### UX: Conversion Hook Messaging Audit — M2/M5

**Milestone: M2 (Sprint 3) for hook UX, M5 (Sprint 7) for Stripe integration.**

Conversion messaging should be value-driven, not limit-driven. Hooks trigger on natural value moments (Save, History, Premium Bntos, Team) — not artificial run caps on browser bntos.

**What to audit:**
- Existing `quota.ts` error messages — reframe from "limit reached" to value-driven CTAs
- Quota enforcement only applies to server-side bntos (M4). Browser bntos never show quota errors
- Conversion hooks: "Save this workflow" (signup), "View your history" (signup), "Run AI/shell bntos" (Pro), "Share with team" (Pro)

- [ ] `@bnto/backend` — Review and update error messages: separate browser (no limits) from server-side (quota) paths
- [ ] `apps/web` — Design conversion hook components (Save prompt, History prompt, Premium bnto upsell)
- [ ] `apps/web` — Ensure all CTAs route to pricing page with value messaging, not "you've hit a limit"

### Schema-Driven Config Panel (Single Source of Truth)

**Prior art:** Atomiton project (`~/Code/atomiton`) — `createFieldsFromSchema` auto-derives UI field configs from schemas. ~70-80% of fields need zero UI code. See `packages/@atomiton/nodes/src/core/utils/createFieldsFromSchema.ts`.

**Problem:** Frontend hardcodes per-node config shapes. Two sources of truth (Go engine + frontend).

**Solution:** Define node parameter schemas in Go, expose as structured metadata, auto-derive config panel UI. Five implementation layers:

1. **Go engine schema declarations** — `ParameterSchema` struct per node type, schema registry
2. **API exposure** — `GET /nodes/{type}/schema` and `GET /nodes/schemas` endpoints
3. **TypeScript consumption** — `createFieldsFromSchema()` utility, `useNodeSchema()` hook
4. **Dynamic config panel** — Generic `ConfigPanel` rendering from `FieldConfig[]`, per-bnto overrides (~20-30%)
5. **Pipeline integrity tests** — E2E + contract tests verifying schema-to-UI pipeline

See detailed task breakdown in `.claude/archive/schema-driven-config-panel.md` (archived from original plan).

### UX: Execution Activity Feed — M2 (Sprint 3)

**Updated from "Animated Run Counter."** With browser-first, there's no run limit to count down. Instead, show an activity feed / recent executions indicator that reinforces the value of signing up (persistence, history).

- [ ] `apps/web` — Design activity indicator for bnto tool pages (recent executions, total runs)
- [ ] `apps/web` — For anonymous users: "You've run 12 bntos this session. Sign up to save your history."
- [ ] `apps/web` — For authenticated users: animated activity feed with execution count and last-run status

### Premium: Cloud Drive Export (Post-MVP) — M5+

**Premium conversion hook.** After running a bnto, Pro users can auto-save results directly to their cloud drive — Google Drive, OneDrive, SharePoint, Dropbox. No manual download-and-upload cycle.

**Why it's a strong hook:** The browser-first experience is "drop files → process → download." Adding "→ save to Google Drive" removes the last friction step for users who process files regularly. It's a natural Pro feature because it requires server-side OAuth + API calls.

- [ ] `apps/web` — Design cloud drive export UX (post-execution "Save to..." button with provider icons)
- [ ] `apps/api` — OAuth integration for Google Drive, OneDrive (server-side token management)
- [ ] `@bnto/backend` — Store connected cloud drive credentials per user (Pro only)
- [ ] `apps/api` — Upload execution output to user's connected drive
- [ ] `apps/web` — E2E test: Pro user saves output to connected Google Drive

### UX: Two-Column Bnto Tool Page Layout

**User feedback:** The current single-column bnto tool page layout forces users to scroll below the fold to configure settings and then run. On wider viewports, the Settings panel, drop zone, and Run button should be visible without scrolling.

**Proposed layout:** Bento box grid varios panels needed like dropzone, config, and progress. Settings panel + Run button, drop zone + file list + execution progress/results. (current behavior preserved).

- [ ] `apps/web` — Responsive two-column layout for `[bnto]/page.tsx` (Settings + Run on left, files + progress on right)
- [ ] `apps/web` — Ensure all 6 Tier 1 bnto pages work correctly in two-column mode
- [ ] `apps/web` — Update E2E screenshots for new layout (all bnto-config, execution-flow, file-drop specs)
- [ ] `apps/web` — Mobile breakpoint preserves current single-column stack

### Recursive Workflow Composability (Web App)

The Go engine supports recursive `Definition.Nodes`. The web app must preserve this composability. Guard rails (not new tasks — apply when building related features):

- Config panels must work at any nesting depth
- Execution progress must be recursive (group nodes show children's progress)
- JSON editor must represent recursive structure faithfully
- Visual editor (Sprint 4) must support drill-down into group nodes

### DX: Agent Persona Skills & Skill Refactor

**Priority: Next.** Define developer persona skills that agents load for domain-specific context. This improves output quality by priming agents with the right mental model for the work they're about to do.

**What to build:**
- **Persona skills** in `.claude/skills/personas/<name>/SKILL.md` — each defines a domain expert role with context, vocabulary, and quality standards. All set `user-invocable: false` (Claude-only reference, not user-invoked):
  - `rust-expert` — Rust/WASM developer. Owns `engine/`. Knows ownership, lifetimes, wasm-bindgen, wasm-pack. Educational comments.
  - `go-engineer` — Go engine developer. Owns `archive/engine-go/`, `archive/api-go/`. Node type system, BntoService, registry.
  - `frontend-engineer` — React/Next.js developer. Owns `apps/web/`. Component system, theming, animation, user journey E2E.
  - `core-architect` — Transport-agnostic API. Owns `packages/core/`. Client/service/adapter pattern, React Query, integration testing.
  - `backend-engineer` — Convex backend. Owns `packages/@bnto/backend/`, `packages/@bnto/auth/`. Schema, auth, quota, scheduled functions, `convex-test`.
  - `security-engineer` — Cross-cutting security. Owns trust boundaries across all packages. Auth model, input validation, defense in depth.
  - `project-manager` — Strategic alignment. Owns roadmap, plan, milestones, sprint transitions, backlog prioritization.
- **All skills updated** to reference personas via `personas/<name>/SKILL.md` paths and load domain-appropriate persona(s) including security when relevant.

- [x] `.claude/skills/` — Create persona skill files (rust-expert, go-engineer, frontend-engineer, core-architect, project-manager)
- [x] `.claude/skills/` — Rename `/groom` to `/project-manager`, update content to persona-aware
- [x] `.claude/skills/` — Update `/pickup`, `/code-review`, `/pre-commit` to load relevant persona before execution
- [x] `.claude/skills/` — Test: run `/pickup` on an `[engine]` task and verify Rust persona context is loaded
- [x] `.claude/skills/` — Add security-engineer and backend-engineer personas, restructure all personas to folder/SKILL.md format with frontmatter

---

## Reference

| Document | Purpose |
|----------|---------|
| `.claude/journeys/` | User journey test matrices — auth, engine, API, web app |
| `.claude/strategy/bntos.md` | Predefined Bnto registry — slugs, fixtures, SEO targets, tiers |
| `.claude/rules/pages.md` | SEO URL implementation rules |
| `.claude/rules/architecture.md` | Run quota schema, R2 transit rules |
| `.claude/strategy/core-principles.md` | Trust commitments |
| `.claude/strategy/cloud-desktop-strategy.md` | Architecture, technology decisions, execution model |
| `.claude/decisions/auth-evaluation.md` | Auth migration decision (Better Auth → @convex-dev/auth) |
| `.claude/rules/code-standards.md` | Code philosophy, Bento Box Principle |
| `.claude/skills/` | Agent skills (pickup, project-manager, code-review, pre-commit, personas/) |
| Notion: "SEO & Monetization Strategy" | Pricing, revenue projections, quota limits |
