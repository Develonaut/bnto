# Request: an index of a folder tree, written to a folder an env value names

**Received:** October 24, 2026, from a consumer that pins bnto by commit. **Evaluated:** October 24, 2026, per [feature-requests.md](../scopes/process/feature-requests.md).
**Source:** a three-item request file the consumer sent to Ryan (not in the repo). Every result below was rerun in a sandbox, with a temp `HOME` and `BNTO_HOME`, against `main` at #745.

The consumer builds a text index of a large folder tree:

- a heading for each top folder and second-level folder, with a file count and total size;
- one line for each file at the third level;
- one rolled-up line for each deeper subtree.

The recipe reads sizes from disk metadata only, and writes the index into a folder named by an environment value.

## 1. A field holding the first N `/`-separated segments of an item's relative name

`a/b/c/d.bin` gives `a` at N=1, `a/b` at N=2 and `a/b/c` at N=3. A shallower path, or a top-level file, comes back whole. Backslashes come out as `/`.

- **Verdict:** doable today, clumsily.
- **Today:** six nodes after `file-collect` with `flatten: false`, two of them renaming files just to compute one field. `file-rename`'s regex `find` runs on the whole relative path's stem (`bnto-file/src/rename/transform.rs:54-59`). The steps:
  1. `file-inspect`, which sets `filename` (the relative name) and `folder`.
  2. `edit-fields` saves `rel: {{item.filename}}`.
  3. `file-rename` with `find: ^([^/]+(?:/[^/]+){0,N-1})(?:/.*)?$` and `replace: $1/_`.
  4. `file-inspect` again, whose `folder` is now the leading part.
  5. `edit-fields` saves `lead: {{item.folder}}`.
  6. `file-rename` with `pattern: {{item.rel}}` restores the name. Then `summarize` with `groupBy: lead`.
  - **`flatten: false` is required.** `file-collect` defaults to `flatten: true`, which strips the folders, so every file becomes its own group (`x`, `y`, `d`, `top`, `r`) with no error.
  - **Edge case:** a path shallower than N loses its extension (`top`, not `top.bin`).
  - **Edge case ([ISS-23](../ISSUES.md#iss-23-file-rename-splits-the-extension-on-a-dot-anywhere-in-the-path)):** a file with no extension in a dotted folder lands in the wrong group: `v1.2/README` groups as `v1/_.2`, not `v1.2`.
  - Windows names are fine, since `file-collect` keeps folders with `slash_path` when `flatten` is off.
  - Without step 1, step 2 fails: the item has no `filename` field yet.

  The recipe at N=1 is in the reply below. Over `tree/` holding `a/x.bin`, `a/b/y.bin`, `a/b/c/d.bin`, `top.bin` and `q/r.bin`, `bnto check lead.bnto.json` passes and `bnto run lead.bnto.json --yes` exits 0. The `report` step prints:

  ```
  a: 3
  top: 1
  q: 1
  ```

- **Defects found:** [ISS-23](../ISSUES.md#iss-23-file-rename-splits-the-extension-on-a-dot-anywhere-in-the-path) (`file-rename` splits the extension anywhere in the path), [ISS-24](../ISSUES.md#iss-24-edit-fields-leaves-file-as-text) (`edit-fields` leaves `{{file}}` as text).
- **Generic piece:** **template filters** (already in [PLAN.md's backlog](../PLAN.md#engine-from-the-october-21-dogfood-sessions)): `{{item.filename | split('/') | first(2) | join('/')}}`. Prior art: n8n `.split('/').slice(0,2).join('/')`, NiFi `getDelimitedField`, Zapier Text > Split, and Liquid or Jinja `split`/`join`.
  - Jinja's `first` takes no count. Settle `first(n)` against `regex_replace` (Ansible-style) in the `/workflow-expert` review.
- **Won't build:** a `top-folder` node or a `depth` param on `file-inspect`. Each serves one use case, and no comparable tool has one.
- **Tracked in:** PLAN.md → Template filters (acceptance cases added).

## 2. A byte count written as a readable size, with a fixed unit and decimals

At base 1024 with GB and 2 decimals, 3000000000 gives `2.79 GB`; with 1 decimal, 16000000000 gives `14.9 GB`. 1500 and 0 both give `0.00 GB`. Text input (`"3000000000"`) reads as a number, and a non-number fails, naming the item.

- **Verdict:** missing generic piece.
- **Today:** `summarize` gives `sum_<field>` as a raw number, and no node divides, rounds or formats a number. `transform` has no processor. Stopgap: `shell-run` with `numfmt` or `awk`.
- **Generic piece:** a **`filesizeformat` template filter**, Jinja's own name, with Django and Python `humanize` as prior art. Its options:
  - `binary` (base 1024, as Jinja has);
  - `unit` (fixed, so columns line up);
  - `decimals`.

  `{{item.sum_size | filesizeformat(binary=true, unit='GiB', decimals=2)}}`. It reads numbers the way `summarize` does (`summarize/total.rs:1-6`), and replaces the CLI's hand-rolled size formatter (`bnto/src/tui/format.rs:19`). tonkotsu keeps its own (`tonkotsu/src/format_size.rs:4`): it's a standalone crate with no `bnto-core` dependency.

- **For review:** the request's `2.79 GB` is base 1024. Strict labels give `2.79 GiB`, or `3.00 GB` at base 1000. A desktop file manager's label is one platform's opinion, so the filter shouldn't default to it silently.
- **Won't build:** a "human size" node, or a `format` param on `summarize`. Formatting is a template concern: `file-render` and `report` need it too.
- **Tracked in:** PLAN.md → Template filters.

## 3. The output node's `directory` with `{{env.*}}`

- **Verdict:** defect, a silent wrong result like ISS-1 (fixed in #741).
- **Today**, with `secrets: [{"key": "SOME_KEY"}]`, `directory: {{env.SOME_KEY}}` and `SOME_KEY` set:
  - `bnto check` passes.
  - `bnto run` exits 0 with "1 file written to {{env.SOME_KEY}}/", and the file lands in a folder literally named `./{{env.SOME_KEY}}/`.
  - With `SOME_KEY` unset, the run is refused ("Missing required secrets") but the folder is still created first.
  - Other leftovers are dropped too: `out-{{ctx.nope}}-{{item.x}}` writes to `out--{{item.x}}/`.
  - The consumer writes with `file-copy` meanwhile, whose `destination` already fills `{{env.*}}` from declared secrets.
- **Defects found:** [ISS-22](../ISSUES.md#iss-22-the-output-nodes-directory-leaves-env-as-a-literal-folder-name). While reproducing it: [ISS-21](../ISSUES.md#iss-21-the-tui-deletes-the-contents-of-a-recipes-output-folder-before-each-run), the TUI deletes the contents of a recipe's output folder before every run.
- **Generic piece:** none; it's a fix.
  - The output directory fills every namespace a run has, and refuses any placeholder left over, as unset `{{vars.*}}` already do and `{{fields.*}}` do since #741 (in the engine) and #742 (before consent).
  - `bnto check` flags the ones no run can fill.
  - ISS-21 is fixed first, so filling `{{env.*}}` can't point the TUI's wipe at a real folder.
- **Tracked in:** ISSUES.md.

## Reply

> Thanks, these were useful. One warning first: **please run this recipe with `bnto run`, not the TUI, for now.** While reproducing your report we found that the TUI empties a recipe's output folder before each run. It's being fixed first.
>
> **What works today: leading path segments.** Possible, but clumsy. Collect with `file-collect` and `flatten: false` (the default strips the folders, and every file then lands in a group of its own), then:
>
> 1. `file-inspect`.
> 2. `edit-fields` with `rel: {{item.filename}}`.
> 3. `file-rename` with `find: ^([^/]+(?:/[^/]+){0,N-1})(?:/.*)?$` and `replace: $1/_`.
> 4. `file-inspect` again: its `folder` is now the leading part.
> 5. `edit-fields` with `lead: {{item.folder}}`.
> 6. `file-rename` with `pattern: {{item.rel}}`.
>
> Then group with `summarize` (`groupBy: lead`). Paths shallower than N lose their extension. Here it is at N=1, tested on a small tree, with a `report` step that prints each group and its count:
>
> ```json
> {
>   "id": "leading-segments",
>   "type": "group",
>   "version": "1.0.0",
>   "name": "Leading Segments",
>   "nodes": [
>     { "id": "input", "type": "input", "parameters": { "mode": "none" } },
>     { "id": "list", "type": "file-list", "parameters": { "paths": ["tree"] } },
>     { "id": "walk", "type": "file-collect", "parameters": { "flatten": false } },
>     { "id": "inspect", "type": "file-inspect", "parameters": {} },
>     {
>       "id": "save",
>       "type": "edit-fields",
>       "parameters": { "values": { "rel": "{{item.filename}}" } }
>     },
>     {
>       "id": "cut",
>       "type": "file-rename",
>       "parameters": { "find": "^([^/]+(?:/[^/]+){0,0})(?:/.*)?$", "replace": "$1/_" }
>     },
>     { "id": "inspect-lead", "type": "file-inspect", "parameters": {} },
>     {
>       "id": "lead",
>       "type": "edit-fields",
>       "parameters": { "values": { "lead": "{{item.folder}}" } }
>     },
>     { "id": "restore", "type": "file-rename", "parameters": { "pattern": "{{item.rel}}" } },
>     { "id": "group", "type": "summarize", "parameters": { "groupBy": "lead" } },
>     {
>       "id": "print",
>       "type": "report",
>       "parameters": { "message": "{{item.lead}}: {{item.count}}" }
>     },
>     { "id": "output", "type": "output", "parameters": { "mode": "none" } }
>   ],
>   "edges": [
>     { "id": "e1", "source": "input", "target": "list" },
>     { "id": "e2", "source": "list", "target": "walk" },
>     { "id": "e3", "source": "walk", "target": "inspect" },
>     { "id": "e4", "source": "inspect", "target": "save" },
>     { "id": "e5", "source": "save", "target": "cut" },
>     { "id": "e6", "source": "cut", "target": "inspect-lead" },
>     { "id": "e7", "source": "inspect-lead", "target": "lead" },
>     { "id": "e8", "source": "lead", "target": "restore" },
>     { "id": "e9", "source": "restore", "target": "group" },
>     { "id": "e10", "source": "group", "target": "print" },
>     { "id": "e11", "source": "print", "target": "output" }
>   ]
> }
> ```
>
> **Bugs, and what to do meanwhile:**
>
> - **`{{env.*}}` in the output directory:** a bug, thanks for the clear repro. bnto will fill it, and will refuse a run whose output directory still holds an unfilled placeholder, instead of creating a folder by that name. Until then, keep writing with `file-copy`, or pass `--output "$SOME_KEY"`.
> - **Two smaller bugs** we found while trying the recipe above: `file-rename` mishandles a folder name containing a dot, which can put a file with no extension in a dotted folder (`v1.2/README`) in the wrong group; and `edit-fields` doesn't fill `{{file}}`.
>
> **What bnto will add:** template filters, so each outcome is one expression in any field of any node instead of a dedicated node.
>
> - **Leading segments:** along the lines of `{{item.filename | split('/') | first(2) | join('/')}}`. The exact syntax is still to be settled.
> - **Readable sizes:** no node does this yet; `shell-run` with `numfmt` or `awk` is the stopgap. The filter will be along the lines of Jinja's `filesizeformat`, with a fixed unit and decimals, usable wherever templates are. How base-1024 units are labelled (`GB` or `GiB`) is still to be decided.
>
> **What bnto won't add:** a `top-folder` node, a `depth` param on `file-inspect`, a "human size" node or a `format` param on `summarize`. Each would serve one use case, while a filter works in every field of every node, `file-render` and `report` included.
