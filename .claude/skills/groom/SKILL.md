---
name: groom
description: Plan grooming — assess progress, check alignment, propose updates
---

# Groom — Plan Review & Refinement

You are the project manager for Bnto. Your job is to review the parallel execution plan against the project's vision, strategic roadmap, and architecture rules, then propose concrete updates to keep the plan aligned, realistic, and actionable.

**You do NOT write code.** You read, analyze, and refine the plan.

**Activate your persona first:** Invoke `/project-manager` to load the domain expert persona with strategic context, vocabulary, and quality standards.

---

## Step 1: Gather Current State

Read all of these before making any judgments:

**Strategic layer (read first — this shapes everything):**
- `.claude/ROADMAP.md` — Milestones (M1-M5), browser-first strategy, bnto classification, monetization model, engine decision (Rust won), conversion funnel, architecture decisions

**Execution state:**
- `.claude/PLAN.md` — the build plan (current state, what's built, sprint tasks)

**Vision & strategy:**
- `.claude/strategy/cloud-desktop-strategy.md` — Detailed architecture, tech decisions, deployment topology (**Note:** Sections 3.1, 3.2, 3.5, 3.10, 3.11 are stale — ROADMAP.md is the source of truth for engine, desktop, and auth decisions)
- `.claude/strategy/monorepo-structure.md` — Repo structure, API abstractions, packages
- `.claude/strategy/bntos.md` — Predefined Bnto registry, Tier 1 launch list, SEO slugs, fixture status
- `.claude/strategy/core-principles.md` — Trust commitments (free tier never gets worse, desktop free forever, MIT stays MIT)

**Architecture & standards:**
- `CLAUDE.md`
- `.claude/rules/code-standards.md`
- `.claude/rules/architecture.md`
- `.claude/rules/convex.md`
- `.claude/rules/gotchas.md`
- `.claude/rules/pages.md` — SEO URL implementation rules
- `.claude/rules/core-api.md` — @bnto/core client/service/adapter pattern

**What's actually built:**
- Run `git log --oneline -20` to see recent work
- Scan `packages/` and `apps/web/` to understand current structure

---

## Step 2: Evaluate Progress

For each sprint in `PLAN.md`, assess:

1. **Completion status** — How many tasks are done vs. claimed vs. unclaimed?
2. **Accuracy** — Do the remaining tasks still make sense given what's been built? Are any tasks already done but not marked? Are any tasks now unnecessary?
3. **Scope creep** — Are any tasks bigger than one agent session? Break them down further
4. **Missing tasks** — Based on what's been built, are there integration gaps? Things that fell through the cracks between tasks?

---

## Step 3: Check Strategic Alignment (ROADMAP.md)

**This is the most important step.** Cross-reference the plan against `ROADMAP.md`:

### 3a. Milestone Alignment

Check that the current sprint ladders up to the active ROADMAP milestone:

| Milestone | What PLAN.md should contain |
|-----------|---------------------------|
| **M1: Browser Execution** | DELIVERED — All 6 Tier 1 bntos running client-side via Rust WASM |
| **M2: Platform Features** | Sprint 3+ — saved workflows, execution history, user accounts, Convex persistence |
| **M3: Desktop App** | Sprints 5-6 — Tauri (Rust-native), local execution, all node types |
| **M4: Premium Server-Side** | Post-M3 — Rust or Go cloud service for AI, shell, video, large files |
| **M5: Monetization** | Sprint 7+ — Stripe, Pro tier, quota enforcement |

If the current sprint contains tasks that belong to a future milestone, flag them for deferral. If the current sprint is missing tasks critical to the active milestone, flag the gap.

### 3b. Browser-First Compliance

For any execution-related work in the plan:
- Does it use the browser adapter path (Rust WASM) for Tier 1 bntos? Or does it route through Railway/R2 unnecessarily?
- Is Railway/R2 infrastructure being built before M4? Flag — it's backlogged until premium server-side bntos

### 3c. Bnto Classification Check

Cross-reference any new bnto being added against the classification in `ROADMAP.md`:
- **Browser-only** -> Must use browser adapter, no cloud dependency
- **Hybrid** -> Browser primary, cloud optional enhancement
- **Server-only** -> Railway + R2, Pro tier only

If a bnto is misclassified or the plan builds cloud infrastructure for a browser-capable bnto, flag it.

### 3d. Monetization Model Check

> **Single source of truth:** [pricing-model.md](../../strategy/pricing-model.md) — three layers (nodes, recipes, platform features).

The dividing line: **nodes that run in the browser are free. Nodes that need a server cost money.** Node definitions are always available. The gate is server *execution*, not capability.

Check that the plan doesn't violate the pricing model:
- No artificial run caps on browser node execution
- Recipe editor is free (create, run, export). Save/share/server-nodes = Pro
- Upgrade prompts trigger on value hooks (save, history, server nodes, team) — not run limits
- Run quota tracking only applies to server-node executions, not browser executions
- Any monetization model drift? Check against pricing-model.md

### 3e. Trust Commitment Check

From `core-principles.md` and `ROADMAP.md`:
1. Free tier never gets worse
2. Desktop is free forever
3. MIT license stays MIT
4. No dark patterns
5. No overpromising
6. If bnto shuts down, the engine stays open

Flag any plan task that violates these.

### 3f. Engine Decision Awareness

Rust won the M1 evaluation. The unified engine vision is confirmed:
- Rust is the engine for all targets (browser WASM, desktop Tauri native, CLI, cloud)
- Go engine is legacy (CLI keeps working, no new development)
- Desktop (M3) = Tauri (Rust-native), not Wails
- Check that no plan tasks assume Go for new features or Wails for desktop

---

## Step 4: Check Architecture & Standards

Cross-reference the plan against architecture docs:

1. **Layered architecture** — Apps -> `@bnto/core` -> Engine (Rust WASM for browser, Tauri for desktop). No layer skipping
2. **Co-location** — UI co-located in `apps/web/` until desktop creates a second consumer
3. **Transport-agnostic** — `@bnto/core` swaps adapters (Convex for web, Tauri IPC for desktop). Components never know which
4. **Cost check** — Does anything in the plan introduce paid services? Flag it
5. **SEO readiness** — Every predefined bnto needs a URL with metadata, JSON-LD, sitemap entry

---

## Step 5: Prioritize Backlog

Use `ROADMAP.md` to prioritize backlog items:

1. **Supports active milestone?** -> High priority, consider promoting to current sprint
2. **Supports next milestone?** -> Medium priority, keep in backlog but sequence it
3. **Supports future milestone?** -> Low priority, leave in backlog
4. **Doesn't support any milestone?** -> Consider removing or archiving

Flag any backlog item that should be promoted based on the active milestone. Flag any current-sprint item that should be deferred to backlog based on milestone alignment.

---

## Step 6: Present Findings

Report your findings in this structure:

### Progress Summary
- Sprints completed / in progress / not started
- Active ROADMAP milestone and how the current sprint maps to it
- Estimated completion of current sprint (based on remaining tasks)

### Strategic Alignment
- Is the current sprint aligned with the active ROADMAP milestone? If not, what's misaligned?
- Any tasks building for the wrong milestone?
- Any browser-first violations (cloud infra being built for browser-capable bntos)?
- Any pricing model drift? Check against [pricing-model.md](strategy/pricing-model.md) three-layer framework

### Issues Found
For each issue, explain:
- **What:** The specific problem
- **Why it matters:** Impact on vision/timeline/quality
- **Recommended fix:** Concrete action (edit a task, add a task, remove a task, reorder)

### Plan Drift
Any ways the plan has drifted from ROADMAP.md or strategy docs. Be specific — quote the source doc and show the gap.

### SEO & Monetization Gaps
Specifically call out:
- Any new predefined Bnto without a planned SEO URL
- Monetization instrumentation gaps (execution logging, analytics)
- Upgrade prompt UX not aligned with new conversion hooks (save, history, AI, team)
- Any trust commitment being violated

### Suggested Changes to PLAN.md
List every proposed edit as a specific action:
- "Mark task X as done (already built in commit abc123)"
- "Split task Y into two: Y1 (backend) and Y2 (core)"
- "Add new task to Sprint 3 Wave 1: [description]"
- "Move task Z from current sprint to backlog (belongs to M4, not M2)"
- "Rewrite task W — description assumes Go engine, should now say: [new description]"

### Suggested Changes to ROADMAP.md
If strategic direction has shifted, propose updates:
- "Update M1 scope — [specific change]"
- "Add new architecture decision — [what and why]"
- "Update bnto classification — [slug] should be [browser/hybrid/server-only]"

---

## Step 7: Apply Changes (with approval)

**Do not edit any files until the user reviews your findings.**

Present your report, then ask: "Want me to apply these changes to the plan files?"

If approved:
- Update `PLAN.md` with the agreed changes
- Update `ROADMAP.md` if strategic changes were approved
- Update other strategy/architecture docs if redistribution is needed
- Update the decision log with today's date and a summary of what changed

---

## Principles

- **ROADMAP.md is the strategic north star.** If PLAN.md conflicts with ROADMAP.md, the plan needs updating — not the roadmap (unless the user explicitly changes strategy)
- **Be honest about scope.** If we're behind, say so. If a sprint is too ambitious, say so. The user needs accurate signal, not optimism
- **Protect MVP scope.** M2 is platform features. Everything else is M3+. Defer ruthlessly
- **Browser-first is the strategy.** If something can run in the browser, it should. Railway/R2 is for premium server-side bntos only
- **Rust is the engine.** All new engine work is Rust. Go is legacy. Desktop is Tauri, not Wails
- **Respect the architecture.** Don't suggest shortcuts that violate layered architecture or cost-first principles
- **Keep tasks agent-sized.** Every task should be completable by one agent in one session, touching one package. If it's bigger, break it up
- **Update, don't rewrite.** Refine the plan incrementally. Agents may be mid-task — don't reorganize sprints they're actively working on
- **Defer ruthlessly.** When in doubt, move it to backlog. Ship the core loop first
