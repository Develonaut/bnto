# Node Vocabulary

The verbs and shared terms every engine node is named and built with. A verb is a promise: two nodes with the same verb do the same kind of thing to what flows through them, whatever their domain. Read this before naming a node, adding a param, or writing a node's label and description. Companion to [engine-node-patterns.md](engine-node-patterns.md) and [generic-building-block-nodes.md](../../decisions/generic-building-block-nodes.md).

---

## Node Type Names

```
<domain>-<verb>[-<object>]      image-resize, file-copy, image-strip-exif
```

- **Domain** is the kind of thing the node works on, and matches the crate (`bnto-image` → `image-*`). Domains: `file`, `image`, `vector`, `spreadsheet`, `shell`, `http`. New domains get a new crate.
- **Verb** comes from the [Verb Registry](#verb-registry) below. A verb that isn't listed gets added here first, in the same PR as the node, with its class and contract.
- **Object** only when the verb alone is ambiguous in that domain (`image-strip-exif`: strip what?). Never repeat the domain (`image-resize-image`).
- **No noun names.** A noun like `shell-command` says what a node is about, not what it does. `shell-command` is being renamed (see [Audit](#audit-october-2026)).

**Two tiers have no domain**, because they work on any item:

| Tier           | Naming    | Nodes                                                       | Why bare                                                                |
| -------------- | --------- | ----------------------------------------------------------- | ----------------------------------------------------------------------- |
| **Structural** | Bare noun | `input`, `output`, `loop`, `group`, `parallel`              | They shape the pipeline, not the items. Keywords of the recipe language |
| **Item-level** | Bare verb | `assert`, `report`, `summarize`, `edit-fields`, `transform` | They read or set fields, never file content, so every domain uses them  |

A new bare name needs the same justification: it must work on every item regardless of domain.

**Same verb, same meaning, at the domain's unit.** `file-filter` keeps or drops items. A future `spreadsheet-filter` keeps or drops rows inside each table. Both are `filter`: keep what matches, drop the rest, change nothing else.

---

## Verb Classes

Every verb belongs to one class. The class is the contract: what the node does to the item stream, whether it rewrites content, and whether it touches anything outside the run. This is the "CRUD" layer. Pick the class first, then the verb.

| Class        | Stream shape                                                             | Content                           | Outside the run                      | Output name                                                           |
| ------------ | ------------------------------------------------------------------------ | --------------------------------- | ------------------------------------ | --------------------------------------------------------------------- |
| **Source**   | none → items                                                             | New items from outside            | Reads only                           | The source's own name                                                 |
| **Change**   | 1 → 1 per item                                                           | Rewritten                         | None                                 | `-<past participle>` suffix, or new extension when the format changes |
| **Annotate** | 1 → 1 per item                                                           | Untouched (`FileData::Path`)      | None                                 | Unchanged                                                             |
| **Select**   | N → ≤ N                                                                  | Untouched                         | None                                 | Unchanged                                                             |
| **Expand**   | 1 → N                                                                    | New items from one item's content | None                                 | Derived from the source                                               |
| **Combine**  | N → 1 (or N → groups)                                                    | New item(s)                       | None                                 | Named by a `filename` param                                           |
| **Place**    | N → N, each flagged with what happened (`copied`, `skipped`, `deleted`…) | Untouched                         | **Writes, moves or deletes on disk** | Unchanged                                                             |
| **Observe**  | N → N pass-through                                                       | Untouched                         | Prints, or fails the run             | Unchanged                                                             |
| **Run**      | Delegates                                                                | Whatever the tool does            | **Whatever the tool does**           | Whatever the tool produces                                            |

Consequences agents rely on:

- **Annotate, Select, Place and Observe nodes never call `into_bytes()`.** They pass `FileData::Path` through. See [FileData Selection](engine-node-patterns.md#filedata-selection).
- **Place and Run are the only classes with side effects.** They are what dry-run, consent and trust care about. A node in another class that writes to disk is in the wrong class or needs a new verb.
- **Change nodes are pure.** Same input and params give the same bytes, which is what golden tests check.

---

## Verb Registry

The approved verbs. "Not" lists the near-synonyms this verb rules out, so two nodes never say the same thing two ways.

### Source

| Verb      | Means                                                       | Nodes                    | Not                            |
| --------- | ----------------------------------------------------------- | ------------------------ | ------------------------------ |
| `collect` | Find files on disk by searching a folder with a pattern     | `file-collect`           | `scan`, `find`, `glob`, `walk` |
| `list`    | Turn an explicit list of paths into items, found or missing | `file-list`              | `load`, `open`                 |
| `request` | Call a URL and turn the response into an item               | `http-request` (planned) | `fetch`, `get`, `call`         |

`collect` searches. `list` takes the paths it's given. A node that reads a file's _content_ into items is `read` (Expand), not a Source.

### Change

| Verb        | Means                                                                   | Nodes                                  | Not                         |
| ----------- | ----------------------------------------------------------------------- | -------------------------------------- | --------------------------- |
| `compress`  | Make the payload smaller in the same format, with a quality trade-off   | `image-compress`                       | `shrink`, `minify`          |
| `optimize`  | Rewrite the structure smaller with no visible change (lossless cleanup) | `vector-optimize`                      | `minify`, `compress`        |
| `convert`   | Same content, another format **within the domain**                      | `image-convert`, `spreadsheet-convert` | `transcode`, `export`, `to` |
| `rasterize` | Vector → raster. A cross-domain convert gets its own verb               | `vector-rasterize`                     | `convert`, `render`         |
| `decode`    | Text encoding → bytes. Inverse: `encode`                                | `file-decode`                          | `parse`, `unpack`           |
| `resize`    | Change dimensions                                                       | `image-resize`                         | `scale`                     |
| `pad`       | Grow the canvas around the content                                      | `image-pad`                            | `extend`, `border`          |
| `overlay`   | Draw one thing on top of each item                                      | `image-overlay`                        | `watermark`, `composite`    |
| `strip`     | Remove embedded metadata, leave the content as is                       | `image-strip-exif`                     | `remove`, `scrub`, `clean`  |
| `clean`     | Remove junk _data_: empty rows, stray whitespace, duplicates            | `spreadsheet-clean`                    | `tidy`, `sanitize`, `strip` |
| `rename`    | Change names (file names, column headers), nothing else                 | `file-rename`, `spreadsheet-rename`    | `relabel`, `retitle`        |
| `transform` | Compute new values from expressions                                     | `transform` (no processor yet)         | `map`, `compute`            |

Planned verbs already in this class: `crop`, `rotate` (image), `encode`.

### Annotate

| Verb      | Means                                                  | Nodes                | Not                         |
| --------- | ------------------------------------------------------ | -------------------- | --------------------------- |
| `inspect` | Attach facts about each file (size, type, times, hash) | `file-inspect`       | `stat`, `probe`, `describe` |
| `lookup`  | Attach the matching row from a table, by key           | `spreadsheet-lookup` | `join`, `enrich`, `match`   |
| `edit`    | Set fields on each item from values or templates       | `edit-fields`        | `set`, `assign`, `tag`      |

Matching records by key is always `lookup`. `join` means concatenate (see Combine).

### Select

| Verb     | Means                                        | Nodes         | Not                       |
| -------- | -------------------------------------------- | ------------- | ------------------------- |
| `filter` | Keep what matches a condition, drop the rest | `file-filter` | `select`, `where`, `keep` |
| `sort`   | Reorder by one or more fields                | `file-sort`   | `order`, `rank`           |

### Expand

| Verb   | Means                                                           | Nodes              | Not                        |
| ------ | --------------------------------------------------------------- | ------------------ | -------------------------- |
| `read` | Turn one file's content into one item per record (row, element) | `spreadsheet-read` | `parse`, `explode`, `load` |

Planned: `split` (one file → many files of the same kind, e.g. PDF pages), `extract` (pull embedded things out, e.g. images from a PDF).

### Combine

| Verb        | Means                                                                | Nodes               | Not                            |
| ----------- | -------------------------------------------------------------------- | ------------------- | ------------------------------ |
| `join`      | Concatenate content, in arrival order, with a separator              | `file-join`         | `concat`, `append`, `merge`    |
| `merge`     | Combine same-shaped structured files into one, reconciling structure | `spreadsheet-merge` | `join`, `union`, `combine`     |
| `render`    | Write one text file from a template over the items                   | `file-render`       | `format`, `template`, `export` |
| `summarize` | Group items by fields, one item per group with counts and totals     | `summarize`         | `aggregate`, `group`           |

`join` knows nothing about structure. `merge` does (CSV headers, PDF pages). `group` is the structural container, never a verb.

### Place

| Verb     | Means                                                              | Nodes                | Not                     |
| -------- | ------------------------------------------------------------------ | -------------------- | ----------------------- |
| `copy`   | Put a copy of each item under a destination folder                 | `file-copy`          | `save`, `write`         |
| `move`   | Move each item's file under a destination folder                   | `file-move`          | `relocate`, `transfer`  |
| `delete` | Remove each item's file from disk                                  | `file-delete`        | `remove`, `trash`       |
| `export` | Write a record of the items to a path you name; items pass through | `spreadsheet-export` | `save`, `dump`, `write` |

`render` emits a file into the stream. `export` writes one to disk as the run goes, even in a dry run, because it is a record of the run.

### Observe

| Verb     | Means                                       | Nodes    | Not               |
| -------- | ------------------------------------------- | -------- | ----------------- |
| `report` | Print a templated line to the run's output  | `report` | `log`, `print`    |
| `assert` | Fail the run when a comparison doesn't hold | `assert` | `check`, `verify` |

### Run

| Verb  | Means                                          | Nodes                               | Not                         |
| ----- | ---------------------------------------------- | ----------------------------------- | --------------------------- |
| `run` | Run an external program and collect its output | `shell-run` (today `shell-command`) | `exec`, `execute`, `invoke` |

`run` matches `bnto run`. A future `recipe` node that runs another recipe is structural, not a `run` verb.

---

## Labels and Descriptions

The label is the verb in title case, then what it acts on, plural: **Verb Objects**.

| Good                | Bad           | Why                           |
| ------------------- | ------------- | ----------------------------- |
| Resize Images       | Image Resizer | Noun                          |
| Convert CSV to JSON | CSV to JSON   | No verb                       |
| Overlay Images      | Overlay Image | Acts on every item, so plural |
| Inspect Files       | File Metadata | Noun                          |

- **The label's verb is the type's verb.** `image-convert` is "Convert …", never "Change …" or "Transform …".
- **A Combine node that makes one file may be singular** ("Render Text File", "Merge CSV").
- **The description opens with the same verb** and states the class's promise: what passes through unchanged, what is written to disk.
- **"Folder", never "directory"**, in labels and descriptions. Users say folder.

---

## Shared Params

When two nodes need the same idea, they use the same param name, type and values. Check this table before naming a param, and add to it when a second node needs a new one.

| Concept                                | Param              | Type / values                                   | Used by                                       |
| -------------------------------------- | ------------------ | ----------------------------------------------- | --------------------------------------------- |
| Folder results go under                | `destination`      | string                                          | `file-copy`, `file-move`                      |
| Folder relative paths resolve from     | `base`             | string                                          | `file-delete`, `file-list`                    |
| Make missing folders                   | `createDirs`       | boolean, default `true`, label "Create Folders" | `file-copy`, `file-move`                      |
| Name of the one file a node makes      | `filename`         | string                                          | `file-join`, `file-render`, `file-decode`     |
| An existing file is in the way         | `conflict`         | `skip` \| `overwrite` \| `rename` (\| `newer`)  | `file-copy`. Any new node that writes to disk |
| Text put between parts                 | `separator`        | string                                          | `file-join`, `file-render`, `file-rename`     |
| Drop repeated records                  | `removeDuplicates` | boolean                                         | `spreadsheet-clean`, `spreadsheet-merge`      |
| Output quality (lossy encoders)        | `quality`          | number 1–100, default 80, `quality_param_def()` | every lossy image/vector output               |
| Field delimiter                        | `delimiter`        | `comma` \| `semicolon` \| `tab` \| `pipe`       | `spreadsheet-convert`, `spreadsheet-read`     |
| What to do with items that don't match | `unmatched`        | `keep` \| `drop` \| `fail`                      | `spreadsheet-lookup`                          |

Rules:

- **camelCase**, like the rest of the recipe format (`inputCardinality`, `children`).
- **One name per meaning.** `case` is letter case (`lower` \| `upper` \| `title`). Case sensitivity is `caseSensitive: boolean`, not another `case`.
- **Booleans read as a yes/no question about the node's behaviour**: `recursive`, `flatten`, `createDirs`. No `no*` / `disable*` negatives.
- **Enum values are kebab-case** (`first-file`, `top-left`, `not-equals`).
- **Conflict handling is the `conflict` enum,** never a bare `overwrite: boolean`. A node that can't handle a conflict yet refuses the run and says why.

---

## Adding or Changing a Verb

1. Check the registry. Use an existing verb if its meaning fits, even if another word sounds nicer.
2. If none fits, add a row in the right class in the same PR as the node: verb, meaning, node, and the synonyms it rules out.
3. A new verb never duplicates an existing one's meaning. If it does, use the existing one.
4. Renaming a shipped node type or param is a breaking change. It needs a `bnto migrate` step (`engine/crates/bnto/src/migrate.rs`) so old recipes keep working, and its own PR.

---

## Audit (October 2026)

Deviations from this doc in shipped nodes. Each fix is its own PR, tracked in [PLAN.md](../../PLAN.md#node-vocabulary-alignment). Remove a row when its PR merges.

| Node / area                       | Deviation                                                                                                                                                                                                               | Fix                                                                                              | Breaking     |
| --------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ | ------------ |
| `shell-command`                   | Noun name                                                                                                                                                                                                               | Rename to `shell-run`, label "Run Shell Command"                                                 | Yes, migrate |
| 14 params                         | snake_case: `create_dirs`, `tolerance_ms`, `name_pattern`, `pattern_mode`, `min_size`, `max_size`, `include_hash`, `max_length`, `counter_start`, `counter_pad`, `key_column`, `match_on`, `name_column`, `search_path` | camelCase                                                                                        | Yes, migrate |
| `file-sort` `case`                | Means case sensitivity; `file-rename` `case` means letter case                                                                                                                                                          | `caseSensitive: boolean`                                                                         | Yes, migrate |
| `spreadsheet-merge` `deduplicate` | Same idea as `spreadsheet-clean` `removeDuplicates`                                                                                                                                                                     | `removeDuplicates`                                                                               | Yes, migrate |
| `spreadsheet-export` `overwrite`  | Bare boolean for a conflict                                                                                                                                                                                             | `conflict` enum                                                                                  | Yes, migrate |
| `assert` `op` value `not_equals`  | snake_case enum value                                                                                                                                                                                                   | `not-equals`                                                                                     | Yes, migrate |
| `image-overlay` output            | Suffix `-overlay`                                                                                                                                                                                                       | `-overlaid`                                                                                      | Output names |
| `vector-optimize` output          | Keeps the input name; other Change nodes add a suffix                                                                                                                                                                   | `-optimized`                                                                                     | Output names |
| Labels                            | "CSV to JSON", "SVG to Image", "Shell Command", "Overlay Image"                                                                                                                                                         | "Convert CSV to JSON", "Rasterize SVG", "Run Shell Command", "Overlay Images"                    | No           |
| Labels / descriptions             | "Create Directories" (`file-copy`), "Traverse a directory" (`file-collect`)                                                                                                                                             | "Create Folders", "folder"                                                                       | No           |
| `transform` vs `edit-fields`      | Both set values from expressions. `transform` has no processor yet                                                                                                                                                      | Decide before `transform` gets a processor: fold it into `edit-fields`, or define the split here | n/a          |
