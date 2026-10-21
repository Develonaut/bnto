# Decision: Conditional Steps (`when` guards)

**Created:** October 2026
**Status:** Decided. The engine side shipped in keep-awake PR 3 (`feat/when-guard`); surfaces, scoped fields and OS support follow in PRs 4-8 ([keep-awake-recipe.md](../strategy/keep-awake-recipe.md)).
**Applies to:** the executor in `bnto-core`, so every consumer (CLI, TUI, WASM).

---

## The problem

The engine had no conditional execution. `always` (closing steps) was the only node-level control key. Making keep-awake run on Linux and Windows needed "run this step only on macOS". Without a general conditional, that would have meant a one-off `platforms` override on `shell-run`, which [generic-building-block-nodes.md](generic-building-block-nodes.md) flags: build the one generic piece instead.

## Options considered

| Option                                 | Examples                                                               | Verdict                                                                                                                                                                                                                                   |
| -------------------------------------- | ---------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Per-item router node (multi-output)    | n8n If/Switch, Node-RED switch, NiFi RouteOnAttribute, Make Router     | Dataflow tools route items through ports. bnto recipes are step pipelines authored in JSON, CLI and TUI (the editor is frozen), so a router would need port-aware edges everywhere. Kept as a possible future node next to `file-filter`. |
| Step guard (`if:` / `when:` on a step) | GitHub Actions `if: runner.os`, Ansible `when: os_family`, Argo/Tekton | **Chosen.** Per-OS steps are the textbook guard case, and the shape fits a step pipeline.                                                                                                                                                 |
| `platforms` override on `shell-run`    | —                                                                      | Rejected: one node's special case, and `platforms` already means runtime target (browser / cli / server).                                                                                                                                 |

## The decision

A node-level key, a sibling of `always`, never inside `parameters` (the processor's, which refuse undeclared keys):

```json
{ "id": "mac", "type": "group",
  "when": { "value": "{{ctx.platform}}", "op": "equals", "expected": "macos" },
  "nodes": [ … ] }
```

- **Comparison:** `op` is one of `equals`, `not-equals`, `after`, `before`, the same words and code as `assert` (`bnto-core/src/compare.rs`, shared). `not_equals` is accepted until assert's audit rename lands. `value` and `expected` take every template a param does: own fields, `{{ctx.*}}`, `{{env.*}}`, `{{node.<id>.*}}`, `{{item.*}}` in a loop. A template that is the whole string keeps its JSON type, so the guard compares the text of a number (`{{node.x.count}}` reads `3`, not `""`).
- **When it's decided:** in `execute_node`, before the node starts (`executor/guard.rs`). So once per call: per file under auto-iteration, per iteration inside a loop.
- **Skipping:** the node never starts. It emits `NodeSkipped { …, reason }` in place of `NodeStarted`, returns its input files unchanged and records nothing for `{{node.<id>.*}}`. A skipped container never starts its children.
- **`when` beats `always`:** a false guard skips even a closing step. A guard says whether a step belongs in this run at all; `always` says it survives an earlier failure.
- **if/else and switch** are guards, not new nodes: two steps with opposite guards, or several on one value.
- **Bad guards:** an unknown op fails the run when the node is reached; `bnto check` reports it first, along with a malformed placeholder in `value`/`expected` and `when` on an `input`/`output` node (which never runs). A guard without `value`, `op` or `expected` doesn't parse.

## Trust and pre-flight see every branch

Guards are decided at run time, so everything that runs before the first step ignores them on purpose:

- **Consent** lists every shell command in the recipe, guarded or not (SEC-16). A user approving keep-awake on macOS approves what it would run anywhere.
- **Raw param checks** (`executor/raw_params.rs`) refuse a bad step inside a branch for another OS, so a branch can't carry a refused step past pre-flight.
- Branches stay ordinary `nodes` children, so dependency checks, migration and `bnto check` walk them as before.

## Trade-offs

- **No expression language.** One comparison per guard. "macOS and display on" is a guarded group inside a guarded group. Revisit if real recipes need `and`/`or` often.
- **Skips are silent until PR 4.** The CLI, TUI and `--dry-run` don't show skipped steps yet. `bnto run --json` already carries `skip_reason` on a skipped top-level node.
- **Container fields come later (PR 5).** Until then a guard reads only its own node's fields, so an if/else pair on one field needs container-scoped fields.
