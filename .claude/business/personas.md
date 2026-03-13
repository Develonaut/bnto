# User Personas & Pain Points

**Last Updated:** February 2026
**Status:** Current — aligned with casual-user-first positioning
**Previously:** Notion — "User Personas & Pain Points"

---

## Overview

bnto is for anyone who needs to get a repeatable task done without friction. That starts with the simplest possible case — a designer dropping files to compress, a small team cleaning up a weekly CSV export — and scales naturally to more complex flows as users grow into them.

We are not building for the Fortune 500. We are building for the 1-5 person team, the solo operator, the indie founder, the designer who just needs their Tuesday to be easier. Everyone else is ignoring them. We're not.

**The test for every feature:** *"Does this make someone's task easier, faster, or more reliable — without requiring them to understand how it works?"*

---

## Persona 1: The Casual User

### Who they are
Designers, content creators, marketers, and small team members with repeatable tasks they do manually today. Not developers. Don't want to be. Just want the thing done.

**Real examples:**
- A designer who compresses images before every client delivery
- A 3-person startup that renames and organizes files before sending to a vendor
- A content creator who batch-converts video thumbnails every week
- A small agency that cleans up a CSV export from their project management tool every Monday

### Their current workflow
Google "compress images online." Bookmark TinyPNG. Use CloudConvert for something else. Google again next time. A different tool for every job. No memory, no reuse, nothing connects.

### Where they're underserved
Single-purpose tools are fragmented — five bookmarks to do what one bnto flow handles. Automation platforms (Zapier, Make) look like they were built for IT departments. Nothing just runs on your machine. Everything wants an account, an internet connection, and eventually a credit card.

### What they need from us
- Open the browser, pick a recipe, drop files, run. No account required.
- Pre-built recipes for the most common tasks (compress images, convert formats, rename files, clean CSV)
- Results fast. Download. Done. Come back next week and it's still there.
- A desktop app they can keep in the dock for offline use — free forever, no limits.

### SEO entry point
This persona arrives via search. "Compress PNG online free." "Batch resize images." "Convert CSV to JSON." They land on `bnto.io/compress-images` or `bnto.io/clean-csv` — the app is pre-loaded with the recipe they were looking for. They run it before they know what bnto is. That's the goal.

### Free tier fit
A casual user processing images twice a week uses browser execution — which is free and unlimited. They may never pay, and that's fine — browser execution costs us $0. They're the audience we're building trust with and the word-of-mouth that grows the product.

---

## Persona 2: The Solo Developer / Indie Hacker

### Who they are
Software engineers working on personal projects, side businesses, or small startups. They automate tasks regularly but don't want to maintain infrastructure for every one-off job. Comfortable with CLI tools and JSON.

### Their current workflow
Write a bash or Python script. Run it manually or set up a cron job. No error handling, no progress feedback, scripts scattered across projects. When things get complex, reach for Zapier (expensive) or n8n (requires Docker and a running server).

### Where they're underserved
Scripts are fragile and unshareable. Platforms are either too expensive or too heavy. No portable, versionable workflow format that works without infrastructure.

### What they need from us
- A CLI binary that just works
- A clean JSON format for defining workflows — version-controllable, shareable, inspectable
- Local execution with zero infrastructure
- Desktop app for visual feedback without learning a new platform
- The web app for running flows without keeping a machine open

### Free tier fit
This user runs more flows and builds habits around the tool. Browser execution is free and unlimited, so they never hit a wall on core usage. They're the highest-intent upgrade candidate for Pro — they understand the value. When they want to save recipes, access execution history, or use server-side nodes (AI, shell, video), the upgrade message resonates: *"Pro for $8/month. Persistence, collaboration, premium compute. Yeah, obviously."*

---

## Persona 3: The Power User / Small Team Lead

### Who they are
Engineers or operators responsible for recurring workflows at a small company. They arrive at bnto after starting as a casual user or solo dev — bnto grew with them. Now they're running more complex flows, sharing them with teammates, and want history and reliability.

### What they need from us
- Shareable flows across a small team (Pro: up to 5 members on one account)
- Processing history with re-run capability
- Larger file support (Pro: up to 500MB)
- Priority processing queue — they're running things on a schedule, not ad hoc
- Eventually: scheduled/triggered flows (post-MVP)

### Free tier fit
This user is a natural Pro subscriber. The team sharing alone is worth $8/month. They represent the retained, recurring revenue base.

---

## The Growth Path

Personas are not silos — they're a natural progression. A casual user compresses images and saves the flow. They come back and add a resize step. Then they're passing output through a rename node. Before long they're a power user who discovered automation organically, without ever hitting a wall or feeling like the tool grew beyond them.

This is the growth model: **the simplest case is the gateway.** The person who starts with "compress images" eventually builds a 20-node pipeline. Same product, same mental model, complexity just scales. We never push users toward complexity — we just make sure the path is always open if they want it.

```
Casual User              Solo Dev / Indie          Power User / Team
(just get it done)       (automate & reuse)        (compose & scale)
      |                         |                         |
  one recipe               a few recipes            flows of flows
  free forever            probably Pro              definitely Pro
```

The desktop app is free at every point on that spectrum. The web app's free tier serves the casual user completely. The Pro tier is a natural yes for anyone who builds a habit around bnto.

---

## Validation Questions

1. Can a non-technical user run their first recipe (image compression) without reading any documentation?
2. Do the SEO landing URLs (`bnto.io/compress-images`, etc.) convert search traffic into first-time users?
3. At what point do users naturally convert to Pro — do they hit a value moment (Save, History, Server Nodes) and upgrade, or do they churn?
4. Does the desktop app drive word-of-mouth from casual users who never pay?
5. What recipes are most requested that aren't in the launch lineup?

---

*Every feature we build should pass this test: which persona does it serve, and does it reduce their friction without adding friction for anyone else?*
