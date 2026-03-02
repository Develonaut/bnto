# Decision: Analytics Tooling

**Date:** March 1, 2026
**Status:** Decided — PostHog
**Sprint:** Sprint 3, Wave 1

---

## Context

bnto just launched. No users yet, no visibility into what's happening. We need product analytics to understand tool usage, conversion funnels, and where people drop off. The backend already tracks execution events in Convex (`executionEvents` table), but we have zero frontend analytics — no page views, no funnel tracking, no session data.

The previous privacy policy made premature promises about "no third-party tracking" and "no advertising." These have been removed (March 2026) to avoid boxing in future business model decisions.

## Requirements

- **Product analytics** — funnels, retention, feature usage (not just page views)
- **Conversion tracking** — anonymous → signed up, free → Pro (when Pro launches)
- **Tool usage** — which bntos get used, completion rates, drop-off points
- **Performance** — page load times, Core Web Vitals correlation
- **Privacy-reasonable** — no selling data to ad networks, but standard analytics is fine
- **Cost-effective** — free tier sufficient for early stage

## Candidates Evaluated

| | PostHog | Plausible | Vercel Analytics | Custom (Convex) |
|---|---|---|---|---|
| **Type** | Full product analytics | Traffic analytics | Web vitals + traffic | DIY |
| **Funnels** | Yes | No | No | Build it yourself |
| **Retention** | Yes | No | No | Build it yourself |
| **Session replay** | Yes (free tier) | No | No | No |
| **Feature flags** | Yes | No | No | No |
| **Custom events** | Yes | Goals only | Limited | Yes |
| **Free tier** | 1M events/mo | None (cloud) | 2.5K events/mo | $0 (already built) |
| **Self-hostable** | Yes | Yes | No | N/A |
| **Bundle size** | ~7KB gzipped | ~1KB | ~1KB | 0 (backend only) |
| **Cookies** | Optional (cookieless mode) | None | None | N/A |
| **Privacy** | First-party cookie, no third-party sharing | Privacy-first | First-party | Full control |

## Decision: PostHog

**PostHog is the right tool for this stage.** Rationale:

1. **One tool, not two.** Plausible gives page views. PostHog gives page views + funnels + retention + session replay + feature flags. At this stage, we need product insight, not just traffic numbers.

2. **Free tier is generous.** 1M events/month is more than enough for launch through early growth. No cost until meaningful scale.

3. **Self-hostable escape hatch.** If we outgrow the free tier or want full control, we can self-host on Railway (already in our infra).

4. **Cookieless mode available.** Can run without cookies entirely (uses localStorage or nothing). No cookie consent banner needed in most jurisdictions.

5. **Session replay included.** Invaluable for debugging UX issues at launch when we can't observe users directly.

6. **Feature flags for Pro rollout.** When Sprint 7 (Stripe) ships, PostHog feature flags can gate Pro features without a custom system.

## What we're NOT doing

- **Not self-hosting yet.** PostHog Cloud (US) is fine for now. Self-host when scale demands it.
- **Not adding a cookie consent banner.** PostHog in cookieless mode doesn't require one. Revisit if we enable cookies later.
- **Not replacing Convex execution events.** The backend `executionEvents` table stays for server-side analytics and quota tracking. PostHog handles the frontend/UX layer.

## Integration approach

- Install `posthog-js` in `apps/web`
- Initialize in a provider component (client-side only)
- Auto-capture page views and clicks (PostHog default)
- Add custom events for key moments: tool execution started, execution completed, file uploaded, download triggered
- Identify users on sign-in (link anonymous → authenticated sessions)
- Respect DNT header as a courtesy (not legally required, but good practice)

## Two-layer analytics model

| Layer | Tool | What it tracks |
|---|---|---|
| **Frontend** | PostHog | Page views, funnels, feature usage, session replay, conversion |
| **Backend** | Convex (`executionEvents`) | Execution metrics, quota, per-tool performance, server health |

PostHog answers "what are users doing?" Convex answers "how is the system performing?"
