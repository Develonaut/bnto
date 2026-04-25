# Strategy: `tonkotsu` — Rich Terminal Forms for Ratatui

**Last Updated:** April 17, 2026

## Why This Exists

The TUI detail screen's parameter editing is functional but basic — trailing `_` cursor, no cursor movement, no placeholder text, no inline validation, no filterable selects. Charm's [huh](https://github.com/charmbracelet/huh) library set the standard for polished terminal forms in Go, but **no Rust equivalent exists**. `inquire` can't embed in ratatui. `ratatui-form` is v0.1.1 with 3 field types. `tui-textarea` is mature but does one thing (multiline text).

We build `tonkotsu`: a standalone, open-source ratatui form crate that anyone can use. For bnto, it replaces the hand-built detail screen controls. For the Rust community, it fills the gap between ratatui's low-level widget system and a ready-to-use form library.

**Parallel to `@bnto/form` on the web.** Same concept (schema-driven forms, control registry, per-field validation), different target (terminal vs browser). The web package maps `NodeParamControl` to React components. The Rust crate maps field types to ratatui widgets.

---

## Design Principles

1. **TEA-native** — Fields don't own state. State lives in caller's model, fields render it. Pure `update()` functions, no side effects.
2. **Pure functions over traits** — Control logic and rendering are plain functions. No `Widget`/`StatefulWidget` trait implementations. Returns `Vec<Line>` that callers compose into their layout. Testable without a terminal.
3. **Zero bnto dependency** — The crate depends on `ratatui` and `crossterm`. Nothing from `bnto-core`, `bnto-engine`, or any bnto internals. The bnto CLI crate consumes `tonkotsu` and maps engine metadata onto form fields.
4. **Batteries included, opinions optional** — Ships with sensible defaults (colors, key bindings, layout) but everything is overridable. Bring your own theme.
5. **Incremental adoption** — Use one field type or the whole form system. Fields work standalone.

---

## Crate Architecture

### Dependency Graph

```
bnto (CLI binary)
  +-- tonkotsu (new crate — ratatui form widgets)
  +-- bnto-engine (existing — registry, recipes)
  +-- bnto-core (existing — metadata types)

tonkotsu (standalone, open-source)
  +-- ratatui (0.30)
  +-- crossterm (for key event types — already transitive via ratatui)
  +-- unicode-segmentation (for grapheme-safe cursor)
  +-- (vendored: ~300 lines from tui-slider for slider rendering math)
```

`tonkotsu` has **no knowledge of bnto**. It provides generic form fields. The `bnto` CLI crate maps engine `ParameterType` + `Constraints` onto `tonkotsu` field types.

### Crate Location

```
engine/crates/tonkotsu/
+-- Cargo.toml
+-- README.md
+-- src/
    +-- lib.rs              # Public API re-exports
    +-- field.rs            # Field struct, FieldKind enum, FieldState enum
    +-- form.rs             # FormModel, FormMessage, update()
    +-- theme.rs            # FormTheme trait + default theme
    +-- render.rs           # Top-level form renderer
    +-- keys.rs             # Key event -> FormMessage mapping
    +-- controls/           # Pure state-transition functions (no ratatui)
    |   +-- mod.rs
    |   +-- text_input.rs   # Cursor operations, char insert/delete, word boundaries
    |   +-- select.rs       # Filter, highlight, cycling
    |   +-- confirm.rs      # Boolean toggle
    |   +-- number.rs       # Parse, validate, step, clamp
    |   +-- validation.rs   # Built-in validators (not_empty, min_len, range, pattern)
    +-- widgets/            # Pure rendering functions (returns Vec<Line>)
        +-- mod.rs
        +-- text_input.rs   # Cursor-tracked text input with placeholder
        +-- select.rs       # Vertical list with filter, compact cycle mode
        +-- confirm.rs      # Side-by-side Yes/No buttons
        +-- number.rs       # Value + slider bar visualization
        +-- error.rs        # Inline validation error line
```

### Public API Surface

```rust
// --- Core types ---
pub struct FormModel { fields: Vec<Field>, focused: usize, scroll_offset: usize, viewport_height: usize }
pub struct Field { id: String, label: String, kind: FieldKind, state: FieldState, value: String, ... }
pub enum FieldKind { Text { placeholder, char_limit }, Select { options, filterable }, Confirm { affirmative, negative }, Number { min, max, step, suffix } }
pub enum FieldState { Idle, TextEditing { buffer, cursor }, SelectExpanded { highlight, filter, filtered_indices }, NumberEditing { buffer, cursor } }
pub enum FormMessage { FocusNext, FocusPrev, StartEdit, EditChar(char), CursorLeft, ... }

// --- Pure functions ---
pub fn update(model: FormModel, msg: FormMessage) -> FormModel
pub fn render_form(model: &FormModel, theme: &dyn FormTheme, area: Rect) -> Vec<Line<'static>>

// --- Theming ---
pub trait FormTheme { fn text(&self) -> Style; fn selected(&self) -> Style; fn muted(&self) -> Style; fn error(&self) -> Style; fn border(&self) -> Style; fn heading(&self) -> Style; }

// --- Key mapping ---
pub fn map_key_event(key: KeyEvent, model: &FormModel) -> Option<FormMessage>

// --- Builders ---
pub fn text(id: &str) -> FieldBuilder
pub fn select(id: &str) -> FieldBuilder
pub fn confirm(id: &str) -> FieldBuilder
pub fn number(id: &str) -> FieldBuilder
```

### Usage Example (for any ratatui app)

```rust
use tonkotsu::{FormModel, text, select, confirm, number, update, render_form, map_key_event};

// Build a form
let form = FormModel::new(vec![
    text("name").label("Recipe Name").placeholder("My Recipe").required().build(),
    select("format").label("Output Format")
        .options(&[("jpeg", "JPEG"), ("png", "PNG"), ("webp", "WebP")])
        .filterable()
        .build(),
    confirm("overwrite").label("Overwrite existing?").build(),
    number("quality").label("Quality").range(1.0, 100.0).suffix("%").value("80").build(),
]);

// In your TEA update loop:
if let Some(msg) = map_key_event(key_event, &form) {
    form = update(form, msg);
}

// In your render function:
let lines = render_form(&form, &my_theme, inner_area);
frame.render_widget(Paragraph::new(lines), inner_area);
```

---

## Field Types

### TextInput

**UX model:** Inline editing with visible cursor, placeholder text, character limit. Cursor math follows patterns from tui-input (headless text input crate) — grapheme-safe, word-boundary-aware.

| Feature             | Implementation                                                                            |
| ------------------- | ----------------------------------------------------------------------------------------- |
| Cursor position     | Byte offset in `FieldState::TextEditing { cursor }`. Rendered as inverse-styled character |
| Left/Right movement | Grapheme-aware via `unicode-segmentation` (pattern from tui-input)                        |
| Home/End            | Jump to buffer start/end                                                                  |
| Word jump           | Ctrl+Left/Right — previous/next word boundary                                             |
| Delete word         | Ctrl+W — delete word behind cursor                                                        |
| Placeholder         | Shown in muted style when value is empty and field is idle                                |
| Character limit     | Optional. Rejects input beyond limit                                                      |

**Idle display:** `  Recipe Name   My Recipe` (placeholder in muted if empty)
**Editing display:** `> Recipe Name   Compress I|mages` (cursor shown as block)

### Select

**UX model:** Two modes — compact cycling for small option sets, expanded vertical list with filter for large ones.

| Feature                    | Implementation                                           |
| -------------------------- | -------------------------------------------------------- |
| Compact mode (<=5 options) | `< JPEG >` — arrow keys cycle                            |
| Expanded mode (>5 options) | Vertical list with highlight, one option per line        |
| Filter-as-you-type         | Type to narrow options, case-insensitive substring match |
| Option labels              | Separate display label from stored value                 |
| Wrapping                   | Navigation wraps last-to-first and vice versa            |

**Compact display:** `  Output Format   < JPEG >`
**Expanded display:**

```
> Output Format
    Filter: pn_
    > PNG
      (2 of 3 options)
```

### Confirm

**UX model:** Side-by-side Yes/No buttons. Active side highlighted.

| Feature       | Implementation                                            |
| ------------- | --------------------------------------------------------- |
| Display       | `[ Yes ]  No` or `Yes  [ No ]` — active in selected style |
| Toggle        | Space, Left/Right arrows, `y`/`n` shortcuts               |
| Custom labels | Configurable affirmative/negative text                    |

### Number

**UX model:** Arrow-key stepping when bounded, text entry for precise values. Slider visualization powered by vendored tui-slider rendering math.

| Feature              | Implementation                                                                                             |
| -------------------- | ---------------------------------------------------------------------------------------------------------- |
| Bounded stepping     | Arrow keys step by auto-computed increment                                                                 |
| Slider visualization | Vendored from tui-slider — Unicode partial block characters for sub-cell precision, color gradient presets |
| Text entry           | Enter opens text editing mode for precise input                                                            |
| Validation on commit | Must be a number, must be within bounds                                                                    |
| Suffix display       | `80%`, `1920px`                                                                                            |
| Slider styles        | Curated subset of tui-slider's 40+ presets, mapped through FormTheme                                       |

**Idle display:** `  Quality   < 80% >  ████████████████░░░`
**Editing display:** `> Quality   80_` (cursor-tracked text input)

---

## State Model

### Per-Field State

Every field carries its own state and error. No global editing flag.

```rust
pub struct Field {
    pub id: String,
    pub label: String,
    pub kind: FieldKind,
    pub state: FieldState,
    pub value: String,
    pub default: Option<String>,
    pub description: Option<String>,
    pub error: Option<String>,
    pub validator: Option<ValidatorFn>,
    pub visible: bool,
}

pub type ValidatorFn = fn(&str) -> Option<String>;
```

### FieldState Transitions

```
Idle --Enter--> TextEditing { buffer: value.clone(), cursor: value.len() }
Idle --Enter--> SelectExpanded { highlight: current_index, filter: "", filtered: all }
Idle --Enter--> NumberEditing { buffer: value.clone(), cursor: value.len() }
Idle --Space--> (toggle confirm value directly, stay Idle)
Idle --arrows-> (cycle select / step number directly, stay Idle)

TextEditing --Enter--> validate -> Idle (commit) or stay TextEditing (error)
TextEditing --Esc----> Idle (cancel, discard buffer)

SelectExpanded --Enter--> Idle (commit highlighted option)
SelectExpanded --Esc----> Idle (cancel, keep previous value)

NumberEditing --Enter--> validate -> Idle (commit) or stay NumberEditing (error)
NumberEditing --Esc----> Idle (cancel, discard buffer)
```

---

## Message Taxonomy

```rust
pub enum FormMessage {
    // Navigation
    FocusNext,
    FocusPrev,

    // Edit lifecycle
    StartEdit,
    CommitEdit,
    CancelEdit,

    // Text input (TextEditing / NumberEditing)
    EditChar(char),
    EditBackspace,
    DeleteForward,
    CursorLeft,
    CursorRight,
    CursorHome,
    CursorEnd,
    CursorWordBack,
    CursorWordForward,
    DeleteWordBack,

    // Inline actions (no edit mode needed)
    ToggleConfirm,
    CycleNext,
    CyclePrev,
    ResetDefault,

    // Select list (SelectExpanded)
    SelectHighlightNext,
    SelectHighlightPrev,
    SelectConfirm,
    SelectFilterChar(char),
    SelectFilterBackspace,

    // Viewport
    Resize { height: usize },
}
```

---

## Theming

```rust
pub trait FormTheme {
    fn text(&self) -> Style;       // Body text
    fn selected(&self) -> Style;   // Focused/active items
    fn muted(&self) -> Style;      // Placeholder, descriptions, inactive
    fn error(&self) -> Style;      // Validation errors
    fn border(&self) -> Style;     // Decorative borders
    fn heading(&self) -> Style;    // Section headings, labels
}
```

A `DefaultTheme` ships with the crate using terminal-safe colors (`Color::Reset` for text, `Color::DarkGray` for muted, `Color::Red` for error). The bnto CLI wraps its existing `Theme` struct to implement `FormTheme`.

---

## Integration with bnto

The bnto CLI bridges engine metadata to `tonkotsu` fields:

```rust
fn param_to_field(param: &ParamEntry) -> tonkotsu::Field {
    let kind = match &param.param_type {
        ParameterType::Boolean => FieldKind::Confirm { .. },
        ParameterType::Enum { options } => FieldKind::Select { options, filterable: options.len() > 5 },
        ParameterType::Number => FieldKind::Number { min, max, step: None, suffix },
        _ => FieldKind::Text { placeholder: None, char_limit: None },
    };
    Field::new(&param.name, &param.label, kind)
        .value(&param.value)
        .default(param.default.as_deref())
        .description(param.description.as_deref())
        .build()
}
```

The detail screen delegates to `tonkotsu::update()`, `tonkotsu::render_form()`, and `tonkotsu::map_key_event()`.

**visible_when** stays in the bnto layer — domain-specific logic. The form crate exposes `field.visible: bool` that the caller manages.

---

## What Ships vs What's Deferred

### Ships (Phase 1)

| Feature               | huh equivalent                      |
| --------------------- | ----------------------------------- |
| TextInput with cursor | `huh.NewInput()`                    |
| Select with filter    | `huh.NewSelect()`                   |
| Confirm (Yes/No)      | `huh.NewConfirm()`                  |
| Number with slider    | No huh equivalent (bnto innovation) |
| Per-field validation  | `field.Validate()`                  |
| Built-in validators   | `ValidateNotEmpty`, etc.            |
| Keyboard navigation   | Tab/Shift+Tab, j/k                  |
| Reset to default      | No huh equivalent                   |
| Theme trait           | `huh.ThemeCharm()`                  |
| Scroll/viewport       | Auto-scroll focused field into view |

### Deferred (Phase 2+)

| Feature                  | Why defer                            |
| ------------------------ | ------------------------------------ |
| Suggestions/autocomplete | Complex; add when a use case appears |
| External editor          | Overkill for short param values      |

---

## huh Parity — Gap Analysis (April 2026)

**Goal:** Make `tonkotsu` the Rust equivalent of Charm's [huh](https://github.com/charmbracelet/huh) library — "A simple, powerful library for building interactive forms and prompts in the terminal."

Sprint 15 delivered Display/Edit mode, FilePath, TextArea, and fuzzy Select. Four gaps remain to reach huh parity.

### Current State vs huh

| huh Feature                           | tonkotsu Status                                       | Gap           |
| ------------------------------------- | ----------------------------------------------------- | ------------- |
| Input (single-line text)              | **Parity** — `TextInput` (Phase 1)                    | —             |
| Text (multi-line)                     | **Parity** — `TextArea` (Sprint 15)                   | —             |
| Select                                | **Parity** — `Select` with fuzzy filter (Sprint 15)   | —             |
| **MultiSelect**                       | **Missing**                                           | **Sprint 16** |
| Confirm                               | **Parity** — `Confirm` (Phase 1)                      | —             |
| FilePicker (via Bubbles)              | **Parity** — `FilePath` (Sprint 15)                   | —             |
| Number/Slider                         | **Ahead of huh** — `Number` with tui-slider (Phase 1) | —             |
| **Full-screen focused editing**       | **Missing** — FullScreenEdit FormMode                 | **Sprint 16** |
| **Field grouping** (multi-page forms) | **Missing**                                           | **Sprint 16** |
| **Note/read-only field**              | **Missing**                                           | **Sprint 16** |
| Dynamic field properties              | Partial — validation exists                           | Minor — defer |
| Theming                               | **Parity** — `FormTheme` trait (Phase 1)              | —             |
| Accessible mode                       | **Missing**                                           | Defer to v2   |
| Spinner                               | N/A (execution screen has its own)                    | —             |

### Remaining Gaps (Sprint 16 Wave 1)

**1. FullScreenEdit FormMode** — The signature huh UX. When editing a field, the form hides all other fields and renders a dedicated full-screen panel showing only the focused field's control — label header, full widget, helper footer. Display mode identical to DisplayEdit (compact one-liners). This is how huh operates: one field at a time, full focus. Becomes the default form mode.

**2. MultiSelect field type** — Choose multiple options from a list. Display: `"Tags: image, vector (2 selected)"`. Edit: checkboxes with Space to toggle, Enter to confirm. huh's `NewMultiSelect()` equivalent.

**3. Field grouping** — huh organizes forms into groups that render as pages. `FieldGroup` wraps fields into named sections. In FullScreenEdit mode, groups become navigable pages (next/prev). In DisplayEdit mode, groups render as visual sections with headers. Enables multi-page wizard forms.

**4. Note field type** — Read-only `FieldKind::Note` for displaying informational text between fields. huh uses `NewNote()` for descriptions, instructions, and section context. Display: styled text block, not editable.

### After Sprint 16

With these four additions, `tonkotsu` reaches functional parity with huh:

| Capability   | huh                                           | tonkotsu (post Sprint 16)                                                |
| ------------ | --------------------------------------------- | ------------------------------------------------------------------------ |
| Field types  | 5 (Input, Text, Select, MultiSelect, Confirm) | 8 (Text, TextArea, Select, MultiSelect, Confirm, Number, FilePath, Note) |
| Form modes   | 1 (full-screen focused)                       | 3 (Inline, DisplayEdit, FullScreenEdit)                                  |
| Grouping     | Groups as pages                               | FieldGroup with page navigation                                          |
| Theming      | 5 presets + custom                            | Trait-based + DefaultTheme                                               |
| Validation   | Per-field                                     | Per-field with built-in validators                                       |
| Architecture | Bubble Tea model                              | TEA-native (pure functions)                                              |
| Standalone   | Yes                                           | Yes (zero bnto dependency)                                               |

**tonkotsu advantages over huh:** Number/slider field (no huh equivalent), FilePath with inline directory browser (huh delegates to Bubbles), three form modes (huh has one), pure-function rendering (easier to test). tonkotsu is the Rust community's answer to huh.

---

## Phased Delivery (PRs)

### PR 1: Crate scaffold + TextInput

New `engine/crates/tonkotsu/`. Field/FieldState/FormModel/FormMessage types. TextInput control logic (cursor ops). TextInput widget rendering (cursor, placeholder). ~25 tests.

### PR 2: Select field (compact + expanded + filter)

Select control logic (cycling, filter). Select widget rendering (compact + expanded). Filter-as-you-type. ~20 tests.

### PR 3: Confirm + Number fields + tui-slider vendor

Confirm widget (Yes/No buttons). Number widget with vendored tui-slider rendering math (~300 lines, adapted to pure-function model). Slider style presets mapped through FormTheme. Number validation (bounds, parse). ~20 tests.

### PR 4: Validation + Theme

ValidatorFn, built-in validators. Error rendering. FormTheme trait + DefaultTheme. ~15 tests.

### PR 5: Form-level API + render_form

FormModel builder. Top-level update/render/map_key_event. Scroll/viewport. Description on focus. Reset-to-default. ~15 tests.

### PR 6: bnto integration

Replace detail screen controls with tonkotsu. Bridge ParamEntry -> Field. Remove old editing state. ~10 tests.

---

## Ecosystem Research & Dependency Decisions

We surveyed the Rust TUI ecosystem for existing crates that could be used as dependencies, vendored, or referenced. The goal: don't rebuild what already exists, but don't take on heavy dependencies either.

### Decision Summary

| Crate                    | Version | Decision                | Rationale                                                                                                                                                                                                                                                                             |
| ------------------------ | ------- | ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **tui-slider**           | v0.3.2  | **Vendor** (~300 lines) | Beautiful slider with 40+ preset styles. MIT licensed. ratatui 0.30 exact match. Only adds `unicode-width` (we already have it). But it implements the `Widget` trait — we use pure functions returning `Vec<Line>`. Vendor the core rendering math, adapt to our pure-function model |
| **tui-input**            | v0.15.2 | **Reference**           | Headless single-line text input — cursor tracking without rendering. Clean API (`Input::new()`, `handle_event()`, `value()`, `cursor()`). Good reference for our cursor math, but its `HandleEvent` trait model doesn't fit TEA messages. Copy the cursor operations pattern          |
| **rat-widget**           | v0.33   | **Reference**           | Most complete widget suite (TextInput, NumberInput, Slider, Checkbox, Radio, Choice, ComboBox). Different event model (`HandleEvent` trait, not TEA). Too heavy as dependency (pulls in entire rat ecosystem). Cherry-pick ideas: masked input, number formatting, focus management   |
| **tui-textarea**         | v0.7    | **Reference**           | Mature multiline editor. Overkill for single-line param inputs, but excellent reference for grapheme-aware cursor handling, undo/redo, and clipboard ops                                                                                                                              |
| **ratatui-form**         | v0.1.1  | **Skip**                | Only 3 field types, v0.1.1, minimal functionality. We're building something much more complete                                                                                                                                                                                        |
| **tui-prompts**          | —       | **Skip**                | Archived / unmaintained                                                                                                                                                                                                                                                               |
| **unicode-segmentation** | v1.x    | **Depend**              | Required for grapheme-safe cursor operations. Standard, minimal, widely used                                                                                                                                                                                                          |
| **crossterm**            | v0.28+  | **Depend**              | Already in our dependency tree via ratatui. Used for `KeyEvent` types in our key mapping                                                                                                                                                                                              |

### tui-slider Deep Dive

The user explicitly wants tui-slider's visual quality: "I really like those and would like to bring them in."

**What we take:**

- Unicode block character rendering for slider bars (`█`, `░`, partial blocks for sub-cell precision)
- Color gradient presets (40+ styles — we'll ship a curated subset that works with our theme system)
- `SliderState` concept — a value + min/max/step that computes position

**What we adapt:**

- tui-slider implements ratatui's `Widget` trait. We use pure functions returning `Vec<Line>`. We vendor the rendering math and wrap it in our function-based API
- tui-slider is standalone (render-only). We integrate it into our Number field — slider is one visual element alongside the label, value display, and optional text-entry mode

**Vendor scope:** ~300 lines for horizontal slider rendering (we skip vertical mode). The `unicode-width` dependency is already transitively available via ratatui.

### tui-input Deep Dive

**What we learn from:**

- Clean separation between state (cursor position, value buffer) and rendering
- Word boundary detection for Ctrl+Left/Right jumps
- Grapheme-aware operations (delete, insert, cursor move)

**What we build ourselves:**

- Our text input state lives in `FieldState::TextEditing { buffer, cursor }`, not a separate struct
- Rendering is a pure function, not a trait impl
- Key handling goes through `FormMessage`, not `HandleEvent`

### Ecosystem Gap Confirmed

No existing Rust crate provides a complete, TEA-compatible form widget set. The closest is `rat-widget`, but it uses a fundamentally different event model (`HandleEvent` trait with `Regular`/`MouseOnly`/`DoubleClick` qualifiers) and pulls in the entire `rat` ecosystem. Building `tonkotsu` fills a genuine gap.

---

## Open Source Considerations

- **Crate name:** `tonkotsu`
- **License:** MIT (matches bnto)
- **Minimum ratatui version:** 0.30
- **MSRV:** Match bnto workspace
- **README:** Usage examples, field type gallery, theme customization, comparison to huh
- **No bnto branding in the crate API** — generic types, bnto integration lives in the CLI crate

## Verification

```bash
cargo test -p tonkotsu        # New crate tests
task wasm:test                 # Existing tests still pass
task cli:test                  # CLI golden tests still pass
cargo run -p bnto -- tui       # Manual: test each control type
```
