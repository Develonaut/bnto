# Feature Funnel & Tier Matrix

**Last Updated:** March 3, 2026
**Status:** Active — single source of truth for feature tiers and conversion hooks
**Previously:** Notion — "Feature Funnel & Tier Matrix"

> This document defines the feature tiers across all app surfaces and the conversion hooks that move users through the funnel.

---

## The Three Tiers

| Tier | Who | What they get | Conversion trigger |
|---|---|---|---|
| **Unauthenticated** | First-time visitor, arrived via SEO. No account. | Full browser execution. Browser-local history (10 entries, read-only). Export `.bnto.json`. No custom recipes. No persistence beyond current browser. | AccountGate at value moments (save, custom recipes, dashboard features) |
| **Free Account** | Signed up. Uses the product regularly. | Everything unauth gets, plus: server-synced history (7-day retention), re-run from history, save up to 3 recipes, create custom recipes, cross-device access. | Pro feature visibility (server nodes grayed, sharing locked, 3-recipe cap, 7-day retention limit) |
| **Pro** ($8/mo) | Power user or team. Needs server compute, sharing, or longer retention. | Everything free gets, plus: 30-day history retention, unlimited saves, server-node execution (AI, shell, video), team sharing (5 members), API access, cloud drive export, 500 MB file limit. | N/A (already paying) |

---

## Surface-by-Surface Matrix

### Recipe Tool Pages (`/compress-images`, `/clean-csv`, etc.)

The hook. Must be completely frictionless. No login wall before value.

| Capability | Unauth | Free Account | Pro |
|---|---|---|---|
| Run browser tools | Full, unlimited | Full, unlimited | Full, unlimited |
| Download results | Full, no watermarks | Full | Full |
| Configure recipe params | Full | Full | Full |
| File size limit | 25 MB | 25 MB | 500 MB |
| Execution tracking | Browser-local (IndexedDB) | Convex (server-synced) | Convex (server-synced) |
| Post-run nudge | Soft banner: "Your runs are saved locally. Sign up to keep them forever." | None | None |
| Server nodes (AI, shell, video) | Visible in editor, grayed out | Visible, grayed with "Pro" badge | Full access |

---

### My Recipes (`/my-recipes`)

**The first layered carrot feature.** This is where the tier model proves itself. Each level gives users just enough to see the value of the next level.

#### Design philosophy: See it -> Use it -> Power it

- **Unauth (bare bones):** A receipt. "Here's what you did." Read-only, minimal, no actions. The list itself is the hook.
- **Free (functional):** Now it's useful. Re-run, save, create custom recipes. This is a real dashboard.
- **Pro (enriched):** Power tools. Longer retention, sharing, server nodes, cloud export.

#### Concrete Limits (decided)

| Limit | Unauth | Free Account | Pro |
|---|---|---|---|
| History storage | Browser-local (IndexedDB) | Server-synced (Convex) | Server-synced (Convex) |
| History entry cap | **10 entries** (oldest rotated out) | Unlimited (within retention window) | Unlimited (within retention window) |
| History retention | Until browser data cleared | **7 days** | **30 days** |
| Saved recipes | None (export only) | **3 recipes** | Unlimited |
| Custom recipe creation | No | Yes (in editor) | Yes (in editor) |

---

### Unauth: Bare Bones

**What they see:** A minimal read-only list of their recent browser-local runs. No bells, no whistles. Just a receipt.

| Element | Visible? | Interactive? | Notes |
|---|---|---|---|
| Page access | Yes | — | Not proxy-blocked. Accessible to everyone. |
| History list | Yes — last 10 runs | **Read-only.** No re-run, no expand. | Shows: recipe name, timestamp, status. That's it. |
| Usage stats | Minimal — run count only | No | Single number: "X runs this session." |
| Saved tab | **AccountGated** — blurred with sign-up prompt | No | "Create an account to save recipes and build your own." |
| Re-run button | No | No | Free Account feature. |
| Export from history | No | No | Use export on the recipe tool page itself. |
| Create custom recipe | No | No | AccountGated. "Make an account to create your own recipes." |
| Conversion banner | **Yes — persistent** | CTA links to /signin | "Your history lives in this browser only. Create an account to keep it forever, re-run past recipes, and build your own." |

**The key:** Unauth `/my-recipes` is deliberately limited. It's proof that the product tracked their work, and a preview of what they'd get by signing up.

---

### Free Account: Functional

**What they see:** A real, useful dashboard. History they can act on. Recipes they can save and create.

| Element | Visible? | Interactive? | Notes |
|---|---|---|---|
| Page access | Full | Full | No gates, no blurs. |
| History list | Yes — 7-day retention, server-synced | **Full.** Re-run, expand details. | Works across devices. Entries >7 days cleaned by Convex cron. |
| Usage stats | Full — total runs, plan badge, last activity | Yes | Shows "Free" plan badge. |
| Saved tab | Yes | Yes — up to **3 saved recipes** | At limit: "Upgrade to Pro for unlimited saves." |
| Re-run button | Yes | Yes | One-click re-run from history. |
| Export from history | Yes | Yes — `.bnto.json` download | Export any history entry as a recipe file. |
| Create custom recipe | Yes | Yes — opens editor | Full editor access. Save counts toward 3-recipe limit. |
| Pro upgrade nudge | Yes — subtle | CTA links to /pricing | Visible when approaching limits. |

**The key:** Free account is where bnto becomes genuinely useful. The limits (3 saves, 7-day retention) are real but not punishing.

---

### Pro: Enriched

**What they see:** Everything Free gets, with power-user capabilities unlocked.

| Element | Visible? | Interactive? | Notes |
|---|---|---|---|
| History list | Yes — **30-day** retention | Full | 4x the retention of Free. |
| Usage stats | Full — "Pro" badge | Yes | Server-node usage stats when applicable. |
| Saved tab | Yes | **Unlimited saves** | Build a library. |
| Sharing | Yes | Yes — up to 5 team members | Share button on each saved recipe. |
| Cloud drive export | Yes | Yes — Google Drive, OneDrive | Auto-export results to connected storage. |
| Server nodes in history | Yes | Yes | History shows server-node executions with usage tracking. |
| File size | 500 MB per file | Yes | 20x the free limit. |

**The key:** Pro doesn't gate anything Free users already had — it *adds*. The upgrade never feels like getting back something that was taken away.

---

### Recipe Editor (`/editor` or inline)

Requires an account. Unauth users can run predefined recipes on tool pages but cannot create custom ones.

| Capability | Unauth | Free Account | Pro |
|---|---|---|---|
| Open editor | **AccountGated** | Yes | Yes |
| Create + run recipes (browser nodes) | No | Yes | Yes |
| Export `.bnto.json` | No | Yes | Yes |
| Save recipe to account | No | Yes (up to 3) | Yes (unlimited) |
| Share recipe | No | No | Yes |
| Use server nodes in editor | Visible, not runnable | Visible, not runnable | Full access |

---

### Pricing Page (`/pricing`)

| Capability | Unauth | Free Account | Pro |
|---|---|---|---|
| View page | Full | Full | Full |
| Primary CTA | "Create free account" | "Upgrade to Pro" | "Manage subscription" |

### Settings (`/settings`)

| Capability | Unauth | Free Account | Pro |
|---|---|---|---|
| Access | Redirected to /signin (proxy) | Full | Full |
| Profile management | N/A | Yes | Yes |
| Plan management | N/A | Free tier info + upgrade CTA | Billing, usage, upgrade/downgrade |

---

## Conversion Hooks

Each hook fires at a natural value moment. Never artificial. Never blocking core functionality.

| Hook | Trigger | Converts to | Mechanism |
|---|---|---|---|
| **Save** | User wants to persist a recipe | Free Account | AccountGate overlay on save action |
| **History** | User sees browser-local history, wants it everywhere | Free Account | Banner on history tab |
| **Dashboard** | User visits /my-recipes, sees partial view | Free Account | AccountGate on Saved tab, conversion banner |
| **Server Nodes** | User adds AI, shell, or video node in editor | Pro | "Pro" badge on node, tooltip explains requirement |
| **Team** | User wants to share recipes with collaborators | Pro | Share button triggers Pro upgrade prompt |
| **Cloud Drive** | User wants auto-export to cloud storage | Pro | Cloud drive option triggers Pro upgrade prompt |
| **File Size** | User drops a file > 25 MB | Pro | Error message with "Upgrade to Pro for 500 MB files" |
| **History Retention** | Free user's 7-day history starts expiring | Pro | "Upgrade to Pro for 30-day retention" |

---

## The Funnel Visualized

```
Unauth visitor (SEO landing)
  → Runs tools, sees value
  → Browser-local history builds up
  → Wants to save / keep history
  → AccountGate conversion moment
  → Signs up (free)
  → Free Account (server-synced history)
  → Needs AI / sharing / retention
  → Pro upgrade ($8/mo)
```

---

## AccountGate: The Primary Conversion Component

AccountGate is not a blocker — it's an enticer. It shows users what they're missing by rendering real content behind a blur with a friendly sign-up prompt.

**Where AccountGate appears:**
- `/my-recipes` Saved tab (fully gated)
- `/my-recipes` "Create custom recipe" action (gated for unauth)
- Recipe editor access (gated for unauth)
- Save action in recipe editor (gated for unauth)
- Future: any surface where persistence or creation = account required

**Where AccountGate does NOT appear:**
- Recipe tool pages (never block running predefined recipes)
- Download/export of execution results (never block output)
- Pricing page (obviously)

**AccountGate copy guidelines:**
- Honest, not urgent. No countdown timers, no "limited time."
- Explains what they get: "Sign up to save your recipes and keep your history across devices."
- Two CTAs: "Sign in" (returning) and "Create account" (new)
- Never mentions Pro — AccountGate is Free Account conversion only

---

## Browser-Local Execution History

Unauth users get browser-local execution tracking via IndexedDB/localStorage. This is a deliberate strategic choice:

**Why track locally for unauth users:**
- Gives them a taste of persistence that makes the account conversion more compelling
- They see their runs building up and think "I don't want to lose this"
- Makes the `/my-recipes` page useful even without an account
- Respects our principle: "No mandatory accounts before showing value"

**What's stored locally:**
- Recipe slug, timestamp, status (completed/failed), duration
- NOT the actual input/output files (too large for browser storage)
- **Capped at 10 entries, oldest rotated out** — deliberately small to create urgency

**What's NOT stored locally:**
- Saved recipes (requires account)
- Execution output files (ephemeral, download-only)
- Usage analytics beyond basic run count
- Re-run capability (requires account)

---

## Decided Limits

- [x] **Free account recipe save limit:** 3 recipes. At limit, show "Upgrade to Pro for unlimited saves."
- [x] **Free history retention:** 7 days (server-synced via Convex). Entries older than 7 days cleaned up by cron. Pro = 30 days.
- [x] **Browser-local history cap:** 10 entries. Oldest rotated out. Deliberately small — just enough to show value, not enough to feel complete.

## Open Decisions (TBD)

- [ ] **Post-run nudge timing:** Show after first run? Third run? Only when they try to leave?
- [ ] **History expiry notification:** When Free user's 7-day entries start expiring, do we notify? Email? In-app banner?
- [ ] **Recipe save limit UX:** When at 3/3, can they overwrite an existing save? Or must they delete first?

---

## Companion Documents

- [pricing-strategy.md](pricing-strategy.md) — pricing rationale, conversion psychology, revenue projections
- [business-model.md](business-model.md) — unit economics, competitive analysis
- [seo-monetization.md](seo-monetization.md) — acquisition, search volume, target queries
- **Repo:** `.claude/strategy/pricing-model.md` — operational rules for agents (what to enforce in code)

---

*Every feature decision passes the test: "Does this feel fair to the designer who just wants to compress some images?" If yes, ship it. If no, go back to the drawing board.*
