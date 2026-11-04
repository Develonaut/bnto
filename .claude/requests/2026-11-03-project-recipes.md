# Request: a project's own recipes, listed and run from anywhere inside it

**Received:** November 3, 2026, from the maintainer (dogfooding: bnto as a project's task runner). **Evaluated:** November 3, 2026, per [feature-requests.md](../scopes/process/feature-requests.md).
**Source:** a conversation with the maintainer. Every result below was run in a sandbox, with a temp `HOME` and `BNTO_HOME`, on bnto 0.19.0.

## 1. Run `bnto tui` inside a project and see that project's recipes

- **Outcome:** from any folder inside a project, `bnto tui` (and `bnto list`, `bnto run <slug>`) shows and runs the recipes kept in the project, beside the library and the built-ins.
- **Verdict:** missing generic piece. It was already planned ([dogfooding.md](../strategy/dogfooding.md#project-recipes-7a-7j), then 7a-7d).
- **Today:** an example project with three recipes in `.bnto/recipes/` runs by path from its root (`bnto run .bnto/recipes/optimize-images.bnto.json`: 2 WebP files written), and `bnto tui <file>` opens one recipe. There's no list of them, and from `src/` the same recipe lists nothing, exits 0 and leaves an empty `src/public/images/`. `BNTO_HOME=./.bnto` shows them, but moves config, approvals and logs into the project and hides the library (and turns out to be a trust hole: ISS-126).
- **Defects found:** [ISS-122](../ISSUES.md#iss-122-file-collects-pattern-treats-ab-literally) while building the example; [ISS-123](../ISSUES.md#iss-123-file-list-uses-an-absolute-base-as-written), [ISS-124](../ISSUES.md#iss-124-bnto-install-hands-a-recipes-binary-to-the-package-manager-unchecked), ISS-125 (fixed in step 7c), [ISS-126](../ISSUES.md#iss-126-a-relative-bnto_home-resolves-against-the-current-folder), ISS-127 (fixed in step 7b) from the design's security review and cold reviews.
- **Generic piece:** project recipes in `.bnto/recipes/`, found by walking up, run at the project root, consent on first run, never shadowing. Prior art: npm `scripts` (package root), Task (Taskfile folder), `just` (justfile folder), `.github/workflows`. Design: [project-recipes.md](../decisions/project-recipes.md).
- **Won't build:** a project `config.toml`, a `--project` flag, overriding built-ins from a project, editing project recipes in the TUI (yet). Reasons in the design's "Won't build".
- **Tracked in:** [project-recipes.md](../decisions/project-recipes.md) (steps 7a-7j, branch `feat/project-recipes`); [ISSUES.md](../ISSUES.md) batch 5e.

## Reply

**Security first:** don't point `BNTO_HOME` at a project's `.bnto/` folder (`BNTO_HOME=./.bnto`). A relative `BNTO_HOME` makes that folder bnto's home, its `state/trusted.json` the trust store and its recipes trusted library recipes ([ISS-126](../ISSUES.md#iss-126-a-relative-bnto_home-resolves-against-the-current-folder)).

1. **Works today:** run a project's recipes by path from its root, `bnto run .bnto/recipes/<name>.bnto.json`, or open one in the TUI with `bnto tui .bnto/recipes/<name>.bnto.json`. Run them from the root: relative paths resolve against the current folder.
2. **Bugs found:** a `{a,b}` pattern in `file-collect` matches nothing (ISS-122; list the extensions with `file-filter`'s regex mode instead), and five trust and path bugs that the feature fixes (ISS-123 to ISS-127).
3. **Will add:** a `.bnto/recipes/` folder in the project, found from any subfolder, listed in `bnto list` and a TUI Project pane, and run at the project root so relative paths mean the same thing wherever you start. Each project recipe asks before its first run, showing where it came from and every folder it touches.
4. **Won't add:** project recipes overriding a built-in or library recipe, a project `config.toml`, a `--project` flag, or editing project recipes in the TUI (yet). See the design's "Won't build".
