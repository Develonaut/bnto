# Recipe Editors — Unified Strategy

**Created:** April 16, 2026
**Status:** Planning — defining the editor experience across Web and TUI
**Related:** [editor-architecture.md](editor-architecture.md), [tui-strategy.md](tui-strategy.md), [config-controls.md](config-controls.md), [visual-editor.md](visual-editor.md), [code-editor.md](code-editor.md)

---

## Why This Document Exists

The editor is the product. If creating or editing a recipe feels like work — if there's a moment of confusion, a dead end, a "what do I do now?" — we lose people. This document defines the universal editor experience across Web and TUI: what editor types exist, how they work, when each one shines, and how they all share the same foundation.

The goal is the "it just works" feeling. Someone who has never seen bnto should be able to open the editor, understand what a recipe does, and modify it — or build one from scratch — without reading documentation.

---

## Design Philosophy

### Progressive Disclosure Is Everything

The #1 enemy of editor UX is **showing everything at once**. A new user staring at a canvas full of nodes, each with a dozen configuration fields, will close the tab. A power user staring at a wizard that asks them 8 questions before they can change one parameter will close the tab.

**The solution:** Start simple. Reveal complexity as the user asks for it.

| Layer        | What the user sees                          | When it appears                   |
| ------------ | ------------------------------------------- | --------------------------------- |
| **Glance**   | Recipe name + what it does in plain English | Always visible                    |
| **Steps**    | Ordered list of nodes with labels           | Default view (List editor)        |
| **Config**   | Editable parameters for one node            | On selection (click/Enter)        |
| **Advanced** | Visibility conditions, expressions, JSON    | On demand (toggle/switch to Code) |

Every interaction should feel like zooming in, not like switching contexts.

### Sensible Defaults Eliminate Decisions

Every parameter ships with a default from the engine's `ParameterDef`. When a user adds an "image-compress" node, quality is already 80, format is already JPEG. They can run the recipe immediately without configuring anything. Configuration is optimization, not setup.

### Inline Guidance, Not Documentation

The editor never says "see docs." Instead:

- Parameter labels and descriptions come from `ParameterDef.label` and `ParameterDef.description`
- Presets offer named quick-picks ("Draft | Balanced | Maximum" for quality)
- Placeholder text shows examples ("e.g., 800" for width)
- Constraints are communicated through the control itself (slider range, not an error message)

### Undo Everything

Every editor type supports undo/redo. There is no action the user can take that they can't reverse. This eliminates fear of experimentation — "just try it, you can always undo."

### No Dead Ends

Every screen has a clear next action. An empty editor says "Add a step" with a single button. A configured recipe says "Run" prominently. After execution, results show "Run again" or "Edit recipe." The user is never left staring at a blank screen wondering what to do next.

---

## The Five User Tasks

Everything we build serves these five tasks. Each editor type must handle all five, but each has natural strengths.

| Task            | What the user is doing                           | Success =                                           |
| --------------- | ------------------------------------------------ | --------------------------------------------------- |
| **Create**      | Building a new recipe from nothing               | A valid `.bnto.json` with sensible defaults         |
| **Understand**  | Reading an existing recipe to grasp what it does | "I get it" in under 5 seconds                       |
| **Configure**   | Tweaking parameters on an existing recipe        | Changed quality from 80 to 60, done                 |
| **Restructure** | Adding, removing, or reordering nodes            | Added a resize step before compress                 |
| **Fork**        | Taking a recipe and making it their own          | "Compress for Web" becomes "Compress for Instagram" |

---

## Four Editor Types

Each editor is a **view** of the same underlying recipe state. Users switch between them freely — no data migration, no loss. The shared foundation (Zustand store on web, struct-based state in TUI) ensures consistency.

### Overview

| Editor     | Primary Metaphor     | Complexity Level | Best For                                      | Web Tech                     | TUI Tech                  |
| ---------- | -------------------- | ---------------- | --------------------------------------------- | ---------------------------- | ------------------------- |
| **List**   | Playlist / step list | Low              | Understand, Configure, Restructure            | React sortable list          | ratatui navigable list    |
| **Wizard** | Guided questionnaire | Lowest           | Create, Fork                                  | Step-by-step form            | `dialoguer`-style prompts |
| **Visual** | Spatial canvas       | Medium-High      | Restructure (complex), Understand (branching) | React Flow (`@xyflow/react`) | Read-only ASCII graph     |
| **Code**   | Text editor          | Highest          | All (power users)                             | CodeMirror 6                 | `$EDITOR` / embedded      |

### Default Editor by Context

| Context                           | Default Editor         | Why                                                     |
| --------------------------------- | ---------------------- | ------------------------------------------------------- |
| "New recipe" (from scratch)       | **Wizard**             | Zero decisions upfront — walk the user through it       |
| "New recipe" (from template/fork) | **List**               | Template is already populated — user reviews and tweaks |
| "Edit existing recipe"            | **List**               | Most recipes are linear — list is the fastest overview  |
| "View recipe details" (read-only) | **List**               | Instant comprehension — scan the steps                  |
| Power user preference             | **Code** or **Visual** | Remembered per-user, sticky across sessions             |

### Editor Switching

The user can switch editors at any time via a mode toggle. The toggle is always visible (toolbar on web, status bar key hint in TUI).

**Web:** Segmented control in the editor toolbar — `List | Visual | Code`. Clicking switches the active editor component. The store doesn't change. Split view (`List + Code`, `Visual + Code`) is supported.

**TUI:** Key binding (e.g., `Tab` or number keys `1/2/3/4`) switches the active screen. A status line shows which editor is active: `[L]ist  [V]isual  [C]ode`.

---

## Editor 1: List

**The center of gravity.** This is the editor most users see most of the time. It answers the question "what does this recipe do?" in one glance, and lets you change anything with one click.

### What It Looks Like

```
Web:
┌─────────────────────────────────────────────┐
│  Compress Images for Web                     │
│  Reduce image size while maintaining quality │
│                                              │
│  ┌─ 1. Input ─────────────────────────────┐ │
│  │  📥 File Upload                         │ │
│  │  Accepts: png, jpg, webp               │ │
│  └────────────────────────────────────────┘ │
│                                              │
│  ┌─ 2. Compress ──────────────── ✏️ ──────┐ │
│  │  🖼️ Compress Images                     │ │
│  │  Quality: ███████████░░░ 80%           │ │
│  │  ▸ 3 more settings                     │ │
│  └────────────────────────────────────────┘ │
│                                              │
│  ┌─ 3. Output ────────────────────────────┐ │
│  │  📤 Download                            │ │
│  │  Format: Original                      │ │
│  └────────────────────────────────────────┘ │
│                                              │
│  [ + Add Step ]                              │
│                                              │
│           [ ▶ Run ]                          │
└─────────────────────────────────────────────┘

TUI:
╭─ Compress Images for Web ───────────────────╮
│                                              │
│  1. 📥 Input                                 │
│     File Upload · png, jpg, webp            │
│                                              │
│ ▸2. 🖼️  Compress Images           quality 80 │
│     ▸ 3 more settings                       │
│                                              │
│  3. 📤 Output                                │
│     Download · Original format              │
│                                              │
╰──────────────────────────────────────────────╯
  j/k navigate  Enter expand  a add  d delete  r run
```

### Interaction Model

| Action              | Web                                 | TUI                                             | Shared Concept                         |
| ------------------- | ----------------------------------- | ----------------------------------------------- | -------------------------------------- |
| **Navigate**        | Click a step, or arrow keys         | `j`/`k` (vim) or arrow keys                     | Cursor highlights one step             |
| **Expand/collapse** | Click step or chevron               | `Enter` or `l`                                  | Toggle inline config panel             |
| **Configure**       | Edit form controls inline           | Edit values inline (text input, select, toggle) | Schema-driven form from `ParameterDef` |
| **Reorder**         | Drag handle, or `Shift+↑`/`Shift+↓` | `Shift+J`/`Shift+K` or `K`/`J` (move mode)      | Swap step with neighbor                |
| **Add step**        | Click "+ Add Step" → node picker    | `a` → node picker overlay                       | Browse by category, search by name     |
| **Remove step**     | Select → Delete key or trash icon   | `d` on highlighted step → confirm               | Remove + reflow remaining steps        |
| **Quick edit**      | Inline — click a value to edit      | `e` on highlighted param → type → Enter         | Single-value edit without expanding    |
| **Undo/Redo**       | `Cmd+Z` / `Cmd+Shift+Z`             | `u` / `Ctrl+R`                                  | Shared undo stack in store/state       |

### Expanded Step (Inline Config)

When a step is expanded, its configurable parameters appear inline — no separate panel, no modal. The controls are driven by the engine's `ParameterDef`:

```
Web (expanded):
┌─ 2. Compress ──────────────────────────────┐
│  🖼️ Compress Images                         │
│                                              │
│  Quality          ███████████░░░ 80%        │
│                   Draft | Balanced | Maximum│
│                                              │
│  Format           [JPEG ▾]                  │
│                                              │
│  ▸ Show advanced (width, height, aspect)    │
└──────────────────────────────────────────────┘

TUI (expanded):
  2. 🖼️  Compress Images
     Quality       ██████████░░░░░░ 80%
                   [Draft] [Balanced] [Maximum]
     Format        JPEG ▾
     ▸ Show 3 advanced settings
```

### What Makes It Great

- **Instant comprehension.** A vertical list of labeled steps. Top to bottom, done.
- **Inline editing.** No context switch to configure a node. Expand, tweak, collapse.
- **Works for 90% of recipes.** Most recipes are linear pipelines (A → B → C). The list IS the pipeline.
- **Progressive disclosure.** Collapsed = overview. Expanded = config. "Show advanced" = power user.
- **Portable.** A list looks like a list in a browser and in a terminal. Same mental model, same keyboard shortcuts.

### Handling Non-Linear Recipes

Container nodes (group, loop, parallel) render as nested/indented sub-lists:

```
  1. 📥 Input
  2. 🔄 Loop (for each file)
     ├─ 2a. 🖼️ Compress
     ├─ 2b. 🖼️ Resize
     └─ 2c. 📁 Rename
  3. 📤 Output
```

This preserves the list metaphor while showing structure. Deeply nested containers (loop inside group inside parallel) would be hard to read — but that's a power-user scenario better served by the Visual or Code editor.

---

## Editor 2: Wizard

**The on-ramp.** The wizard is how every new recipe starts. It asks questions, the user answers, and a valid recipe appears at the end. Zero blank-canvas anxiety.

### Design Principles

- **One question at a time.** Never show a form with 10 fields. Show one question, get one answer, advance.
- **Smart defaults.** "What kind of files?" → Images is pre-selected if the user came from an image recipe page.
- **Skip when possible.** If a question has only one sensible answer, skip it entirely.
- **Exit early.** At any point, "Done — use defaults for the rest" creates a valid recipe.
- **End in the List editor.** The wizard produces a recipe and drops the user into the List editor to review and refine.

### Flow

```
Step 1: What do you want to do?
  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐
  │  🖼️ Images    │  │  📊 CSV Data  │  │  📁 Files     │
  │  Compress,   │  │  Clean,      │  │  Rename,     │
  │  resize,     │  │  convert,    │  │  organize    │
  │  convert     │  │  merge       │  │              │
  └──────────────┘  └──────────────┘  └──────────────┘
  (or: search "compress")

Step 2: Which operation?
  ○ Compress — Reduce file size
  ● Resize — Change dimensions
  ○ Convert — Change format (PNG → WebP)

Step 3: Configure (contextual)
  Max width: [1920]  px
  Maintain aspect ratio: [✓]

Step 4: Output
  ○ Download files
  ● Save to folder
  ○ Preview in browser

→ Recipe created! Opening in List editor...
```

### Interaction Model

| Action            | Web                          | TUI                         | Shared Concept                        |
| ----------------- | ---------------------------- | --------------------------- | ------------------------------------- |
| **Select option** | Click card or radio button   | Arrow keys + Enter          | Single choice from options            |
| **Type value**    | Input field with placeholder | Text input with placeholder | Validated on advance                  |
| **Advance**       | Click "Next" or Enter        | Enter                       | Move to next question                 |
| **Go back**       | Click "Back" or breadcrumb   | Backspace or Esc            | Return to previous question           |
| **Skip to end**   | "Use defaults" link          | `s` (skip)                  | Create recipe with remaining defaults |
| **Cancel**        | "Cancel" or Esc              | `q` or Esc                  | Return to previous screen             |

### Wizard Steps Per Category

The wizard is **category-aware**. The engine's `NodeTypeInfo` and `ParameterDef` metadata drive which questions appear.

| Category        | Step 1              | Step 2                                                      | Step 3                                                  | Step 4      |
| --------------- | ------------------- | ----------------------------------------------------------- | ------------------------------------------------------- | ----------- |
| **Image**       | "Images" selected   | Pick operation (compress/resize/convert/strip-exif/overlay) | Operation-specific params (quality, dimensions, format) | Output mode |
| **Spreadsheet** | "CSV Data" selected | Pick operation (clean/rename/convert/merge)                 | Operation-specific params (trim, dedup, columns)        | Output mode |
| **File**        | "Files" selected    | Pick operation (rename)                                     | Rename params (find/replace, case, pattern)             | Output mode |
| **Multi-step**  | Pick first category | Configure first step                                        | "Add another step?" → repeat                            | Output mode |

### What Makes It Great

- **Zero blank-canvas anxiety.** The user never faces an empty editor. They answer questions and a recipe materializes.
- **Impossible to create an invalid recipe.** Every choice leads to a valid state.
- **Discovery.** Users learn what bnto can do by browsing the wizard options.
- **Fast.** A simple 3-node recipe (input → process → output) takes 4 clicks/keypresses.

---

## Editor 3: Visual (Canvas)

**The spatial thinker's tool.** Nodes arranged on a 2D canvas. Click to configure, drag to rearrange. The "wow factor" editor that makes bnto look like a real workflow tool.

### Design Principles

- **Bento box metaphor.** Compartments in a box, not wires in a graph. Position implies order, not explicit connections.
- **No edges.** Execution order = position order (top-left to bottom-right). No spaghetti wires, no connection validation.
- **Elevation = status.** Surface Cards rise during execution (idle → pending → active → completed). The "building materializing" effect from Motorway's spring system.

### What It Looks Like

```
Web:
┌─────────────────────────────────────────────────┐
│  [Palette]  [List | Visual | Code]  [Run]       │
├──────┬──────────────────────────────────────────┤
│      │                                          │
│ 📥   │  ┌──────────┐  ┌──────────────────┐     │
│ Input│  │  📥       │  │  🖼️              │     │
│      │  │  Input    │  │  Compress        │     │
│ 🖼️   │  │  images   │  │  quality: 80     │     │
│ Image│  └──────────┘  └──────────────────┘     │
│      │                                          │
│ 📊   │  ┌──────────────────┐  ┌──────────┐     │
│ CSV  │  │  🖼️              │  │  📤       │     │
│      │  │  Resize          │  │  Output   │     │
│ 📁   │  │  1920 x auto     │  │  download │     │
│ File │  └──────────────────┘  └──────────┘     │
│      │                                          │
│      │        [ + Add Node ]                    │
│      │                                          │
└──────┴──────────────────────────────────────────┘

TUI (read-only overview):
╭─ Compress Images for Web ───────────────────╮
│                                              │
│  ┌────────┐   ┌──────────┐   ┌────────┐    │
│  │ Input  │──▸│ Compress │──▸│ Output │    │
│  │ images │   │ q: 80    │   │download│    │
│  └────────┘   └──────────┘   └────────┘    │
│                                              │
╰──────────────────────────────────────────────╯
  ← visual overview (read-only) · press L for list
```

### Interaction Model

| Action          | Web                                 | TUI                                 |
| --------------- | ----------------------------------- | ----------------------------------- |
| **Select node** | Click compartment                   | Arrow keys + Enter                  |
| **Configure**   | Config panel slides in from right   | Switch to List editor for config    |
| **Add node**    | Drag from palette, or click "+ Add" | `a` → picker → placed automatically |
| **Remove node** | Select + Delete key                 | `d` on selected node                |
| **Rearrange**   | Drag compartment to new position    | Not supported — use List editor     |
| **Zoom**        | Scroll wheel, pinch-to-zoom         | Not applicable                      |
| **Pan**         | Click + drag background             | Not applicable                      |

### TUI Limitations

The TUI's visual editor is intentionally **read-only** — a spatial overview for understanding complex recipes, not for editing. Editing happens in the List or Code editor. The visual view is a "map" — you look at it to understand the shape, then switch to List to make changes.

This is a deliberate choice: trying to make a fully interactive canvas in a terminal would be fighting the medium. The TUI's strength is keyboard-driven editing (List), not mouse-driven spatial arrangement.

### What Makes It Great

- **Instant understanding of complex recipes.** Branching, parallel execution, nested containers — all visible at a glance.
- **The "wow" factor.** Looks like a real workflow tool. Great for marketing, demos, screenshots.
- **Execution visualization.** Compartments physically rise during execution — satisfying, informative, delightful.

### When to Use Another Editor

The visual editor adds cognitive overhead for simple recipes. A 3-node linear pipeline doesn't benefit from a 2D canvas — the List editor shows it more clearly in less space. The visual editor shines when:

- The recipe has 5+ nodes
- There are container nodes (loops, groups, parallel)
- The user wants to understand the overall shape of a complex recipe

---

## Editor 4: Code

**The power user's escape hatch.** Raw `.bnto.json` with syntax highlighting, schema validation, and autocompletion. Full control, no abstraction.

### Design Principles

- **Schema-aware.** JSON Schema from the engine drives validation, autocompletion, and hover tooltips. The code editor knows what a valid recipe looks like.
- **Slash commands.** Type `/` to insert a node template — the Notion pattern. Bridges the gap between "code editor" and "visual editor" ergonomics.
- **Instant feedback.** Inline error squiggles, not post-save validation. The user knows immediately if something is wrong.

### What It Looks Like

```
Web (CodeMirror 6):
┌─────────────────────────────────────────────┐
│  root > nodes > [1] > parameters > quality  │  ← breadcrumb
├─────────────────────────────────────────────┤
│  1  {                                       │
│  2    "type": "group",                      │
│  3    "nodes": [                            │
│  4      {                                   │
│  5        "id": "input-1",                  │
│  6        "type": "input",                  │
│  7        "parameters": {                   │
│  8          "mode": "file-upload",          │
│  9          "extensions": ["png","jpg"]     │
│ 10        }                                 │
│ 11      },                                  │
│ 12      {                                   │
│ 13        "id": "compress-1",               │
│ 14        "type": "image-compress",         │
│ 15        "parameters": {                   │
│ 16          "quality": 80                   │
│ 17  ~~~~~~~~~~~~~~~~~~~~ ← squiggle         │
│ 18        }                                 │
│ 19      }                                   │
│ 20    ]                                     │
│ 21  }                                       │
└─────────────────────────────────────────────┘

TUI:
  Opens $EDITOR (vim, nano, etc.) with the .bnto.json file.
  On save+exit, validates and reloads into the TUI.
  Alternatively: `:edit` command opens embedded text view.
```

### Interaction Model

| Action           | Web                                  | TUI                                    |
| ---------------- | ------------------------------------ | -------------------------------------- |
| **Edit**         | Type in CodeMirror 6                 | Type in `$EDITOR`                      |
| **Autocomplete** | Schema-driven (Ctrl+Space or auto)   | N/A (relies on `$EDITOR` capabilities) |
| **Insert node**  | Slash command: `/image` → template   | N/A                                    |
| **Validate**     | Inline diagnostics (real-time)       | On save: validation pass before reload |
| **Format**       | Format on save (Cmd+S)               | `jq` or built-in formatter             |
| **Navigate**     | Click breadcrumb, Cmd+G (go to node) | `$EDITOR` features                     |

### What Makes It Great

- **Maximum control.** Everything is visible. Nothing is hidden behind UI abstractions.
- **Copy-paste.** Users can paste recipe JSON from docs, GitHub, or other users.
- **Version control friendly.** The `.bnto.json` is the recipe — users can diff, merge, and track changes.
- **CLI/TUI natural.** Terminal users think in files. Opening a JSON file in their editor is the most natural thing.

---

## Shared Foundation

All four editors share the same state and the same source of truth. Switching editors is instant — it's rendering a different component over the same data.

### Package Architecture

The editor is a **standalone library** on both platforms. It owns editing state, operations, and validation. Consumers (apps, CLI, TUI) feed it a recipe and get back the edited result. This centralizes all editing logic and ensures every consumer gets the same behavior.

```
Web:
  @bnto/editor (packages/editor/)
    ├── store/          — Zustand state (nodes, configs, undo, validation)
    ├── actions/        — Pure functions (addNode, removeNode, updateParam, reorder)
    ├── hooks/          — React bindings (thin wrappers over actions)
    ├── adapters/       — Definition ↔ editor state conversion
    └── index.ts

  Consumed by:
    apps/web           — List, Visual, Code, Wizard UI components
    apps/desktop       — Same React UI (future, Tauri)

Rust:
  bnto-editor (engine/crates/bnto-editor/)     ← NEW crate, to be extracted
    ├── state.rs        — EditorModel struct (nodes, configs, undo, validation)
    ├── operations.rs   — Pure functions (add_node, remove_node, update_param, reorder)
    ├── validation.rs   — Schema validation against ParameterDef
    ├── wizard.rs       — Wizard flow state machine
    └── lib.rs

  Consumed by:
    bnto (CLI crate)    — TUI screens import bnto-editor for editing
    bnto-wasm           — Could expose editor operations to web (future)
```

**Why a separate crate (not a module inside `bnto`)?** The CLI crate (`bnto`) is a binary — it handles argument parsing, TUI rendering, and OS interaction. The editor is a library — pure state management and operations with no I/O. Separating them follows the Bento Box Principle: the editor crate is testable in isolation (`cargo test -p bnto-editor`) and reusable by future consumers (desktop, WASM editor operations).

**Extraction timing:** The TUI's editor logic currently lives in `engine/crates/bnto/src/tui/screens/detail.rs` and `detail_loader.rs`. As we build the List editor and Wizard for TUI, the editing state and operations should be extracted into `bnto-editor`. Same pattern as the web: co-locate first, extract when the boundary is clear.

### State Layer

| Platform | State Container                       | Owned State                                                                                                                  |
| -------- | ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| **Web**  | Zustand store (`createEditorStore()`) | `nodes`, `edges`, `configs`, `definition`, `metadata`, `undoStack`, `redoStack`, `isDirty`, `validationErrors`               |
| **TUI**  | Rust struct (`EditorModel`)           | Equivalent fields: `nodes`, `configs`, `definition`, `metadata`, `undo_stack`, `redo_stack`, `is_dirty`, `validation_errors` |

### Operations (Platform-Agnostic)

These operations are the universal vocabulary. Every editor type ultimately dispatches one of these operations to the state layer.

| Operation                           | What it does                                        | Dispatched by                                                                      |
| ----------------------------------- | --------------------------------------------------- | ---------------------------------------------------------------------------------- |
| `addNode(type, position?)`          | Insert a new node with defaults from `ParameterDef` | Wizard (step selection), List (+ Add), Visual (palette drag), Code (slash command) |
| `removeNode(nodeId)`                | Remove node + cascade cleanup                       | List (delete key), Visual (delete key), Code (delete JSON block)                   |
| `updateParam(nodeId, param, value)` | Update a single parameter value                     | List (inline edit), Visual (config panel), Code (edit JSON value)                  |
| `moveNode(nodeId, newPosition)`     | Reorder node in the pipeline                        | List (drag/keyboard reorder), Visual (drag)                                        |
| `setMetadata(field, value)`         | Update recipe name, description, version            | All editors (metadata section)                                                     |
| `undo()` / `redo()`                 | Restore previous/next state snapshot                | All editors (keyboard shortcut)                                                    |
| `validate()`                        | Run validation against schema + cross-node rules    | Automatic on every mutation                                                        |
| `export()`                          | Serialize state to `.bnto.json`                     | All editors (export/save action)                                                   |

### Schema-Driven Config (The Engine Contract)

The engine's `ParameterDef` is the single source of truth for node configuration. Every editor type consumes it to render controls.

```
Engine (Rust)
  ParameterDef {
    name, label, description,
    param_type (Number | String | Boolean | Enum | Object | File),
    default, constraints (min/max/required),
    placeholder, visible_when, required_when,
    surfaceable,
    group, suffix, control, accept, presets, inverted
  }
      ↓ catalog.snapshot.json
      ↓ codegen (task nodes:generate)
TypeScript (@bnto/nodes)
  Generated Zod schemas + NODE_PARAM_FIELDS
      ↓
  @bnto/form → Web controls
  @bnto/editor → List/Visual config panels

Engine (Rust) → directly
  TUI detail screen → TUI controls
```

### Control Matrix: ParameterType → Platform Control

Every parameter type maps to a specific control on each platform:

| `ParameterType` | Has bounds? | Web Control           | TUI Control            | List Editor (inline)              |
| --------------- | ----------- | --------------------- | ---------------------- | --------------------------------- |
| `Number`        | min + max   | `Slider`              | `tui-slider`           | Slider with value label           |
| `Number`        | unbounded   | `Input type="number"` | Text input (validated) | Compact number input              |
| `String`        | —           | `Input`               | Text input             | Text input                        |
| `Boolean`       | —           | `Switch`              | `[x]`/`[ ]` toggle     | Toggle switch                     |
| `Enum`          | —           | `Select` dropdown     | Select list / cycle    | Dropdown or inline cycle          |
| `Object`        | —           | `KeyValueEditor`      | Read-only JSON         | "Edit" link → opens expanded view |
| `File`          | —           | File picker           | File picker screen     | "Choose file" button              |

### ParameterDef Metadata → UI Affordances (Cross-Platform)

| `ParameterDef` field   | Web affordance                                 | TUI affordance                           | List editor affordance                    |
| ---------------------- | ---------------------------------------------- | ---------------------------------------- | ----------------------------------------- |
| `label`                | Form field label                               | Param name in list                       | Step summary label                        |
| `description`          | Tooltip on hover                               | Help text below control / in status line | Tooltip on hover (web), status line (TUI) |
| `default`              | Pre-filled value                               | Pre-filled value in muted style          | Shown in step summary when unchanged      |
| `placeholder`          | Ghost text in input                            | Ghost text in input                      | Ghost text                                |
| `constraints.min/max`  | Slider bounds, input validation                | Slider bounds, input validation          | Same                                      |
| `constraints.required` | Asterisk marker, prevent empty submit          | Asterisk, prevent empty commit           | Asterisk                                  |
| `visible_when`         | Show/hide field reactively                     | Show/hide field reactively               | Show/hide in expanded view                |
| `required_when`        | Dynamic required marker                        | Dynamic required marker                  | Same                                      |
| `presets`              | Quick-pick chips below slider                  | Quick-pick chips below slider            | Inline preset buttons                     |
| `suffix`               | Unit label after value ("80%", "1920px")       | Unit label after value                   | Unit label                                |
| `group`                | Group related params under heading             | Group related params under heading       | Collapsed group                           |
| `control`              | Override control type (e.g., "file", "slider") | Override control type                    | Override control type                     |
| `inverted`             | Flip boolean display                           | Flip boolean display                     | Same                                      |
| `surfaceable`          | Show in config panel (true) or hide (false)    | Show in param list (true) or hide        | Show in expanded step (true) or hide      |

---

## The Node Picker (Shared Across Editors)

When the user adds a step/node, they see the **node picker** — a categorized browser of available node types. This is shared across List, Visual, and Wizard editors.

### What It Shows

From the engine's `NodeTypeInfo`:

- **Category grouping:** Image, Spreadsheet, File, Data, Network, Control, System, Vector, Video
- **Per node:** Icon (Lucide name), label, one-line description, platform badges
- **Search:** Fuzzy match on name + description

### Platform Rendering

| Platform         | Node Picker Style                             |
| ---------------- | --------------------------------------------- |
| **Web (List)**   | Popover panel anchored to "+ Add Step" button |
| **Web (Visual)** | Sidebar palette (always visible) or popover   |
| **Web (Code)**   | Slash command menu (inline in editor)         |
| **TUI**          | Full-screen overlay with categories + search  |

---

## First-Time Experience (FTX)

The first 30 seconds determine whether the user stays. Here's the ideal first-time flow:

### Web

1. User lands on a recipe page (e.g., `/recipes/compress-images`)
2. Clicks "Try it" → recipe loads in the List editor (read-only feel, with "Edit" affordance)
3. Can immediately run it by dragging files to the input area
4. After running, sees results → "Edit this recipe" opens the List editor in edit mode
5. Makes one change (quality slider) → runs again → sees the difference
6. Hooked.

### TUI

1. User runs `bnto tui`
2. Recipe browser shows predefined recipes with search
3. Selects "Compress Images" → detail screen shows config summary
4. Adjusts quality → selects files → runs
5. Sees results → "Edit this recipe" drops into the List editor
6. Hooked.

### Web: "New Recipe"

1. User clicks "New Recipe" → Wizard starts
2. "What kind of files?" → picks Images
3. "What do you want to do?" → picks Compress
4. "Quality?" → slides to 80 (default, can skip)
5. Done → drops into the List editor with a 3-node recipe (input → compress → output)
6. "Run" button is prominent. No dead end.

### TUI: "New Recipe"

1. User runs `bnto tui` → presses `n` for new
2. Wizard prompts: "Category?" → Images
3. "Operation?" → Compress
4. "Quality?" → accepts default 80
5. Done → drops into the List editor
6. `r` to run, `f` to pick files

---

## Competitive Analysis

What we can learn from products that nail editing UX:

| Product             | What they do well                                                           | What we steal                                                      |
| ------------------- | --------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| **Apple Shortcuts** | Vertical step list, expandable config, drag to reorder, "Add Action" search | List editor layout, inline expansion, category-based action picker |
| **Zapier**          | Numbered steps, test-as-you-build, clear data flow between steps            | Step numbering, inline testing per node                            |
| **n8n**             | Visual canvas + form-based config, JSON toggle per node                     | Per-node JSON toggle (show raw params as JSON)                     |
| **Raycast**         | Script command wizard, minimal questions, smart defaults                    | Wizard flow, skip-to-end with defaults                             |
| **Linear**          | Keyboard-first, `Cmd+K` for everything, instant transitions                 | TUI keyboard model, command palette                                |
| **Notion**          | Slash commands, blocks that compose, drag to reorder                        | Slash commands in Code editor, composable steps                    |
| **plop.js**         | Interactive prompts, template-driven generation                             | Wizard prompts in TUI, template-based creation                     |

### The Apple Shortcuts Parallel

Apple Shortcuts is the closest analog to what we're building. Their editing experience is:

1. **List of actions** (our List editor) — vertical, scrollable, each action expandable
2. **Action library** (our Node Picker) — categorized, searchable, with descriptions
3. **Per-action config** (our inline config) — schema-driven fields per action type
4. **Run** button always visible
5. **Gallery** of pre-built shortcuts (our predefined recipes)

They don't have a visual canvas or a code editor. For most users, the list + library is enough. We add the visual and code editors for power users who need them — but the list is the foundation.

---

## Editor Feature Matrix

Which features are available in which editor, on which platform:

| Feature            | List (Web)         | List (TUI)       | Wizard (Web)    | Wizard (TUI)    | Visual (Web)        | Visual (TUI) | Code (Web)    | Code (TUI) |
| ------------------ | ------------------ | ---------------- | --------------- | --------------- | ------------------- | ------------ | ------------- | ---------- |
| **View recipe**    | Full               | Full             | N/A             | N/A             | Full                | Read-only    | Full          | Full       |
| **Add node**       | Picker             | Picker           | Guided          | Guided          | Palette/Picker      | N/A          | Slash cmd     | Manual     |
| **Remove node**    | Delete key         | `d` key          | N/A             | N/A             | Delete key          | N/A          | Delete JSON   | Manual     |
| **Reorder**        | Drag / Shift+Arrow | Shift+J/K        | N/A             | N/A             | Drag                | N/A          | Cut/paste     | Manual     |
| **Configure node** | Inline expand      | Inline expand    | Step-by-step    | Step-by-step    | Side panel          | N/A (switch) | Edit JSON     | Edit JSON  |
| **Undo/Redo**      | Cmd+Z              | `u`/Ctrl+R       | Back button     | Backspace       | Cmd+Z               | N/A          | Cmd+Z         | `$EDITOR`  |
| **Search nodes**   | Picker search      | Picker search    | Category browse | Category browse | Palette search      | N/A          | N/A           | N/A        |
| **Validate**       | Real-time          | On action        | Per-step        | Per-step        | Real-time           | N/A          | Inline diag   | On save    |
| **Export**         | Toolbar            | Key binding      | End of wizard   | End of wizard   | Toolbar             | N/A          | Toolbar       | Save file  |
| **Execution viz**  | Step status icons  | Step status text | N/A             | N/A             | Elevation animation | N/A          | N/A           | N/A        |
| **Split view**     | + Code             | N/A              | N/A             | N/A             | + Code              | N/A          | + List/Visual | N/A        |

---

## Implementation Phasing

Build in order of user impact. The List editor is the highest-impact, most-universal feature.

### Phase 1: List Editor (both platforms)

**The foundation.** Ship this first on both web and TUI. Covers 90% of editing needs.

| Task                      | Platform | What                                                          |
| ------------------------- | -------- | ------------------------------------------------------------- |
| List view component       | Web      | Ordered step list with expand/collapse, driven by store state |
| Inline config rendering   | Web      | `ParameterDef` → form controls inside expanded steps          |
| Reorder (drag + keyboard) | Web      | DnD with `@dnd-kit` or similar, plus Shift+Arrow              |
| Add/Remove steps          | Web      | Node picker popover + delete with undo                        |
| List screen               | TUI      | Navigable list with `j`/`k`, expand with Enter, edit with `e` |
| Inline config rendering   | TUI      | `ParameterDef` → TUI controls inside expanded steps           |
| Reorder (keyboard)        | TUI      | `Shift+J`/`Shift+K` to swap                                   |
| Add/Remove steps          | TUI      | `a` → picker overlay, `d` → confirm delete                    |

### Phase 2: Wizard (both platforms)

**The on-ramp.** The Wizard ensures new users can create recipes without any prior knowledge.

| Task                         | Platform | What                                                                              |
| ---------------------------- | -------- | --------------------------------------------------------------------------------- |
| Wizard flow component        | Web      | Step-by-step form, one question at a time, category → operation → config → output |
| Category/operation selection | Web      | Card grid for categories, radio list for operations                               |
| Config step                  | Web      | Schema-driven form from `ParameterDef` (reuse List editor controls)               |
| Wizard-to-List handoff       | Web      | Wizard completes → store populated → switch to List editor                        |
| Wizard prompts               | TUI      | `dialoguer`-style interactive prompts: select, input, confirm                     |
| Category/operation selection | TUI      | Select prompt with category grouping                                              |
| Config step                  | TUI      | Param prompts with validation                                                     |
| Wizard-to-List handoff       | TUI      | Wizard completes → state populated → switch to List screen                        |

### Phase 3: Visual Editor (web only, TUI read-only)

**The spatial view.** Builds on existing React Flow prototype.

| Task                    | Platform | What                                                     |
| ----------------------- | -------- | -------------------------------------------------------- |
| Canvas from store       | Web      | React Flow controlled mode, compartment nodes from store |
| Config side panel       | Web      | Selected node → side panel with schema-driven form       |
| Execution visualization | Web      | DOM-direct progress via data attributes + CSS            |
| Node palette            | Web      | Sidebar with category grouping and search                |
| ASCII graph view        | TUI      | Read-only box-drawing overview of recipe structure       |

### Phase 4: Code Editor (web, TUI via $EDITOR)

**The power tool.** Builds on existing CodeMirror 6 strategy.

| Task                 | Platform | What                                                   |
| -------------------- | -------- | ------------------------------------------------------ |
| CM6 with JSON Schema | Web      | CodeMirror 6, schema validation, autocompletion, hover |
| Slash commands       | Web      | Inline node insertion via `/` trigger                  |
| Store sync           | Web      | Bidirectional sync between CM6 document and store      |
| Breadcrumb panel     | Web      | JSON path navigation above editor                      |
| $EDITOR integration  | TUI      | Open `.bnto.json` in user's editor, validate on return |

---

## Acceptance Criteria

### Universal (all editors, both platforms)

- [ ] Switching editors preserves all state — no data loss, no reparse
- [ ] Every node type's parameters render correct controls from `ParameterDef`
- [ ] `visible_when` / `required_when` conditions work reactively
- [ ] Undo/redo works across all mutations (add, remove, reorder, configure)
- [ ] Adding a node pre-fills all parameters with engine defaults
- [ ] Validation errors are surfaced inline (not in a separate panel/dialog)
- [ ] Export produces valid `.bnto.json` that the engine can execute
- [ ] Empty state has a clear call-to-action (never a blank screen)

### List Editor

- [ ] Recipe is comprehensible in under 5 seconds (scan the steps)
- [ ] Expanding a step shows its configurable parameters with correct controls
- [ ] "Show advanced" toggle hides non-essential params by default
- [ ] Reorder works via drag (web) and keyboard (both)
- [ ] Container nodes render as indented sub-lists
- [ ] Step numbers update automatically on reorder/add/remove
- [ ] (Web) Keyboard navigation works: Arrow keys to navigate, Enter to expand, Delete to remove
- [ ] (TUI) Vim keys work: `j`/`k` navigate, `l`/Enter expand, `h`/Esc collapse, `d` delete, `a` add

### Wizard

- [ ] A new user can create a valid recipe in under 30 seconds
- [ ] Every wizard path produces a valid recipe (no invalid states possible)
- [ ] "Skip to end with defaults" is always available
- [ ] Back navigation works at every step
- [ ] Wizard ends in the List editor with the created recipe loaded
- [ ] Categories and operations are driven by engine metadata (not hardcoded)

### Visual Editor

- [ ] (Web) Compartment nodes render with correct category colors and icons
- [ ] (Web) Clicking a node opens config panel with schema-driven form
- [ ] (Web) Execution state is visualized via surface elevation animation
- [ ] (Web) Zoom-to-fit keeps all nodes visible
- [ ] (TUI) Read-only graph shows recipe structure with box-drawing characters
- [ ] (TUI) Pressing `L` switches to List editor for editing

### Code Editor

- [ ] (Web) JSON Schema validation produces inline error diagnostics
- [ ] (Web) Autocompletion suggests valid property names and enum values
- [ ] (Web) Slash commands insert complete, valid node JSON blocks
- [ ] (Web) Changes sync to store (debounced) and reflect in other editors
- [ ] (TUI) `$EDITOR` integration: temp file created, validated on save, state updated on return

---

## Resolved Questions

1. **Recipe naming in wizard:** Generate automatically ("Compress Images v1"). Naming slows people down. Users rename in the List editor's metadata section when they care.

2. **List editor: collapsed step summary:** Label + one "hero" parameter (quality for compress, format for convert, pattern for rename). The engine adds a `hero_param` field to `ParameterDef` (or we infer it: first required param, or first param with a non-default value).

3. **TUI visual editor:** Build as a lightweight read-only view. Useful for understanding complex recipes at a glance. Small addition once the data model exists. No interactive editing — that fights the terminal's grain.

4. **Editor memory:** Global preference, stored in localStorage (web) or config file (TUI). User's preferred editor opens by default, switchable per-session.

5. **Per-node JSON toggle:** Yes — "Show JSON" on an expanded step reveals the raw JSON for that node. Great progressive disclosure pattern (like n8n). Power users love it, casual users ignore it.

---

## References

| Document                                         | Covers                                                                     |
| ------------------------------------------------ | -------------------------------------------------------------------------- |
| [editor-architecture.md](editor-architecture.md) | Shared editor layer — store, hooks, package strategy, switchable editors   |
| [visual-editor.md](visual-editor.md)             | Bento box visual editor — compartment design, grid layout, execution state |
| [code-editor.md](code-editor.md)                 | CodeMirror 6 — tech choice, slash commands, JSON Schema                    |
| [tui-strategy.md](tui-strategy.md)               | TUI architecture — TEA, screens, theming, Rust implementation              |
| [config-controls.md](config-controls.md)         | Config panel controls — schema-to-control mapping, what's built vs missing |
| [engine-execution.md](engine-execution.md)       | Execution pipeline — progress events, cancellation, streaming              |
