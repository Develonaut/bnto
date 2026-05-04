# tonkotsu

Rich terminal forms for ratatui. Standalone crate with zero bnto dependency.

## Overview

`tonkotsu` provides form field types with pure-function state transitions and rendering. Fields return `Vec<Line>` instead of implementing ratatui's `Widget` trait — the host application owns layout and rendering, tonkotsu owns field content.

Follows TEA (The Elm Architecture): `FormModel` holds state, `update()` is a pure function `(FormModel, FormMessage) -> FormModel`, and `render_form()` produces lines. All state transitions are testable without a terminal.

## Field Types

| Field         | Builder       | What It Does                                          |
| ------------- | ------------- | ----------------------------------------------------- |
| `TextInput`   | `text()`      | Single-line text with placeholder and validation      |
| `TextArea`    | `textarea()`  | Multi-line text with line wrapping                    |
| `Select`      | `select()`    | Single-choice from options with arrow-key cycling     |
| `MultiSelect` | `multiselect()`| Multi-choice with checkbox toggling                  |
| `Number`      | `number()`    | Bounded number with step size and suffix annotation   |
| `Confirm`     | `confirm()`   | Yes/No toggle                                         |
| `FilePath`    | `file_path()` | Filesystem path with browse and autocomplete          |
| `Note`        | `note()`      | Read-only display text (non-interactive)              |

## Usage

```rust
use tonkotsu::{FormModel, text, select, number, update, render_form, map_key_event};

let form = FormModel::new(vec![
    text("name").label("Recipe Name").placeholder("My Recipe").build(),
    number("quality").label("Quality").range(1.0, 100.0).suffix("%").value("80").build(),
    select("format").label("Format").options(vec!["JPEG", "PNG", "WebP"]).build(),
]);
```

## Grouping

Fields can be organized into visual groups via `FieldGroup`. Groups render with a header and contain a subset of form fields.

## Viewport

`Viewport` provides scrollable content management — tracks visible rows, cursor position, page up/down, and boundary wrapping. Used by the host to scroll long forms.

## Theming

`FormTheme` trait defines colors for labels, values, borders, and focus indicators. `DefaultTheme` provides the default palette. The host application implements `FormTheme` to match its own color scheme.

## Directory Structure

```
src/
├── lib.rs            # Public API re-exports
├── field.rs          # Field enum, FieldState, builder constructors
├── field_builder.rs  # FieldBuilder with fluent API
├── form.rs           # FormModel, FormMessage, update()
├── form_nav.rs       # Form navigation (focus, tab order)
├── group.rs          # FieldGroup for visual grouping
├── keys.rs           # Key event → FormMessage mapping
├── messages.rs       # FormMessage enum variants
├── render.rs         # render_form() → Vec<Line>
├── theme.rs          # FormTheme trait + DefaultTheme
├── validators.rs     # Built-in validators (not_empty, min_len, range, pattern)
├── viewport.rs       # Scrollable content viewport
├── controls/         # Per-field-type input handling
├── widgets/          # Per-field-type rendering
├── demo.rs           # Kitchen-sink demo helpers
└── bin/demo/         # tonkotsu-demo binary
tests/
├── journeys.rs       # Form interaction journey tests
└── demo_snapshot.rs  # Demo snapshot tests
```

## Development

```bash
cargo test -p tonkotsu            # Unit + journey tests
task form:demo                    # Launch kitchen-sink demo TUI
```
