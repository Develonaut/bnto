# Use Case: Azure DevOps Team Dashboard

**Date:** 2026-02-21
**Status:** Concept — MVP use case for future implementation
**Phase:** Post-Phase 1 (requires cloud execution + http-request node)
**Persona:** Platform team lead managing cross-team work intake

---

## Persona

**Role:** Platform engineering team lead at a mid-to-large software org.

**Context:**
- Leads a team that operates as an enablement layer across multiple engineering teams
- Receives work requests from many avenues — direct asks, ADO boards, Slack, email, escalations
- Needs visibility into progress across efforts: test completion, story burndown, bug triage status
- Currently uses ADO dashboards but finds them clunky and hard to customize
- Lives in the terminal and uses Claude Code daily as part of their workflow
- Wants a tool they can configure once, run on demand, and share with teammates

**Pain points:**
- ADO built-in dashboards are rigid and hard to tailor to cross-team views
- No single view that aggregates work intake from multiple sources
- Reporting requires manual effort — pulling data, formatting, sharing
- Existing dashboards don't reflect the team's actual mental model of how work flows

---

## Scenario

The user wants to define a Bento workflow that:

1. **Authenticates** with Azure DevOps using a PAT token (stored via bnto secrets)
2. **Queries** multiple ADO projects/boards using WIQL to pull work items, test results, and sprint data
3. **Transforms** the raw API responses into a normalized shape suitable for reporting
4. **Outputs** a structured report — initially as JSON or CSV, eventually as a rendered dashboard view

### Example Flow

```
[http-request: ADO API] -> [transform: normalize work items]
[http-request: ADO API] -> [transform: normalize test results]
[group: merge datasets]  -> [transform: compute metrics]
                          -> [output: structured report]
```

### What This Exercises in Bnto

| Capability | Node Type | Notes |
|-----------|-----------|-------|
| API authentication | http-request | PAT token via Authorization Basic header, secret from bnto secrets |
| WIQL queries | http-request | POST to _apis/wit/wiql endpoint |
| Batch work item fetch | http-request + loop | WIQL returns IDs only; batch fetch details in groups of 200 |
| Data normalization | transform | Reshape ADO response into flat reporting structure |
| Metric computation | transform | Calculate completion percentages, velocity, triage counts |
| Multi-source aggregation | group | Combine results from multiple API calls |
| Parameterization | Config / template vars | Org URL, project names, query filters in the .bnto.json |

---

## MVP Definition

### Phase A: Single-Project Query

A .bnto.json that queries a single ADO project and outputs a JSON report:

- Fetch active work items for a given area path
- Fetch test run results for a given sprint
- Compute: total items, items by state, percent complete, test pass rate
- Output as report.json

**Success criteria:** Run bnto run ado-report.bnto.json and get a meaningful JSON file with real ADO data.

### Phase B: Multi-Project Dashboard

Extend to query multiple projects/teams and produce a comparative report:

- Configurable list of projects and queries in the .bnto.json
- Loop over projects, fetch and transform each
- Aggregate into a single report with per-project sections
- Output as JSON or CSV

**Success criteria:** One .bnto.json file produces a cross-team status report.

### Phase C: Rendered Output (Future)

Once Bnto has richer output capabilities:

- Render the report as formatted terminal output, HTML file, or web dashboard view
- Could integrate with Bnto Cloud web app as a report viewer execution result type
- The Control-inspired terminal aesthetic could live here as a themed output renderer

---

## ADO API Reference

**Base URL:** https://dev.azure.com/{organization}

**Key endpoints:**
- POST /{project}/_apis/wit/wiql?api-version=7.1 — Run a WIQL query (returns work item IDs)
- GET /_apis/wit/workitems?ids={ids}&fields={fields}&api-version=7.1 — Batch fetch work items (max 200)
- GET /{project}/_apis/test/runs?api-version=7.1 — List test runs
- GET /{project}/_apis/test/runs/{runId}/results?api-version=7.1 — Get test results
- GET /{project}/_apis/work/teamsettings/iterations?api-version=7.1 — Get sprint iterations

**Authentication:** Basic auth with PAT token — Authorization: Basic base64(:pat)

**Rate limits:** Approximately 30 requests/second per user. 429 responses include Retry-After header.

**WIQL note:** Queries return a max of 20,000 work item IDs. Actual field data requires a separate batch fetch call.

---

## Node Type Gaps

Current Bnto node types that need validation or enhancement for this use case:

| Need | Current Support | Gap |
|------|----------------|-----|
| HTTP POST with JSON body | http-request supports this | Verify WIQL POST body works |
| Auth header from secrets | http-request + bnto secrets | Verify secret interpolation in headers |
| Batch ID splitting | loop + transform | May need array chunking in transform expressions |
| JSON path extraction | transform | Verify deep object traversal in Go templates |
| Metric aggregation | transform | May need arithmetic in template expressions |
| File output | file-system write operation | Verify JSON serialization to file |

---

## Why This Use Case Matters

1. **Real-world validation** — Proves Bnto can handle API-driven data pipelines, not just file/image processing
2. **Dogfooding** — The Bnto team uses this daily, creating tight feedback loops
3. **Template potential** — ADO Project Dashboard becomes a compelling pre-built template in Bnto Cloud
4. **Demonstrates composability** — Shows how http-request + transform + loop compose into something useful
5. **Portable** — Teammates can run bnto run ado-report.bnto.json with their own PAT token immediately

---

## Related Documents

- [Cloud + Desktop Strategy](CLOUD_DESKTOP_STRATEGY.md) — Overall product architecture
- [PLAN.md](../PLAN.md) — Master implementation checklist
- [Bento Box Principle](../BENTO_BOX_PRINCIPLE.md) — Code organization philosophy
