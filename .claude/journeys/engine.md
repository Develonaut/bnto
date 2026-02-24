# Engine — User Journey Test Matrix

**Domain:** Go CLI engine — workflow execution, node types, validation, fixtures
**Status:** Phase 0 complete (>90% coverage). Maintained as new node types ship.
**Last Updated:** February 23, 2026

---

## Why This Matters

The engine is the product. Every web app feature, every cloud execution, every desktop run is a UI on top of `bnto run`. If the CLI doesn't work correctly, nothing above it matters. These journeys verify the engine from the user's perspective — what they'd experience if running bnto from the command line.

---

## Gate Map

Engine operations and their validation checkpoints.

| Step | What Happens | Validation Gate | Error Behavior |
|------|-------------|----------------|----------------|
| Load workflow | Parse `.bnto.json` | JSON schema validation | Clear error: "invalid workflow: [reason]" |
| Validate nodes | Check node types exist in registry | Registry lookup | Clear error: "unknown node type: [type]" |
| Validate connections | Check node inputs/outputs wire correctly | Connection validator | Clear error: "invalid connection: [details]" |
| Resolve paths | Resolve input/output file paths | Path resolver | Clear error: "file not found: [path]" |
| Execute nodes | Run each node in sequence/parallel | Context cancellation + per-node timeout | Clear error: "executing node [id]: [reason]" |
| Collect output | Gather results from all nodes | Output validation | Clear error: "node [id] produced no output" |

---

## Journey Matrix

### Core Execution

| # | Journey | Steps | Pass Criteria |
|---|---------|-------|---------------|
| **E1** | Simple workflow executes | `bnto run compress-images.bnto.json` with valid input | Exit code 0. Output files exist. File sizes smaller than input. |
| **E2** | Multi-node workflow executes | `bnto run` with workflow containing 2+ nodes in sequence | All nodes execute in order. Output from node N is input to node N+1. |
| **E3** | Parallel node execution | `bnto run` with parallel node group | All parallel branches execute. Results collected from all branches. |
| **E4** | Loop node iterates | `bnto run` with loop over multiple files | Each item processed. Output contains results for every input item. |
| **E5** | Nested group execution | `bnto run` with group → loop → node (3+ levels deep) | Recursive execution works. Output correct at every depth. |

### Tier 1 Fixtures (Predefined Bntos)

Each fixture is a real-world workflow that a user would run. These are the integration tests.

| # | Journey | Fixture | Pass Criteria |
|---|---------|---------|---------------|
| **E10** | Compress images | `compress-images.bnto.json` | Input PNGs/JPEGs compressed. Output files smaller. Quality within configured range. |
| **E11** | Resize images | `resize-images.bnto.json` | Output images match target dimensions. Aspect ratio preserved (if configured). |
| **E12** | Convert image format | `convert-image-format.bnto.json` | Input format converted to target format (e.g., PNG → WebP). Output files valid. |
| **E13** | Rename files | `rename-files.bnto.json` | Files renamed per pattern. No data loss. Original content preserved. |
| **E14** | Clean CSV | `clean-csv.bnto.json` | Empty rows removed. Whitespace trimmed. Headers normalized. Row count reduced. |
| **E15** | Rename CSV columns | `rename-csv-columns.bnto.json` | Column headers renamed per mapping. Data rows unchanged. |

### Validation & Error Handling

| # | Journey | Steps | Pass Criteria |
|---|---------|-------|---------------|
| **E20** | Invalid JSON rejected | `bnto run malformed.json` | Non-zero exit code. Error message mentions JSON parse failure. No panic. |
| **E21** | Unknown node type rejected | `bnto run` with `"type": "nonexistent"` | Non-zero exit code. Error message names the unknown type. No panic. |
| **E22** | Missing input file handled | `bnto run` with path to nonexistent file | Non-zero exit code. Error message includes the missing path. No panic. |
| **E23** | Cancellation respected | Start long workflow → cancel via context | Execution stops. Partial output cleaned up or clearly marked. No hang. |
| **E24** | Node error wrapped with context | Force a node-level error | Error message includes node ID and descriptive context, not bare "error". |

---

## Cross-Domain Dependencies

| Journey | Also Touches | Notes |
|---------|-------------|-------|
| E10-E15 | API (cloud execution) | Same fixtures run on Railway via Go HTTP API |
| E10-E15 | Web (tool pages) | Each fixture maps to a `/[bnto]` URL |
| E1-E5 | API (endpoint contracts) | API wraps CLI commands — same execution paths |

---

## Implementation Notes

- All tests via `go test -race` in `engine/pkg/` and `engine/tests/integration/`
- Fixtures in `engine/tests/fixtures/workflows/` and `engine/examples/`
- Golden test system compares output against known-good snapshots
- E10-E15 are the highest priority — they map directly to shipped product features
