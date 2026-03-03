# Pricing Model — Operational Rules

**Last Updated:** March 3, 2026
**Status:** Active — operational rules for agents writing code
**Full strategy:** Notion → "Pricing Model & Monetization Strategy" (business rationale, conversion psychology, revenue projections)
**Feature tiers & conversion funnel:** Notion → "Feature Funnel & Tier Matrix" (surface-by-surface breakdown, concrete limits, AccountGate placement)

---

## The Dividing Line

> **Nodes that can run in your browser are free. Nodes that need a server cost money.**
>
> The node *definitions* are always available to everyone (they're in `@bnto/nodes`, MIT licensed). The *execution* of server nodes is what costs money.

---

## Terminology

Use these terms consistently across all code, docs, and UI copy.

| Term | Definition |
|---|---|
| **Node** | An atomic processing unit. One operation: compress, resize, rename, transform, etc. |
| **Node type** | A category of node: `image`, `csv`, `file`, `transform`, `ai`, `shell-command`, etc. |
| **Recipe** | A `.bnto.json` composition of one or more nodes into a pipeline. |
| **Predefined recipe** | A recipe curated by bnto, shipped with the product, with its own SEO page. |
| **Custom recipe** | A recipe created by a user in the recipe editor. |
| **Browser node** | A node that executes 100% client-side (Rust WASM or JS). Cost to bnto: $0. |
| **Server node** | A node that requires server-side execution (Railway). Cost to bnto: real CPU time. |
| **Execution** | A single run of a recipe against input files. |

**Deprecated terms:** "flow," "workflow," "predefined Bnto" (as a vague catch-all). Use "recipe" for the `.bnto.json` composition. Use "node" for the atomic unit. Use "execution" for a run.

---

## Node Classification

| Category | Node Types | Execution | User Access |
|---|---|---|---|
| **Browser nodes** | `image`, `csv`, `file`, `transform`, `archive`, `pdf` | Client-side (Rust WASM / JS) | Free, unlimited, forever |
| **Server nodes** | `ai`, `shell-command`, `video`, `http-request` (unrestricted) | Server-side (Railway) | Pro tier, usage-based |

**On desktop, everything is free** — including AI (BYOK) and shell-command.

---

## Recipe Classification

| Type | Access |
|---|---|
| **Predefined recipes** (browser nodes) | Always free. No account needed. Run on tool pages. |
| **Custom recipes** (browser nodes) | Requires free account to create (editor is AccountGated for unauth). Free to create, run, export. Save up to 3. Pro for unlimited saves or sharing. |
| **Any recipe with server nodes** | Pro required for execution. |

---

## Three-Tier Gate (Unauth → Free → Pro)

**Full tier matrix with concrete limits:** Notion → "Feature Funnel & Tier Matrix"

| Action | Unauth | Free Account | Pro |
|---|---|---|---|
| Run predefined recipes (browser) | Unlimited | Unlimited | Unlimited |
| Recipe editor (create custom) | No (AccountGated) | Yes | Yes |
| Export `.bnto.json` | Yes (from tool page) | Yes | Yes |
| Save recipes to account | No | **3 recipes** | Unlimited |
| Execution history (browser-local) | **10 entries** (read-only, no re-run) | 10 entries (plus server-synced) | 10 entries (plus server-synced) |
| Execution history (server-synced) | No | **7-day retention** (re-run, cross-device) | **30-day retention** |
| `/my-recipes` dashboard | Bare-bones (read-only history list) | Full (re-run, save, create) | Full + sharing, cloud export |
| File size limit | 25 MB | 25 MB | 500 MB |
| Server-node execution | No | No | Usage-based |
| Team sharing | No | No | Yes (up to 5 members) |
| API access | No | No | Yes |
| Cloud drive export | No | No | Yes |
| Desktop app | Free forever | Free forever | Free forever |

---

## Conversion Hooks

Upgrade prompts trigger at natural value moments — never on artificial browser execution limits.

| Hook | Trigger |
|---|---|
| **Save** | User wants to persist a recipe to their account |
| **History** | User wants to view past executions |
| **Server nodes** | User adds an AI, shell, or video node |
| **Team** | User wants to share recipes with collaborators |
| **Cloud Drive** | User wants to auto-export results to Google Drive, etc. |

---

## What We Refuse To Do

1. **No per-task pricing** on browser execution.
2. **No artificial caps on browser execution.**
3. **No feature gating on node definitions.** The gate is server *execution*, not *capability*.
4. **No mandatory accounts before showing value.**
5. **No watermarks or quality reduction** on free output.
6. **No proprietary format.** `.bnto.json` runs anywhere the engine runs.
7. **No dark patterns.**
8. **No decreasing free capabilities.**

---

## Unauthenticated Users

- Run all predefined browser recipes, unlimited
- **No recipe editor access** — custom recipe creation requires a free account (AccountGated)
- Export `.bnto.json` from tool page results
- **Browser-local execution history** — last 10 runs tracked in IndexedDB. Read-only list on `/my-recipes` (no re-run, no details). Deliberately minimal to create upgrade pressure
- `/my-recipes` accessible but bare-bones — read-only history list, AccountGated Saved tab, conversion banner
- No server-side persistence — Convex-backed history, saved recipes, and cross-device sync require an account
- No server-side session — browser execution is 100% client-side

---

*This file contains operational rules for agents. Full business strategy (pricing rationale, conversion psychology, revenue projections, competitive positioning) lives in Notion.*
