# ADO Dashboard — Use Case & Concept

**Date:** 2026-02-21
**Status:** Concept — Ready for Planning
**Related:** [CLOUD_DESKTOP_STRATEGY.md](CLOUD_DESKTOP_STRATEGY.md), [PLAN.md](../PLAN.md)

---

## Summary

Build an Azure DevOps (ADO) dashboard as a real-world use case that exercises and showcases Bnto's workflow automation capabilities. Instead of building a standalone CLI tool, the ADO dashboard is implemented as **Bnto workflows** — `.bnto.json` files that use existing and new node types to fetch ADO data, transform it, and produce reports.

This serves three purposes:

1. **Dogfooding** — Uses Bnto to solve a real problem the team has (ADO reporting and triage)
2. **Showcase** — Demonstrates Bnto's power for data pipelines and API integrations
3. **Template library** — The resulting `.bnto.json` files become pre-built templates other users can adapt

---

## The Problem

The platform team at work manages incoming requests from multiple avenues (different ADO projects, boards, backlogs). Current pain points:

- ADO's built-in dashboards are clunky and don't surface the data the way the team needs
- No consolidated view across multiple request sources for triage
- Manual effort to pull progress data (test completion, effort tracking, sprint progress)
- No easy way to generate quick reports with charts showing status at a glance

The team needs a way to:

- Pull work items, test results, and sprint data from ADO's REST API
- Transform and aggregate that data into meaningful metrics
- Visualize progress with bar charts, progress bars, and summary tables
- Configure different "dashboards" for different reporting needs
- Distribute this to teammates who can authenticate and run it themselves

---

## How This Maps to Bnto

Rather than building a bespoke application, the ADO dashboard is a **set of Bnto workflows** that chain together API calls, data transformations, and output formatting. This is exactly the kind of multi-step automation Bnto was designed for.

### New Node Types Needed

| Node Type | Purpose | Priority |
|-----------|---------|----------|
| `ado` | Azure DevOps API client — authenticate via PAT, run WIQL queries, fetch work items, test results, build status | High |
| `report` | Output formatter — takes transformed data and produces structured output (Markdown tables, ASCII charts, JSON summary) | Medium |
| `aggregate` | Data aggregation — group-by, count, sum, average, percentage calculations on collections | Medium |

### Leveraging Existing Node Types

| Existing Node | Role in ADO Dashboard |
|---------------|----------------------|
| `http-request` | Fallback for any ADO REST endpoints not covered by the `ado` node |
| `transform` | Reshape API responses, extract fields, compute derived values |
| `loop` | Iterate over work item collections, process each item |
| `group` | Orchestrate the fetch → transform → report pipeline |
| `parallel` | Fetch from multiple ADO projects/boards simultaneously |
| `edit-fields` | Add computed fields to work item data (e.g., days since created) |

### Example Workflow: Sprint Progress Dashboard

```
group "Sprint Progress Dashboard"
├── ado "Fetch Sprint Work Items"           → WIQL query for current sprint
├── aggregate "Calculate Progress"          → Group by state, count, percentages
├── transform "Format for Charts"           → Shape data for bar chart output
└── report "Generate Dashboard"             → Produce formatted output with charts
```

### Example `.bnto.json` (Conceptual)

```json
{
  "id": "sprint-dashboard",
  "type": "group",
  "name": "Sprint Progress Dashboard",
  "nodes": [
    {
      "id": "fetch-sprint-items",
      "type": "ado",
      "parameters": {
        "operation": "wiql",
        "query": "SELECT [System.Id], [System.Title], [System.State], [System.WorkItemType] FROM WorkItems WHERE [System.IterationPath] = @CurrentIteration AND [System.TeamProject] = '{{.project}}'",
        "fields": ["System.Id", "System.Title", "System.State", "System.WorkItemType", "System.AssignedTo"]
      }
    },
    {
      "id": "aggregate-by-state",
      "type": "aggregate",
      "parameters": {
        "input": "{{index . \"fetch-sprint-items\" \"workItems\"}}",
        "groupBy": "System.State",
        "operations": ["count", "percentage"]
      }
    },
    {
      "id": "generate-report",
      "type": "report",
      "parameters": {
        "format": "terminal",
        "title": "Sprint Progress — {{.project}}",
        "sections": [
          {
            "type": "bar-chart",
            "data": "{{index . \"aggregate-by-state\" \"groups\"}}",
            "labelField": "key",
            "valueField": "count"
          },
          {
            "type": "table",
            "data": "{{index . \"fetch-sprint-items\" \"workItems\"}}",
            "columns": ["System.Id", "System.Title", "System.State", "System.AssignedTo"]
          }
        ]
      }
    }
  ]
}
```

---

## Configuration

Dashboard configurations live in YAML files that define which ADO projects, queries, and visualizations to use. The Bnto workflow references the config, and the config can be swapped out per team or project.

```yaml
# ~/.config/bnto/ado-dashboards.yaml
dashboards:
  sprint-progress:
    name: "Sprint Progress"
    project: "MyProject"
    team: "Platform Team"
    iteration: "@CurrentIteration"
    charts:
      - type: bar
        metric: work-item-state-distribution
      - type: progress
        metric: test-completion-percentage

  triage-queue:
    name: "Triage Queue"
    sources:
      - project: "ProjectA"
        query: "SELECT ... WHERE [System.State] = 'New'"
      - project: "ProjectB"
        query: "SELECT ... WHERE [System.State] = 'New'"
    sortBy: "System.CreatedDate"
    groupBy: "source"
```

---

## Authentication

ADO authentication uses Personal Access Tokens (PATs) managed through Bnto's existing secrets system:

```bash
bnto secrets set ADO_PAT "your-personal-access-token"
bnto secrets set ADO_ORG "https://dev.azure.com/your-org"
```

The `ado` node reads these from the secrets store at runtime. No credentials in workflow files.

---

## Distribution Story

This is where the Bnto model really shines:

1. **Author creates dashboard workflows** — `.bnto.json` files + YAML config
2. **Teammate installs Bnto** — single binary, `brew install bnto` or download
3. **Teammate authenticates** — `bnto secrets set ADO_PAT ...`
4. **Teammate runs dashboards** — `bnto run sprint-dashboard.bnto.json`

No web server, no Docker, no complex setup. Just a binary and a config file. This is the portable CLI story that motivated the project — it just happens to use Bnto as the engine instead of a bespoke tool.

---

## Phasing

### Phase A: `ado` Node Type

- Implement `ado` node in `engine/pkg/node/library/ado/`
- Operations: `wiql` (query), `getWorkItems` (batch fetch), `getTestRuns`, `getBuildStatus`
- Authentication via Bnto secrets (`ADO_PAT`, `ADO_ORG`)
- Use Microsoft's ADO REST API directly (Go `net/http`, no SDK dependency)
- WIQL query returns IDs → batch fetch full work items (200 per request max)
- Unit tests with mock HTTP server
- Integration test with a fixture `.bnto.json`

### Phase B: `aggregate` Node Type

- Implement `aggregate` node in `engine/pkg/node/library/aggregate/`
- Operations: `groupBy`, `count`, `sum`, `average`, `percentage`, `sortBy`
- Works on any collection of objects (not ADO-specific)
- Unit tests with sample data sets

### Phase C: `report` Node Type

- Implement `report` node in `engine/pkg/node/library/report/`
- Output formats: `terminal` (ASCII charts + tables), `markdown`, `json`
- Chart types: bar chart (Unicode blocks), progress bar, table
- Terminal output uses ANSI colors for the styled look
- Markdown output for sharing in ADO wikis, Slack, etc.

### Phase D: Dashboard Templates

- Create 3-5 example `.bnto.json` dashboard workflows
- Sprint progress, triage queue, test completion, build status, effort burndown
- YAML config schema for dashboard definitions
- Add to `engine/examples/` as showcase templates
- Documentation for how to customize and extend

---

## Aesthetic Note

The original vision included a "Control" (Remedy game) terminal aesthetic — black backgrounds, amber text, retro computer boot sequences. While the TUI approach was abandoned in favor of Bnto workflows, the `report` node's terminal output format can still incorporate this aesthetic:

- ANSI color theming (amber `#FFB000` on black)
- Unicode box-drawing characters for panels and borders
- Styled headers and section dividers
- Optional "boot sequence" output mode for dramatic flair

This is a nice-to-have that can be layered on after the core functionality works.

---

## Relationship to Bnto Roadmap

This use case doesn't block or conflict with the existing Phase 1-4 roadmap. It's an **adjacent effort** that:

- Adds 3 new node types to the engine (benefits all users)
- Creates showcase templates (benefits the template library for Phase 1 cloud)
- Dogfoods the workflow system on a real problem (surfaces engine bugs/gaps)
- Could become a featured "ADO Integration" section in Bnto's marketing

The `ado` node type is ADO-specific, but `aggregate` and `report` are **general-purpose** nodes that benefit any data pipeline workflow. They fill gaps in the current node library around data summarization and formatted output.

---

## Claude Code Prompt

Use the following prompt to have Claude Code review this document, validate it against the existing architecture, and integrate it into the repo documentation:

```
Read the following files in order before doing anything else:

1. .claude/CLAUDE.md
2. .claude/BENTO_BOX_PRINCIPLE.md
3. .claude/PLAN.md
4. .claude/strategy/CLOUD_DESKTOP_STRATEGY.md
5. .claude/strategy/MONOREPO_STRUCTURE.md
6. .claude/strategy/ADO_DASHBOARD_USE_CASE.md

I've added a new strategy document at .claude/strategy/ADO_DASHBOARD_USE_CASE.md that describes an Azure DevOps dashboard use case for Bnto. This was developed through a research and planning conversation and represents a real-world use case that exercises Bnto's workflow engine.

Please do the following:

1. **Review the ADO use case document** against the existing architecture docs. Flag any conflicts with the layered architecture, Bento Box Principle, or current roadmap. Suggest corrections if anything doesn't align.

2. **Validate the proposed node types** (`ado`, `aggregate`, `report`) against the existing node type patterns in `engine/pkg/node/library/`. Confirm they follow the same structure (definition.go, executable interface, unit tests). Note any implementation concerns.

3. **Update PLAN.md** to add the ADO Dashboard phases (A through D) as a new section. Position it as a parallel effort that doesn't block Phase 1-4. Label it something like "Parallel Track: ADO Dashboard Use Case" and add it after the existing phases.

4. **Review the example .bnto.json structure** in the use case doc. Confirm the parameter patterns match how existing nodes define their parameters. Flag anything that would require engine changes beyond adding new node types.

5. **Do NOT implement any code yet.** This is a planning and documentation task only. The goal is to get the strategy doc reviewed, validated, and integrated into the project plan so we can implement it methodically in future sessions.

After completing the review, summarize what you found and any recommendations.
```
