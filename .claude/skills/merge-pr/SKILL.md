---
name: merge-pr
description: Verify and merge a pull request into main
---

# Merge PR

Verify a pull request is ready to merge and merge it after user approval. This skill handles CI verification, description auditing, conflict detection, and squash merge.

**This is NOT a code review.** Run `/code-review` separately before `/merge-pr`. This skill checks merge readiness — CI, conflicts, description accuracy — not code quality.

## Arguments

| Flag | Description |
|------|-------------|
| `--skip-e2e` | Skip the mandatory E2E test step (Step 3b). Only the user can pass this flag — agents must never add it on their own. |

**Usage:** `/merge-pr --skip-e2e` or `/merge-pr 42 --skip-e2e`

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

**Determine CI Gate status:**
- All checks passing = CI PASS
- Any check failing = CI FAIL — list the failing checks with their names
- Any check pending = CI PENDING — list pending checks, ask user if they want to wait

**If CI is not passing, present the failures and stop.** Do not offer to merge with failing CI.

## Step 3b: E2E Tests Against Vercel Preview (MANDATORY)

**This step is MANDATORY unless the user passed `--skip-e2e`.** You MUST run E2E tests against the Vercel preview deployment before merging. You do NOT get to decide whether to skip this step — regardless of what files were changed. Only the user can explicitly grant a skip via the `--skip-e2e` flag or by telling you directly.

If `--skip-e2e` was passed, note "E2E Tests: SKIPPED (user opted out)" in the merge readiness summary and proceed to Step 4.

### Why

There is no E2E CI workflow. E2E tests run locally on the developer's machine against the deployed Vercel preview. This eliminates cross-platform screenshot differences (macOS font rendering vs Linux) and gives immediate, accurate feedback.

### How to Run

1. **Find the Vercel preview URL** for this PR:

```bash
# Get the preview URL from the PR's deployment status
gh pr view <number> --json statusCheckRollup --jq '.statusCheckRollup[] | select(.context | contains("vercel")) | .targetUrl'
```

If no Vercel preview URL is found or the deployment is still in progress:
- **Notify the user** that the preview deployment is not ready yet
- **Ask the user** whether they want you to wait and retry, or provide the URL manually
- **Do NOT proceed without a working preview URL.** You cannot skip E2E tests because the deployment isn't ready — you must wait or get the URL from the user
- If the user says to wait, poll every 30 seconds (up to 5 minutes) for the deployment to complete, then retry

2. **Run E2E tests against the preview:**

```bash
cd apps/web && BASE_URL=<vercel-preview-url> pnpm exec playwright test
```

If the Vercel deployment has protection enabled and a bypass secret is available:

```bash
cd apps/web && BASE_URL=<vercel-preview-url> VERCEL_AUTOMATION_BYPASS_SECRET=<secret> pnpm exec playwright test
```

3. **Report full results.** Present:
   - Total tests run, passed, failed, skipped
   - List every failure with test name and error message
   - If screenshots were captured, note any mismatches

### Pass / Fail Criteria

- **ALL tests must pass.** Zero failures.
- **Screenshot mismatches count as failures.** If visual regression tests fail, the screenshots differ from baselines — this blocks merge.
- **Known "01 Issue" hydration failures** (React 19 + Radix `useId()` SSR mismatch) are acceptable ONLY when there are zero screenshot mismatches. Report them to the user but they do not block merge.
- **Flaky tests that pass on retry** should be noted but do not block merge (Playwright retries are configured).

### If Tests Fail

**Do NOT offer to merge.** Report the failures and stop. The user must decide how to proceed:
- Fix the failures and re-run
- Explicitly grant a skip (their decision, not yours)

### Proof of Work

Include the E2E results in the merge readiness summary (Step 6). Specifically:
- The Vercel preview URL tested against
- The full test result summary (e.g., "96 passed, 0 failed, 0 skipped")
- Any notable warnings or known issues encountered

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
- E2E Tests: PASS (N passed, 0 failed) / FAIL (list failures) / NOT RUN (blocked)
  - Preview URL: <vercel-preview-url>
- Merge conflicts: None / CONFLICTING
- Description: Accurate / Needs update (details)

### Changed Files
<file list from --name-only>

### Recommendation
Ready to merge / Blocked (reasons)
```

**Always explicitly state the source branch and target branch.** The user must be able to confirm at a glance that the PR is merging the right branch into the right target.

**If blocked:** explain what needs to happen before merging. Stop here.

**If ready:** present the full summary including E2E proof of work, then **ask the user for explicit final approval before merging.** You MUST wait for the user to confirm — never auto-merge, even if every check passes. Present merge strategy options:

- **Squash merge** (default) — single commit on main, clean history
- **Merge commit** — preserves branch history
- **Rebase** — linear history, individual commits preserved

## Step 7: Merge

**Only proceed after explicit user confirmation.** Never auto-merge. The user must see the full merge readiness summary (including E2E results) and explicitly approve before you execute the merge command.

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
3. Suggest `git checkout main && git pull` to update local

**If merge fails:** report the error. Common causes:
- Branch protection rules not met (missing reviews, required checks)
- Merge conflict appeared between check and merge
- Permission issues

Report the specific error and suggest next steps.
