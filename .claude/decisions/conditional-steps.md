# Decision: Conditional Steps (`when` guards)

**Created:** October 2026
**Status:** Decided. The engine side shipped in keep-awake PR 3 (`feat/when-guard`); surfaces shipped in PR 4, scoped fields in PR 5 and recipe OS support in PR 6 (#710), the web badge in PR 7 (#711) and OS-scoped dependencies in PR 8 (#712); keep-awake's Linux and Windows branches (PRs 10-11, #714-#715) are its first multi-OS recipe ([keep-awake-recipe.md](../strategy/keep-awake-recipe.md)).
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

- **Comparison:** `op` is one of `equals`, `not-equals`, `after`, `before`, the same words and code as `assert` (`bnto-core/src/compare.rs`, shared). `not_equals` is accepted until assert's audit rename lands. `value` and `expected` take every template a param does: the fields in scope (see below), `{{ctx.*}}`, `{{env.*}}`, `{{node.<id>.*}}`, `{{item.*}}` and `{{loop.*}}` in a loop. Inside a loop `{{item.*}}` is the item the step's files agree on: one file's, the loop's with none, and several files that give a key two values fail the step ([loop-item-current.md](loop-item-current.md), ISS-110). A template that is the whole string keeps its JSON type, so the guard compares the text of a number (`{{node.x.count}}` reads `3`, not `""`).
- **When it's decided:** in `execute_node`, before the node starts (`executor/guard.rs`). So once per call: per file under auto-iteration, per iteration inside a loop.
- **Skipping:** the node never starts. It emits `NodeSkipped { …, reason }` in place of `NodeStarted`, returns its input files unchanged and records nothing for `{{node.<id>.*}}`. A skipped container never starts its children.
- **`when` beats `always`:** a false guard skips even a closing step. A guard says whether a step belongs in this run at all; `always` says it survives an earlier failure.
- **if/else and switch** are guards, not new nodes: two steps with opposite guards, or several on one value.
- **Bad guards:** an unknown op fails the run when the node is reached; `bnto check` reports it first, along with a malformed placeholder in `value`/`expected` and `when` on an `input`/`output` node (which never runs). A guard without `value`, `op` or `expected` doesn't parse.

## Trust and pre-flight see every branch

Guards are decided at run time, so everything that runs before the first step ignores them on purpose:

- **Consent** lists every shell command in the recipe, guarded or not (SEC-16). A user approving keep-awake on macOS approves what it would run anywhere.
- **Raw param checks** (`executor/raw_params.rs`) refuse a bad step inside a branch for another OS, so a branch can't carry a refused step past pre-flight.
- Branches stay ordinary `nodes` children, so migration and `bnto check` walk them as before. Dependency checks are the one exception: they skip a branch that can't run on this OS (see "Dependencies in scope" below), since a missing tool there can never matter.

## Trade-offs

- **No expression language.** One comparison per guard. "macOS and display on" is a guarded group inside a guarded group. Revisit if real recipes need `and`/`or` often.
- **Where skips show (PR 4).** The CLI prints `skipped: <reason>` in place of a step's line; the TUI marks the step `– (skipped)`; consent lines carry `(when …)` for every guard on a step's path; `bnto dry-run` marks commands skipped on this host when it can decide now (guards on `{{node.*}}` or `{{item.*}}` wait for the run); `bnto run --json` carries `skip_reason`. The web editor keeps `when` (and `always`) on load and save, and settles a skipped node to idle.

## Fields in scope (PR 5)

An if/else pair on one field needs both steps to read it, so a container's `fields` reach every node inside it, at any depth. The nearest declaration of a name wins: a step's own field shadows its group's, an inner group's shadows an outer one's, even a declaration with no default. A group's own `when` reads its own fields. Nothing leaks out: a step after the group doesn't see them.

- **One rule everywhere:** `bnto_core::executor::field_scope` (`declare`, `along`) computes what a node sees. The executor carries it down the `PipelineContext` (`fields`), and `bnto check` and the CLI's dry run (`host_skip`, shell previews) walk the same path, so a preview reads what the run reads.
- **`--param key=value`** sets the node that declares field `key`, wherever it is (`bnto/src/field_owner.rs`), and falls back to a raw param on the first processing step when none does. When two nodes declare `key`, the run stops before anything happens and asks for `nodeId:key=value`, naming both. No bundled recipe declares a field twice.
- **Unfilled fields (ISS-1, #741):** a `{{fields.*}}` that no node in scope declares, or whose field has no value, stops the run before any step instead of reaching it as text (`bnto_core::missing_fields`). Guards don't hide one. `bnto check` reports the undeclared ones; a declared field with no value is left to the run (`-p`, the TUI form). `bnto run` and `bnto dry-run` refuse before consent and name the `-p` that fills it (#742).
- **Unchanged:** SEC-13 still checks every declaration's value before the run (`field_values.rs`). The TUI form lists each field once, on the node that declares it, and sends `nodeId:key`.
- **Considered:** merging every node's fields into one recipe-wide set. Rejected: two unrelated steps with a `format` field would clash, and a recipe-level surface is a separate, deferred decision ([recipe-fields.md](../strategy/recipe-fields.md)).

## OS support (PR 6)

Which OSes a recipe runs on is worked out from its guards, never written by hand, so it can't drift from what the recipe does.

- **Platform guard:** a guard whose `value` is exactly `{{ctx.platform}}`, whose `op` is `equals` or `not-equals`, and whose `expected` has no template. Anything else (a field, a loop item, a reversed or padded comparison, an ordering op) might hold on any OS, so it rules none out. `HostPlatform::as_str` is both what `{{ctx.platform}}` reads and what a guard is compared with, through the same `compare` the run uses.
- **The rule** (`bnto_core::os_support`): `runs_on(node, os)` holds when the node's guard allows `os` and the node is a primitive or holds a step that `runs_on(os)`. Guards nest: a Linux group inside a "not Windows" group runs on Linux only. A recipe's `os` is the OSes on which some root step runs, in macOS, Linux, Windows order. A recipe with no steps at all runs everywhere; an empty container counts for nothing.
- **Where it shows:** `bnto info` prints `Runs on:`; `bnto list` and the TUI detail header tag a recipe that misses an OS (`[macOS]`); the catalog snapshot carries `os` for each built-in, generated into `GeneratedRecipe.os` and `Recipe.os` in `@bnto/registry` (optional on `Recipe`: user recipes aren't analysed in TS). On bnto.io (PR 7), a CLI recipe limited to some OSes carries a "macOS only" badge on its page and cards, and every CLI recipe's JSON-LD claims its `operatingSystem`; browser recipes show nothing (they run anywhere a browser does).
- **Unsupported host:** `bnto run` checks before anything else, so before consent and before any step: "keep-awake runs on macOS. This machine runs Linux." It exits 1 through the usual verdict (`--json` carries the message). A host bnto has no name for (a BSD) and a definition that doesn't parse are let through, to fail their own way.
- **The TUI and `bnto dry-run` refuse too** (follow-up), with the same message: the TUI's run stops before preparing inputs and shows it as the run's error, and `bnto dry-run` exits 1 without printing a preview of a run that would be refused.
- **`bnto check` (follow-up):** it walks the same platform guards (`check/os_scope.rs`), narrowing the values `{{ctx.platform}}` can read (`macos`, `linux`, `windows`, `unknown`) at each one. It reports a guard expecting a value it never reads (`darwin`, `macOS`), a step its guards rule out of every OS (so a recipe that runs nowhere fails check), and, through `NodeProcessor::platform_problems`, what a node's params break on the platforms it can reach: shell-run's `powershell` off Windows.

## Dependencies in scope (PR 8)

A tool that ships with one OS (keep-awake's `caffeinate` and `osascript`) can't be installed on the others, so checking it everywhere failed `bnto doctor` on Linux and Windows for good (fix B dropped them until now).

- **`requires` on any node**, not only the root: the same `Dependency` list, beside `when` and `always`. The tracker planned it for containers only. It went on every node because a step's own `requires` is the most precise scope, and `runs_on` already treats a step and a group the same way, so limiting it would only have added a refusal.
- **The rule** (`bnto_engine::scoped_deps`): on an OS, a recipe needs its root `requires` and, for every node that `runs_on` that OS, the node's own `requires` and its processor's. A node ruled out takes its whole subtree with it, so guards nest the way `runs_on` does. A guard on anything but `{{ctx.platform}}` rules no OS out. A host bnto has no name for (a BSD) needs everything. **Root `requires` behave exactly as before:** needed on every OS, even one the recipe doesn't run on.
- **One binary, two declarations** (a macOS group and a Linux group each asking for `ffmpeg` with their own hint) stay two entries, one per OS; on each OS the first one listed wins, as before.
- **Where it shows:** the run's pre-flight, `bnto install` (one recipe or `--all`) and `bnto dry-run` check only what this OS needs. `bnto doctor` checks those and lists the rest as `skip caffeinate (macOS only)`; a dependency checked here that some OS doesn't need carries the same note (`ok caffeinate (macOS only)`). `bnto info` lists every dependency with its note, and for one this OS doesn't need, shows the install hint for an OS that does.
- **Install hints:** a bundled dependency needs a per-OS hint (`installHints`) for every OS that needs it (`every_bundled_dependency_installs_on_every_os_that_needs_it`), not for all three.
- **Considered:** an OS list on `Dependency` itself (`"platforms": ["macos"]`). Rejected: it would say again what the guards already say, and could drift from them.
- **`requires` on an `input`/`output` node** is a `bnto check` finding (follow-up), like `when` and `always` there.
