# TUI Strategy

**Created:** April 9, 2026
**Status:** Planning
**Related:** [engine-expansion.md](engine-expansion.md), [PLAN.md](../PLAN.md), [design-language.md](design-language.md)

---

## Vision

`bnto tui` launches an interactive terminal UI that makes recipe browsing, file selection, execution, and results feel as satisfying as the web app — within the constraints of a terminal. The TUI is the CLI's rich mode: same engine, same recipes, richer interface.

**Design north star:** Charm (Go ecosystem) for visual polish and UX philosophy. Yazi and GitUI (Rust/ratatui) for architecture and implementation patterns.

---

## Design Principles

### Bento Box in the Terminal

The same principles that govern our TypeScript codebase govern the TUI:

| Principle                 | Web application                         | TUI application                                    |
| ------------------------- | --------------------------------------- | -------------------------------------------------- |
| **Single responsibility** | One component per file                  | One screen per module, one widget per file         |
| **Composition**           | Small components compose in JSX         | Small widgets compose in `render()`                |
| **Pure functions first**  | Actions are pure, hooks are wrappers    | `update()` is pure, event loop is the wrapper      |
| **Clear boundaries**      | Apps → Core → Engine                    | TUI → Engine (direct link, no adapters)            |
| **Size limits**           | Files < 250 lines, functions < 20 lines | Same. Rust files < 250 lines, functions < 20 lines |
| **YAGNI**                 | Don't build for hypothetical futures    | MVP screens only — no premature features           |

### Motorway Design Language → Terminal

3D surface elevation and spring animations don't exist in a terminal. But the _feelings_ they create do. Here's what transfers:

| Web (Motorway)                  | TUI equivalent                                    | The feeling it creates |
| ------------------------------- | ------------------------------------------------- | ---------------------- |
| Warm cream background           | Muted/dim background with warm accent colors      | Warmth, not clinical   |
| Terracotta primary              | Bold/colored primary text (terracotta ANSI 256)   | Brand identity         |
| Golden accent                   | Yellow/gold highlight for active/selected items   | Visual hierarchy       |
| Teal secondary                  | Cyan/teal for secondary information               | Cool counterpoint      |
| Generous `rounded-lg` radius    | Rounded box-drawing characters (`╭╮╰╯`)           | Friendly, not sharp    |
| Spring pop-in animation         | Quick reveal (no animation, but instant feedback) | Responsiveness         |
| `Pressable` spring feel         | Immediate visual response on keypress             | Tactile satisfaction   |
| Generous whitespace             | Padding inside panels, spacing between sections   | Breathing room         |
| `font-display` (Geist) headings | Bold + uppercase for headers                      | Typography hierarchy   |
| `font-mono` for code            | Default terminal font for all content             | Clean and readable     |
| Skeleton loading states         | Spinner + "Loading..." with progress context      | No dead air            |

**What we don't do:**

- No ASCII art logos or decorative borders — the content is the hero
- No slow animations or transitions — terminals reward instant feedback
- No color overload — reserve color for meaning (status, selection, categories)

### Elm Architecture (TEA)

Every screen follows the same pattern. This is our "pure actions + thin hooks" pattern translated to Rust:

```
Model (state)  →  View (render)  →  Event (input)  →  Update (state transition)
     ↑                                                        |
     └────────────────────────────────────────────────────────┘
```

- **Model**: Plain Rust struct. No side effects, no I/O.
- **Update**: Pure function `(Model, Message) → Model`. All business logic lives here. **This is where tests live.**
- **View**: Renders `Model` to a ratatui `Frame`. Visual concern only.
- **Event**: Maps terminal events to `Message` variants. Thin translation layer.

This maps directly to our existing patterns:

- `Model` = `EditorState` (plain data struct)
- `Update` = `actions/addNode.ts` (pure function, testable with plain objects)
- `View` = React component (renders state)
- `Event` = hook wrapper (translates user input to action calls)

---

## North Star References

### 1. Charm (Go) — Design Gold Standard

What to steal:

- **Rounded borders everywhere** — `╭╮╰╯` as default, never sharp `┌┐└┘`
- **Adaptive terminal colors** — detect light/dark terminal, adjust palette
- **Contextual help** — footer bar shows available keys for current screen
- **Generous padding** — 1-2 cells inside every border, whitespace between panels
- **Synchronized rendering** — double-buffer via ratatui (no flicker)

### 2. Yazi (Rust, 18k stars) — File Navigation

What to steal:

- **Miller columns** for file browsing (parent / current / preview)
- **Vim keybindings** as default (`j/k` navigation, `/` search, `Space` select)
- **Async I/O** — file listing never blocks the UI
- **Status line** — current path, selection count, file metadata

### 3. GitUI (Rust, 19k stars) — Multi-Panel Workflow

What to steal:

- **Tab-based panel switching** — `1/2/3/4` or `Tab` to switch focus
- **Progressive disclosure** — main list → detail panel → action confirmation
- **Keyboard shortcut bar** at bottom of each panel
- **Popup dialogs** for confirmations (layered over main content)

### 4. Television (Rust, 6k stars) — Search & Selection

What to steal:

- **Fuzzy search** over recipe list
- **Two-pane layout** — list on left, preview on right
- **Minimal chrome** — content fills the space, borders are subtle

---

## System Decomposition

Five independent systems, each following the Bento Box principle. Each system is a folder with focused files. Each system's state logic is testable with `cargo test` — no terminal needed.

```
engine/crates/bnto/src/tui/
├── mod.rs              # Public API: launch_tui() entry point
├── app.rs              # App state machine (which screen is active)
├── event.rs            # Terminal event loop (crossterm → Message)
├── theme.rs            # Color palette, border styles, layout constants
├── screens/
│   ├── mod.rs          # Screen trait + screen registry
│   ├── browser.rs      # System 1: Recipe browser
│   ├── detail.rs       # System 2: Recipe detail + config
│   ├── picker.rs       # System 3: File picker
│   ├── execution.rs    # System 4: Execution progress
│   └── results.rs      # System 5: Results summary
└── widgets/
    ├── mod.rs
    ├── help_bar.rs      # Contextual keyboard shortcut footer
    ├── search_input.rs  # Fuzzy search input field
    ├── file_list.rs     # File browser list with selection
    ├── progress_bar.rs  # Per-file/per-node progress
    └── status_line.rs   # Bottom status bar
```

**File count:** ~18 files, each < 250 lines. No god modules.

### System 1: Recipe Browser

**Purpose:** Browse, search, and select a recipe to run.

**Model:**

```rust
struct BrowserModel {
    recipes: Vec<RecipeSummary>,    // from builtin_recipes()
    filtered: Vec<usize>,          // indices into recipes after search
    selected: usize,               // cursor position in filtered list
    search_query: String,          // current search text
    category_filter: Option<String>, // active category filter
}
```

**Messages:** `SearchChanged(String)`, `SelectNext`, `SelectPrev`, `CategoryChanged(Option<String>)`, `Confirm`, `Quit`

**Tests (pure, no terminal):**

- Filter recipes by search query (fuzzy match on name + description)
- Filter by category
- Combined search + category filter
- Cursor wraps at boundaries
- Empty search shows all recipes
- Confirm returns selected recipe

**Layout:**

```
╭─ bnto ──────────────────────────────────────────╮
│                                                  │
│  Search: compress_                               │
│                                                  │
│  IMAGE                                           │
│  > Compress Images     Reduce file size          │
│    Resize Images       Change dimensions         │
│    Convert Format      PNG, JPEG, WebP           │
│                                                  │
│  SPREADSHEET                                     │
│    Clean CSV           Remove empty rows         │
│                                                  │
╰──────────────────────────────────────────────────╯
  ↑↓ navigate  / search  Enter select  q quit
```

### System 2: Recipe Detail + Config

**Purpose:** Show recipe details and let the user override parameters before running.

**Model:**

```rust
struct DetailModel {
    recipe: RecipeSummary,
    params: Vec<ParamEntry>,       // editable parameter list
    focused_param: usize,          // which param has focus
    editing: bool,                 // whether a param value is being edited
    edit_buffer: String,           // current edit text
}
```

**Messages:** `FocusNext`, `FocusPrev`, `StartEdit`, `EditChanged(String)`, `CommitEdit`, `CancelEdit`, `Confirm`, `Back`

**Tests:**

- Default params populated from recipe metadata
- Edit a param value (start → type → commit)
- Cancel edit restores previous value
- Confirm returns params map
- Back returns to browser without changes

### System 3: File Picker

**Purpose:** Browse filesystem and select input files for the recipe.

**Model:**

```rust
struct PickerModel {
    current_dir: PathBuf,
    entries: Vec<DirEntry>,        // files in current directory
    selected: Vec<PathBuf>,        // multi-selected files
    cursor: usize,                 // current highlight
    filter_extensions: Vec<String>, // from recipe accept spec
}
```

**Messages:** `CursorUp`, `CursorDown`, `ToggleSelect`, `EnterDir`, `ParentDir`, `Confirm`, `Back`

**Tests:**

- Toggle selection adds/removes from selected set
- Enter directory updates entries
- Parent directory navigates up
- Extension filter hides non-matching files
- Confirm with empty selection is prevented
- Sort order: directories first, then files alphabetically

### System 4: Execution Progress

**Purpose:** Show live execution progress — per-file, per-node status.

**Model:**

```rust
struct ExecutionModel {
    status: ExecutionStatus,       // Idle, Running, Complete, Failed
    files: Vec<FileProgress>,      // per-file progress
    nodes: Vec<NodeProgress>,      // per-node status
    elapsed: Duration,
    error: Option<String>,
}
```

**Messages:** `ProgressEvent(ProgressEvent)`, `Complete(Vec<OutputFile>)`, `Failed(String)`, `Cancel`

**Tests:**

- Progress event updates correct file entry
- Node status transitions (pending → active → complete)
- Elapsed time accumulates
- Cancel sets status to cancelled
- Failure captures error message

**Layout:**

```
╭─ Compress Images ────────────────────────────────╮
│                                                   │
│  Running...                              3.2s     │
│                                                   │
│  photo-1.jpg    ████████████████████████  done     │
│  photo-2.jpg    ██████████░░░░░░░░░░░░░  47%      │
│  photo-3.jpg    ░░░░░░░░░░░░░░░░░░░░░░░  waiting  │
│                                                   │
│  Nodes: image-compress ✓  output ⋯               │
│                                                   │
╰───────────────────────────────────────────────────╯
  Esc cancel
```

### System 5: Results Summary

**Purpose:** Show execution results — output files, sizes, timing.

**Model:**

```rust
struct ResultsModel {
    outputs: Vec<OutputFile>,      // output files with paths + sizes
    total_time: Duration,
    savings: Option<SizeSavings>,  // before/after for compression
    cursor: usize,
}
```

**Messages:** `CursorUp`, `CursorDown`, `OpenFile`, `OpenFolder`, `RunAnother`, `Quit`

**Tests:**

- Savings calculation (percentage, absolute)
- Format file sizes (human-readable)
- Format duration (human-readable)
- Open file constructs correct OS command
- Run another returns to browser

**Layout:**

```
╭─ Results ─────────────────────────────────────────╮
│                                                    │
│  Compressed 3 files in 4.1s                        │
│  2.3 MB → 890 KB (61% smaller)                     │
│                                                    │
│  photo-1.jpg    780 KB → 290 KB   ↓ 63%            │
│  photo-2.jpg    920 KB → 340 KB   ↓ 63%            │
│  photo-3.jpg    640 KB → 260 KB   ↓ 59%            │
│                                                    │
│  Output: ./output/                                 │
│                                                    │
╰────────────────────────────────────────────────────╯
  o open file  O open folder  r run another  q quit
```

---

## TDD Approach

### What's testable without a terminal

Every `update()` function is a pure function: `(Model, Message) → Model`. These are testable with standard `cargo test` — no terminal, no mocking, no I/O.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_filters_recipes_by_name() {
        let model = BrowserModel::new(sample_recipes());
        let model = update(model, Message::SearchChanged("compress".into()));
        assert_eq!(model.filtered.len(), 1);
        assert_eq!(model.recipes[model.filtered[0]].name, "Compress Images");
    }

    #[test]
    fn cursor_wraps_at_end() {
        let model = BrowserModel::new(sample_recipes());
        let model = update(model, Message::SelectNext); // 0 → 1
        let model = update(model, Message::SelectNext); // 1 → 2
        let model = update(model, Message::SelectNext); // wraps to 0
        assert_eq!(model.selected, 0);
    }
}
```

### What requires manual/integration testing

- **Rendering**: Visual layout, colors, alignment. Manual verification in a terminal.
- **Event loop**: Key mapping, resize handling. Integration test with a headless terminal (future, not MVP).
- **Engine integration**: Running actual recipes. Covered by existing CLI golden tests — TUI reuses the same `run_pipeline()` path.

### Test count targets (MVP)

| System       | Unit tests | What they cover                             |
| ------------ | ---------- | ------------------------------------------- |
| Browser      | ~10        | Search, filter, cursor, category, selection |
| Detail       | ~8         | Param editing, defaults, commit/cancel      |
| Picker       | ~10        | Navigation, selection, filtering, sort      |
| Execution    | ~8         | Progress events, status transitions, cancel |
| Results      | ~6         | Formatting, savings, navigation             |
| App (router) | ~5         | Screen transitions, back navigation         |
| **Total**    | **~47**    | All pure state logic                        |

---

## User Flow

```
bnto tui
  │
  ├─ Recipe Browser (System 1)
  │     Search, filter by category, select recipe
  │     ↓ Enter
  │
  ├─ Recipe Detail (System 2)
  │     View params, override if needed
  │     ↓ Enter (confirm) or skip if no params
  │
  ├─ File Picker (System 3)
  │     Browse filesystem, multi-select files
  │     ↓ Enter (confirm selection)
  │
  ├─ Execution (System 4)
  │     Live progress — per-file bars, per-node status
  │     ↓ Auto-transition on completion
  │
  └─ Results (System 5)
        Output summary, open files, run another
        ↓ r → back to Browser, q → exit
```

---

## Dependencies

Add to `engine/crates/bnto/Cargo.toml`:

```toml
[dependencies]
ratatui = "0.29"
crossterm = "0.28"
```

Both are mature, well-maintained, and the de facto standard for Rust TUI. No additional deps needed for MVP.

**Existing deps we reuse:**

- `bnto-engine` — recipe registry, pipeline executor, progress events
- `colored` — already imported, but TUI uses ratatui's styling instead
- `indicatif` — CLI progress bars, not used in TUI mode (ratatui handles rendering)

---

## What's NOT in MVP

These are real features that belong in later iterations, not Sprint 10:

- **Fuzzy search** (basic substring match is fine for 15 recipes)
- **Mouse support** (keyboard-first, mouse later)
- **Config file** (`~/.bnto/tui.toml` for custom keybindings, colors)
- **Recipe preview pane** (Television-style side preview)
- **History** (recently run recipes, recent file paths)
- **Drag-and-drop files** (terminal limitation)
- **Responsive layout** (handle small terminals gracefully — later)
- **Themes** (dark/light detection, custom palettes)
- **Golden snapshot tests for rendering** (capture terminal frame output to string buffers, compare against committed golden files for visual regression)

---

## Resolved Decisions

1. **Elm Architecture (TEA) over ad-hoc state.** Pure `update()` functions are testable, composable, and match our existing patterns. All north star apps (Yazi, GitUI) use this.

2. **One module per screen, not one module per widget.** Screens are the unit of composition. Widgets are shared pieces extracted when two screens need them. Start with screens, extract widgets when duplication appears.

3. **Rounded borders (`╭╮╰╯`) as default.** Matches our "round more, not less" design principle. Sharp corners reserved for nested/inner borders if needed.

4. **No animation library.** Unlike the web app where CSS springs are essential, the TUI relies on instant feedback. No `tachyonfx` or frame-based animation for MVP. The "springy" feel comes from immediate responsiveness, not visual animation.

5. **Reuse `builtin_recipes()` directly.** The TUI reads from the same recipe registry as the CLI. No separate TUI catalog or data layer.
