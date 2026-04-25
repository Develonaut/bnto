# TUI Controls — Bubbles-Inspired UX Overhaul

**Created:** April 24, 2026
**Status:** Planned
**Reference:** [Charm Bubbles](https://github.com/charmbracelet/bubbles) — Go TUI component library

---

## Problem

Our TUI form controls (`tonkotsu`) and file picker work but feel basic compared to polished TUI tools like Charm Bubbles. Two specific pain points:

1. **The file picker is a separate screen, not a form control.** When a recipe needs a file path, the user navigates to a full-screen picker. Bubbles treats file selection as a form field — the field shows the current path inline, and activating it opens a focused browser view. This is the interaction model we should adopt.

2. **Form controls don't have display/edit modes.** Our form fields are always "live" — the full control is always visible. Bubbles controls show a compact **display** value (label + current value) and switch to a **focused edit view** when activated. This makes forms scannable and reduces visual noise.

---

## Charm Bubbles Component Inventory

| Component           | Description                                                                                | bnto Equivalent                                | Gap                                       |
| ------------------- | ------------------------------------------------------------------------------------------ | ---------------------------------------------- | ----------------------------------------- |
| **TextInput**       | Single-line, Unicode, paste, in-place scroll, placeholder                                  | `tonkotsu::TextInput`                          | Parity — ours is solid                    |
| **TextArea**        | Multi-line input, vertical scroll                                                          | None                                           | **Missing**                               |
| **FilePicker**      | Hierarchical directory browser, ext filter, perms/size display, vim keys (g/G/j/k/J/K/h/l) | `bnto::tui::screens::picker` (separate screen) | **Not a form control** — full-screen only |
| **List**            | Paginated list with fuzzy filtering, spinner, status messages, auto-help                   | `tonkotsu::Select` (cycling + basic filter)    | **No fuzzy matching**                     |
| **Table**           | Column/row navigation, tabular data                                                        | None                                           | Not needed yet                            |
| **Spinner**         | Customizable animation frames                                                              | Execution screen spinner                       | Already have for execution                |
| **Progress**        | Gradient fills, customizable meter                                                         | Execution screen progress                      | Already have for execution                |
| **Viewport**        | Scrollable content area with mouse support                                                 | Not applicable                                 | N/A                                       |
| **Paginator**       | Dot or numeric page indicators                                                             | Not applicable                                 | N/A                                       |
| **Help**            | Auto-generated help text from keybindings                                                  | Footer key hints                               | Partial — manual, not auto-generated      |
| **Timer/Stopwatch** | Count up/down timers                                                                       | Execution elapsed timer                        | Already have                              |
| **Key**             | Non-visual keybinding manager                                                              | `keys.rs` dispatch                             | Partial — hardcoded, not remappable       |

---

## Key UX Principle: Display Mode vs Edit Mode

This is the core change. Every Bubbles form control follows this pattern:

```
DISPLAY MODE (scanning)          EDIT MODE (focused)
┌──────────────────────┐        ┌──────────────────────────────┐
│ Recipe Name: My Rec… │  ──►   │ Recipe Name                  │
│ Quality: 80%         │ Enter  │ ┌──────────────────────────┐ │
│ Format: jpeg         │        │ │ My Recipe Name█           │ │
│ Input: ~/photos/     │        │ └──────────────────────────┘ │
└──────────────────────┘        │ Enter to confirm, Esc cancel │
                                └──────────────────────────────┘
```

**Display mode:** Compact one-line summary per field. Label + formatted value. Arrow keys move between fields. Enter activates the focused field.

**Edit mode:** The selected field expands to show its full control (text input, select dropdown, file browser, etc.). Other fields are hidden or dimmed. Enter confirms, Esc cancels.

### Why This Matters

1. **Scannability** — Users see all fields at a glance in display mode. Current approach shows all controls expanded simultaneously, which is noisy.
2. **Focus** — Edit mode eliminates distractions. The user is editing one thing.
3. **File picker as form field** — The killer feature. A file path field shows `~/photos/cat.jpg` in display mode. Enter opens a browser. Selecting a file returns to display mode. No full-screen navigation.

---

## Current Architecture

### `tonkotsu` (standalone crate)

```
engine/crates/tonkotsu/
├── src/
│   ├── lib.rs           # Public API re-exports
│   ├── field.rs         # Field, FieldKind, FieldBuilder, FieldState
│   ├── form.rs          # FormModel, FormMessage, update()
│   ├── render.rs        # render_form() — Vec<Line> output
│   ├── keys.rs          # map_key_event() — key → FormMessage
│   ├── theme.rs         # FormTheme trait + DefaultTheme
│   ├── validators.rs    # Validation functions
│   ├── controls/        # Per-control rendering + update logic
│   │   ├── dispatch.rs  # TEA dispatch to per-type handlers
│   │   ├── text.rs      # TextInput control
│   │   ├── select.rs    # Select control
│   │   ├── confirm.rs   # Confirm (boolean) control
│   │   └── number.rs    # Number (slider) control
│   ├── widgets/         # Reusable rendering pieces
│   └── demo/            # Kitchen-sink demo app
```

**Key design decisions:**

- Pure functions returning `Vec<Line<'static>>` — no Widget trait impls
- TEA-native: `FormModel` + `FormMessage` + `update()` + `render_form()`
- Zero bnto dependency — standalone crate
- ~105 tests

### File Picker (in main `bnto` crate)

```
engine/crates/bnto/src/tui/screens/
├── picker.rs            # PickerModel, PickerMessage, FileEntry
├── picker_update.rs     # update() — pure state transitions
├── picker_loader.rs     # Filesystem I/O (load_entries, extensions_for_recipe)

engine/crates/bnto/src/tui/
├── render_picker.rs     # draw_picker() — renders full screen
├── widgets/file_list.rs # render_file_list() — entry rendering
```

**Key facts:**

- The picker is a **screen** (full terminal), not a **form control**
- `PickerModel` has: cursor, selected (BTreeSet), entries, extensions, show_hidden, viewport, nav_history
- 13 message variants: CursorDown/Up, ToggleSelect, EnterDir, ParentDir, Confirm, PageDown/Up, GoToTop/Bottom, ToggleHidden, SelectAll, Resize
- ~30 tests
- Rendering: `▸ / dirname` for dirs, `▸ file.jpg [x] 283 KB` for files

---

## Implementation Plan

### Wave 1 — Form Control Display/Edit Mode (foundational)

The core UX change. Every field type gets display and edit rendering.

**Task 1: Display/Edit mode infrastructure in `tonkotsu`**

Add `FormMode` (Display/Edit) to `FormModel`. In display mode, `render_form()` renders each field as a compact one-liner. Pressing Enter on the focused field switches to edit mode (only that field's full control visible). Enter/Esc in edit mode returns to display with updated/original value.

```rust
// Display mode rendering per field type:
// TextInput:  "Recipe Name: My Recipe"
// Number:     "Quality: 80%"
// Select:     "Format: jpeg"
// Confirm:    "Strip metadata: yes"
// FilePath:   "Input: ~/photos/cat.jpg"

// Edit mode: only the active field shows its full control
```

Changes to:

- `form.rs` — Add `FormMode` enum, mode transitions in `update()`
- `render.rs` — Branch on mode for display vs edit rendering
- `field.rs` — Add `display_value()` method per FieldKind
- `keys.rs` — Enter toggles mode, Esc exits edit mode

**Task 2: FilePath field type in `tonkotsu`**

New `FieldKind::FilePath` that renders as a path string in display mode. Edit mode shows an inline directory browser (adapted from the picker logic). This requires extracting the core picker state machine from `bnto` into a reusable form.

Options:

- **Option A:** Move picker model into `tonkotsu` as a new control type
- **Option B:** Keep picker in `bnto` but add a simpler path-entry control to `tonkotsu` that just accepts typed paths
- **Option C (recommended):** Extract picker logic into `tonkotsu` as `controls/file_path.rs`. The standalone screen picker in `bnto` then becomes a thin wrapper around the form control.

Option C is recommended because:

- `tonkotsu` stays standalone (no bnto dependency — the picker model is pure state machine)
- The full-screen picker screen reuses the same control, just rendered larger
- One codebase for file selection UX, two rendering contexts

### Wave 2 — Picker Polish (parallel)

Bring the picker up to Bubbles quality with features our users will notice.

**Task 3: Inline search/filter in picker**

Type to filter entries by filename. Case-insensitive substring match. Shows match count. Backspace clears. This is the single highest-impact picker improvement — Bubbles List component's fuzzy filter is its signature feature.

**Task 4: File metadata columns**

Aligned columns showing permissions and human-readable sizes (like Bubbles). Toggle with `p`. Symlink detection with `->` indicator.

**Task 5: Breadcrumb path header**

Replace plain path with styled breadcrumb segments. Visual hierarchy showing the navigation trail.

### Wave 3 — Form Control Refinements (depends on Wave 1)

**Task 6: Select with fuzzy filter**

Enhance `tonkotsu::Select` to support fuzzy substring matching when typing. The current cycling behavior (up/down arrows) is preserved when no filter text is active. When the user types, entries filter down. This matches Bubbles' List component pattern.

**Task 7: TextArea field type**

New `FieldKind::TextArea` for multi-line text. Display mode shows first line + `(N lines)`. Edit mode shows a scrollable multi-line editor with line wrapping.

---

## Bubbles Keyboard Conventions (adopt where possible)

| Action           | Bubbles Keys          | Our Current Keys | Adopt?                  |
| ---------------- | --------------------- | ---------------- | ----------------------- |
| Move down        | `j`, `↓`              | `↓`              | Yes — add `j` in picker |
| Move up          | `k`, `↑`              | `↑`              | Yes — add `k` in picker |
| Page down        | `J`, `PgDn`           | `PgDn`           | Yes — add `J` in picker |
| Page up          | `K`, `PgUp`           | `PgUp`           | Yes — add `K` in picker |
| Jump to first    | `g`                   | `Home`           | Yes — add `g` in picker |
| Jump to last     | `G`                   | `End`            | Yes — add `G` in picker |
| Enter directory  | `l`, `→`, `Enter`     | `Enter`, `→`     | Yes — add `l`           |
| Parent directory | `h`, `←`, `Backspace` | `Backspace`, `←` | Yes — add `h`           |

Adding vim keys alongside existing arrow keys is additive — nothing breaks.

---

## Out of Scope (for now)

- **Auto-generated help from keybindings** — Nice Bubbles feature, not a priority
- **Remappable key bindings** — Our keys are hardcoded; fine for now
- **Table component** — No current use case
- **Mouse support** — Terminal mouse is unreliable across terminals
- **File preview** — Would be nice but adds significant complexity (file type detection, rendering)

---

## Test Strategy

All work is TDD-first (RED tests before implementation):

| Area                          | Test approach                           | Estimated count |
| ----------------------------- | --------------------------------------- | --------------- |
| Display/edit mode transitions | Unit tests on FormModel state machine   | ~6              |
| FilePath field type           | Unit tests on display + browser state   | ~8              |
| Picker search/filter          | Unit tests on filter logic + rendering  | ~5              |
| Picker metadata display       | Unit tests on formatting + rendering    | ~4              |
| Picker breadcrumb             | Unit tests on path segment rendering    | ~3              |
| Select fuzzy filter           | Unit tests on filter matching + cycling | ~4              |
| TextArea field type           | Unit tests on multi-line input + scroll | ~5              |
| **Total**                     |                                         | **~35**         |

---

## References

| Document                                                  | Relevance                                                             |
| --------------------------------------------------------- | --------------------------------------------------------------------- |
| [Charm Bubbles](https://github.com/charmbracelet/bubbles) | Reference implementation — interaction patterns, keyboard conventions |
| [tonkotsu-strategy.md](tonkotsu-strategy.md)              | Original tonkotsu design decisions, huh inspiration                   |
| [tui-strategy.md](tui-strategy.md)                        | TUI architecture, TEA pattern, Motorway design language               |
| [tui-user-journey.md](tui-user-journey.md)                | Screen flow, picker integration points                                |
