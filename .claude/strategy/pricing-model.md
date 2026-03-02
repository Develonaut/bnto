# Pricing Model — Operational Rules

**Last Updated:** February 25, 2026
**Status:** Active — operational rules for agents writing code
**Full strategy:** Notion → "Pricing Model & Monetization Strategy" (business rationale, conversion psychology, revenue projections)

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
| **Predefined recipes** (browser nodes) | Always free. No account needed. |
| **Custom recipes** (browser nodes) | Free to create, run, export. Pro to save or share. |
| **Any recipe with server nodes** | Pro required for execution. |

---

## Free vs Pro Gate

| Action | Free | Pro |
|---|---|---|
| Run predefined recipes (browser) | Unlimited | Unlimited |
| Run custom recipes (browser nodes) | Unlimited | Unlimited |
| Recipe editor (create + run + export) | Yes | Yes |
| All node definitions visible | Yes | Yes |
| Save recipes to account | No | Yes |
| Execution history | No | Yes (30-day retention) |
| Server-node execution | No | Usage-based |
| Team sharing | No | Yes (up to 5 members) |
| Priority server-side processing | No | Yes |
| API access | No | Yes |
| Cloud drive export | No | Yes |
| Desktop app | Free forever | Free forever |

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

- All browser recipes, unlimited
- Recipe editor, create and run
- Export `.bnto.json` files
- No persistence — closing the browser loses unsaved work
- No server-side session — browser execution is 100% client-side

---

*This file contains operational rules for agents. Full business strategy (pricing rationale, conversion psychology, revenue projections, competitive positioning) lives in Notion.*
