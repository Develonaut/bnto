# SEO & Monetization Strategy

**Last Updated:** February 2026
**Status:** Active — decisions finalized, implementation in progress
**Previously:** Notion — "SEO & Monetization Strategy"

---

## Pricing Model

> **Updated February 2026:** The monetization model has shifted from cloud-first/run-capped to browser-first/value-driven. Browser execution is free unlimited. Run caps apply to server-side premium bntos only.

### Browser Execution (Free, Unlimited)

- All Tier 1 bntos run 100% client-side — files never leave the user's machine
- No run limits. No account required. No file size limits (browser memory is the practical cap)
- Zero cost to us — browser does all the work
- Rationale: compressing images shouldn't cost money when the user's browser is doing it

### Pro Tier (Web) — Value-Driven

- $8/month or $69/year (annual = ~2 months free)
- **What you're paying for:** Persistence, collaboration, premium compute — not permission to keep using browser tools
- Saved workflows + execution history (30-day retention)
- Team sharing up to 5 members (no per-seat pricing)
- Server-side premium bntos (AI inference, shell commands, video processing) — usage-based
- Priority processing for server-side bntos
- API access
- Cloud drive export (Google Drive, OneDrive — post-MVP)
- Rationale: cheaper than Convertio ($9.99), impulse-purchase price point

### Server-Side Bntos (Pro, Usage-Based)

- AI, shell-command, video, large file operations — things browsers can't do
- Real compute cost (Railway CPU) — run-based pricing makes sense here
- Usage tracked per execution
- Desktop users get these free via BYOK (Bring Your Own Key) for AI

### Desktop App

- Free forever, unlimited — non-negotiable brand promise
- Trust signal, top-of-funnel, word-of-mouth driver
- Obsidian model: free local, paid cloud services
- Never a "desktop Pro" tier. Ever.

---

## Conversion Funnel (Value-Driven)

Users convert when they want something the browser can't provide alone.

| Hook | Trigger | What They're Buying |
|---|---|---|
| **Save** | "I want to keep this workflow" | Persistence (Convex-backed workflow storage) |
| **History** | "I need my execution history for audit" | Execution log retention (30-day Pro) |
| **Premium Bntos** | "I need AI to classify these images" | Server-side compute (usage-based) |
| **Team** | "My team needs shared workflows" | Collaboration (up to 5 members, no per-seat) |
| **Cloud Drive** | "Auto-save results to Google Drive" | Convenience (OAuth + server-side upload) |

### Upgrade Prompt Copy

> "Want to save this recipe for next time? Sign up for free to keep your history.
> Need AI, shell commands, or team sharing? Pro is $8/month. That's it."

Two honest choices. No artificial limits on browser tools. Desktop option visible.

---

## SEO Strategy: Target Queries by Bnto

Each predefined Bnto targets a specific high-intent search query. The URL IS the tool — users land ready to run.

### Tier 1 Launch Bntos

| Bnto | URL | Primary Query | Est. Monthly Searches |
|---|---|---|---|
| Compress Images | `/compress-images` | "compress images online free" | 100K+ |
| Resize Images | `/resize-images` | "resize images online" | 200K+ |
| Convert Image Format | `/convert-image-format` | "convert png to webp online" | 50K+ |
| Rename Files | `/rename-files` | "batch rename files online" | 20K+ |
| Clean CSV | `/clean-csv` | "clean csv online free" | 15K+ |
| Rename CSV Columns | `/rename-csv-columns` | "rename csv columns online" | 8K+ |

### SEO Content Strategy (Phase 3+)

Bannerbear playbook: documentation and tutorials drove more conversions than any other channel.

---

## Revenue Projections (Updated for Browser-First)

- Infrastructure cost per browser user: **$0**
- Infrastructure cost per server-side execution: ~$0.005–0.01
- Browser execution eliminates the cost-per-user equation for Tier 1 bntos entirely

### Revenue Milestones by Sprint

| Sprint | What Ships | Revenue Implication |
|---|---|---|
| Sprint 2B | Browser execution (M1 MVP) | All Tier 1 free, unlimited. SEO live. |
| Sprint 3 | Platform features | Accounts + conversion hooks (Save, History). |
| Sprint 7 | Stripe + Pro tier | **First revenue.** Pro: persistence, collaboration, premium compute. |

---

## Acquisition Strategy

1. **SEO** — highest leverage, compounds over time
2. **Product Hunt** — post-Sprint 2B
3. **Community seeding** — Reddit, Indie Hackers
4. **Desktop downloads** — app directories, roundups

---

## The Trust Commitments

1. **Browser bntos are free unlimited** — no artificial caps, ever
2. Desktop free forever — no "desktop Pro"
3. MIT license stays MIT
4. No dark patterns
5. No overpromising
6. If bnto shuts down, the engine stays open

---

## What's Always Free

> **The dividing line:** Nodes that can run in your browser are free. Nodes that need a server cost money. Node definitions are always available (MIT licensed). The gate is server execution, not capability.

| Layer | Free? |
|---|---|
| All browser-node recipes (predefined + custom) | Yes, always, unlimited |
| Recipe editor (create, run, export) | Yes, always |
| Desktop app (all nodes including AI via BYOK) | Yes, always |
| Save recipes to account | Pro |
| Execution history (30-day retention) | Pro |
| Server-node execution (AI, shell, video) | Pro (usage-based) |
| Team sharing (up to 5 members) | Pro |
| API access | Pro |
| Cloud drive export | Pro |

### Removed: Run Caps and Quota System

The old "25 runs/month" free tier model and all associated quota infrastructure have been **fully removed** from the codebase as of March 2026. Browser execution costs us $0 — capping it was artificial and hostile. Server-node run limits will be re-introduced when the Pro tier ships (Sprint 7).

### Recipe Editor: Free (Community Decision)

The recipe editor is free. Anyone can create, run, and export `.bnto.json` recipes using browser nodes. Pro gates kick in for persistence (save), server-node execution, and collaboration (share). This decision enables a future community recipe marketplace.

---

*Reference this document when making pricing, acquisition, or revenue decisions. For the definitive operational rules, see `.claude/strategy/pricing-model.md`.*
