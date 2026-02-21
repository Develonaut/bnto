# Navigation & Platform UX Research

**Last Updated:** February 2026
**Status:** Research Complete -- Informing Navigation Design

---

## Overview

Research into how workflow automation tools and developer platforms organize navigation, specifically around workflow management, execution monitoring, and the builder/consumer duality. Platforms studied across workflow automation and developer tools to inform Bnto's navigation architecture.

---

## Workflow Automation Tools

### n8n

**Main nav (desktop sidebar):** Workflows, Credentials, Executions, Variables, Tags
**Canvas nav:** Node palette (left panel), node configuration (right panel), execution controls (top bar)

**Key patterns:**
- Main nav organized by **resource type** (workflows, credentials, executions)
- Workflow canvas is the primary workspace -- full-screen, immersive
- **Workflows list is the home screen.** Not a dashboard, not analytics -- your workflows front and center
- Execution history is a first-class nav item, not buried in settings
- Node palette appears on canvas via search or sidebar -- always one click away
- Credentials managed globally, referenced by workflows -- clean separation

**Relevance:** Validates workflow-list-as-home. Bnto should similarly center on the workflow library. Canvas-first editing is the destination for Phase 4's visual editor.

---

### Make (Integromat)

**Main nav (left sidebar):** Scenarios, Templates, Connections, Webhooks, Data stores, Devices
**Scenario builder:** Module palette (bottom), data flow visualization (center), scenario controls (top)

**Key patterns:**
- Nav organized by **resource type** with clear separation between building (Scenarios) and configuring (Connections, Webhooks)
- **Templates as a first-class nav item.** Not hidden in a menu -- browseable, installable, one click to start building
- Scenario builder is circular/radial layout -- visually distinct from n8n's linear flow
- Each module shows inline status and data preview during test runs
- Execution history per scenario, not global
- Folders organize scenarios into groups

**Relevance:** Templates as a top-level nav item is a strong pattern. Bnto should make workflow templates prominent, not buried. Per-workflow execution history (vs. global) is worth considering.

---

### Zapier

**Main nav (left sidebar):** Home, Zaps, Tables, Interfaces, Canvas, Chatbots
**Zap editor:** Trigger + action steps (vertical), configuration panels (right)

**Key patterns:**
- Nav organized by **product feature** (Zaps, Tables, Canvas are separate products)
- Home is a dashboard with recent activity and suggestions -- not the workflow list
- Zap editor is strictly vertical/linear -- trigger at top, actions below
- Step-by-step configuration with inline testing per step
- **Folder + tag organization** for Zaps
- Search is prominent -- with thousands of integrations, findability is critical

**Relevance:** Zapier's feature sprawl (Tables, Interfaces, Canvas, Chatbots) shows what happens when you keep adding products. Bnto should resist this -- stay focused on workflows. Their folder + tag organization pattern is worth adopting.

---

### GitHub Actions

**Nav context:** Nested within repository navigation. Actions tab -> Workflows sidebar.

**Key patterns:**
- No standalone nav -- Actions is a tab within the repo context
- Left sidebar lists workflows by name (from `.yml` files)
- Workflow runs shown as a timeline with status badges
- Run detail shows job graph (parallel/sequential steps visualized)
- **Workflow definitions are files, not UI-created.** The "editor" is your code editor
- Manual dispatch ("Run workflow" button) with input parameters

**Relevance:** Workflow-as-file model aligns directly with `.bnto.json`. The run history timeline with status badges is a clean pattern. Manual dispatch with parameters is a must-have for Bnto.

---

## Developer Platforms

### Vercel

**Main nav (top bar):** Overview, Deployments, Analytics, Logs, Storage, Settings
**Project nav:** Sidebar with project list, project detail with tabs

**Key patterns:**
- Nav organized by **concern** (what happened, how it performed, what went wrong)
- **Deployments list is the activity feed** -- chronological, status-coded, with preview URLs
- Logs are a first-class nav item -- critical for debugging
- Minimal nav items -- 6 total. No feature bloat
- Real-time deployment status with live log streaming

**Relevance:** Deployment list as activity feed maps to Bnto's execution list. Real-time log streaming is essential for workflow execution monitoring. Minimal nav count is a good constraint.

---

### Railway

**Main nav (left sidebar):** Projects, with each project expanding to show services
**Project view:** Canvas-style layout showing services and their connections

**Key patterns:**
- **Canvas layout for infrastructure** -- services as nodes, connections as edges
- Each service has tabs: Deployments, Settings, Variables, Metrics
- **Deploy logs stream in real-time** with ANSI color support
- Minimal chrome -- the workspace dominates the viewport
- Project-level organization (group related services)

**Relevance:** Canvas layout for connected services is analogous to workflow nodes. Real-time log streaming with color is excellent. Minimal chrome principle applies to Bnto's workflow canvas.

---

## Universal Patterns Discovered

### 1. Workflow/Resource List is Home

Every workflow tool -- n8n, Make, Zapier -- lands you on your workflow list, not a dashboard or analytics page. The workflows ARE the product. Bnto should follow this pattern: the home screen is your workflow library.

### 2. Execution History is First-Class

No workflow tool buries execution history. It's either a top-level nav item (n8n) or prominently displayed per workflow (Make, GitHub Actions). Execution status and history must be immediately accessible.

### 3. Create Button is Always Prominent

Every platform keeps creation one click away -- n8n's "Add workflow" button, Make's "Create a new scenario," GitHub Actions' "New workflow." The primary action is always visible.

### 4. Canvas for Building, List for Managing

The workflow editor (canvas/builder) is a full-screen immersive experience. The workflow list (management) is a standard list/grid view. These are two distinct modes, not one hybrid view.

### 5. Templates Accelerate Onboarding

Make elevates templates to a top-level nav item. Zapier's home page suggests templates. n8n has a template library. Starting from a working example is universally better than starting from scratch.

### 6. Minimal Nav Items

The best tools keep the persistent nav to 5-7 items. Every additional nav item dilutes attention. n8n has 5, Vercel has 6. Zapier's expansion to 6+ products shows the cost of feature creep.

---

## Implications for Bnto

### Navigation Model

**Sidebar (persistent, 5-6 items) -- workflow-first:**

| Icon | Label | Purpose |
|---|---|---|
| Layers | Workflows | Your workflow library (home screen) |
| Play | Executions | Execution history and monitoring |
| Plus | Create | New workflow (the primary action) |
| Puzzle | Templates | Browse and install workflow templates |
| Settings | Settings | Account, preferences, credentials |

### Workflow Detail View

**Workflow tabs (context-dependent):**

| Tab | Shows | When visible |
|---|---|---|
| Editor | JSON editor (Phase 1) / Visual canvas (Phase 4) | Always |
| Runs | Execution history for this workflow | Always |
| Settings | Workflow configuration, triggers, parameters | Always |
| Logs | Real-time execution logs | During/after execution |

### Execution Monitoring

- Execution list shows status badges (running, completed, failed, cancelled)
- Real-time log streaming during execution
- Per-node status in visual editor (Phase 4)
- Execution detail shows input/output data at each node

### What This Means for Bnto

- **No "dashboard" page.** The workflow list IS the home screen. No vanity metrics, no activity graphs -- just your workflows.
- **No mode switching.** Editing and monitoring are tabs on the same workflow, not separate sections of the app.
- **Templates are prominent.** Not buried in a help menu. First-class nav item for discoverability.
- **Execution status is real-time.** WebSocket subscriptions (via Convex) push status updates instantly.

### Future-Proofing

| Future feature | Where it fits |
|---|---|
| Team workspaces | Workspace switcher above nav |
| Credentials/Secrets | New nav item or Settings sub-page |
| Marketplace (shared workflows) | Under Templates or new nav item |
| Notifications | Badge on nav item or top bar |
| Visual canvas editor (Phase 4) | Replaces JSON editor in Workflow detail |

### Terminology Direction

Bnto's atomic unit is the **"workflow."** Supporting terminology:

| Concept | Term | Avoid |
|---|---|---|
| A workflow definition | Workflow | Zap, Scenario, Pipeline |
| A single run | Execution | Run (too ambiguous), Job |
| A step in a workflow | Node | Step (too generic), Action, Module |
| A type of node | Node type | Integration, Connector |
| Saved workflow templates | Template | Recipe, Blueprint |
| Running a workflow | Execute / Run | Trigger (reserved for event triggers) |

Keep terminology precise and consistent. "Workflow" is the primary noun, "execution" is the primary verb-noun.

---

## Sources

### Workflow Tools
- [n8n Documentation -- Workflows](https://docs.n8n.io/workflows/)
- [n8n GitHub -- UI Components](https://github.com/n8n-io/n8n/tree/master/packages/editor-ui)
- [Make Documentation -- Scenarios](https://www.make.com/en/help/scenarios)
- [Zapier Documentation -- Zaps](https://help.zapier.com/hc/en-us/categories/8496309697421-Build-Zaps)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)

### Developer Platforms
- [Vercel Documentation](https://vercel.com/docs)
- [Railway Documentation](https://docs.railway.app/)
