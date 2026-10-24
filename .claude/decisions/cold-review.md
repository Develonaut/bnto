# Decision: Cold Review Before Work Counts as Done

**Created:** October 24, 2026
**Status:** Decided by Ryan on October 24, 2026. Required for every PR from #745 on. #745 was the first PR to get one.
**Applies to:** every PR, by every agent: [`/code-review`](../skills/code-review/SKILL.md), [pre-commit.md](../rules/pre-commit.md), `/pre-commit`, `/pickup`, and CLAUDE.md's Agent Workflow.

---

## The problem

The work was reviewed by the agent that wrote it. `/code-review` ran in the author's own session, with the author's context, assumptions and blind spots, and fixed what it found itself. It was required only inside `/pickup`, and optional in `/pre-commit` (behind `-r`). The rule docs (pre-commit.md, the Agent Workflow) didn't mention review at all.

A self-review checks the work against what the author meant, not against what the repo's docs require. The author's blind spots slip through that way. The first cold reviews, of the PR that introduced them (#745), showed it. They found two ordering gaps its author had missed:

- the review ran before work that still needed reviewing;
- the PR description was written after the review that's meant to check it.

## What a cold review is

Ryan's definition: hand the body of work to a separate agent on the best model available, and ask it to review the work against the repo's docs, so the work meets bnto's standards, practices and patterns.

- **Cold:**
  - The reviewer is a fresh agent with none of the author's conversation.
  - It is never a fork, and never the author re-reading its own diff.
  - It gets the work and its brief, never the author's reasoning, so it judges what was done, not what was intended.
- **The best model:** the reviewer runs on the most capable model the session offers, set explicitly when launching it, never a smaller one than the author's.
- **Against the docs:**
  - The reviewer reads CLAUDE.md, the universal rules and the scope rules for what changed, and checks the work against them.
  - It checks the brief too: that RED tests came first and test the behavior, and that the PR's claims are true.
- **The reviewer reports, the author fixes.** Every finding cites a file and line, the rule it breaks (a doc link), a severity and evidence. The author fixes each one or justifies it in the PR, so a disagreement is visible rather than silently overridden.

## Where it sits

The steps run in this order:

1. **The work is complete:** code, tests, cleanup, and the PLAN.md and ISSUES.md updates.
2. **`task check` passes.**
3. **Commit and draft:** the author commits on the branch, without pushing, and drafts the PR description.
4. **The cold review runs** on the branch, its commits and the draft description.
5. **Each finding is answered:** the author fixes or justifies it, commits the fixes, updates the draft, and re-runs `task check`.
6. **A second cold review** runs if the fixes changed more than wording. If it still finds blocking or should-fix problems, the author fixes them and asks the user before a third.
7. **The author pushes** and opens the PR.

It runs once per PR, before its first push. After the PR opens, a commit that changes more than wording (a CI fix, a requested change) gets its own cold review before it's pushed.

The PR description records the outcome in one line, e.g. `Cold review: 3 findings, 2 fixed, 1 justified (below)`.

## Considered

- **Keep self-review and make it mandatory:** rejected. It doesn't remove the author's blind spots, which is the point of a cold review.
- **A separate `/cold-review` skill beside `/code-review`:** rejected. Two review skills would drift, and Ryan's framing is that a cold review is the code review. `/code-review` now is the cold review, and its checklist is the reviewer's procedure.
- **Let the reviewer fix what it finds:** rejected. A reviewer that edits becomes a second author, and its changes would need their own review. Reporting keeps the author accountable for the work.
