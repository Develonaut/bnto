---
name: lighthouse-audit
description: Run Lighthouse CI, identify failing audits, fix issues, and verify score improvements
args: "[--local | --ci]"
---

# Lighthouse Audit

Run Lighthouse CI against all public routes, identify failing accessibility/SEO/performance audits, fix the source components, and verify score improvements. This is a propose-then-execute workflow — findings are presented to the user before any fixes are applied.

## Arguments

| Flag | Description |
|------|-------------|
| `--local` | (Default) Run `task seo:audit` locally to produce fresh reports |
| `--ci` | Parse reports from the latest CI run via `gh run download` |

## Step 1: Activate Personas

Invoke `/frontend-engineer` and `/nextjs-expert`. Lighthouse audits span component accessibility (frontend) and rendering strategy (Next.js).

## Step 2: Acquire Reports

### Local mode (default)

Run the full Lighthouse CI audit locally. This builds the app and runs Lighthouse against all 10 public routes.

```bash
task seo:audit
```

This executes:
1. `pnpm --filter @bnto/web build` — full production build
2. `lhci autorun --config=lighthouserc.json --collect.numberOfRuns=1` — starts the production server, audits all URLs

**Stop if the build fails.** Fix build errors before proceeding.

Reports are written to `.lighthouseci/` as JSON files (one per URL).

### CI mode (`--ci`)

Download the latest Lighthouse CI artifacts from GitHub Actions:

```bash
# Find the latest Lighthouse CI run
gh run list --workflow="Lighthouse CI" --limit=1 --json databaseId,status,conclusion

# Download artifacts
gh run download <run-id> --name lhci-results --dir .lighthouseci/
```

**Stop if no recent run exists or the run failed before producing reports.**

## Step 3: Parse & Triage

Extract failing audits from the JSON reports in `.lighthouseci/`.

For each report file:

```bash
# List all JSON report files
ls .lighthouseci/lhr-*.json
```

Parse each report and extract:
- **Category scores** — performance, accessibility, best-practices, seo (0-1 scale, displayed as 0-100)
- **Failing audits** — any audit with `score < 1` in the accessibility, seo, or best-practices categories
- **Affected elements** — the `details.items[].node.snippet` and `details.items[].node.selector` for each failing audit

**Rank by impact:**
1. `error`-level category failures (accessibility, best-practices, seo with score < 0.9) — these block CI
2. `warn`-level category failures (performance with score < 0.9) — advisory
3. Individual audit failures within passing categories — improvement opportunities

## Step 4: Present Findings (USER APPROVAL GATE)

Present a structured report to the user. **Do not apply any fixes until the user approves.**

### Format

```
## Lighthouse Audit Results

### Category Scores (per URL)

| URL | Performance | Accessibility | Best Practices | SEO |
|-----|-------------|---------------|----------------|-----|
| /   | 95          | 87            | 100            | 100 |
| ... | ...         | ...           | ...            | ... |

### Failing Audits

#### [audit-id] — [audit title]
- **Category:** accessibility / seo / best-practices
- **Score:** 0.X
- **Weight:** X (contribution to category score)
- **Affected URLs:** /, /compress-images, ...
- **Affected elements:**
  - `<element snippet>` — selector: `.css-selector`
- **Suggested fix:** [brief description]

### Proposed Fix Plan

1. Fix [audit-id] in [Component.tsx] — [what to change]
2. Fix [audit-id] in [file] — [what to change]
...
```

**Wait for user approval before proceeding to Step 5.**

## Step 5: Research Sources

For each failing audit, trace the affected elements back to their source components:

1. **Read the CSS selector and HTML snippet** from the audit details
2. **Search the codebase** for the component that renders that element:
   - Use `data-testid`, `data-slot`, class names, or element structure to identify the source file
   - Check `apps/web/components/` and `apps/web/app/` directories
3. **Read the source file** to understand the current implementation before proposing changes

**Key search patterns:**
- `aria-*` attributes → grep for the attribute in component files
- Heading levels → grep for `<h1>`, `<h2>`, `<Heading level=` etc.
- Color contrast → trace the Tailwind classes to `globals.css` tokens
- Link names → find the `<a>` or `<Link>` component rendering the element

## Step 6: Apply Fixes

Apply minimal, semantic fixes — one audit at a time. Follow the common fix patterns below.

### Common Fix Patterns

| Audit ID | Issue | Fix |
|----------|-------|-----|
| `aria-prohibited-attr` | ARIA attribute not allowed on element's implicit role | Add an explicit `role` attribute that permits the ARIA attr, or remove the ARIA attr |
| `color-contrast` | Text doesn't meet WCAG AA contrast ratio (4.5:1 normal, 3:1 large) | Remove opacity modifiers (e.g., `/80` → full opacity), use semantic tokens (`text-foreground`, `text-muted-foreground`) |
| `heading-order` | Heading levels skip (e.g., h1 → h3) | Adjust heading level to maintain sequential order, or use `as="p"` on the Heading component if it's not a real heading |
| `label` | Form element has no associated label | Add `aria-label`, `aria-labelledby`, or a `<label htmlFor>` element |
| `link-name` | Link has no discernible text | Add `aria-label` to icon-only links, or add visually-hidden text |
| `image-alt` | Image has no `alt` attribute | Add descriptive `alt` text (or `alt=""` for decorative images) |
| `meta-description` | Page is missing a meta description | Fix `generateMetadata()` in the page's route file |
| `document-title` | Page is missing a `<title>` | Fix `generateMetadata()` to include a title |
| `html-has-lang` | `<html>` missing `lang` attribute | Add `lang="en"` to `<html>` in root `layout.tsx` |
| `target-size` | Touch target is too small (< 24x24 CSS px) | Increase padding or min-width/min-height on the interactive element |
| `list` | List items not wrapped in `<ul>`, `<ol>`, or `<menu>` | Wrap list items in the appropriate list element |

### Fix Rules

- **Minimal changes only.** Fix the specific audit failure, don't refactor surrounding code.
- **Semantic fixes preferred.** Use proper HTML semantics (labels, roles, headings) over ARIA workarounds when possible.
- **No visual regressions.** Fixes should not change the visual appearance of the page. If a fix requires a visual change (e.g., removing opacity for contrast), note it for user review.
- **One audit at a time.** Apply fixes for one audit ID, then move to the next. This makes it easy to verify each fix.
- **Follow existing patterns.** Check how similar elements are handled elsewhere in the codebase before introducing a new pattern.

## Step 7: Verify

Run the full verification suite to confirm fixes work and nothing is broken.

### Automated checks

```bash
# TypeScript compilation
task ui:build

# Linting
task ui:lint

# Unit tests
task ui:test
```

**Stop if any check fails.** Fix the issue before proceeding.

### Re-run Lighthouse

```bash
task seo:audit
```

Parse the new reports and compare against the baseline from Step 3:

```
## Before / After Comparison

| URL | Category | Before | After | Delta |
|-----|----------|--------|-------|-------|
| /   | a11y     | 87     | 100   | +13   |
| ... | ...      | ...    | ...   | ...   |

### Audits Fixed
- [audit-id]: FAIL → PASS (N elements fixed)
- ...

### Audits Remaining
- [audit-id]: still failing (reason)
- ...
```

**If new failures appear that weren't in the baseline, investigate immediately.** The fix may have introduced a regression.

## Step 8: E2E Screenshots (if visual changes)

If any fix changed the visual appearance of a page (opacity adjustments, element additions, layout changes):

```bash
# Check if dev server is running
lsof -ti:4000

# If port 4000 is active:
cd apps/web && pnpm exec playwright test --update-snapshots && pnpm exec playwright test

# If port 4000 is NOT active:
E2E_PORT=4001 pnpm --filter @bnto/web exec playwright test --update-snapshots
E2E_PORT=4001 pnpm --filter @bnto/web exec playwright test
```

Both runs required — first to update, second to verify stability.

If no visual changes were made, skip this step.

## Step 9: Summary

Present the final summary before committing:

```
## Lighthouse Audit Summary

### Scores (before → after)

| URL | Perf | A11y | BP | SEO |
|-----|------|------|----|-----|
| /   | 95→95 | 87→100 | 100→100 | 100→100 |
| ... | ...  | ...    | ...     | ...     |

### Audits Fixed
- [audit-id] in [file]: [what was changed]
- ...

### Files Changed
- `apps/web/components/foo/Bar.tsx` — added aria-label to icon link
- ...

### Verification
- TypeScript build: PASS
- Lint: PASS
- Unit tests: PASS
- Lighthouse re-run: all categories ≥ 0.9
- E2E screenshots: updated / no changes needed
```

### Key Rules

- **Agent cannot deem failures "pre-existing."** All failing audits must be reported to the user. Only the user decides what to ignore.
- **Fixes must be minimal and semantic.** No visual regressions, no unnecessary refactoring.
- **Re-run Lighthouse after fixes.** Before/after comparison is mandatory proof of work.
- **Follow `/pre-commit` before committing.** This skill does NOT replace the pre-commit checklist.
