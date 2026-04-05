# Pricing Model — Reference (TABLED)

**Last Updated:** April 5, 2026
**Status:** TABLED — monetization paused as of April 2026. This document is preserved as a reference for when revenue strategy is revisited. No Stripe, no Pro tier, no feature gates until the engine has proven value and community adoption. See [ROADMAP.md](../ROADMAP.md) for current strategic direction.

**Previous status:** Active operational rules (March 2026)
**Full strategy:** `pricing-strategy.md` in private business docs (`BNTO_PRIVATE_DOCS_PATH` in `.env.local`) — business rationale, conversion psychology, revenue projections
**Feature tiers & conversion funnel:** `feature-funnel.md` in private business docs — surface-by-surface breakdown, concrete limits, AccountGate placement

---

## The Dividing Line

> **Nodes that run locally are free. Nodes that need a managed server cost money.**
>
> The node _definitions_ are always available to everyone (they're in `@bnto/nodes`, MIT licensed). The _execution_ of server nodes is what would cost money when monetization is reactivated. "Locally" means CLI, browser (WASM), and desktop — all cost $0 to us.

---

## Terminology

Use these terms consistently across all code, docs, and UI copy.

| Term                  | Definition                                                                                 |
| --------------------- | ------------------------------------------------------------------------------------------ |
| **Node**              | An atomic processing unit. One operation: compress, resize, rename, transform, etc.        |
| **Node type**         | A category of node: `image`, `csv`, `file`, `transform`, `ai`, `shell-command`, etc.       |
| **Recipe**            | A `.bnto.json` composition of one or more nodes into a pipeline.                           |
| **Predefined recipe** | A recipe curated by bnto, shipped with the product, with its own SEO page.                 |
| **Custom recipe**     | A recipe created by a user in the recipe editor.                                           |
| **Local node**        | A node that executes locally (CLI native, browser WASM, desktop native). Cost to bnto: $0. |
| **Server node**       | A node that requires managed server-side execution. Cost to bnto: real CPU time.           |
| **Execution**         | A single run of a recipe against input files.                                              |

**Deprecated terms:** "flow," "workflow," "predefined Bnto" (as a vague catch-all). Use "recipe" for the `.bnto.json` composition. Use "node" for the atomic unit. Use "execution" for a run.

---

## Node Classification

| Category         | Node Types                                                     | Execution                         | User Access              |
| ---------------- | -------------------------------------------------------------- | --------------------------------- | ------------------------ |
| **Local nodes**  | `image`, `csv`, `file`, `video`, `transform`, `archive`, `pdf` | Local (CLI native / browser WASM) | Free, unlimited, forever |
| **Server nodes** | `ai`, `shell-command`, `http-request` (unrestricted)           | Server-side (M4, technology TBD)  | Pro tier, usage-based    |

**On desktop, everything is free** — including AI (BYOK) and shell-command.

---

## Recipe Classification

| Type                                   | Access                                                                                                                         |
| -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| **Predefined recipes** (browser nodes) | Always free. No account needed. Run on tool pages.                                                                             |
| **Custom recipes** (browser nodes)     | Free for all users. No account needed. Create, run, export, auto-save locally (unlimited). Sign in for cloud sync and sharing. |
| **Any recipe with server nodes**       | Pro required for execution.                                                                                                    |

---

## Three-Tier Gate (Unauth → Free → Pro)

**Full tier matrix with concrete limits:** `feature-funnel.md` in private business docs (`BNTO_PRIVATE_DOCS_PATH`)

| Action                            | Unauth                                | Free Account                               | Pro                             |
| --------------------------------- | ------------------------------------- | ------------------------------------------ | ------------------------------- |
| Run predefined recipes (browser)  | Unlimited                             | Unlimited                                  | Unlimited                       |
| Recipe editor (create custom)     | Yes (auto-saves locally)              | Yes                                        | Yes                             |
| Export `.bnto.json`               | Yes (from tool page)                  | Yes                                        | Yes                             |
| Save recipes locally              | Unlimited (localStorage)              | Unlimited (localStorage)                   | Unlimited (localStorage)        |
| Save recipes to cloud             | No                                    | Unlimited                                  | Unlimited                       |
| Execution history (browser-local) | **10 entries** (read-only, no re-run) | 10 entries (plus server-synced)            | 10 entries (plus server-synced) |
| Execution history (server-synced) | No                                    | **7-day retention** (re-run, cross-device) | **30-day retention**            |
| `/my-recipes` dashboard           | Local recipes + upsell banner         | Full (local + cloud-synced)                | Full + sharing, cloud export    |
| File size limit                   | 25 MB                                 | 25 MB                                      | 500 MB                          |
| Server-node execution             | No                                    | No                                         | Usage-based                     |
| Team sharing                      | No                                    | No                                         | Yes (up to 5 members)           |
| API access                        | No                                    | No                                         | Yes                             |
| Cloud drive export                | No                                    | No                                         | Yes                             |
| Desktop app                       | Free forever                          | Free forever                               | Free forever                    |

---

## Conversion Hooks

Upgrade prompts trigger at natural value moments — never on artificial browser execution limits.

| Hook             | Trigger                                                 |
| ---------------- | ------------------------------------------------------- |
| **Save**         | User wants to persist a recipe to their account         |
| **History**      | User wants to view past executions                      |
| **Server nodes** | User adds an AI, shell, or video node                   |
| **Team**         | User wants to share recipes with collaborators          |
| **Cloud Drive**  | User wants to auto-export results to Google Drive, etc. |

---

## What We Refuse To Do

1. **No per-task pricing** on browser execution.
2. **No artificial caps on browser execution.**
3. **No feature gating on node definitions.** The gate is server _execution_, not _capability_.
4. **No mandatory accounts before showing value.**
5. **No watermarks or quality reduction** on free output.
6. **No proprietary format.** `.bnto.json` runs anywhere the engine runs.
7. **No dark patterns.**
8. **No decreasing free capabilities.**

---

## Unauthenticated Users

- Run all predefined browser recipes, unlimited
- **Full recipe editor access** — create custom recipes, auto-saved to localStorage
- **Unlimited local recipe saves** — localStorage is the user's machine, costs us $0
- Export `.bnto.json` from tool page results
- **Browser-local execution history** — last 10 runs tracked in IndexedDB
- `/my-recipes` accessible — local recipes with upsell banner ("Sign in to sync across devices")
- No server-side persistence — cloud sync, cross-device access, and shared recipes require an account
- No server-side session — browser execution is 100% client-side

---

_This file contains operational rules for agents. Full business strategy lives in private docs (`BNTO_PRIVATE_DOCS_PATH` in `.env.local`) — see `pricing-strategy.md` (rationale, conversion psychology) and `competitive-positioning.md`._
