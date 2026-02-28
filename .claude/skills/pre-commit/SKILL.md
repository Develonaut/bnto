---
name: pre-commit
description: Run the mandatory pre-commit checklist
---

# Pre-Commit Checklist

Run through ALL steps below. If any step fails, STOP, fix the issue, and restart from the automated checks.

You are NOT allowed to deem any failures as "pre-existing" or skip them. Report ALL failures to the user and let them decide.

## Step 0: Read the Standards

Before reviewing ANY code, read and internalize the project's coding standards and architecture rules:

```
.claude/CLAUDE.md                  # Master reference — architecture, layering, tech stack
.claude/rules/code-standards.md    # Single responsibility, file/function size limits (Bento Box Principle)
.claude/rules/                     # All rule files (if present)
```

**Read ALL of these files now.** You will be reviewing code against every rule in them. The checklist items below are reminders — the rule files and CLAUDE.md are the source of truth.

## Step 0b: Activate Domain Personas

Identify which packages the changed files belong to and invoke the relevant persona skill(s):

| Changed files in... | Persona skill |
|---|---|
| `engine/` | `/rust-expert` |
| `archive/engine-go/`, `archive/api-go/` | `/go-engineer` |
| `apps/web/` | `/frontend-engineer` + `/nextjs-expert` |
| `packages/core/` | `/core-architect` |
| `packages/@bnto/backend/`, `packages/@bnto/auth/` | `/backend-engineer` |

**If changes touch auth, security headers, input validation, file uploads, or Convex mutations**, also invoke `/security-engineer` — the security persona owns trust boundaries across all packages.

**If changes touch `apps/web/`**, always invoke `/nextjs-expert` alongside `/frontend-engineer` — the Next.js expert catches framework-level performance issues, server/client boundary violations, and rendering strategy mistakes that the frontend engineer's component-level review won't cover.

**If changes touch E2E tests, test fixtures, test helpers, or screenshot baselines**, also invoke `/quality-engineer` — the quality persona owns testing strategy, E2E infrastructure, journey-based test design, screenshot regression workflows, and knows how to run tests correctly (port isolation, two-run screenshot verification, selector patterns).

**Invoke the matching persona skill(s) now.** Each persona is a domain expert with specialized quality standards, gotchas, and performance patterns that inform a deeper review. If changes span multiple packages, invoke all relevant personas — a commit touching `engine/` and `apps/web/` needs both `/rust-expert` and `/frontend-engineer` activated.

## Context

### Files changed
!`git diff --name-only HEAD 2>/dev/null; git ls-files --others --exclude-standard`

### Current branch
!`git branch --show-current`

## Step 1: Code Review

Run `/code-review` to audit all changed files against the project's coding standards, architecture rules, and known gotchas. This catches structural and architectural issues that linters miss.

**Fix any violations before proceeding.** The code review covers: architecture & layer compliance, Bento Box, TypeScript, Go code quality, React Query & state management, performance, code quality, test coverage, and stale artifacts.

## Step 2: Automated Checks

Run these sequentially — stop on first failure:

### Go checks
```bash
task vet               # go vet — must pass clean
task test              # engine tests with race detector — must pass
task api:test          # API server tests with race detector — must pass
```

### TypeScript checks
```bash
task ui:build          # TypeScript compilation — must pass
task ui:test           # Frontend tests — must pass
task ui:lint           # Lint all TS packages — must pass
```

If any fail: fix the errors, re-run from the top.

## Step 3: Test Coverage & Quality Verification

Run `/test-review` to evaluate test quality against the project's value matrix. This reviews both coverage (are the right things tested?) and quality (are tests testing behavior or implementation details?).

The test review covers: Go engine fixtures, Convex auth enforcement, Core transforms, E2E user journeys, and flags wasteful tests (testing framework behavior, over-mocking, testing implementation details).

**Fix any findings before proceeding.** Delete wasteful tests, add missing high-value tests, rewrite anti-pattern tests.

**If you're writing or updating E2E tests**, invoke `/quality-engineer` for guidance on selectors, screenshot workflows, port isolation, and journey-based test design. The quality engineer owns the E2E infrastructure and testing patterns.

### Did you touch UI? (MANDATORY check)

Ask yourself: **did you create, modify, or wire up ANY component, dialog, form, page, or layout that a user will see or interact with?** This includes:

- Components in `apps/web/components/` (even "presentational only" — they render on screen)
- Wiring in `apps/web/` (routes, dialogs, pages)
- Changes to props, layout, styling, or behavior of existing UI

**If yes — you MUST write or update e2e tests with screenshot assertions.** This is non-negotiable. Unit tests alone are not proof that UI works. The user needs to see tangible visual evidence that the feature renders correctly.

**Required e2e coverage:**
- Add to or create spec files in `apps/web/e2e/`. Use existing helpers and patterns from sibling spec files.
- Test the actual user flow, not just that a page renders.
- Include `await expect(page).toHaveScreenshot()` assertions — at minimum:
  - One screenshot of the primary UI state the change introduces or modifies
  - One screenshot of any new dialog, modal, or form in its populated state
- Run the e2e tests and confirm screenshots are generated

**"It's just a UI component" is not an excuse to skip e2e tests.** If it renders on screen, it gets tested on screen. A `[ui]` task that creates a form component used by a `[web]` dialog still needs an e2e test proving the dialog works end-to-end.

**If you genuinely believe no e2e test is needed** (e.g., pure internal refactor with zero visual change), you MUST ask the user for explicit approval before skipping. Do not decide this on your own.

If screenshots already exist and the change modifies visual output, run with `--update-snapshots` after confirming the new appearance is correct.

## Step 4: Security Review

Run `/security-review` to audit the changed files and overall project for security issues — leaked secrets, auth gaps, input validation, XSS vectors, cloud service misconfigurations, and dependency vulnerabilities.

**This is a full audit, not a quick scan.** The security review covers: secret scanning, Go API security (auth, CORS, input validation, execution sandboxing), Convex function security (auth enforcement, quota bypass, upload validation), web app security (headers, XSS, cookies), infrastructure (GitHub, Vercel, Railway, R2, Convex deployment), dependencies, and open source readiness.

**If the security review surfaces any HIGH or CRITICAL findings, STOP.** Report them to the user before proceeding to the commit step. The user decides whether to fix now or defer.

## Step 5: Proof of Work Summary

Before committing, present a summary to the user:

1. **Code review result** — confirm `/code-review` passed clean (or list fixes made)
2. **Did you touch UI?** — Yes or No. If you created, modified, or wired up any component, dialog, form, page, or layout — the answer is Yes.
3. **If yes:** What e2e tests did you write or update? List spec files and the flows they cover. List screenshot assertions. If no e2e tests, explain why and confirm user approved the skip.
4. **If no UI touched:** What unit/integration tests were written? List test files and what they cover.
5. **TS checks result** — confirm `task ui:build`, `task ui:test`, `task ui:lint` passed clean
7. **Security review result** — confirm `/security-review` passed clean, or list any HIGH/CRITICAL findings and user's decision (fix now or defer)
8. **Files changed** — files created/modified, with brief description of each

## Step 6: Commit & Branch Workflow

**Branch-based development is mandatory.** `main` is protected — all changes go through PRs with CI gate.

### Branching

1. **Create a feature branch** before committing: `git checkout -b <type>/<short-description>` (e.g., `feat/execution-history`, `fix/skeleton-layout-shift`, `chore/eslint-config`)
2. **Branch naming:** `feat/`, `fix/`, `chore/`, `refactor/`, `test/` prefixes. Lowercase, hyphen-separated.
3. **Never commit directly to `main`.** Branch protection requires PRs to pass the CI Gate check.

### Committing

Stage only relevant files, then commit per these rules:

- Summarize the "why", not the "what"
- Keep subject under 72 characters
- Use imperative mood ("Add feature" not "Added feature")
- Do NOT include `Co-Authored-By`, `Generated with Claude Code`, or "Test Plan" sections
- Do NOT bundle unrelated changes

Present the proposed commit message to the user for approval before committing.

### Pushing & PRs

- Only commit YOUR OWN work. If `git status` shows changes from other agents or unrelated work, DO NOT stage or commit those files. Only stage files you personally created or modified as part of this task.
- **Push to your feature branch**, then create a PR targeting `main`.
- **CI must pass** before merging. The `CI Gate` check (Rust + TypeScript) is required.
- **NEVER force-push to `main`** or merge without CI passing.
- Ask the user before pushing if you're unsure. A request to "commit" does not imply "push."
