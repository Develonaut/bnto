# Config Panel Controls — Strategy & Matrix

**Last Updated:** March 14, 2026
**Purpose:** Single source of truth for what control renders for each node parameter, what controls exist, and what's missing. Agents building new nodes or controls read this first.

---

## How Controls Work

The config panel is **schema-driven**. Every node type defines its parameters as a Zod schema in `@bnto/nodes/schemas/`. The editor's `SchemaField` component infers which control to render based on the Zod type and optional metadata hints.

```
Zod schema (definition) → inferFieldType() → CONTROL_REGISTRY → renders UI control
```

**Files involved:**

| File                                      | Role                                               |
| ----------------------------------------- | -------------------------------------------------- |
| `@bnto/nodes/schemas/{nodeType}.ts`       | Zod schema + `NodeParamMeta` per param             |
| `@bnto/nodes/schemas/inferFieldType.ts`   | Zod type → `FieldControl` string                   |
| `@bnto/editor/components/SchemaField.tsx` | Dispatches `FieldControl` → control component      |
| `@bnto/editor/components/controls/`       | Control component implementations                  |
| `@bnto/ui/interaction/`                   | Base UI primitives (Input, Select, Slider, Switch) |

---

## Available Controls

| Control    | `FieldControl` key | Renders                                        | Inferred From                                | `@bnto/ui` Primitive  |
| ---------- | ------------------ | ---------------------------------------------- | -------------------------------------------- | --------------------- |
| **Text**   | `text`             | Single-line text input                         | `z.string()` (fallback)                      | `Input`               |
| **Number** | `number`           | Numeric input (unbounded or single constraint) | `z.number()` without both min+max            | `Input type="number"` |
| **Slider** | `slider`           | Bounded range slider with value display        | `z.number()` with both `.min()` AND `.max()` | `Slider`              |
| **Switch** | `switch`           | Boolean toggle                                 | `z.boolean()`                                | `Switch`              |
| **Select** | `select`           | Dropdown with options                          | `z.enum([...])`                              | `Select`              |

### Missing Controls (need to build)

| Control             | `FieldControl` key | Renders                                  | Needed For                          | `@bnto/ui` Primitive Needed |
| ------------------- | ------------------ | ---------------------------------------- | ----------------------------------- | --------------------------- |
| **Textarea**        | `textarea`         | Multiline text input                     | Expressions, patterns, long text    | `Textarea`                  |
| **TagPicker**       | `tagPicker`        | Chip/tag input with add/remove           | `z.array(z.string())` params        | `TagPicker`                 |
| **KeyValueEditor**  | `keyValue`         | Add/remove key→value pairs               | `z.record(z.string())` params       | `KeyValueEditor`            |
| **ExpressionInput** | `expression`       | Rich text with pill tokens for `{{var}}` | Template expressions with variables | `ExpressionInput`           |

---

## Node Parameter Matrix

### Legend

- ✅ = Correct control, no work needed
- ⚠️ = Works but could be better
- ❌ = Wrong control, needs fix
- 🚩 = Not renderable (Zod type not supported by inference)

---

### Image Node (`image`)

| Parameter        | Zod Type                                  | Current | Ideal  | Status |
| ---------------- | ----------------------------------------- | ------- | ------ | ------ |
| `operation`      | `z.enum(["compress","convert","resize"])` | select  | select | ✅     |
| `compression`    | `z.number().min(1).max(100)`              | slider  | slider | ✅     |
| `format`         | `z.enum(["jpeg","png","webp"])`           | select  | select | ✅     |
| `quality`        | `z.number().min(1).max(100)`              | slider  | slider | ✅     |
| `width`          | `z.number().min(1)`                       | number  | number | ✅     |
| `height`         | `z.number().min(1)`                       | number  | number | ✅     |
| `maintainAspect` | `z.boolean()`                             | switch  | switch | ✅     |

**Status: 7/7 correct.** Image node is the gold standard.

---

### File-System Node (`file-system`)

| Parameter   | Zod Type                            | Current | Ideal          | Status                                     |
| ----------- | ----------------------------------- | ------- | -------------- | ------------------------------------------ |
| `operation` | `z.enum(["rename"])`                | select  | select         | ✅                                         |
| `find`      | `z.string()`                        | text    | text           | ✅                                         |
| `replace`   | `z.string()`                        | text    | text           | ✅                                         |
| `case`      | `z.enum(["lower","upper","title"])` | select  | select         | ✅                                         |
| `prefix`    | `z.string()`                        | text    | text           | ✅                                         |
| `suffix`    | `z.string()`                        | text    | text           | ✅                                         |
| `pattern`   | `z.string()`                        | text    | **expression** | ⚠️ Has `{{name}}`, `{{ext}}` template vars |

**Status: 6/7 correct.** `pattern` should use ExpressionInput when available.

---

### Spreadsheet Node (`spreadsheet`)

| Parameter          | Zod Type                     | Current | Ideal        | Status                                     |
| ------------------ | ---------------------------- | ------- | ------------ | ------------------------------------------ |
| `operation`        | `z.enum(["clean","rename"])` | select  | select       | ✅                                         |
| `trimWhitespace`   | `z.boolean()`                | switch  | switch       | ✅                                         |
| `removeEmptyRows`  | `z.boolean()`                | switch  | switch       | ✅                                         |
| `removeDuplicates` | `z.boolean()`                | switch  | switch       | ✅                                         |
| `columns`          | `z.record(z.string())`       | text    | **keyValue** | 🚩 Key-value editor (old→new column names) |

**Status: 4/5 correct.** `columns` needs KeyValueEditor.

---

### Input Node (`input`)

| Parameter     | Zod Type                               | Current | Ideal         | Status                                        |
| ------------- | -------------------------------------- | ------- | ------------- | --------------------------------------------- |
| `mode`        | `z.enum(["file-upload","text","url"])` | select  | select        | ✅                                            |
| `accept`      | `z.array(z.string())`                  | text    | **hidden**    | 🚩 Hide — derive from `extensions` internally |
| `extensions`  | `z.array(z.string())`                  | text    | **tagPicker** | 🚩 Chip input with preset file type groups    |
| `label`       | `z.string()`                           | text    | text          | ✅                                            |
| `multiple`    | `z.boolean()`                          | switch  | switch        | ✅                                            |
| `maxFileSize` | `z.number()`                           | number  | number        | ✅                                            |
| `maxFiles`    | `z.number()`                           | number  | number        | ✅                                            |

**Status: 4/7 correct.** `extensions` needs TagPicker. `accept` should be hidden (leaky abstraction).

---

### Output Node (`output`)

| Parameter      | Zod Type                                   | Current | Ideal          | Status                         |
| -------------- | ------------------------------------------ | ------- | -------------- | ------------------------------ |
| `mode`         | `z.enum(["download","display","preview"])` | select  | select         | ✅                             |
| `filename`     | `z.string()`                               | text    | **expression** | ⚠️ Has `{{name}}` template var |
| `zip`          | `z.boolean()`                              | switch  | switch         | ✅                             |
| `label`        | `z.string()`                               | text    | text           | ✅                             |
| `autoDownload` | `z.boolean()`                              | switch  | switch         | ✅                             |

**Status: 4/5 correct.** `filename` should use ExpressionInput when available.

---

### Loop Node (`loop`)

| Parameter        | Zod Type                              | Current | Ideal          | Status                                 |
| ---------------- | ------------------------------------- | ------- | -------------- | -------------------------------------- |
| `mode`           | `z.enum(["forEach","times","while"])` | select  | select         | ✅                                     |
| `items`          | `z.string()`                          | text    | **expression** | ❌ Raw template syntax exposed to user |
| `count`          | `z.number()`                          | number  | number         | ✅                                     |
| `condition`      | `z.string()`                          | text    | **expression** | ❌ Raw expression syntax               |
| `breakCondition` | `z.string()`                          | text    | **expression** | ❌ Raw expression syntax               |

**Status: 2/5 correct.** Three expression params are the worst UX in the editor today.

---

### Group Node (`group`)

| Parameter | Zod Type                            | Current | Ideal  | Status |
| --------- | ----------------------------------- | ------- | ------ | ------ |
| `mode`    | `z.enum(["sequential","parallel"])` | select  | select | ✅     |

**Status: 1/1 correct.**

---

### Transform Node (`transform`)

| Parameter    | Zod Type               | Current | Ideal                                           | Status              |
| ------------ | ---------------------- | ------- | ----------------------------------------------- | ------------------- |
| `expression` | `z.string()`           | text    | **textarea** (interim) / **expression** (later) | ⚠️                  |
| `mappings`   | `z.record(z.string())` | text    | **keyValue**                                    | 🚩 Key-value editor |

**Status: 0/2 correct.** Both params need new controls.

---

### Parallel Node (`parallel`)

| Parameter       | Zod Type                            | Current | Ideal                          | Status                                |
| --------------- | ----------------------------------- | ------- | ------------------------------ | ------------------------------------- |
| `tasks`         | `z.array(z.record(z.unknown()))`    | text    | **hidden** (complex structure) | 🚩 Not user-editable via config panel |
| `maxWorkers`    | `z.number()`                        | number  | number                         | ✅                                    |
| `errorStrategy` | `z.enum(["failFast","collectAll"])` | select  | select                         | ✅                                    |

**Status: 2/3 correct.** `tasks` should be hidden or handled by the visual editor structure.

---

## Summary: What Needs Building

### Priority 1 — v1 Blockers (Sprint 5)

| Component          | Where                       | What it does                    | Params it fixes                                      | Reference Pattern                                                              |
| ------------------ | --------------------------- | ------------------------------- | ---------------------------------------------------- | ------------------------------------------------------------------------------ |
| **Textarea**       | `@bnto/ui` + `@bnto/editor` | Multiline text input            | Transform `expression`, interim for Loop expressions | `shadcn-blocks/components/textarea/`                                           |
| **TagPicker**      | `@bnto/ui` + `@bnto/editor` | Chip input with add/remove tags | Input `extensions`                                   | `shadcn-blocks/components/combobox/combobox-multi-select-1.tsx`                |
| **KeyValueEditor** | `@bnto/ui` + `@bnto/editor` | Add/remove key→value pairs      | Spreadsheet `columns`, Transform `mappings`          | Adapt from `shadcn-blocks/components/input/input-special-1.tsx` (list pattern) |

### Priority 2 — Expression Input (Sprint 5 or fast-follow)

| Component           | Where          | What it does                                                                     | Params it fixes                                                                     | Reference Pattern                     |
| ------------------- | -------------- | -------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- | ------------------------------------- |
| **ExpressionInput** | `@bnto/editor` | Rich text with `{{var}}` rendered as visual pill tokens, variable picker popover | Loop `items`/`condition`/`breakCondition`, Output `filename`, File-system `pattern` | See `strategy/expression-input-ux.md` |

### Priority 3 — Cleanup

| Change                     | Where                         | What                                                                                                                         |
| -------------------------- | ----------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| **Hide `accept`**          | `@bnto/nodes` schema metadata | Add `hidden: true` to Input node `accept` param. Derive MIME types from `extensions` internally                              |
| **Hide `tasks`**           | `@bnto/nodes` schema metadata | Add `hidden: true` to Parallel node `tasks` param. Structure managed by visual editor                                        |
| **Add `options` metadata** | `@bnto/nodes` schemas         | Add human-readable labels to all `z.enum()` params that currently show raw values (Loop `mode`, Input `mode`, Output `mode`) |

---

## Adding a New Control

When a new Zod type or UX need arises:

1. **Build the UI primitive** in `@bnto/ui/interaction/` (e.g., `Textarea.tsx`, `TagPicker.tsx`)
2. **Add a Motorway showcase** in `apps/web/app/(dev)/motorway/` — every control gets its own `ShowcaseSection` under the Controls tab. Column layout showing variants, states, and edge cases. This is the living catalog — if it's not on Motorway, it doesn't exist.
3. **Create the control component** in `@bnto/editor/components/controls/` (e.g., `TextareaControl.tsx`)
4. **Register it** in `@bnto/editor/components/controls/index.ts` — add to `CONTROL_REGISTRY`
5. **Add inference** in `@bnto/nodes/schemas/inferFieldType.ts` — map the Zod type to the new `FieldControl`
6. **Update this matrix** — add the control to the "Available Controls" table, update affected params
7. **Test** — unit test the control, verify it renders for the expected params

## Adding a New Node Type

When adding a new node type with parameters:

1. **Define the Zod schema** in `@bnto/nodes/schemas/{nodeType}.ts`
2. **Check every param against this matrix** — does a suitable control exist?
3. **If not, build the control first** (see "Adding a New Control" above)
4. **Add `NodeParamMeta`** with labels, descriptions, visibility conditions, and control hints
5. **Add a row to this matrix** for the new node type
6. **Test** — verify the config panel renders all params with correct controls

---

## Reference: shadcn-blocks Patterns

These are available in `/Users/ryan/Code/shadcn-blocks` as implementation references:

| Pattern                  | File                                               | Use for                                   |
| ------------------------ | -------------------------------------------------- | ----------------------------------------- |
| Multi-select with badges | `components/combobox/combobox-multi-select-1.tsx`  | TagPicker base                            |
| Field wrappers           | `components/field/field-advanced-1.tsx`            | Label + description + input composition   |
| Input groups             | `components/input-group/input-group-buttons-1.tsx` | Composite inputs (input + action buttons) |
| File/array lists         | `components/input/input-special-1.tsx`             | Array item list with remove buttons       |
| Textarea                 | `components/textarea/textarea-form-1.tsx`          | Multiline text base                       |
| Form + Zod               | `components/form/form-advanced-*.tsx`              | React Hook Form + Zod patterns            |
