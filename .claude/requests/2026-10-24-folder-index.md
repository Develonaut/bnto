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
- **Today:** five nodes, all of them renaming files just to compute one field. `file-rename`'s regex `find` runs on the whole relative path's stem (`bnto-file/src/rename/transform.rs:54-59`). The steps:
  1. `edit-fields` saves `rel: {{item.filename}}`.
  2. `file-rename` with `find: ^([^/]+(?:/[^/]+){0,N-1})(?:/.*)?$` and `replace: $1/_`.
  3. `file-inspect`, whose `folder` is now the leading part.
  4. `edit-fields` saves `lead: {{item.folder}}`.
  5. `file-rename` with `pattern: {{item.rel}}` restores the name, then `summarize` with `groupBy: lead`.
  - N=1 over `a/x.bin`, `a/b/y.bin`, `a/b/c/d.bin`, `top.bin` and `q/r.bin` grouped as `a` (3 files), `top` and `q`.
  - **Edge case:** a path shallower than N loses its extension (`top`, not `top.bin`).
  - Windows names are fine, since `file-collect` already uses `slash_path`.
- **Defects found:** [ISS-23](../ISSUES.md) (`file-rename` splits the extension anywhere in the path), [ISS-24](../ISSUES.md) (`edit-fields` leaves `{{file}}` as text).
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

  `{{item.sum_size | filesizeformat(binary=true, unit='GiB', decimals=2)}}`. It reads numbers the way `summarize` does (`summarize/total.rs:1-6`), and replaces the two hand-rolled size formatters (`bnto/src/tui/format.rs:19`, `tonkotsu/src/format_size.rs:4`).

- **For review:** the request's `2.79 GB` is base 1024. Strict labels give `2.79 GiB`, or `3.00 GB` at base 1000. A desktop file manager's label is one platform's opinion, so the filter shouldn't default to it silently.
- **Won't build:** a "human size" node, or a `format` param on `summarize`. Formatting is a template concern: `file-render` and `report` need it too.
- **Tracked in:** PLAN.md → Template filters.

## 3. The output node's `directory` with `{{env.*}}`

- **Verdict:** defect, a silent wrong result like ISS-1.
- **Today**, with `secrets: [SOME_KEY]`, `directory: {{env.SOME_KEY}}` and `SOME_KEY` set:
  - `bnto check` passes.
  - `bnto run` exits 0 with "1 file written to {{env.SOME_KEY}}/", and the file lands in a folder literally named `./{{env.SOME_KEY}}/`.
  - With `SOME_KEY` unset, the run is refused ("Missing required secrets") but the folder is still created first.
  - Other leftovers are dropped too: `out-{{ctx.nope}}-{{item.x}}` writes to `out--{{item.x}}/`.
- **Defects found:** [ISS-22](../ISSUES.md). While reproducing it: [ISS-21](../ISSUES.md), the TUI deletes the contents of a recipe's output folder before every run.
- **Generic piece:** none; it's a fix.
  - The output directory fills every namespace a run has, and refuses any placeholder left over, as `{{vars.*}}` and `{{fields.*}}` already do (#741).
  - `bnto check` flags the ones no run can fill.
  - ISS-21 is fixed first, so filling `{{env.*}}` can't point the TUI's wipe at a real folder.
- **Tracked in:** ISSUES.md.

## Reply

> Thanks, these were useful. Each outcome, and where it stands:
>
> **Leading path segments:** possible today, but clumsy. Save `{{item.filename}}` with `edit-fields`. Then use `file-rename` with `find: ^([^/]+(?:/[^/]+){0,N-1})(?:/.*)?$` and `replace: $1/_`, and run `file-inspect`, whose `folder` is now the leading part. Copy it into a field, rename back with `pattern: {{item.rel}}`, and group with `summarize`. Paths shallower than N lose their extension. Rather than a dedicated node, bnto is getting template filters, so this becomes `{{item.filename | split('/') | first(2) | join('/')}}` in any field of any node.
>
> **Readable sizes:** not possible with nodes yet; `shell-run` is the stopgap. It's coming as a `filesizeformat` template filter, with a fixed unit and decimals, usable wherever templates are. Base-1024 units will most likely be labelled `GiB`.
>
> **`{{env.*}}` in the output directory:** a bug, thanks for the clear repro. bnto will fill it, and will refuse a run whose output directory still holds an unfilled placeholder, instead of creating a folder by that name. Until then, keep `file-copy`, or pass `--output "$SOME_KEY"`.
>
> We also found and logged two related bugs: `file-rename` mishandles a folder name containing a dot, and `edit-fields` doesn't fill `{{file}}`.
