# Pricing Model & Monetization Strategy

**Last Updated:** February 25, 2026
**Status:** Active — single source of truth for business model, pricing rationale, and conversion strategy
**Previously:** Notion — "Pricing Model & Monetization Strategy"
**Supersedes:** All "25 runs/month" and quota/run-limit references in older docs (fully removed from codebase March 2026)
**Repo companion:** `.claude/strategy/pricing-model.md` (operational rules only — this file has the full business strategy)

---

## The Dividing Line

The free/premium split follows **cost to bnto** — not capability, not node types, not features:

> **Nodes that can run in your browser are free. Nodes that need a server cost money.**
> The node *definitions* are always available to everyone (they're in `@bnto/nodes`, MIT licensed). The *execution* of server nodes is what costs money.

This is the only principle you need. Everything else follows from it.

---

## Three-Layer Framework

### Layer 1: Nodes

Nodes are the atoms. Every node type falls into one of two execution categories:

| Category | Examples | Execution | Cost to Bnto | User Access |
|---|---|---|---|---|
| **Browser nodes** | `image`, `csv`, `file`, `transform`, `archive`, `pdf` | Client-side (Rust WASM) | $0 | Free, unlimited, forever |
| **Server nodes** | `ai`, `shell-command`, `video`, `http-request` (unrestricted) | Server-side (Railway) | Real CPU cost | Pro tier, usage-based |

**On desktop, everything is free** — including AI (BYOK) and shell-command. The user's machine does the work.

### Layer 2: Recipes

Recipes compose nodes into pipelines:

| Type | Description | Access |
|---|---|---|
| **Predefined recipes** | Curated by bnto, ship with the product, have SEO pages | Always free if they use browser nodes. No account needed. |
| **Custom recipes** | Created by users in the recipe editor | Free to create, run, and export (browser nodes). Pro to save, share, or use server nodes. |

**The recipe editor is free.** Anyone can open the editor, compose browser nodes into a custom recipe, run it, and export the `.bnto.json` file.

| Action | Free | Pro |
|---|---|---|
| Open the recipe editor | Yes | Yes |
| Create a recipe with browser nodes | Yes | Yes |
| Run a recipe with browser nodes | Yes, unlimited | Yes, unlimited |
| Export recipe as `.bnto.json` | Yes | Yes |
| Use server nodes in a recipe | No | Yes (usage-based) |
| Save recipe to account | No | Yes |
| Access execution history | No | Yes (30-day retention) |
| Share recipe with team | No | Yes (up to 5 members) |

### Layer 3: Platform Features

| Feature | Free | Pro ($8/mo or $69/yr) |
|---|---|---|
| Run predefined recipes (browser) | Unlimited | Unlimited |
| Run custom recipes (browser nodes) | Unlimited | Unlimited |
| Recipe editor (create + run + export) | Yes | Yes |
| All node definitions visible | Yes | Yes |
| Save recipes to account | No | Yes |
| Execution history (30-day retention) | No | Yes |
| Server-node execution (AI, shell, video) | No | Usage-based |
| Team sharing (up to 5 members) | No | Yes |
| Priority server-side processing | No | Yes |
| API access | No | Yes |
| Cloud drive export (Google Drive, OneDrive) | No | Yes |
| Desktop app | Free forever | Free forever |

---

## Pricing Rationale

### Why $8/month

Cheaper than Convertio ($9.99/month). Comparable to iLoveIMG ($5/month) but significantly more capable. More expensive than TinyPNG ($39/year = $3.25/month) but bnto does infinitely more. At $8, it's an impulse purchase for a freelance designer — roughly two coffees.

Underpricing at $5 requires too many users to hit revenue goals. Overpricing at $12 increases friction for a utility tool.

### Why one tier, not two

The old model had a Starter (~$5-8) and Pro (~$15-20). Research showed that in this category, two paid tiers create decision paralysis. One paid tier is simpler to explain, support, and market. If team demand is strong, add a Team tier later — but don't build it speculatively.

### Why no per-seat pricing

The target user is a 1-5 person team. Charging per seat turns a $8 decision into a $40 decision before they've even tried it. One flat price for up to 5 team members removes the mental math.

### Annual billing

$69/year ("2 months free") — aim for 40%+ annual uptake. At 90 paying users with 40% annual, month 6 cash collected is significantly higher than MRR suggests.

---

## Conversion Funnel

Users convert when they want something the browser can't provide alone. These are natural upgrade hooks — not artificial limits.

| Hook | Trigger | What They're Buying | Psychology |
|---|---|---|---|
| **Save** | "I want to keep this recipe" | Persistence (Convex-backed storage) | Most natural — they've created value and want to keep it |
| **History** | "I need my execution history for audit" | Execution log retention (30-day Pro) | Comes after repeated use — they're already hooked |
| **Server nodes** | "I need AI to classify these images" | Server-side compute (Railway, usage-based) | Capability upgrade — fair, because it costs us real money |
| **Team** | "My team needs shared recipes" | Collaboration (up to 5 members, no per-seat) | Organizational need — willingness to pay is higher |
| **Cloud Drive** | "Auto-save results to Google Drive" | Convenience (OAuth + server-side upload) | Workflow integration — power user behavior |

**The natural conversion hook:** Free users can create, run, and export — but every time they close the browser, unsaved recipes are gone. The "Save" button is the most natural upgrade prompt in the world.

**Upgrade prompt copy:**
> "Want to save this recipe for next time? Pro is $8/month.
> Need AI, shell commands, or team sharing? That's Pro too. That's it."

Two honest choices. Desktop option visible. No artificial limits on browser tools.

---

## Community Recipe Ecosystem (Future)

The free recipe editor enables a community recipe marketplace:

1. **Anyone** can create and export `.bnto.json` files (free)
2. **Pro users** can save recipes to their account and share with team
3. **Community marketplace** (future): users submit recipes, others browse and use them
4. **Revenue share with creators** (future): community creators earn from their recipes

**Why the editor is free:** It fosters community. People create recipes, export `.bnto.json` files, share them. The format is open (MIT licensed, human-readable). This enables a future community recipe marketplace. Gating the editor kills the compose story that differentiates bnto from TinyPNG/CloudConvert.

---

## Server-Side Compute Economics

Server nodes run on Railway. Real cost per execution:

| Node Type | Typical Duration | CPU Cost | Why It's Pro |
|---|---|---|---|
| `ai` (LLM inference) | 2-30s | ~$0.01-0.05 | API proxy cost + Railway CPU |
| `shell-command` (ffmpeg, etc.) | 1-60s | ~$0.005-0.02 | CPU-intensive, requires container |
| `video` processing | 5-120s | ~$0.01-0.10 | Heavy CPU, memory-intensive |
| `http-request` (unrestricted) | 0.5-30s | ~$0.001-0.01 | CORS bypass, server-side fetch |

**Desktop users get these free** via BYOK (Bring Your Own Key) for AI and local binary execution for shell-command/video.

**Cost per free browser user: $0.** Browser execution happens on the user's machine. A million free browser users costs nothing extra.

---

## What We Refuse To Do

These are non-negotiable. They are the brand.

1. **No per-task pricing.** Ever. We never charge per execution for browser bntos.
2. **No artificial caps on browser execution.** Costs us $0 → free unlimited to users.
3. **No feature gating on node definitions.** Every node type is visible. The gate is server *execution*, not *capability*.
4. **No mandatory accounts before showing value.** Drop files, run, download. Account comes after.
5. **No watermarks or quality reduction** on free output.
6. **No proprietary format.** `.bnto.json` is yours. It runs anywhere the engine runs.
7. **No dark patterns.** No hidden trial expiry, no surprise charges, no urgency theater.
8. **No decreasing free capabilities.** We set the right model from day one and don't move the goalposts.

---

## Unauthenticated Users

Unauthenticated users (no account) get the full free experience:
- All browser recipes, unlimited
- Recipe editor, create and run
- Export `.bnto.json` files
- No persistence — closing the browser loses unsaved work
- No server-side session — browser execution is 100% client-side

---

## Environment Variables

Run limit environment variables (`ANONYMOUS_RUN_LIMIT`, `FREE_PLAN_RUN_LIMIT`) have been fully removed as of March 2026. Browser execution is free unlimited with no caps. Server-node execution limits will be re-introduced when the Pro tier ships (Sprint 7).

---

*Every pricing and product decision passes this test: "Does this feel fair to the designer who just wants to compress some images?" If yes, ship it. If no, go back to the drawing board.*
