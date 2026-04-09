# Bnto — Backlog & Deferred Work

**This file contains all backlog items, tabled sprints, and frozen work.** For the active plan, see [PLAN.md](PLAN.md). For completed sprint history, see [PLAN-HISTORY.md](PLAN-HISTORY.md).

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

### Deep Backlog: Distribution (Desktop + Server)

**Deferred.** Desktop (Tauri) and server-side execution in deep backlog. Revisit after TUI ships and community traction emerges.

- Desktop app (Tauri) — links engine natively like CLI, system webview for React frontend
- Server-side execution — cloud infrastructure for premium nodes (AI, shell, video at scale)

---

## Phase 3: Monetization + Polish — TABLED

**Tabled (April 2026).** Monetization is explicitly paused. Focus is on engine power and fun. Revenue strategy revisited when the tool has community traction. The plan below is preserved for when this becomes relevant.

**"Ready to charge" gate:** Before starting, confirm: real users running bntos, conversion hooks built and tested, people return voluntarily, at least one server-side bnto (AI or shell) ready for Pro tier.

### Sprint 12: Stripe + Pro Tier (M5) — TABLED

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

### Triage: iLovePNG recipe parity — next wave candidates

**Priority: Medium.** When planning the next recipe wave, evaluate iLovePNG's offerings for feasibility: Resize IMAGE, Crop IMAGE, Rotate IMAGE, Watermark IMAGE (done), Blur face, Upscale, Convert to/from JPG, HTML to IMAGE, Meme generator. Several (resize, crop, rotate) are doable with existing `image` crate.

### Triage: Engine documentation — auto-generated docs

**Priority: Low.** Set up `cargo doc` or docs site for the Rust engine. Document crate responsibilities, API surface, architecture. `engine/crates/`.

### Triage: Definition/recipe version migration tool

**Priority: Medium.** `bnto migrate` CLI command for breaking changes to `.bnto.json` node parameters (e.g., `compression`→`quality`). Versioned migration system: detect version, apply sequential transforms, report changes. The `version` field already exists in `Definition`.

### @bnto/i18n: Interpolation + Raw Text Migration

**Priority: Low.** Add `{{variable}}` interpolation support to `t()` so dynamic values (recipe counts, etc.) can live in `en.json` instead of template literals in components. Then migrate all hardcoded `<Text>` strings in landing page components to `t()` calls.

- [ ] `packages/@bnto/i18n` — Add optional `vars` parameter to `t()`: `t(key, { count: 15 })` replaces `{{count}}` in the resolved string
- [ ] `packages/@bnto/i18n` — Unit tests for interpolation (single var, multiple vars, missing var, no vars)
- [ ] `apps/web` — Migrate hardcoded strings in landing page section components to `t()` calls
- [ ] `packages/@bnto/i18n` — Move dynamic recipe count strings to `en.json` with `{{count}}` placeholders

### @bnto/ui: `<SpringIn>` Entrance Animation Component

**Priority: Low.** Homepage shipped using `Card dormant` prop + `ScaleIn`/`SlideUp` instead. `SpringIn` is a nice-to-have refinement for future card-heavy sections, not a blocker.

The springable surface system (grounded → raised with bouncy spring) is the most satisfying animation in Motorways, but it's currently only available as a **state toggle** on `<Card loading>` / `<Surface grounded>`. `<SpringIn>` would bridge this gap: a keyframe-based entrance animation where elements start grounded and spring up to their natural elevated state on mount.

- [ ] `packages/ui` — Create `@keyframes spring-in` in `animations.css`
- [ ] `packages/ui` — Create `SpringIn` component following `ScaleIn` pattern
- [ ] `packages/ui` — Add `spring` prop: `"bouncy" | "bouncier" | "bounciest"`
- [ ] `packages/ui` — Add `elevation` prop: `"sm" | "md" | "lg"`
- [ ] `packages/ui` — Ensure composability with `<Stagger>`
- [ ] `packages/ui` — Respect `motion-safe:` prefix
- [ ] `packages/ui` — Unit tests
- [ ] `apps/web` — Add `SpringIn` demo to Motorway animation showcase tab

---

### Homepage Remaining Polish (low priority, deferred)

- [ ] Piece 10: Recipe page animations (`SlideUp` on header, `ScaleIn` on drop zone, `FadeIn` on config)
- [ ] Piece 11 remaining: Purchase 3-4 category mascot characters, convert to SVG components with size variants
- [ ] Piece 12: FAQ page `ScaleIn` entrance animations

### Triage: Secret/environment variable management for recipes

**Priority: Medium.** Recipes will need secrets (API keys, tokens, env vars) without embedding in `.bnto.json`. Design: how recipes reference variables, how secrets resolve per target (CLI reads env/dotfiles, server reads vault, browser prompts user).

### Triage: E2E teardown cleanup fails in release pipeline

**Priority: Low.** E2E teardown logs `cleanup failed` because `CONVEX_DEPLOYMENT` isn't set in release pipeline. Either pass env var to E2E job or skip cleanup against Vercel preview.

### Triage: Rename registry constructors

**Priority: Low.** `create_default_registry()` → `create_browser_registry()`, `create_native_registry()` → `create_registry()`. The "full" registry should be the default name, the WASM-constrained one the exception.

### Infra: Conventional Commits + Auto-Changelog

**Priority: Low.** Enforce `feat:`, `fix:`, `BREAKING CHANGE:` commit format. Auto-generate `CHANGELOG.md` on release tags. Not blocking anything.

### Infra: Production Deploy Protection (GitHub Environments)

**Priority: Low.** Manual approval step via GitHub Environments for production deploys. Existing tag-based workflow already prevents accidental deploys.

### Infra: Upgrade GitHub Actions to Node.js 24

**Priority: Low (deadline: June 2, 2026).** Upgrade `actions/checkout` to v5 when available. Audit all actions for Node.js 24 compatibility.

### Chore: Upgrade Convex 1.31.7 → 1.33.1

**Priority: Low.** Minor Convex JS SDK update. Bump in `packages/@bnto/backend/`, run `task check`.

### Triage: Responsive GridItem props

**Priority: Triage.** `GridItem` props (`colSpan`, `rowSpan`, `colStart`, `rowStart`) should accept `ResponsiveProp<T>` like the `Grid` `cols` prop does, so spans and positions can vary by breakpoint (mobile/tablet/desktop). Currently only `cols` is responsive — all placement props are static.

`packages/ui/src/layout/Grid.tsx`

### Triage: File Node Ecosystem — BRU-Style Composable File Operations

**Priority: Triage.** Expand the `file` category from 1 recipe to 6-8 with composable node processors inspired by Bulk Rename Utility. Enhance `file-rename` (counter, extension params), add new nodes (`file-collect`, `file-copy`, `file-filter`, `file-sanitize`, `file-metadata`, `svg-optimize`), and extend `image-convert` for vector formats (EPS/AI→SVG, SVG→PNG via `resvg`). Each node unlocks standalone recipes and custom compositions. Full strategy: [file-node-ecosystem.md](.claude/strategy/file-node-ecosystem.md)

### Triage: Homepage hero — BRU-style file recipe showcase

**Priority: Triage.** Add a file operation composition (e.g. `collect → sanitize → rename → copy`) as a "Build Your Own" hero snippet in `BuildYourOwnSection`. Demonstrates composable power vs monolithic tools. Blocked on file node ecosystem implementation.

`apps/web/app/(app)/_components/BuildYourOwnSection.tsx`, `recipeSnippets.ts`

---

## Reference

| Document                                                         | Purpose                                                                        |
| ---------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| `.claude/strategy/bntos.md`                                      | Predefined Bnto registry — slugs, fixtures, SEO targets, tiers                 |
| `.claude/strategy/engine-execution.md`                           | Engine execution architecture — pipeline executor, progress events             |
| `.claude/strategy/cloud-desktop-strategy.md`                     | Architecture, cost analysis, cloud execution topology                          |
| `.claude/strategy/core-principles.md`                            | Trust commitments, key principles                                              |
| `.claude/strategy/expression-input-ux.md`                        | Expression input UX (frozen — reference for future)                            |
| `.claude/rules/`                                                 | Auto-loaded rules (architecture, code-standards, engine-node-patterns, etc.)   |
| `.claude/skills/`                                                | Agent skills (pickup, project-manager, code-review, pre-commit)                |
| Private business docs (`BNTO_PRIVATE_DOCS_PATH` in `.env.local`) | Pricing strategy, revenue projections, SEO monetization (historical — on hold) |
