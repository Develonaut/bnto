---
name: merge-pr
description: Verify and merge a pull request into main
---

# Merge PR

Verify a pull request is ready to merge and merge it after user approval. This skill handles CI verification, description auditing, conflict detection, and squash merge.

**This is NOT a code review.** Run `/code-review` separately before `/merge-pr`. This skill checks merge readiness — CI, conflicts, description accuracy — not code quality.

## Step 1: Identify the PR

Determine which PR to merge:

- **If args contain a PR number** (e.g., `/merge-pr 42`): use that number
- **If no args**: detect from the current branch

```bash
# With args
gh pr view <number> --json number,title,state,headRefName

# Without args — detect from current branch
gh pr view --json number,title,state,headRefName
```

If no PR is found, ask the user for a PR number or a link to the PR.

## Step 2: Fetch PR State

Gather everything needed for the merge readiness assessment:

```bash
# Full PR details
gh pr view <number> --json number,title,body,state,headRefName,baseRefName,mergeable,additions,deletions,changedFiles,statusCheckRollup,reviewDecision,isDraft

# Files changed (for description audit)
gh pr diff <number> --name-only

# Full diff (for description accuracy check)
gh pr diff <number>
```

**Stop immediately if:**

- PR state is not `OPEN` — report the current state and stop
- PR is a draft — tell the user to mark it ready first

## Step 3: CI Verification

Check that all required status checks are passing.

Parse `statusCheckRollup` from Step 2. For each check:

- Name, status (pass/fail/pending), conclusion

**Required checks to verify:**

| Check             | Workflow                                                                  | Blocking?                                                                                                   |
| ----------------- | ------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| **CI Gate**       | `ci.yml` — Rust lint/test + TypeScript build/test/lint                    | Yes — must pass                                                                                             |
| **Lighthouse CI** | `lighthouse.yml` — Performance, Accessibility, Best Practices, SEO audits | Yes — error-level assertions (a11y, best practices, SEO ≥ 90) must pass. Performance warnings are advisory. |

**Determine status for EACH required check:**

- All checks passing = CI PASS
- Any check failing = CI FAIL — list the failing checks with their names and conclusions
- Any check pending = CI PENDING — list pending checks, ask user if they want to wait

**If Lighthouse CI is failing:** Note which categories failed and their scores. If only **performance** warnings (warn level, not error), the check still passes — performance is advisory. If **accessibility, best practices, or SEO** assertions fail (error level), that blocks merge. Suggest running `/lighthouse-audit --ci` to download reports and triage.

**If CI is not passing, present the failures and stop.** Do not offer to merge with failing CI.

## Step 4: Conflict Check

Check the `mergeable` field from Step 2:

- `MERGEABLE` = no conflicts
- `CONFLICTING` = merge conflicts exist — list conflicting files if available, tell user to resolve, stop
- `UNKNOWN` = GitHub is still computing — wait a moment and re-fetch

## Step 5: Description Audit

This is the unique value of this skill — an agent-powered check that the PR description accurately reflects the actual changes.

**Read the full diff** from Step 2. Understand what changed: files added/modified/deleted, the nature of the changes (new feature, bug fix, refactor, config change, etc.).

**Read the PR body** (description). Compare against the diff:

- **Missing description**: PR body is empty or just a template — flag as "needs description"
- **Stale description**: Description mentions files or changes that don't exist in the diff, or misses significant changes — flag specific mismatches
- **Accurate**: Description reasonably reflects the changeset

**Be pragmatic, not pedantic.** A description doesn't need to list every file. It should convey the intent and scope of the changes. Flag only meaningful mismatches — not minor omissions.

## Step 6: Merge Readiness Summary

Present a clear summary to the user:

```
## PR #<number>: <title>

**Branch:** <head>
**Target:** <base> (confirm this is the intended merge target)
**Changes:** <additions> additions, <deletions> deletions across <changedFiles> files

### Checks
- CI Gate: PASS / FAIL / PENDING
- Lighthouse CI: PASS / FAIL (list failing categories + scores) / PENDING
- Merge conflicts: None / CONFLICTING
- Description: Accurate / Needs update (details)

### Changed Files
<file list from --name-only>

### Recommendation
Ready to merge / Blocked (reasons)
```

**Always explicitly state the source branch and target branch.** The user must be able to confirm at a glance that the PR is merging the right branch into the right target.

**If blocked:** explain what needs to happen before merging. Stop here.

**If ready:** present the full summary, then **ask the user for explicit final approval before merging.** You MUST wait for the user to confirm — never auto-merge, even if every check passes. Present merge strategy options:

- **Squash merge** (default) — single commit on main, clean history
- **Merge commit** — preserves branch history
- **Rebase** — linear history, individual commits preserved

## Step 7: Merge

**Only proceed after explicit user confirmation.** Never auto-merge. The user must see the full merge readiness summary and explicitly approve before you execute the merge command.

```bash
# Squash merge (default)
gh pr merge <number> --squash --delete-branch

# Or merge commit if user requested
gh pr merge <number> --merge --delete-branch

# Or rebase if user requested
gh pr merge <number> --rebase --delete-branch
```

After merge:

1. Confirm the merge succeeded
2. Confirm the remote branch was deleted
3. Run the cleanup steps in Step 8

**If merge fails:** report the error. Common causes:

- Branch protection rules not met (missing reviews, required checks)
- Merge conflict appeared between check and merge
- Permission issues

Report the specific error and suggest next steps.

## Step 8: Post-Merge Cleanup

After a successful merge, clean up the local environment so you're back on a fresh `main`.

### Determine your context

```bash
# Are we in a worktree?
git worktree list
```

### If on a feature branch (normal flow):

```bash
# Switch back to main and pull the merged changes
git checkout main && git pull

# Delete the local feature branch
git branch -d <branch-name>
```

### If in a worktree:

```bash
# Get the worktree path and main repo path
git worktree list
```

1. Exit the worktree — switch the session back to the main repo root
2. Remove the worktree:

```bash
cd /Users/ryan/Code/bnto
git worktree remove .claude/worktrees/<name>
```

3. Update main:

```bash
git checkout main && git pull
```

4. Delete the local branch if it still exists:

```bash
git branch -d <branch-name>
```

### Confirm cleanup

Report to the user:

- Current branch: `main`
- Local branch deleted: yes/no
- Worktree removed: yes/no (if applicable)
- `main` is up to date with origin
