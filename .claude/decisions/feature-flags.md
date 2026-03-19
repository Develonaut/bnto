# Decision: Feature Flags & A/B Testing

**Date:** March 19, 2026
**Status:** Research Complete -- Pending Decision
**Context:** Need feature flags, percentage rollouts, user targeting, and A/B testing. Budget is $0. PostHog is already integrated for analytics.

---

## Executive Summary

**Recommendation: PostHog Feature Flags (already integrated).**

PostHog is the clear winner for this project. It is already integrated as the telemetry layer, its feature flags and experiments share the same free tier, it has first-class Next.js App Router support with SSR bootstrapping, and it requires zero additional infrastructure. The 1M free feature flag requests/month is more than sufficient for early-to-mid growth. Every other option either adds operational complexity, introduces a second system to maintain, or has worse Next.js integration.

---

## Option 1: PostHog Feature Flags (RECOMMENDED)

### What It Is

PostHog includes feature flags and A/B testing (called "Experiments") as built-in products alongside analytics, session replay, and surveys. Since bnto already uses PostHog for telemetry (`@bnto/core` telemetry client, `posthogAdapter.ts`), feature flags are available with zero new dependencies.

### Free Tier Limits

| Resource                 | Free Allowance | Overage Cost                                       |
| ------------------------ | -------------- | -------------------------------------------------- |
| Feature flag requests    | 1M/month       | $0.0001/req (1-2M), steps down to $0.00001 at 50M+ |
| Product analytics events | 1M/month       | Usage-based                                        |
| Session recordings       | 5K/month       | Usage-based                                        |
| Survey responses         | 250/month      | Usage-based                                        |
| Team members             | **Unlimited**  | --                                                 |
| Billing limits           | Yes (set cap)  | Hard stop, no surprise bills                       |

No credit card required. Resets monthly. 90%+ of PostHog users stay on the free tier.

### A/B Testing / Experiments

Experiments are powered by feature flags -- there is **no separate meter**. Usage counts against the 1M feature flag requests. PostHog provides both Bayesian and frequentist statistical engines:

- **Bayesian**: Gives probability statements ("96% chance variant B wins")
- **Frequentist**: Welch's t-test with two-sided alternative
- Supports funnel metrics, numeric metrics (revenue per user), and ratio metrics
- Optional winsorization for outlier robustness
- Minimum sample size recommendations before declaring significance

### Next.js App Router Integration

PostHog has first-class Next.js support via `@posthog/next`:

- **Server-side flag bootstrapping**: Evaluates flags server-side via `posthog-node`, passes results to the client as bootstrap data. Hooks return real values immediately -- no network round-trip, no flash of wrong content.
- **Middleware support**: Can evaluate flags in Next.js middleware for edge-level A/B testing (route splitting before page render).
- **App Router native**: Works with React Server Components. Server-side SDK (`posthog-node`) for RSC, client SDK for interactive components.
- **Identity sync**: Middleware seeds an identity cookie (UUIDv7) so server and client evaluations are consistent.

### Local Evaluation (Cost Optimization)

PostHog supports server-side local evaluation -- flag definitions are cached locally and evaluated without network calls:

- Latency drops from ~500ms to 10-20ms
- Each periodic sync counts as only 10 flag requests (vs 1 per evaluation with remote)
- Default polling interval: 30 seconds (configurable)
- Critical for high-traffic pages where every request evaluates flags

### Fit With Existing Stack

| Concern                  | Assessment                                                                |
| ------------------------ | ------------------------------------------------------------------------- |
| Already integrated?      | **Yes** -- `posthog-js` in `@bnto/core`, adapter in `posthogAdapter.ts`   |
| Additional dependencies? | `posthog-node` for server-side (may already be present for proxy)         |
| Architecture alignment?  | Fits `@bnto/core` adapter pattern -- add flag methods to telemetry client |
| PostHog compatibility?   | **It IS PostHog**                                                         |
| Convex compatibility?    | Orthogonal -- flags evaluated client/edge side, Convex handles data       |
| Infrastructure cost?     | $0                                                                        |

### Gotchas

1. **Flag evaluation = API request = cost**. Every `posthog.isFeatureEnabled()` call that goes to the server counts against the 1M limit. Use local evaluation on the server and bootstrapping on the client to minimize this.
2. **Multivariate flags require careful setup**. For A/B tests with multiple variants, each user must be consistently bucketed. PostHog handles this via distinct ID hashing, but you must ensure the distinct ID is stable (the existing identity cookie pattern handles this).
3. **No offline/edge-only mode**. Flags require PostHog connectivity (or cached definitions via local evaluation). The desktop app (Tauri, M3) would need a different approach.
4. **Flag evaluation is eventually consistent**. Changes to flag definitions propagate within the local evaluation polling interval (default 30s).

---

## Option 2: Vercel Flags + Edge Config

### What It Is

Vercel offers two related products:

1. **Vercel Flags** (public beta as of Feb 2026): A feature flag provider built into the Vercel Dashboard with targeting rules, segments, and environment controls.
2. **Edge Config**: A global key-value store with sub-millisecond reads at the edge, usable for manual feature flag storage.
3. **Flags SDK** (`@vercel/flags`): An open-source SDK that works with ANY flag provider (PostHog, LaunchDarkly, custom, or Vercel Flags itself).

### Pricing

| Product                | Hobby (Free)                            | Pro ($20/seat/mo)        |
| ---------------------- | --------------------------------------- | ------------------------ |
| **Vercel Flags**       | 10,000 flag requests/month, then pauses | $30 per 1M flag requests |
| **Edge Config reads**  | Included (see limits)                   | Included (higher limits) |
| **Edge Config stores** | 1 store, 8 KB max                       | 3 stores, 64 KB max      |
| **Flags SDK**          | Free (open source)                      | Free                     |
| **Max flags**          | 100                                     | 10,000                   |

### Next.js Integration

The Flags SDK is designed specifically for Next.js. It integrates with App Router, Pages Router, and middleware. It can read flags from Vercel Flags, Edge Config, PostHog, LaunchDarkly, or any custom provider.

### Assessment

**The Flags SDK is interesting but the Vercel Flags product is not suitable.**

- **10,000 free flag requests is too low.** That is about 300 unique visitors/day evaluating flags on 1 page. Bnto would hit this within the first week of any real traffic.
- **Edge Config (8 KB on Hobby)** can store simple boolean flags manually, but has no targeting, rollout, or A/B testing capabilities. It is a key-value store, not a flag management system.
- **The Flags SDK is provider-agnostic** and could be used with PostHog as the provider. This is the only part worth considering -- it provides a nice DX for flag definitions in code. But it adds a layer of abstraction over PostHog's native SDK without clear benefit for this project's scale.
- **Hobby plan restriction**: Vercel Hobby is for non-commercial, personal use. Bnto is commercial (MIT open source, but the hosted product is commercial). This likely requires the Pro plan ($20/seat/month), which breaks the $0 budget.

**Verdict: Skip.** The free tier is too restrictive, and the Flags SDK adds complexity without solving a problem PostHog doesn't already solve.

---

## Option 3: Unleash (Open Source, Self-Hosted)

### What It Is

The most popular open-source feature flag platform on GitHub. Provides targeting rules, gradual rollouts, custom activation strategies, and integrations with Jira, Datadog, Slack, etc.

### Pricing

| Tier                              | Cost                        | Notes                              |
| --------------------------------- | --------------------------- | ---------------------------------- |
| Open Source (self-hosted)         | Free forever                | Unlimited flags, limited features  |
| Enterprise (self-hosted or cloud) | $75/seat/month (5-seat min) | Advanced targeting, SSO, audit log |

### Self-Hosting Feasibility ($0 Budget)

Unleash requires:

- A Node.js application server
- A PostgreSQL database
- Docker (official image available)

**On a $0 budget, this means:**

- Railway free tier (500 hours/month, 1 GB RAM) -- could work but tight
- Fly.io free tier (3 shared-cpu-1x VMs) -- could work
- A home server or spare VPS

The operational burden is real: you own uptime, backups, upgrades, and security patches. For an early-stage open-source project with one developer, this is a distraction.

### Next.js Integration

Unleash has an official `@unleash/nextjs` SDK with:

- App Router support (Server Components)
- Client-side hooks (`useFlag`, `useVariant`, `useFlagsStatus`)
- Server-side bootstrapping (fetch flags on server, hydrate client)
- Edge middleware support for A/B route splitting

### A/B Testing

The open-source edition supports basic A/B testing via variants (percentage split). Advanced experimentation features (statistical analysis, metrics) require Enterprise. You would need to pipe variant assignments to PostHog for analysis.

### PostHog Compatibility

No native integration. You would assign variants in Unleash, then send the variant assignment as a property on PostHog events for analysis. Two systems to correlate.

### Assessment

**Skip.** The self-hosting burden is not justified. A/B testing requires piping data to PostHog anyway. Adds operational complexity for a capability PostHog already provides.

---

## Option 4: GrowthBook (Open Source)

### What It Is

Open-source feature flagging and A/B testing platform. MIT licensed (core), with commercial features under GrowthBook Enterprise License. Strong focus on experimentation with a built-in statistical engine.

### Pricing

| Tier            | Cost           | Limits                                                  |
| --------------- | -------------- | ------------------------------------------------------- |
| Starter (cloud) | Free           | 3 users, 1M CDN requests/month, 5 GB bandwidth          |
| Pro (cloud)     | $40/user/month | 50 users, 2M CDN requests, advanced stats               |
| Self-hosted     | Free forever   | Unlimited users, unlimited flags, unlimited experiments |

### Self-Hosting

Requires Docker + MongoDB (or PostgreSQL). Same operational burden as Unleash. The $0 self-hosting path requires a free-tier cloud provider.

### Cloud Free Tier

3 users, 1M CDN requests/month. More generous than Vercel Flags but adds another vendor to manage.

### Next.js Integration

GrowthBook has an official Next.js App Router guide:

- JavaScript SDK (not React SDK) for Server Components
- Fetch polyfills with Next.js caching
- Server-side flag evaluation with client hydration
- No dedicated `@growthbook/next` package -- manual setup required

### A/B Testing

GrowthBook's strength. Built-in statistical engine with Bayesian analysis. However, it needs a data source (warehouse, Mixpanel, etc.) to analyze experiment results. PostHog is listed as a supported data source, but the integration adds complexity.

### PostHog Compatibility

GrowthBook can use PostHog as a data source for experiment analysis. But this means: evaluate flags in GrowthBook, send events to PostHog, pipe PostHog data back to GrowthBook for analysis. Three hops vs PostHog's integrated zero-hop approach.

### Assessment

**Skip.** GrowthBook is excellent if you need a dedicated experimentation platform with warehouse-connected analysis. But bnto doesn't have a data warehouse, and PostHog already provides the statistical engine. Adding GrowthBook creates a Rube Goldberg machine of flag evaluation + event tracking + data piping.

---

## Option 5: Flagsmith (Open Source)

### What It Is

Open-source feature flagging and remote configuration service. Can be self-hosted or used as a cloud service.

### Pricing

| Tier             | Cost      | Limits                                       |
| ---------------- | --------- | -------------------------------------------- |
| Free (cloud)     | $0        | **50,000 requests/month**, 1 user, 1 project |
| Start-Up (cloud) | $45/month | 1M requests, 3 users                         |
| Self-hosted      | Free      | Unlimited (Docker + PostgreSQL)              |

### Assessment

**Skip.** The cloud free tier (50K requests, 1 user) is the most restrictive of any option researched. Self-hosting has the same operational burden as Unleash/GrowthBook. Next.js integration exists (isomorphic SDK with middleware support) but is less mature than PostHog's dedicated `@posthog/next` package. No built-in statistical engine for A/B tests -- you would need to pipe data to PostHog anyway.

---

## Option 6: OpenFeature

### What It Is

OpenFeature is **not a feature flag service**. It is an open specification (CNCF sandbox project) that defines a vendor-agnostic API for feature flagging. Think of it as "OpenTelemetry but for feature flags."

### How It Works

- Defines a standard `EvaluationClient` interface
- Providers (plugins) translate between the standard API and specific backends (PostHog, LaunchDarkly, Flagsmith, etc.)
- SDKs available for JavaScript, Go, Python, Java, etc.
- Vercel's Flags SDK supports OpenFeature as a provider

### Assessment

**Not a solution by itself.** OpenFeature is useful if you want to avoid vendor lock-in at the code level -- you write to the OpenFeature API, swap providers without changing application code. But you still need a backend (PostHog, Unleash, etc.).

For bnto, the abstraction is premature. PostHog is already behind the `@bnto/core` adapter pattern. If PostHog is ever replaced, the adapter layer already isolates the change. Adding OpenFeature as a middle layer between `@bnto/core` and PostHog adds indirection without benefit.

**Verdict: Skip for now.** Revisit only if the project needs to support multiple flag providers simultaneously (unlikely).

---

## Option 7: Convex-Native Flags Table

### What It Is

Build a simple `featureFlags` table in Convex. Store flag names, enabled/disabled state, rollout percentages, and user targeting rules directly in the database.

### How It Works

Convex has a blog post on this exact pattern ("Feature Gating" on stack.convex.dev):

1. Create a `flags` table with `name`, `enabled`, `percentage`, `targetedUsers` fields
2. Query flags reactively -- Convex subscriptions mean flag changes propagate instantly
3. Evaluate client-side: hash user ID, compare to percentage for rollouts

### Advantages

- $0 cost (uses existing Convex infrastructure)
- Real-time reactivity (Convex subscriptions update flags instantly, no polling)
- Full control -- no third-party dependency
- Simple to implement (< 100 lines of code for basic flags)

### Limitations

- **No statistical engine.** A/B test analysis requires manual work or piping to PostHog.
- **No targeting UI.** You manage flags via Convex dashboard or custom admin page.
- **No audit log.** Who changed what flag when?
- **No gradual rollout sophistication.** Basic percentage splits, no complex segment targeting without building it.
- **Reinventing the wheel.** PostHog already provides all of this with a polished UI.
- **Server round-trip.** Every flag evaluation is a Convex query (though cached/subscribed).
- **No middleware/edge evaluation.** Convex queries run client-side or in API routes, not at the edge.

### When This Makes Sense

- If the project needed to work completely offline (desktop/Tauri without internet)
- If PostHog was not already integrated
- If the requirement was purely "kill switch" flags with no A/B testing

### Assessment

**Skip as the primary solution.** PostHog provides everything Convex flags would, plus statistical analysis, targeting UI, and edge evaluation. However, a minimal Convex flags table could complement PostHog for **server-side business logic flags** that need to be evaluated within Convex functions (e.g., "is this user on Pro tier?"). That is a different concern from feature flags/experiments.

---

## Comparison Matrix

| Criterion               | PostHog                           | Vercel Flags               | Unleash (self-hosted)   | GrowthBook (cloud)                | Flagsmith (cloud)       | Convex DIY            |
| ----------------------- | --------------------------------- | -------------------------- | ----------------------- | --------------------------------- | ----------------------- | --------------------- |
| **Free tier**           | 1M req/mo                         | 10K req/mo                 | Unlimited               | 1M CDN/mo, 3 users                | 50K req/mo, 1 user      | $0 (existing)         |
| **A/B testing**         | Built-in (Bayesian + Frequentist) | None (flag splitting only) | Basic variants (OSS)    | Built-in (needs data source)      | None built-in           | None                  |
| **Statistical engine**  | Yes                               | No                         | No (OSS)                | Yes (needs warehouse pipe)        | No                      | No                    |
| **Next.js App Router**  | First-class (`@posthog/next`)     | First-class (Flags SDK)    | Official SDK            | Manual setup                      | Isomorphic SDK          | Client-side only      |
| **SSR bootstrapping**   | Yes (server-side eval + hydrate)  | Yes (Edge Config)          | Yes (server bootstrap)  | Yes (RSC + hydrate)               | Yes (isomorphic)        | No (Convex is client) |
| **Edge/Middleware**     | Yes (middleware tutorial)         | Yes (native)               | Yes (Edge middleware)   | No official support               | Yes (middleware)        | No                    |
| **Already integrated?** | **YES**                           | Partial (Vercel deploy)    | No                      | No                                | No                      | Partial (Convex DB)   |
| **Additional infra**    | None                              | None                       | PostgreSQL + app server | MongoDB/PG + app server           | PostgreSQL + app server | None                  |
| **PostHog compat**      | N/A (same tool)                   | Via Flags SDK adapter      | Manual event piping     | Data source integration           | Manual event piping     | Manual event piping   |
| **Targeting UI**        | Yes (dashboard)                   | Yes (dashboard, limited)   | Yes (dashboard)         | Yes (dashboard)                   | Yes (dashboard)         | Build it yourself     |
| **User segments**       | Yes                               | Yes (100 max on free)      | Yes                     | Yes                               | Yes                     | Build it yourself     |
| **Percentage rollouts** | Yes                               | Yes                        | Yes                     | Yes                               | Yes                     | Basic (DIY hash)      |
| **Cost at scale**       | Usage-based (cheap)               | $30/1M req                 | Free (self-hosted)      | $40/user/mo or free (self-hosted) | $45/mo (1M req)         | Free                  |
| **Operational burden**  | None (cloud)                      | None (Vercel-managed)      | High (self-host)        | Medium-High (self-host)           | Medium-High (self-host) | None                  |

---

## Recommendation

### Primary: PostHog Feature Flags

Use PostHog for all feature flags, percentage rollouts, user targeting, and A/B testing. Rationale:

1. **Zero marginal cost.** Already paying for (the free tier of) PostHog. Feature flags share the same 1M request pool.
2. **Zero new dependencies.** The PostHog JS SDK is already in the bundle. Add `posthog-node` for server-side evaluation.
3. **Zero operational burden.** Cloud-hosted, no infrastructure to manage.
4. **Unified analytics + experiments.** Flag assignments and experiment results live in the same system. No data piping between tools.
5. **First-class Next.js support.** Server-side bootstrapping, middleware evaluation, App Router native.
6. **Architecture fit.** Add flag methods to the existing `telemetryClient` in `@bnto/core`, or create a new `featureFlagsClient` following the same adapter pattern. Either way, it fits the layered architecture.

### Implementation Sketch

```
@bnto/core
  adapters/posthog/featureFlagAdapter.ts  -- wraps posthog-js flag evaluation
  clients/featureFlagClient.ts            -- public API: core.flags.isEnabled(), core.flags.getVariant()
  hooks/useFeatureFlag.ts                 -- React binding: const enabled = core.flags.useFlag("pro-tier")

apps/web
  middleware.ts                           -- evaluate flags at the edge for A/B route splitting
  providers/index.tsx                     -- bootstrap flags server-side via @posthog/next
```

### Cost Projection

| Traffic Level           | Monthly Flag Requests (est.) | Cost       |
| ----------------------- | ---------------------------- | ---------- |
| Pre-launch / beta       | < 50K                        | $0         |
| Early growth (1K DAU)   | ~200K                        | $0         |
| Medium growth (10K DAU) | ~2M                          | ~$100/year |
| Breakout (100K DAU)     | ~20M                         | ~$500/year |

At 100K DAU, bnto would be generating significant revenue. The flag costs are negligible relative to the value.

### What NOT to Do

- **Do not add Vercel Flags.** The 10K free limit is a joke for production use.
- **Do not self-host anything.** The operational burden is not justified for a team of this size.
- **Do not build a Convex flags table for feature flags.** PostHog already does this better. A Convex table is appropriate only for server-side business logic (e.g., subscription tier checks within Convex functions).
- **Do not add OpenFeature.** The `@bnto/core` adapter pattern already provides vendor isolation.

---

## Sources

- [PostHog Pricing](https://posthog.com/pricing)
- [PostHog Feature Flags Docs](https://posthog.com/docs/feature-flags)
- [PostHog Experiments Docs](https://posthog.com/experiments)
- [PostHog Next.js Integration](https://posthog.com/docs/libraries/next-js)
- [PostHog Next.js A/B Tests Tutorial](https://posthog.com/tutorials/nextjs-ab-tests)
- [PostHog Next.js Middleware Bootstrap](https://posthog.com/tutorials/nextjs-bootstrap-flags)
- [PostHog Local Evaluation](https://posthog.com/docs/feature-flags/local-evaluation)
- [PostHog Client-Side Bootstrapping](https://posthog.com/docs/feature-flags/bootstrapping)
- [PostHog Cutting Feature Flag Costs](https://posthog.com/docs/feature-flags/cutting-costs)
- [PostHog Bayesian Statistics](https://posthog.com/docs/experiments/statistics)
- [PostHog Frequentist Method](https://posthog.com/docs/experiments/frequentist-method)
- [Vercel Edge Config Limits](https://vercel.com/docs/edge-config/edge-config-limits)
- [Vercel Flags Limits and Pricing](https://vercel.com/docs/flags/vercel-flags/limits-and-pricing)
- [Vercel Flags SDK Reference](https://vercel.com/docs/flags/flags-sdk-reference)
- [Vercel Flags Public Beta Announcement](https://vercel.com/changelog/vercel-flags-is-now-in-public-beta)
- [Flags as Code in Next.js (Vercel Blog)](https://vercel.com/blog/flags-as-code-in-next-js)
- [Zero-Cost Feature Flags with Vercel Edge Config](https://dev.to/hexshift/zero-cost-feature-flags-using-vercel-edge-config-no-saas-needed-394a)
- [Unleash Open Source](https://www.getunleash.io/open-source)
- [Unleash GitHub](https://github.com/Unleash/unleash)
- [Unleash Next.js SDK](https://docs.getunleash.io/sdks/next-js)
- [GrowthBook Pricing](https://www.growthbook.io/pricing)
- [GrowthBook Next.js App Router Guide](https://docs.growthbook.io/guide/nextjs-app-router)
- [GrowthBook GitHub](https://github.com/growthbook/growthbook)
- [Flagsmith Pricing](https://www.flagsmith.com/pricing)
- [Flagsmith Next.js SSR SDK](https://docs.flagsmith.com/clients/next-ssr)
- [OpenFeature Introduction](https://openfeature.dev/docs/reference/intro/)
- [OpenFeature on Flags SDK](https://flags-sdk.dev/providers/openfeature)
- [Convex Feature Gating (stack.convex.dev)](https://stack.convex.dev/feature-gating)
- [Convex Feature Flags](https://www.convex.dev/can-do/feature-flags)
- [Open Source Feature Flag Tools Compared 2026 (FlagShark)](https://flagshark.com/blog/open-source-feature-flag-tools-compared-2026/)
- [PostHog: 8 Best Open Source Feature Flag Tools](https://posthog.com/blog/best-open-source-feature-flag-tools)
- [PostHog + Vercel Integration Guide](https://vercel.com/kb/guide/posthog-nextjs-vercel-feature-flags-analytics)
