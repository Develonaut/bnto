# Business Model & Economics

**Last Updated:** March 2026
**Status:** Validated — competitive research still valid, pricing model updated
**Previously:** Notion — "Business Model & Economics"

> **Browser-First Model:** Browser-capable recipes are **free unlimited** — zero cost to us, no artificial run caps. Pro tier ($8/mo) sells persistence, collaboration, and premium server-side compute (AI, shell, video). All quota infrastructure has been fully removed from the codebase.

---

## Real Infrastructure Costs

| Service | Cost | Notes |
|---|---|---|
| Vercel | $0 | Next.js web app, generous free tier |
| Convex Cloud | $0 | Database + real-time subscriptions, generous free tier |
| Cloudflare R2 | $0 | File storage — 10GB free, zero egress fees, files deleted after processing |
| Railway (Hobby) | $5/mo | Execution API — $5/month with $5 usage included |
| Domain (bnto.io) | ~$1/mo | ~$12/year |
| **Total at launch** | **~$6/month** | Real number, not theoretical |

**Cost per free browser user: $0.** Browser execution happens on the user's machine. There is no server cost, no R2 cost, no compute cost. You could serve 1,000,000 free browser users and pay nothing extra.

**Cost per server-node execution:** ~$0.005-0.01 in Railway compute. Pro-only, usage-tracked.

**The inflection point:** At 5,000-10,000 monthly active users with moderate usage, Railway costs climb to $15-30/month. At that scale, if 3% are paying $8/month, revenue is $1,200-2,400/month against $30-60 in infrastructure — **97%+ gross margin**.

---

## What the Market Taught Us

Every major file-processing web tool follows the same playbook: let users experience real value, then limit volume.

| Tool | Free tier | Paid entry | Primary gate |
|---|---|---|---|
| TinyPNG | 20 images/session, 5MB cap | $39/year | Volume + file size |
| iLoveIMG | 30 images/task, 200MB | $5/month | Batch size + file size |
| Convertio | 100MB, 10 conversions/day | $9.99/month | File size + daily count |
| CloudConvert | 10 conversions/day | $8 (500 credits) | Daily count |
| Smallpdf | 2 tasks/day | $10/month | Daily tasks |
| Zamzar | 2 files/day | $12/month | File count + size |

**What works:** Volume caps with file size as a secondary gate. The core processing is always free — what's limited is how much you can process.

**What doesn't work:** Feature gating on core functionality, per-conversion pricing that scales against power users, forcing account creation before showing value.

**The Evernote lesson:** Don't set limits generously then make them more restrictive. Users revolt. Set the right limit from day one and never decrease it.

---

## The Pricing Model

Browser execution costs us $0 — capping it is artificial and hostile. Server-side nodes have real Railway CPU cost.

**Browser recipes:** Free, unlimited, no caps, no account needed.
**Server-side recipes:** Usage-based, tracked per execution, Pro tier.

### Free Tier (Browser Execution)
- All browser-capable nodes (image, csv, file, transform, archive, pdf) execute via Rust WASM
- No run limits, no file count limits
- No watermarks, no quality reduction, no crippled output
- No account required — unauthenticated users get full browser execution

### Pro Tier: $8/month or $69/year
- Save recipes to your account
- Execution history with re-run capability (30-day retention)
- Server-node execution: AI inference, shell commands, video processing
- Share recipes with up to 4 other users
- Priority processing queue for server-node executions

**Why $8/month:** Cheaper than Convertio ($9.99/month). Comparable to iLoveIMG ($5/month) but significantly more capable. At $8, it's an impulse purchase for a freelance designer.

**Why one tier, not two:** Two paid tiers create decision paralysis. One paid tier is simpler to explain, support, and market.

**Why no per-seat pricing:** Charging per seat turns $8 into $40. One flat price for up to 5 team members.

### Desktop App: Free Forever, Unlimited
This is not a loss leader. It's a strategic asset:
- **Trust signal.** "We're not holding your data hostage."
- **Top-of-funnel.** Desktop users who outgrow single-machine limitations graduate to the web app.
- **Word of mouth.** "There's this app I use, it's free, it just works."
- **The Obsidian model:** Free local, paid cloud services.

---

## Revenue Math: The Path to $500-$1,000 MRR

At $8/month:
- **63 paying users** -> $500 MRR
- **125 paying users** -> $1,000 MRR

At a **3% free-to-paid conversion rate** (realistic for file-processing tools):
- $500 MRR requires ~2,100 monthly active free users
- $1,000 MRR requires ~4,200 monthly active free users

### Conservative 6-month projection

| Month | Free MAU | Paying users | MRR |
|---|---|---|---|
| 1 | 150 | 4 | $32 |
| 2 | 400 | 12 | $96 |
| 3 | 900 | 27 | $216 |
| 4 | 1,500 | 45 | $360 |
| 5 | 2,200 | 66 | $528 |
| 6 | 3,000 | 90 | $720 |

**$500 MRR lands around month 5. $1,000 MRR is 8-10 months.** A strong Product Hunt launch can compress the timeline.

---

## What Triggers Conversion

Users upgrade when they hit a natural value moment, not an artificial wall.

- **Save** — "I want to keep this recipe." Most natural conversion moment.
- **History** — "I need my execution history." Comes after repeated use.
- **Server nodes** — AI inference, shell commands, video processing. Fair — real compute cost.
- **Team sharing** — Multiple people, one $8 account. Genuinely good deal.
- **Cloud Drive Export** — Auto-export to Google Drive, Dropbox. Real API cost.

---

## What We Refuse To Do

1. **No per-task pricing.** Ever.
2. **No artificial caps on browser execution.**
3. **No feature gating on node definitions.** The gate is server *execution*, not *capability*.
4. **No mandatory accounts before showing value.**
5. **No watermarks or quality reduction** on free output.
6. **No proprietary format.** `.bnto.json` is yours.
7. **No dark patterns.**
8. **No decreasing free capabilities.**

---

## Acquisition Strategy

1. **SEO** — highest leverage, compounds over time. File-processing keywords have enormous search volume with clear commercial intent.
2. **Product Hunt** — post-M1 launch. Goal: seed initial user base.
3. **Community seeding** — Reddit, Indie Hackers. Not spam — genuine participation.
4. **Desktop downloads** — app directories, "best free apps" roundups.

---

## Revenue Streams (Priority Order)

1. **Web Pro subscriptions** ($8/mo or $69/yr) — core business
2. **Desktop app is free forever** — brand promise, growth driver
3. **Recipe marketplace** (future) — community-created recipes, revenue share
4. **Team/org tiers** (future) — only if demand is clear

---

## Success Metrics

**Healthy indicators:**
- Free users running browser recipes weekly (retention)
- 3%+ free-to-paid conversion rate
- Infrastructure costs under 10% of revenue
- Desktop downloads growing month-over-month

**Warning signs:**
- Railway costs growing faster than paid users
- Free users never hitting value moments (product isn't sticky)
- Conversion rate below 1% (positioning or pricing problem)

---

*Every pricing and product decision passes this test: "Does this feel fair to the designer who just wants to compress some images?" If yes, ship it. If no, go back to the drawing board.*
