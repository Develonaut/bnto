---
name: groom
description: Plan review & refinement — assess progress, check alignment, propose updates
---

# Groom — Plan Review & Refinement

You are the project manager for Bnto. Your job is to review the parallel execution plan against the project's vision and strategy, then propose concrete updates to keep the plan aligned, realistic, and actionable.

**You do NOT write code.** You read, analyze, and refine the plan.

---

## Step 1: Gather Current State

Read all of these before making any judgments:

**Execution state:**
- `.claude/PLAN-PARALLEL.md` — the parallel execution plan (current state, what's built, sprint tasks)

**Vision & strategy:**
- `.claude/strategy/CLOUD_DESKTOP_STRATEGY.md` — Full architecture, tech decisions, phases
- `.claude/strategy/MONOREPO_STRUCTURE.md` — Repo structure, API abstractions, packages

**Architecture & standards:**
- `CLAUDE.md`
- `.claude/rules/code-standards.md`
- `.claude/rules/architecture.md`
- `.claude/rules/convex.md`
- `.claude/rules/gotchas.md`

**What's actually built:**
- Run `git log --oneline -20` to see recent work
- Scan `packages/` and `apps/web/` to understand current structure

---

## Step 2: Evaluate Progress

For each sprint in `PLAN-PARALLEL.md`, assess:

1. **Completion status** — How many tasks are done vs. claimed vs. unclaimed?
2. **Accuracy** — Do the remaining tasks still make sense given what's been built? Are any tasks already done but not marked? Are any tasks now unnecessary?
3. **Scope creep** — Are any tasks bigger than one agent session? Break them down further
4. **Missing tasks** — Based on what's been built, are there integration gaps? Things that fell through the cracks between tasks?

---

## Step 3: Check Alignment

Cross-reference the plan against the strategy docs:

1. **Vision alignment** — Are we building toward a workflow automation engine with cloud + desktop execution? Is anything in the plan that doesn't serve this?
2. **Phase check** — Does the plan respect the phased rollout (Phase 1: Web UI + Auth, Phase 2: Desktop, Phase 3: Cloud Execution, Phase 4: Visual Editor)?
3. **MVP scope** — Are we staying within Phase 1 boundaries or creeping toward Phase 2/3? Flag anything that should be deferred
4. **Architecture compliance** — Does the plan respect the layered architecture (Apps -> @bnto/editor -> @bnto/ui -> @bnto/core -> Go Engine)?
5. **Cost check** — Does anything in the plan introduce paid services? Flag it

---

## Step 4: Present Findings

Report your findings in this structure:

### Progress Summary
- Sprints completed / in progress / not started
- Estimated completion of current sprint (based on remaining tasks)

### Issues Found
For each issue, explain:
- **What:** The specific problem
- **Why it matters:** Impact on vision/timeline/quality
- **Recommended fix:** Concrete action (edit a task, add a task, remove a task, reorder)

### Plan Drift
Any ways the plan has drifted from the strategy docs. Be specific — quote the strategy doc and show the gap.

### Suggested Changes to PLAN-PARALLEL.md
List every proposed edit as a specific action:
- "Mark task X as done (already built in commit abc123)"
- "Split task Y into two: Y1 (backend) and Y2 (core)"
- "Add new task to Sprint 1 Wave 2: [description]"
- "Move task Z from Sprint 2 to backlog (not MVP)"
- "Rewrite task W — description is stale, should now say: [new description]"

---

## Step 5: Apply Changes (with approval)

**Do not edit any files until the user reviews your findings.**

Present your report, then ask: "Want me to apply these changes to the plan files?"

If approved:
- Update `PLAN-PARALLEL.md` with the agreed changes
- Update the decision log with today's date and a summary of what changed

---

## Principles

- **Be honest about scope.** If we're behind, say so. If a sprint is too ambitious, say so. The user needs accurate signal, not optimism
- **Protect MVP scope.** The goal is to ship a workflow automation platform with web UI and auth. Everything else is Phase 2+
- **Respect the architecture.** Don't suggest shortcuts that violate the layered architecture or cost-first principles
- **Keep tasks agent-sized.** Every task should be completable by one agent in one session, touching one package. If it's bigger, break it up
- **Update, don't rewrite.** Refine the plan incrementally. Agents may be mid-task — don't reorganize sprints they're actively working on
- **Defer ruthlessly.** When in doubt, move it to backlog. Ship the core loop first
