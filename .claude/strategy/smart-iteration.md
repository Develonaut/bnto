# Smart Iteration

**Status:** Delivered
**Last Updated:** March 2026

---

## What Is Smart Iteration?

Recipes like "compress images" currently require explicit `loop` container nodes to iterate over multiple files:

```
input → group → loop → [compress] → output
```

This is technically correct but creates unnecessary ceremony for 90% of recipes. Users expect batch processing to "just work."

**Key discovery:** The engine already auto-iterates primitive nodes individually — `execute_primitive_node` loops over all files internally. The explicit `loop` container only adds unique value for multi-node sub-pipelines (e.g., `resize → convert → compress` running as a sequence per file). For single-processor recipes, the loop is entirely redundant.

**Smart iteration** adds a `settings` object to the Definition with `iteration: "auto" | "explicit"`. When `"auto"`, the engine wraps contiguous per-file processor sequences in implicit per-file loops. Explicit loops still work for users who want fine-grained control.

---

## The `settings` Object

`settings` is a recipe-level configuration container on the root Definition. It's extensible — smart iteration is the first consumer, but future features (e.g., error handling policy, concurrency limits) can add fields without changing the schema shape.

```json
{
  "id": "compress-images",
  "type": "group",
  "settings": {
    "iteration": "auto"
  },
  "nodes": [
    { "id": "input", "type": "input" },
    { "id": "compress", "type": "image-compress", "parameters": { "quality": 80 } },
    { "id": "output", "type": "output" }
  ]
}
```

### Backward Compatibility

- `settings` is **optional** on `PipelineDefinition` (`#[serde(default)]`)
- Missing `settings` → defaults to `{ iteration: "explicit" }`
- `resolved_iteration()` helper returns `Explicit` when settings is absent
- All existing recipes continue to work unchanged — zero behavioral change

---

## `settings.iteration`: Auto vs Explicit

| Mode         | Behavior                                                                        | Who it's for                                             |
| ------------ | ------------------------------------------------------------------------------- | -------------------------------------------------------- |
| `"auto"`     | Engine wraps contiguous per-file processor sequences in implicit per-file loops | Recipe authors who want flat, simple definitions         |
| `"explicit"` | Engine executes exactly what's defined — containers control iteration           | Power users who need fine-grained control over iteration |

**Both modes produce identical output.** This is proven via golden test equivalence — flat (auto) recipes share the same golden output directory as their explicit counterparts.

---

## `inputCardinality` Metadata

Each processor declares how it expects to receive files:

| Value       | Meaning                      | Examples                                  |
| ----------- | ---------------------------- | ----------------------------------------- |
| `"perFile"` | Processes one file at a time | image-compress, image-resize, file-rename |
| `"batch"`   | Needs the full batch at once | (future: zip, concat, merge)              |

This metadata lives on `NodeMetadata` (the engine's self-description). All 6 current processors are `perFile`.

### How the Executor Uses It

In `"auto"` mode, the executor partitions the flat node sequence into runs:

1. **Contiguous `perFile` nodes** → `Run::PerFileSequence` — wrapped in an implicit per-file loop (same semantics as `execute_loop` in container.rs)
2. **Container nodes** → `Run::Container` — dispatched as-is (containers define their own iteration)
3. **Future: `batch` nodes** → act as iteration barriers. The per-file run completes, batch node gets full output, then a new per-file run starts after

### Example: Auto Mode Execution

Given a flat recipe with `settings.iteration = "auto"`:

```
input → resize → convert → compress → output
```

The executor sees three contiguous `perFile` processors. It wraps them in an implicit per-file loop:

```
For each input file:
  file → resize → convert → compress → output
```

This produces identical output to the explicit version:

```
input → loop → [resize → convert → compress] → output
```

---

## Golden Test Equivalence

The strongest correctness proof: auto-iteration recipes produce **byte-identical** output to explicit-loop recipes. Both variants share the same golden output directory — same SHA-256 hashes.

| Recipe                  | Explicit fixture                    | Flat fixture                             | Shared golden dir                 |
| ----------------------- | ----------------------------------- | ---------------------------------------- | --------------------------------- |
| compress-images         | `compress-images.bnto.json`         | `flat/compress-images.bnto.json`         | `golden/compress-images/`         |
| optimize-images-for-web | `optimize-images-for-web.bnto.json` | `flat/optimize-images-for-web.bnto.json` | `golden/optimize-images-for-web/` |
| ...                     | ...                                 | ...                                      | ...                               |

10 explicit + 10 flat = 20 golden tests, all sharing output expectations.

---

## Both Paths Are First-Class

Smart iteration does **not** deprecate explicit loops. Both are permanent, supported features:

- **Auto mode** is the default for new simple recipes. Less ceremony, easier to author, friendlier for casual users and LLM-generated definitions.
- **Explicit mode** is for power users who need fine-grained control: custom iteration logic, mixed per-file and batch processing, nested container hierarchies.

The editor UI surfaces iteration mode in the recipe settings panel (visible when no node is selected), letting users switch between modes. Switching from explicit to auto is a simplification (strip containers); switching from auto to explicit would require materializing implicit loops (future feature).

---

## Recipe Settings Panel (Editor UX)

When no node is selected in the visual editor, the config panel shows recipe-level settings instead of the "Select a node to configure" placeholder:

1. **Recipe name** — editable inline (replaces File Menu → Rename)
2. **Iteration mode** — Auto / Explicit toggle
3. **Future:** Additional recipe-level settings as they're added

This gives the otherwise-empty config panel a useful purpose and makes recipe configuration more discoverable.

---

## Related Documents

- [engine-execution.md](engine-execution.md) — Pipeline executor architecture
- [node-responsibilities.md](../scopes/rust/node-responsibilities.md) — Engine / @bnto/nodes / Editor responsibility matrix
- [engine-node-patterns.md](../scopes/rust/engine-node-patterns.md) — Processor implementation patterns
