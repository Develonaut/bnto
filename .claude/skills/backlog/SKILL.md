---
name: backlog
description: Quick-add a triage item to the backlog without losing focus
args: "<description of the issue or idea>"
---

# Backlog — Quick Add

Append a triage item to `PLAN.md` without breaking flow. This is for observations, bugs, and ideas that come up while working — capture them fast, triage later during `/groom`.

## Instructions

1. Take the user's description (passed as args or from the preceding message)
2. Write a concise backlog entry at the end of the `## Backlog` section in `.claude/PLAN.md`, just before `## Reference`
3. Use this format:

```markdown
### Triage: <short title>

**Priority: Triage.** <1-2 sentence description of the issue or idea, with enough context to evaluate later>

<optional: file paths, screenshots referenced, or related Sprint 5 tasks>
```

4. Confirm what was added in one line — do NOT elaborate, discuss, or propose solutions. The whole point is to not lose focus.

## Rules

- **Do NOT start a discussion.** Add the item and confirm. That's it.
- **Do NOT reorganize the backlog.** Just append.
- **Do NOT assign priority.** Mark as `Triage` — priority is decided during `/groom`.
- **Keep it short.** If the user gave a long explanation, distill to 1-2 sentences.
- **Include context.** If the user referenced a screenshot, file, or specific node/param, mention it.
