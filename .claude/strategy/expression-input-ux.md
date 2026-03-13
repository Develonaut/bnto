# Expression Input UX — Strategy

**Created:** March 13, 2026
**Status:** Research — no implementation planned until Tier 3+ nodes ship
**Related:** [visual-editor.md](visual-editor.md), [node-responsibilities.md](../rules/node-responsibilities.md), [code-editor.md](code-editor.md), [io-nodes.md](io-nodes.md)

---

## Problem

Template expression fields are a usability cliff. The config panel renders them as plain `<Input>` elements with a placeholder hint — the user sees `{{name}}-compressed.{{ext}}` as grayed-out text and has to figure out the syntax, available variables, and escaping rules on their own.

This is fine for Tier 1-2 recipes where structured controls (sliders, selects, switches) handle everything. It becomes untenable when `transform`, `http-request`, and `ai` nodes ship — those require users to reference upstream outputs, write conditional expressions, and compose multi-variable templates.

**The gap:** Users are writing code in a text field with zero editor assistance. No autocomplete, no variable discovery, no syntax highlighting, no validation feedback, no indication of what variables are even available.

---

## Current State

### What the user sees today

The rendering chain for a template field:

```
Engine ParameterDef (placeholder: "{{name}}-compressed.{{ext}}")
  → codegen → @bnto/nodes NodeSchemaDefinition (params.pattern.placeholder)
    → SchemaField dispatches to TextControl
      → <Input placeholder="{{name}}-compressed.{{ext}}" />
```

`TextControl` (`packages/editor/src/components/controls/TextControl.tsx`) is 20 lines — a plain text `<Input>` that passes the placeholder through. No parsing, no token rendering, no validation.

### Template syntax patterns in the codebase

| Pattern | Example | Where used |
|---------|---------|------------|
| **Variable substitution** | `{{name}}`, `{{ext}}`, `{{index}}`, `{{date}}` | File rename pattern |
| **Context references** | `{{.INPUT_DIR}}`, `{{.OUTPUT_DIR}}`, `{{.item}}` | Go engine path templates |
| **Function calls** | `{{index . "list-files" "files"}}` | Loop item source |
| **Compound templates** | `{{.OUTPUT_DIR}}/renamed-{{basename .item}}` | Multi-expression paths |
| **Expr conditions** | `counter < 10`, `item.status == 'done'` | Loop break conditions |

Today, users see these as opaque strings. There's no way to know what variables are available without reading documentation or the placeholder text.

---

## Engine Gap

`ParameterDef` (`engine/crates/bnto-core/src/metadata.rs`) has `placeholder` and `description` as free-text hints, but no machine-readable field that declares which template variables a parameter accepts. The UI has no structured data to power autocomplete or validation.

### What's missing from `ParameterDef`

```rust
// Current — placeholder is a free-text hint
pub placeholder: Option<String>,  // "{{name}}-compressed.{{ext}}"

// Future — machine-readable variable declarations
pub template_variables: Option<Vec<TemplateVariable>>,
```

Where `TemplateVariable` would declare:

| Field | Type | Purpose |
|-------|------|---------|
| `name` | `String` | Variable name (`name`, `ext`, `index`) |
| `label` | `String` | Human-readable label ("Original filename") |
| `description` | `String` | What this variable contains |
| `source` | `VariableSource` | Where it comes from (file metadata, upstream node output, loop context) |
| `example` | `Option<String>` | Example value ("photo-001") |

This metadata would flow through the same codegen pipeline as everything else: engine `metadata()` impl → `catalog.snapshot.json` → `generate-from-catalog.ts` → TypeScript types. The editor reads from generated types, never hardcodes variable lists.

**Important:** This is a description of the gap, not a proposal to implement it now. The engine change is trivial once the UI design is settled — add the field, populate it in each processor's `metadata()`, regenerate.

---

## Competitor Analysis

Every major workflow/automation tool has solved expression inputs. The solutions cluster into two patterns: **pill tokens** (visual, mainstream) and **code editors** (developer-focused).

### Pill Token Pattern (mainstream tools)

| Tool | Input UX | Variable Discovery | Progressive Disclosure |
|------|----------|-------------------|----------------------|
| **Zapier** | Pill tokens inline in text field. Click a pill to insert `{{step.field}}`. Pills are color-coded by source step. | Data picker panel — tree view of upstream step outputs. Search + filter. | Fields are simple text by default. "Use a Custom Value" reveals the token picker. |
| **Make.com** | Color-coded pill bubbles inside a rich text input. Each pill shows the source module name + field path. Drag-and-drop from mapping panel. | Right-side mapping panel always visible during field editing. Shows all available modules and their outputs as a navigable tree. | Toggle between "Map" mode (pills + expressions) and plain text. Simple fields start as plain text. |
| **Apple Shortcuts** | Magic Variables — tappable pill tokens with icons. Variables auto-suggested from upstream actions. Long-press to drill into fields. | Inline suggestions bar above keyboard. "Select Magic Variable" view shows all available variables with previews of their current values. | Actions show structured controls by default. Variables only appear when the user taps into a field that accepts them. |
| **Power Automate** | `/` shortcut opens dynamic content picker with type filtering. Tokens insert as labeled pills. Expression editor for complex formulas. | Dynamic content panel — grouped by trigger/action, searchable, shows data type. Expression tab for functions (`concat()`, `if()`, etc.). | Simple fields stay simple. "Add dynamic content" button or `/` shortcut reveals the picker only when needed. |

### Code Editor Pattern (developer tools)

| Tool | Input UX | Variable Discovery | Progressive Disclosure |
|------|----------|-------------------|----------------------|
| **n8n** | Fixed/Expression toggle per field. Fixed = structured control (dropdown, number). Expression = Monaco-like code editor with syntax highlighting. | Expression editor shows available variables as completions. `$json`, `$node["name"].json`, `$input` etc. | Every field has a small toggle icon. Most users stay in Fixed mode. Expression mode is explicitly opt-in. |
| **Retool** | Every input is a code-aware text field with `{{ }}` syntax highlighting. Autocomplete powered by the app's data model. | Autocomplete dropdown with all available variables, queries, and functions. IDE-style documentation popover. | No progressive disclosure — every field is always expression-capable. Assumes developer audience. |

### Key Patterns

1. **Pill tokens are the dominant pattern.** Zapier, Make.com, Apple Shortcuts, and Power Automate all use visual pill tokens — not raw text with template syntax. Users never type `{{variable_name}}` by hand.

2. **Variable picker is always a panel or popover, not inline autocomplete.** The variable set is small enough that a browsable tree/list is more useful than fuzzy autocomplete. Users need to discover what's available, not just type faster.

3. **Progressive disclosure is universal.** Simple fields start as structured controls (sliders, selects). Expression/template mode is opt-in — a toggle, a button, or a shortcut reveals it. Casual users never see template syntax.

4. **n8n's Fixed/Expression toggle is the cleanest model** for tools that have both structured configs and expression fields. It's explicit, per-field, and reversible. The user chooses their level of power.

---

## Recommended Approach for Bnto

### Why bnto can be simpler

Most workflow tools need complex expression inputs because their entire config surface is key-value pairs and template strings. Bnto is different:

1. **Structured controls dominate.** Tier 1-2 recipes use sliders (quality), selects (format), switches (trim whitespace). The config panel is already purpose-built with typed controls from engine metadata. Template fields are the exception, not the rule.

2. **Linear pipelines.** Bnto recipes are predominantly linear — each node feeds the next. The variable namespace is small: current file metadata + upstream node outputs. Compare to Zapier where any of 50 steps could be referenced.

3. **File-centric.** Most template variables describe file properties (`name`, `ext`, `index`, `date`, `size`). This is a compact, predictable set — not an arbitrary data model.

4. **Code editor escape hatch exists.** The JSON code editor (CodeMirror 6) is already the power-user mode. Users who want full control over template syntax can switch to it. The visual editor's expression input only needs to handle the 80% case.

### Proposed UX layers

```
Layer 1: Structured controls (current — sliders, selects, switches)
  ↓ most users stop here
Layer 2: Pill token input + variable picker (proposed — for template fields)
  ↓ power users who need custom expressions
Layer 3: Expression toggle (proposed — per-field opt-in to raw expression)
  ↓ developers who want full control
Layer 4: JSON code editor (current — CodeMirror 6, full definition editing)
```

### Layer 2: Pill Token Input

Replace `TextControl` for template-capable parameters with a rich input that:

- Renders `{{name}}` as a visual pill token (labeled, color-coded by source)
- Clicking into the field opens a **variable picker popover** showing available variables grouped by source (file metadata, upstream node outputs, loop context)
- Selecting a variable inserts a pill token at the cursor position
- Free text between pills is preserved (e.g., `[pill:name]` `-compressed.` `[pill:ext]`)
- Backspace on a pill selects it; second backspace deletes it
- The underlying value is still a template string (`{{name}}-compressed.{{ext}}`) — pills are a rendering concern, not a data model change

**Variable picker layout:**

```
┌─────────────────────────┐
│ Search variables...     │
├─────────────────────────┤
│ FILE                    │
│  name     Original name │
│  ext      Extension     │
│  index    File number   │
│  date     Current date  │
│  size     File size     │
├─────────────────────────┤
│ UPSTREAM (compress)     │
│  output   Compressed    │
│  quality  Quality used  │
└─────────────────────────┘
```

### Layer 3: Expression Toggle

For fields that support it, add an n8n-style Fixed/Expression toggle:

- **Fixed mode** (default): The structured control renders normally (slider, select, etc.)
- **Expression mode**: The field switches to the pill token input, pre-populated with the current value as a literal

This is per-field, not per-node. A user might set quality with a slider (fixed) but use an expression for the output filename (expression). The toggle is a small icon at the field's trailing edge — unobtrusive, discoverable on hover.

### What NOT to build

- **Full formula language.** Bnto is not a spreadsheet. No `IF()`, `CONCAT()`, `VLOOKUP()`. The template syntax is `{{variable}}` with optional Go-style functions — simple enough to type in the rare cases where pills don't suffice.
- **Drag-and-drop from a mapping panel.** Make.com's always-visible mapping panel is overkill for bnto's linear pipelines. A popover variable picker is sufficient.
- **Per-step color coding.** With 3-5 nodes in a typical recipe, color-coding pills by source step adds visual noise without meaningful disambiguation. Group labels in the picker ("FILE", "UPSTREAM (compress)") are enough.
- **Expression validation/linting in the visual editor.** The code editor handles this via JSON Schema. The visual editor's pill input validates structurally (known variables only), not syntactically.

---

## Engine Changes Needed

When this feature is built, the engine needs one addition to `ParameterDef`:

```rust
/// Template variables this parameter accepts.
///
/// When set, the UI renders a pill token input instead of a plain text field.
/// Each variable declares its name, label, source, and an example value.
/// The editor uses this to power autocomplete and variable picker UI.
///
/// `None` means the parameter doesn't accept template expressions —
/// it renders as a plain text/number/select control.
#[serde(skip_serializing_if = "Option::is_none")]
pub template_variables: Option<Vec<TemplateVariable>>,
```

This follows the established pattern: engine declares capabilities, codegen bridges to TypeScript, editor reads from generated types. No hardcoded variable lists in the UI.

### Variable sources

| Source | Description | Available when |
|--------|-------------|---------------|
| `FileMetadata` | Properties of the current input file (name, ext, size, index) | Always — every recipe processes files |
| `UpstreamOutput` | Output fields from a previous node in the pipeline | Node has upstream connections |
| `LoopContext` | Loop iteration variables (item, index, counter) | Parameter belongs to a node inside a loop container |
| `Global` | Recipe-level variables (INPUT_DIR, OUTPUT_DIR) | Always (Go engine path conventions) |

---

## Phased Rollout

### Phase 1: Current (no expressions needed)

**Tier 1-2 recipes** (compress, resize, convert, clean CSV, rename files, generate thumbnails). All config is handled by structured controls — sliders, selects, switches. Template fields exist but are hidden (`hidden: true`) or pre-filled with defaults. Users never write expressions.

**No work needed.** The current `TextControl` with placeholder hints is sufficient because users don't interact with template fields directly.

### Phase 2: Pill tokens (when transform/http nodes ship)

**Tier 3 recipes** (watermark images, strip metadata, API calls). Users need to reference upstream outputs — "use the filename from the compress step as the watermark text." This is the inflection point where plain text inputs fail.

**Build:**
- `TemplateVariable` struct in engine `ParameterDef`
- `ExpressionInput` component (pill token rendering + variable picker popover)
- `SchemaField` dispatch: if `template_variables` is set, render `ExpressionInput` instead of `TextControl`
- Fixed/Expression toggle for fields that support both structured control and expression mode

### Phase 3: Full expression support (when ai nodes ship)

**Tier 4 recipes** (AI-powered transforms, conditional routing, dynamic configs). Users need conditional expressions, function calls, and complex variable references.

**Build:**
- Expression language documentation integrated into variable picker (function reference tab)
- Expression validation feedback (red underline for unknown variables, type mismatches)
- Autocomplete for function names and variable paths (beyond simple pill insertion)
- Consider whether the code editor's CM6 infrastructure can be reused for inline expression editing

---

## Open Questions

1. **Should pill tokens be editable?** Zapier's pills are opaque — click inserts, backspace deletes. Make.com allows editing the variable path inside the pill. Opaque is simpler and sufficient for bnto's small variable namespace.

2. **How do pills serialize?** The underlying value stays a template string (`{{name}}-compressed.{{ext}}`). Pills are a rendering concern — parse on mount, serialize on change. No data model change needed.

3. **What about the Go engine's template syntax?** The Go engine uses `{{.INPUT_DIR}}` (dot prefix) and function calls (`{{index . "node" "field"}}`). The Rust WASM engine bypasses templates entirely — files are `ArrayBuffer` blobs. When cloud execution (M4) ships, the expression input needs to handle both syntaxes. For now, this is a future consideration.

4. **Should the variable picker show live values?** Apple Shortcuts shows the current value of each magic variable. This requires execution state — the variable picker would need to know what files are loaded and what upstream nodes have produced. Useful but complex. Defer to Phase 3.

---

## References

- [Zapier Field Mapping](https://zapier.com/help/create/customize/add-data-fields-to-actions) — pill token insertion, data picker panel
- [Make.com Mapping Panel](https://www.make.com/en/help/mapping) — color-coded pills, drag-and-drop, always-visible panel
- [n8n Expressions](https://docs.n8n.io/code/expressions/) — Fixed/Expression toggle, Monaco-based expression editor
- [Apple Shortcuts Magic Variables](https://support.apple.com/guide/shortcuts/magic-variables) — inline suggestions, long-press drill-down
- [Power Automate Dynamic Content](https://learn.microsoft.com/en-us/power-automate/use-expressions-in-conditions) — `/` shortcut, type-filtered picker
- [Retool Bindings](https://docs.retool.com/docs/binding-data) — always-on expression support, autocomplete
