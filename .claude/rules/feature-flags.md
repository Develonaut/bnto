# Feature Flags

Feature flags and A/B testing are powered by PostHog, exposed via `core.flags` (the 7th domain on `@bnto/core`). Flags are created in the PostHog Dashboard and consumed in code through the adapter pattern.

**Decision record:** [.claude/decisions/feature-flags.md](../decisions/feature-flags.md)

---

## End-to-End Workflow

```
1. Create flag in PostHog Dashboard (name, type, rollout, targeting)
2. Use flag in code via core.flags hooks or imperative API
3. Ship — flags evaluate client-side, no deploy needed to toggle
4. Monitor in PostHog (flag calls, experiment results)
5. Clean up — remove flag from code + archive in PostHog when graduated
```

---

## Step 1: Create a Flag in PostHog

### PostHog Dashboard

1. Go to **Feature Flags** in the left nav (posthog.com → your project)
2. Click **New feature flag**
3. Configure:

| Field                  | What to enter                                                     |
| ---------------------- | ----------------------------------------------------------------- |
| **Key**                | Lowercase, hyphen-separated. Use the naming convention below      |
| **Description**        | What this flag controls, who it's for, when to remove it          |
| **Type**               | Boolean (on/off) or Multivariate (A/B/C variants)                 |
| **Rollout**            | Percentage of users who see the feature (0-100%)                  |
| **Release conditions** | Optional targeting: by user property, group, cohort               |
| **Payload**            | Optional JSON attached to the flag (config values, copy variants) |

4. Click **Save** — the flag is now live (or staged at 0% rollout)

### Flag Key Naming Convention

```
<scope>-<feature>

Scopes:
  pro-*       — Pro tier gating (pro-save, pro-cloud-exec)
  exp-*       — A/B experiments (exp-cta-copy, exp-hero-layout)
  beta-*      — Beta feature rollouts (beta-code-editor, beta-custom-recipes)
  ops-*       — Operational kill switches (ops-disable-uploads, ops-maintenance)
  dev-*       — Dev-only flags (dev-debug-panel, dev-mock-engine)
```

Examples: `pro-save`, `exp-hero-cta`, `beta-code-editor`, `ops-disable-wasm`

---

## Step 2: Use the Flag in Code

### Boolean Flags (feature gating)

The most common case — show/hide a feature or gate access.

```tsx
"use client";
import { core } from "@bnto/core";

function SaveButton() {
  const canSave = core.flags.useFlag("pro-save");

  // undefined = flags still loading, don't flash wrong state
  if (canSave === undefined) return null;
  if (!canSave) return <UpgradePrompt />;
  return <Button onClick={handleSave}>Save</Button>;
}
```

### Multivariate Flags (A/B tests)

For experiments with multiple variants.

```tsx
"use client";
import { core } from "@bnto/core";

function HeroCTA() {
  const variant = core.flags.useVariant("exp-hero-cta");

  if (variant === undefined) return null; // loading
  if (variant === "action") return <Button>Get started free</Button>;
  if (variant === "value") return <Button>Save hours every week</Button>;
  return <Button>Try bnto</Button>; // control / fallback
}
```

### Flags with Payloads

When the flag carries configuration data (copy, thresholds, URLs).

```tsx
"use client";
import { core } from "@bnto/core";

function Banner() {
  const result = core.flags.useFlagResult("beta-banner");

  if (!result?.enabled) return null;

  // payload is the JSON you set in PostHog Dashboard
  const { title, cta } = result.payload as { title: string; cta: string };
  return <Banner title={title} cta={cta} />;
}
```

### Imperative (non-React, event handlers, analytics)

```typescript
import { core } from "@bnto/core";

// In an event handler or utility function
if (core.flags.isEnabled("ops-disable-uploads")) {
  showMaintenanceMessage();
  return;
}

// Get full result for logging
const result = core.flags.getResult("exp-pricing");
core.telemetry.capture("pricing_viewed", { variant: result?.variant });
```

---

## Step 3: A/B Experiments

### Creating an Experiment in PostHog

1. Go to **Experiments** in the left nav
2. Click **New experiment**
3. Configure:

| Field                   | What to enter                                                                    |
| ----------------------- | -------------------------------------------------------------------------------- |
| **Name**                | Human-readable: "Hero CTA copy test"                                             |
| **Feature flag**        | Select the multivariate flag you created (e.g., `exp-hero-cta`)                  |
| **Goal metric**         | The PostHog event that defines success (e.g., `cta_clicked`, `recipe_completed`) |
| **Secondary metrics**   | Optional additional metrics to track                                             |
| **Minimum sample size** | PostHog recommends a minimum — respect it before declaring results               |

4. Click **Launch** — PostHog handles variant assignment and statistical analysis

### Reading Results

PostHog automatically:

- Assigns users to variants based on their distinct ID (sticky, deterministic)
- Tracks the goal metric per variant
- Runs Bayesian statistical analysis
- Shows probability of each variant winning
- Recommends when you have enough data to decide

### Tracking Experiment Events

Use `useFlagResult` (not `useFlag`) for experiments — it triggers PostHog's `$feature_flag_called` event which links the variant assignment to your goal metrics.

```tsx
const result = core.flags.useFlagResult("exp-hero-cta");

// When the user takes the goal action, capture it
function handleCTAClick() {
  core.telemetry.capture("cta_clicked"); // PostHog correlates this with the variant
  router.push("/signup");
}
```

---

## Step 4: Targeting and Rollout

### Percentage Rollout

In PostHog Dashboard → Feature Flag → **Rollout percentage**:

| Stage            | Percentage                | Purpose                              |
| ---------------- | ------------------------- | ------------------------------------ |
| Internal testing | 0% + target your own user | Verify in production                 |
| Canary           | 5-10%                     | Catch issues with real users         |
| Gradual          | 25% → 50% → 100%          | Ramp up with monitoring              |
| Full             | 100%                      | Feature graduated, ready for cleanup |

### User Targeting

In PostHog Dashboard → Feature Flag → **Release conditions**:

- **By property:** `email contains @bnto.io` (internal team)
- **By cohort:** Users who completed onboarding
- **By group:** Organization-level flags (future)
- **Multiple conditions:** OR logic — any matching condition enables the flag

### Overrides

In PostHog Dashboard → Feature Flag → **Overrides**:

Force-enable or force-disable for specific users by distinct ID. Useful for QA testing specific variants.

---

## Step 5: Clean Up Graduated Flags

**Flags are temporary.** Every flag should be removed once the feature is fully shipped or the experiment is concluded.

### Cleanup Checklist

1. **Set rollout to 100%** (or 0% if killing the feature) and wait one release cycle
2. **Remove all flag checks from code** — replace conditional with the winning path
3. **Archive the flag in PostHog** — Dashboard → Feature Flag → Archive
4. **Remove the flag key from any constants/enums** if you created one
5. **Update tests** — remove any test branches that checked the flag

### Staleness Rule

If a flag has been at 100% rollout for more than 2 weeks without code cleanup, it's stale. PostHog shows "last evaluated" timestamps — use these to identify flags that should be graduated.

---

## API Reference

### React Hooks (reactive, auto-rerender on flag changes)

| Hook                            | Return type                      | Use when                            |
| ------------------------------- | -------------------------------- | ----------------------------------- |
| `core.flags.useFlag(key)`       | `boolean \| undefined`           | Simple on/off gating                |
| `core.flags.useVariant(key)`    | `string \| boolean \| undefined` | A/B test variant selection          |
| `core.flags.useFlagResult(key)` | `FlagResult \| undefined`        | Need payload or experiment tracking |

All hooks return `undefined` while PostHog is loading flags. **Always handle the `undefined` case** — render nothing or a fallback, never flash the wrong state.

### Imperative API (non-React, event handlers)

| Method                       | Return type                      | Use when                                  |
| ---------------------------- | -------------------------------- | ----------------------------------------- |
| `core.flags.isEnabled(key)`  | `boolean \| undefined`           | Event handlers, utilities                 |
| `core.flags.getVariant(key)` | `string \| boolean \| undefined` | Event handlers, analytics                 |
| `core.flags.getResult(key)`  | `FlagResult \| undefined`        | Need full result with payload             |
| `core.flags.subscribe(cb)`   | `() => void`                     | Manual subscription (returns unsubscribe) |
| `core.flags.reload()`        | `void`                           | Force-refresh flags from PostHog          |

### FlagResult Type

```typescript
interface FlagResult {
  key: string; // The flag key
  enabled: boolean; // Whether the flag is on for this user
  variant: string | boolean | undefined; // Variant key (multivariate) or boolean
  payload: unknown; // JSON payload from PostHog Dashboard
}
```

---

## Open Source Consideration: Dashboard-Driven vs Code-Driven

**Current state:** Flags are dashboard-driven. They're created and configured in PostHog's UI, then evaluated in code. This means managing flags requires PostHog Dashboard access.

**This is fine for now** — the bnto team controls the hosted product, and PostHog's free tier has unlimited team members. But it creates a gap for:

- **Self-hosters** who fork bnto and run their own instance — they'd need their own PostHog project to manage flags, or flags would silently evaluate to `undefined`/`false` (safe fallback, but no way to enable features)
- **Open-source contributors** who can't test flag-gated features locally without a PostHog project key
- **Configuration-as-code** — flag definitions don't live in the repo, so they can't be reviewed in PRs, versioned, or rolled back with git

**Future path (not yet needed):** Evaluate code-driven flag definitions — flag keys, variants, and defaults defined in the repo (e.g., a `flags.ts` registry or `.bnto-flags.json`), with PostHog used only for targeting/rollout overrides on the hosted product. This would let self-hosters control flags without PostHog and let contributors see the full flag surface in the codebase. Options to evaluate when this becomes a real need:

1. **Local defaults file** — define all flags with defaults in code, PostHog overrides at runtime
2. **Vercel Flags SDK** — code-defined flags with pluggable providers (PostHog, env vars, static)
3. **Convex flags table** — for server-side business logic flags that need to work without PostHog

Track this decision in the backlog — revisit when self-hosting docs ship or when contributor friction surfaces.

---

## SSR Behavior

All flag functions are **SSR-safe** — they return `undefined`/`false` on the server. Flags evaluate client-side only via the PostHog JS SDK (`window.__bnto_ph__`).

**Implication:** Flag-gated content will flash briefly on first render if the server renders the fallback and the client evaluates the flag differently. For most feature gates this is acceptable. For A/B experiments where flash is unacceptable, consider:

1. Render nothing (`null`) until flags load — prevents flash but shows blank space briefly
2. Future: SSR bootstrapping via `posthog-node` (evaluate flags server-side, pass as bootstrap data to client SDK) — eliminates flash entirely. Not yet implemented.

---

## E2E Testing

Flag evaluations are pushed to `window.__bnto_flags__` when the array exists. In Playwright tests:

```typescript
// Initialize the test hook before navigating
await page.evaluate(() => {
  (window as any).__bnto_flags__ = [];
});

await page.goto("/some-page");

// Assert flag was evaluated
const flags = await page.evaluate(() => (window as any).__bnto_flags__);
expect(flags).toContainEqual({ key: "pro-save", value: true });
```

To test specific flag states, use PostHog overrides in the Dashboard for your test user's distinct ID, or mock the flag adapter in tests.

---

## Rules

1. **Create the flag in PostHog first, then write code.** The flag must exist before any code references it.
2. **Always handle `undefined`.** Hooks return `undefined` while loading — never assume `false`.
3. **Use `useFlagResult` for experiments.** It triggers `$feature_flag_called` which PostHog needs to correlate variants with goal metrics. `useFlag`/`useVariant` do not trigger this event.
4. **One flag per concern.** Don't overload a single flag to gate multiple unrelated features.
5. **Flags are temporary.** Every flag must have a plan for graduation (remove from code when fully shipped).
6. **Follow the naming convention.** `pro-*`, `exp-*`, `beta-*`, `ops-*`, `dev-*` — makes it obvious what type of flag it is.
7. **No flag logic in `@bnto/core` internals.** Flag checks belong in UI components and app-level code, not in adapters or services.
8. **No server-side flag evaluation yet.** Current implementation is client-only. Don't assume flags are available during SSR/SSG.
