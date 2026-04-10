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

### Motorway Design Language → 8-Bit Terminal

3D surface elevation and spring animations don't exist in a terminal. But the _feelings_ they create do. Our approach: **render the Motorway palette through an 8-bit retro lens** — like playing Mini Motorways on a Game Boy Color or SNES. The warm colors stay, the kawaii personality stays, but expressed through chunky pixels and box-drawing characters instead of CSS gradients and spring physics.

**The visual concept:** Imagine the web app's polished Motorway surfaces translated to a retro console. Rounded borders become the friendly `╭╮╰╯`. Terracotta and golden accents become bold ANSI colors. The kawaii sushi mascots become tiny pixel-art sprites built from half-block characters (▀▄█░▓▒). It should feel like a premium retro game UI — not a raw terminal app.

| Web (Motorway)                  | TUI (8-Bit Motorway)                              | The feeling it creates |
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
| Kawaii sushi SVG mascots        | Tiny half-block pixel-art sushi sprites           | Brand personality      |
| 3D surface elevation            | Double-line borders + block shading               | Visual depth/hierarchy |

**What we do:**

- Kawaii sushi mascots as small pixel-art sprites (3-5 lines tall, using ▀▄█░▓▒)
- Chunky, geometric, retro-game aesthetic — think SNES menu screens
- Rounded borders everywhere — `╭╮╰╯` as default, never sharp
- Color used for warmth and meaning, not decoration

**What we don't do:**

- No large ASCII art banners — sprites stay small (3-7 lines), content is the hero
- No slow animations or transitions — terminals reward instant feedback
- No color overload — reserve color for meaning (status, selection, categories)
- No emoji for mascots — they render inconsistently across terminals

### Sushi Mascots in the Terminal

The web app has kawaii sushi characters from Catalyst Labs (see [brand-messaging-audit.md](brand-messaging-audit.md)). The TUI gets terminal-native versions of the same roster — expressed through three tiers depending on context.

#### Tier 1: Emoji (inline, status bars, category labels)

Emoji are the lightest touch — one character, instantly recognizable, zero rendering complexity. Use them for inline context where a full sprite would be overkill.

| Category    | Emoji | Use in TUI                                      |
| ----------- | ----- | ----------------------------------------------- |
| Image       | 🍣    | Category label in recipe browser, status bar    |
| Spreadsheet | 🍱    | Category label, bento = compartments = grid     |
| File        | 🍙    | Category label, onigiri = simple building block |
| Video       | 🐙    | Category label, octopus = multiple streams      |
| Brand       | 🍣    | App header accent, footer                       |

**Emoji gotchas:**

- Width varies across terminals (1 cell vs 2 cells). Use `unicode-width` crate to measure
- Some terminals render monochrome. That's fine — the shape is what matters
- Never rely on emoji as the sole UI indicator — always pair with text

#### Tier 2: Kaomoji (compact, help text, loading states)

Japanese-style emoticons that work in any monospace font. Zero Unicode compatibility issues.

```
Sushi roll:  (●‿●)     or  (°ω°)
Onigiri:     (△‿△)     or  (▽ω▽)
Bento box:   [▪‿▪]     or  {◻‿◻}
Octopus:     (∿‿∿)     or  (~ω~)
```

Use these in:

- Loading spinners: `(°ω°) Browsing recipes...`
- Help text: `(●‿●) Tip: press / to search`
- Empty states: `(△‿△) No files selected yet`
- Error messages: `(∿‿∿) Something went wrong`

#### Tier 3: Half-Block Pixel Art (header, splash, about screen)

Small sprites built from Unicode half-block characters (▀▄█░▓▒). 3-5 lines tall, 8-12 characters wide. These are the "hero" versions — used sparingly for maximum impact.

**The technique:** Each terminal cell is split into top/bottom halves using ▀ (upper) and ▄ (lower), giving 2x vertical pixel resolution. Combined with ratatui's color system, you can create recognizable 8-bit sprites at tiny sizes.

```
Sushi roll (5 lines, colored with theme palette):
 ▄████▄
█░░░░░░█
█▓▓▓▓▓▓█    <- nori (teal)
█░░●░●░█    <- rice + filling (cream + terracotta)
 ▀████▀

Onigiri (4 lines):
   ▄▄
  █░░█
 █▓░░▓█     <- nori wrap (teal)
  ▀▀▀▀

Bento box (3 lines):
╔══╦══╗
║▓▓║░░║     <- compartments with different fills
╚══╩══╝

Octopus (4 lines):
  ▄██▄
  █●●█
 ▄████▄
 ╘╘╘╘╘╘     <- tentacles
```

**These are concepts, not final.** The actual sprites will be refined once we can test rendering across target terminals (kitty, Alacritty, iTerm2, Windows Terminal, GNOME Terminal). The goal is recognizable at 3-5 lines, charming, and consistent with the web mascots' personality.

**Color mapping (Motorway palette → terminal):**

- Sushi rice → white/cream (background token)
- Nori/seaweed → teal (secondary token)
- Fish/filling → terracotta (primary token)
- Bento box frame → golden (accent token)
- Eyes/face → foreground token

#### Where each tier appears

| Context               | Tier    | Example                                    |
| --------------------- | ------- | ------------------------------------------ |
| Recipe browser list   | Emoji   | `🍣 Compress Images`                       |
| Category headers      | Emoji   | `🍱 SPREADSHEET`                           |
| Status bar            | Emoji   | `🍣 bnto v1.0 — Los Angeles theme`         |
| Loading states        | Kaomoji | `(°ω°) Loading recipes...`                 |
| Help/tips             | Kaomoji | `(●‿●) Press Enter to select`              |
| Error states          | Kaomoji | `(∿‿∿) Oops — file not found`              |
| App header (splash)   | Sprite  | Half-block sushi roll next to "bnto" title |
| Settings/about screen | Sprite  | Full mascot roster                         |

### TUI Theming: Accent-Only Strategy

The web themes (Los Angeles, Tokyo, Monaco) assume they control both background and foreground — paired colors that only work together. The TUI can't paint the terminal background reliably, and even if it could, fighting the user's terminal setup violates "go with the grain."

**The solution: themes only control accent colors.** Body text uses the terminal's native foreground (`Color::Reset`), which is always readable against the terminal's native background. Themes express personality through _where_ and _how_ accents are applied — not by repainting the entire surface.

This matches how the old Go TUI handled theming: theme switching changed accent placement and the active color, not the whole palette.

#### Per-Theme Active Colors

Each theme has a distinct **active color** — the single most visible element that gives the theme its identity. This color is used for selected items, borders, and key hints.

| Theme       | Active Color  | RGB              | Personality            |
| ----------- | ------------- | ---------------- | ---------------------- |
| Los Angeles | Terracotta    | `(240, 101, 66)` | Warm California sunset |
| Tokyo       | Electric blue | `(78, 134, 255)` | Cool neon nighttime    |
| Monaco      | Sunset amber  | `(235, 136, 59)` | Golden hour glow       |

All three are mid-brightness saturated colors — readable on both dark and light terminals.

#### Color Role Map

| Role                        | Source                       | Notes                                           |
| --------------------------- | ---------------------------- | ----------------------------------------------- |
| Body text (names, headings) | `Color::Reset`               | Terminal native — always readable               |
| Muted text (descriptions)   | `Color::DarkGray`            | Terminal-safe on both light and dark            |
| Category headers            | `Color::DarkGray` + **bold** | Subtle but distinct from descriptions           |
| Selected/active item        | `theme.active` + **bold**    | Per-theme — the primary differentiator          |
| Borders                     | `theme.active`               | Persistent theme identity across every frame    |
| Key hints (help bar)        | `theme.active` + **bold**    | Consistent accent color for attention           |
| Key descriptions            | `Color::DarkGray`            | Same as muted — recedes behind the key hint     |
| Success status              | `theme.success`              | Per-theme green — mid-brightness, works on both |
| Error status                | `theme.destructive`          | Per-theme red — mid-brightness, works on both   |

#### What This Means for palette.rs

The generated palette constants (`FOREGROUND`, `MUTED_FOREGROUND`, `BACKGROUND`) still exist for web parity, but the TUI **ignores** them for text rendering. The `Theme` struct uses `Color::Reset` and `Color::DarkGray` for terminal-native text, and only pulls accent/status colors from the palette.

The `Theme` struct gains an `active` field — the per-theme hero color. For Tokyo, this uses `FOCUS_RING` (electric blue) instead of `PRIMARY` (terracotta), giving each theme a distinct visual signature.

#### Terminal Dark/Light Auto-Detection (Future)

With accent-only theming, auto-detection becomes less urgent — themes are readable regardless of terminal background. But it's still a nice-to-have for defaulting the theme on first launch:

| Priority | Method                      | Reliability |
| -------- | --------------------------- | ----------- |
| 1        | `--theme` CLI flag          | 100%        |
| 2        | `terminal-colorsaurus`      | ~85%        |
| 3        | `COLORFGBG` env var         | ~40%        |
| 4        | macOS `AppleInterfaceStyle` | macOS only  |
| 5        | Default to Los Angeles      | 100%        |

Not in MVP. The `--theme` flag and Settings screen cover the user need.

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
╭─ 🍣 bnto ───────────────────────────────────────╮
│                                                  │
│  Search: compress_                               │
│                                                  │
│  🍣 IMAGE                                        │
│  > Compress Images     Reduce file size          │
│    Resize Images       Change dimensions         │
│    Convert Format      PNG, JPEG, WebP           │
│                                                  │
│  🍱 SPREADSHEET                                  │
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

**Wave 2+ deps (add when needed):**

```toml
tui-slider = "0.2"            # Slider widget for Number params with min/max bounds
terminal-colorsaurus = "0.4"  # Dark/light terminal detection (used by bat, delta)
unicode-width = "0.2"         # Accurate emoji/CJK width measurement
```

**Existing deps we reuse:**

- `bnto-engine` — recipe registry, pipeline executor, progress events
- `colored` — already imported, but TUI uses ratatui's styling instead
- `indicatif` — CLI progress bars, not used in TUI mode (ratatui handles rendering)

---

## Ecosystem Libraries

Evaluated from [awesome-ratatui](https://github.com/ratatui/awesome-ratatui). Strategy: **vendor small widgets** (cherry-pick into `src/tui/widgets/`, adapted to our pure-data convention), **depend on feature-rich crates** (add to `Cargo.toml`).

### Vendor from ratatui-cheese (MIT)

[ratatui-cheese](https://github.com/shashanktomar/ratatui-cheese) v0.6 — Bubbletea-inspired widget collection. Cherry-pick individual widgets, adapt to our pure-data render convention (return `Line`/`Vec<Line>` instead of writing to `Frame`).

| Widget  | Use in TUI                                              | Wave |
| ------- | ------------------------------------------------------- | ---- |
| Input   | String param editing on detail screen                   | 2    |
| Select  | Enum param selection on detail screen                   | 2    |
| Spinner | Execution in-progress indicator                         | 3    |
| List    | Evaluate for file picker (may overlap our browser list) | 2    |

### Keep as dependency

| Crate        | Version | Use                                  | Wave |
| ------------ | ------- | ------------------------------------ | ---- |
| `tui-slider` | 0.2     | Number params with min/max on detail | 2    |

### Evaluate before building

| Crate            | What it does                          | When to evaluate |
| ---------------- | ------------------------------------- | ---------------- |
| ratatui-explorer | File system tree browser              | Wave 2 (picker)  |
| tachyonfx        | Terminal visual effects (transitions) | Wave 4 (polish)  |
| ratatui-toaster  | Toast notifications                   | Wave 4 (polish)  |

### Not useful for us

- **tui-realm** — full framework with Redux-like state, conflicts with our TEA architecture
- **rat-salsa** — another full framework, same conflict
- **ratzilla** — browser-based TUI (WASM), different target than our native CLI
- **edtui** — vim-like text editor, overkill for our param inputs
- **ratatui-image** — image rendering in terminal, not needed for our workflow
- **tui-nodes** — node graph visualization, we have the web editor for that
- **ratatui-interact** — mouse interaction focus, we're keyboard-first

---

## Param Control Matrix

The detail screen renders editable controls for recipe parameters. The engine's `ParameterType` enum maps to TUI controls — mirroring how `@bnto/form` maps schemas to web form controls.

### ParameterType → TUI Control

| ParameterType         | TUI Control            | Source        | Notes                                      |
| --------------------- | ---------------------- | ------------- | ------------------------------------------ |
| `Number` + has bounds | `tui-slider`           | Dependency    | `SliderState::new(value, min, max)`        |
| `Number` + no bounds  | Input (text)           | Vendor cheese | Parse to f64, validate on commit           |
| `String`              | Input (text)           | Vendor cheese | Show placeholder from `ParameterDef`       |
| `Boolean`             | Toggle `[x]`/`[ ]`     | Hand-build    | ~20 lines, Space to toggle                 |
| `Enum { options }`    | Select (dropdown)      | Vendor cheese | Options from enum variants                 |
| `Object`              | Read-only JSON         | Hand-build    | Display as formatted text, no editing      |
| `File { accept }`     | (skip — picker screen) | —             | File selection handled by dedicated screen |

### ParameterDef Fields → TUI Affordances

| ParameterDef field       | TUI affordance                                    |
| ------------------------ | ------------------------------------------------- |
| `constraints.min/max`    | Slider bounds, input validation                   |
| `constraints.required`   | Visual indicator (asterisk), prevent empty commit |
| `placeholder`            | Ghost text in Input widget                        |
| `default`                | Pre-filled value, shown in muted style            |
| `description`            | Help text below control or in status line         |
| `conditional_visibility` | Show/hide control based on sibling param values   |

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
- **Terminal dark/light auto-detection** (`terminal-colorsaurus` crate — strategy captured above, implement after MVP)
- **Pixel-art sushi sprites** (Tier 3 half-block mascots — design finalized, render after core screens work)
- **Golden snapshot tests for rendering** (capture terminal frame output to string buffers, compare against committed golden files for visual regression)

---

## Resolved Decisions

1. **Elm Architecture (TEA) over ad-hoc state.** Pure `update()` functions are testable, composable, and match our existing patterns. All north star apps (Yazi, GitUI) use this.

2. **One module per screen, not one module per widget.** Screens are the unit of composition. Widgets are shared pieces extracted when two screens need them. Start with screens, extract widgets when duplication appears.

3. **Rounded borders (`╭╮╰╯`) as default.** Matches our "round more, not less" design principle. Sharp corners reserved for nested/inner borders if needed.

4. **No animation library.** Unlike the web app where CSS springs are essential, the TUI relies on instant feedback. No `tachyonfx` or frame-based animation for MVP. The "springy" feel comes from immediate responsiveness, not visual animation.

5. **Reuse `builtin_recipes()` directly.** The TUI reads from the same recipe registry as the CLI. No separate TUI catalog or data layer.

6. **8-bit retro Motorway aesthetic.** The TUI can't do 3D surfaces or spring animations. Instead, we render the Motorway palette through a retro console lens — chunky half-block sprites, warm ANSI colors, rounded box-drawing borders. Like playing Mini Motorways on a Game Boy Color.

7. **Three-tier mascot system.** Emoji (🍣🍙🍱🐙) for inline labels, kaomoji `(●‿●)` for help/loading text, half-block pixel-art sprites for headers/splash. Emoji are primary — sprites are a polish layer added after core functionality works.

8. **Runtime theme switching via Settings screen.** `--theme` CLI flag for startup, `s` key from Browser → Settings screen for live switching. Three themes: Los Angeles (light), Tokyo (dark), Munich (sunset). Already implemented.

9. **Auto-detection deferred.** Terminal dark/light detection (`terminal-colorsaurus`) is captured in strategy but not in MVP. The `--theme` flag and Settings screen cover the user need. Auto-detection is a future quality-of-life improvement.

10. **Accent-only TUI theming.** Themes control accent colors (selected items, borders, key hints, status), not body text or backgrounds. Body text uses `Color::Reset` (terminal native foreground), muted text uses `Color::DarkGray`. This guarantees readability on any terminal background — light or dark. Each theme has a distinct active color: Los Angeles = terracotta, Tokyo = electric blue, Monaco = sunset amber. Borders use the active color for persistent theme identity.

11. **Vendor small widgets, depend on large ones.** Cherry-pick individual widgets from MIT crates (ratatui-cheese) into `src/tui/widgets/`, adapted to our pure-data render convention. Keep crates with extensive APIs (tui-slider) as Cargo dependencies. This avoids framework lock-in while reusing battle-tested code.

12. **Schema-to-control mapping mirrors `@bnto/form`.** The detail screen maps `ParameterType` → TUI control using the same engine metadata that `@bnto/form` uses for web forms. Same source of truth (engine `metadata()`), different rendering target. See Param Control Matrix section.
