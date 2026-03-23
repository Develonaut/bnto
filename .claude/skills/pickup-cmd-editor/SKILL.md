---
name: pickup-cmd-editor
description: Pick up the next CmdEditor phase task
args: ""
---

# Pickup CmdEditor Phase

Picks up the next available task from the CmdEditor build plan. Temporary skill — removed in Phase 8.

## Workflow

### Step 1: Read the Plan

Read `.claude/PLAN-CMD-EDITOR.md`. Find the first phase with **TODO** tasks.

**Blocking rule:** If the previous phase has incomplete tasks (TODO or DOING), do NOT start a new phase. Report the blocker and stop.

### Step 2: Claim a Task

Find the first task in the current phase with status `TODO`. Change its status to `DOING` in the plan file.

### Step 3: Read Context

Before writing any code:

1. Read `.claude/CLAUDE.md` (project rules)
2. Read `.claude/strategy/cmd-editor.md` (CmdEditor design)
3. Read `.claude/strategy/editor-architecture.md` (shared editor layer)
4. Read the **Agent Prompt** for your claimed task — it contains full instructions

### Step 4: Execute

Follow the agent prompt exactly. Respect all rules in `.claude/rules/`:

- `code-standards.md` — Bento Box Principle, size limits
- `components.md` — component patterns, CSS-first states
- `theming.md` — Motorway tokens
- `animation.md` — animation components, motion-safe

### Step 5: Code Review Gate

If the task is a **code review gate** (type: Quality):

1. Run `/code-review` on all changes in the current phase
2. Fix all issues raised
3. Run `/code-review` again — repeat until clean
4. Three clean passes required before proceeding

### Step 6: Mark Done

Change the task status from `DOING` to `DONE` in `.claude/PLAN-CMD-EDITOR.md`.

### Step 7: Phase Complete?

If all tasks in the current phase are `DONE`, run the checkpoint protocol:

1. `task ui:build` — must pass
2. `task ui:test` — must pass
3. Commit with message: `feat(editor): phase N — <description>`

Then report completion and stop. The next `/pickup-cmd-editor` invocation picks up the next phase.
