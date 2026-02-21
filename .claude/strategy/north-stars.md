# North Stars

**Last Updated:** February 2026

---

## Why This Document Exists

Bnto doesn't exist in a vacuum. The ideas, values, and design decisions behind it are shaped by platforms and tools that came first and got things right. This document captures who we look up to, what specifically we admire, and what we carry forward into our own work.

We're standing on the shoulders of giants. This is our acknowledgment of that.

---

## n8n

**What they are:** Open source workflow automation platform (self-hostable alternative to Zapier). 50K+ GitHub stars.

**What we admire:**

- **Fair-code open source model.** n8n pioneered the "fair-code" approach -- source-available, self-hostable, with a sustainable license that protects against cloud providers reselling the product. They proved you can be genuinely open while building a real business. We take this further with MIT licensing -- simpler, more trusted, and our moat is the hosted service, not the license
- **Monorepo engineering excellence.** n8n's monorepo structure directly inspired our package layout. Internal packages scoped under `@bnto/`, clean separation of concerns, shared configs -- we borrowed this pattern because it works beautifully at scale
- **Community-powered ecosystem.** n8n's community builds integrations, shares workflows, and helps each other. The platform is better because it's open. Contributors feel ownership. That's the dynamic we want -- developers building node types and sharing workflow templates, not just consuming them
- **Self-host option builds trust.** Even if most users use the hosted version, the *option* to self-host signals "we're not holding your data hostage." Bnto's local-first architecture (Go CLI + Wails desktop) makes this even stronger -- your workflows run on your machine by default, cloud is optional
- **Visual workflow canvas.** n8n's node-based canvas editor is the gold standard for visual workflow building. Clean, intuitive, with clear data flow visualization. Our visual editor (Phase 4) should meet this bar

**What went right and what we'd do differently:** n8n's TypeScript-only architecture means the execution engine is Node.js. This works but limits performance for CPU-intensive workflows and makes deployment heavier. Bnto uses Go for the engine -- faster execution, single binary deployment, no runtime dependencies. The TypeScript frontend layer consumes the Go engine through adapters.

**What we carry forward:** Monorepo architecture patterns. The proof that open source workflow automation builds massive community. Community as co-builders. The visual canvas as the destination for workflow editing.

---

## Make (formerly Integromat)

**What they are:** Visual workflow automation platform. The most visually intuitive workflow builder in the market.

**What we admire:**

- **Scenario builder UX.** Make's circular module design and visual data flow is arguably the most intuitive workflow builder ever made. Each node shows its status, data preview, and configuration inline. The learning curve is remarkably gentle for a powerful tool
- **Template ecosystem.** Make's template library makes it trivial to get started. Browse templates by use case, click to install, customize. This is the onboarding pattern we want -- don't start from scratch, start from a working template
- **Data mapping visualization.** Make shows exactly what data flows between nodes. You can see the shape of data at every connection point. This eliminates the "what fields are available?" guessing game that plagues other tools
- **Error handling as first-class UX.** When a workflow fails, Make shows exactly which node failed, what data it received, and what went wrong. Error recovery is built into the visual model, not hidden in logs

**What we'd do differently:** Make is closed source and cloud-only. No self-hosting option. Their pricing scales steeply with operations count. Bnto runs locally by default (free, unlimited) with cloud as a paid convenience.

**What we carry forward:** The principle that workflow building should be visual and intuitive. Data flow must be visible. Error handling must be first-class UX, not an afterthought.

---

## Zapier

**What they are:** The workflow automation platform with the largest integration catalog. 7,000+ app integrations.

**What we admire:**

- **Integration breadth.** Zapier's massive app directory proves that the value of a workflow tool scales with what it can connect to. Every new integration multiplies the possible workflows. Our node type registry should be designed to scale similarly
- **"Zap" as a concept.** The naming -- Zaps, triggers, actions -- made workflow automation accessible to non-technical users. The vocabulary itself lowers the barrier. Bnto's `.bnto.json` files serve a similar purpose -- the format should be readable and approachable
- **Templates by use case.** "When I get an email with an attachment, save it to Google Drive" -- Zapier's template descriptions are task-oriented, not technology-oriented. Users find workflows by what they want to accomplish, not by what APIs they need

**What we'd do differently:** Zapier is entirely cloud-based with no local execution. It's also not open source, and pricing scales per task. Bnto's Go engine runs locally with zero cloud dependency. Cloud execution is optional.

**What we carry forward:** Integration breadth as a strategic goal. Task-oriented template descriptions. The principle that workflow automation should be accessible to non-developers.

---

## Temporal

**What they are:** Open source (MIT) durable execution platform. Go-based workflow orchestration for distributed systems.

**What we admire:**

- **Go-based workflow orchestration.** Temporal proved that Go is the right language for workflow engines. Type safety, concurrency primitives, single binary deployment, excellent performance. Bnto's engine is Go for the same reasons
- **Durable execution model.** Temporal's key insight: workflows should survive crashes. State is persisted at every step, and execution can resume from where it left off. While Bnto's workflows are simpler (no long-running distributed transactions), the principle of reliable execution with clear state tracking is foundational
- **SDK design.** Temporal's Go SDK is clean -- activities, workflows, workers with clear separation of concerns. The API surface is small and composable. Our engine's package design (registry, engine, node types) follows similar principles
- **MIT license, sustainable business.** Temporal Cloud is a $100M+ business built on top of an MIT-licensed open source project. Proof that open source + hosted service works at scale in the workflow space specifically

**What we'd do differently:** Temporal is designed for distributed systems engineers building microservice orchestration. It's powerful but complex -- the learning curve is steep for simple automations. Bnto targets the simpler end of the spectrum: file processing, data transformation, HTTP requests, image manipulation. `.bnto.json` files should be readable by anyone.

**What we carry forward:** Go as the engine language. Reliable execution with clear state. MIT licensing with hosted service monetization. Clean SDK design with separation of concerns.

---

## GitHub Actions

**What they are:** CI/CD and workflow automation built into GitHub. YAML-based workflow definitions.

**What we admire:**

- **Workflow-as-code in the repo.** `.github/workflows/*.yml` files live alongside your code. Version controlled, reviewable, portable. Bnto's `.bnto.json` files follow the same philosophy -- workflow definitions are data files that live in your project, not locked in a cloud dashboard
- **Marketplace of actions.** GitHub Actions' marketplace lets you compose workflows from community-built steps. `uses: actions/checkout@v4` is a one-line integration. Our node type registry should enable similar composability
- **Event-driven triggers.** Push, PR, schedule, webhook, manual dispatch -- GitHub Actions supports diverse trigger types cleanly. Bnto should support similar trigger diversity for workflow execution
- **Matrix builds and reusable workflows.** The ability to parameterize and reuse workflow definitions is powerful. `.bnto.json` files should support parameterization and composition similarly

**What we'd do differently:** YAML is notoriously error-prone (indentation bugs, type coercion). JSON with a schema (`.bnto.json` + JSON Schema validation) is more explicit and tool-friendly. Also, GitHub Actions is tied to GitHub -- Bnto workflows are portable and run anywhere.

**What we carry forward:** Workflow definitions as version-controlled files. Community-contributed node types/actions. Diverse trigger support. Parameterization and reusability.

---

## What These North Stars Have in Common

The pattern across all five:

1. **Workflows as the atomic unit** -- every platform centers on defining, executing, and managing automated workflows
2. **Visual comprehension** -- the best tools make data flow visible and workflow state obvious at a glance
3. **Open ecosystems win** -- the most successful tools have community-contributed integrations, templates, and extensions
4. **Local + cloud spectrum** -- users want the option to run locally (fast, free, private) with cloud as a convenience layer
5. **Simplicity scales** -- the tools that win long-term are the ones that stay approachable as they add power

Bnto sits at the intersection of all five: n8n's open source community and monorepo patterns, Make's visual intuition and UX excellence, Zapier's integration breadth and accessibility, Temporal's Go engine reliability and MIT licensing, and GitHub Actions' workflow-as-code portability -- all in a tool that runs locally first with cloud as an option.

---

*When making product decisions, ask: "Would this make our North Stars proud, or would it make us more like the tools we're trying to improve upon?"*
