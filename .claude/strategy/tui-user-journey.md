# TUI User Journey

**Created:** April 17, 2026
**Status:** Planning
**Related:** [tui-strategy.md](tui-strategy.md), [recipe-editors.md](recipe-editors.md), [tui-data-persistence.md](tui-data-persistence.md)

---

## Vision

The TUI is homebase. When you run `bnto`, you land in YOUR space — your recipes, your tools, your preferences. The predefined recipe catalog is a place to discover and pull from, not the default view. The TUI is a personal recipe workbench, not a read-only catalog browser.

**The mental model:** Spotify, not a record store. You open to your library, not the entire catalog. The catalog is there when you want to discover something new. Your library is where you live.

**CLI change:** `bnto` (no arguments) launches the TUI. The `tui` subcommand still works for backward compatibility and flag passing (`bnto tui --theme tokyo`). All existing subcommands (`run`, `list`, `info`, `doctor`) are unchanged.

---

## Design Principles

### Library-First, Not Catalog-First

The home screen is a menu, not a recipe list. The first thing you see is a choice: go to your recipes, browse the catalog, create something new, or change settings. This is intentional — it frames the TUI as YOUR tool, not a product demo.

### Same File, Full Ownership

Every recipe in the bnto ecosystem is a `.bnto.json` file. When a user copies a predefined recipe into their library, they get a full copy of that file. They own it completely — rename it, change every parameter, restructure the nodes, delete it. There's no "linked" or "read-only" state. The library IS a directory of `.bnto.json` files.

### Run From Anywhere

Users can run recipes from two places: their library (quick access to their personalized copies) or the catalog (try something without committing). Both paths lead to the same Detail → Picker → Execution → Results flow.

### Progressive Commitment

The journey from "curious" to "power user" is:

1. Browse the catalog, run a predefined recipe as-is
2. Copy a recipe to your library for quick re-runs
3. Tweak parameters on your copy
4. Edit the recipe structure (add/remove/reorder nodes)
5. Create recipes from scratch

Each step is optional. Most users live at step 2-3.

---

## Screen Map

```
bnto (or bnto tui)
  │
  HOME ──────────────────────────────────────────
  │  Main menu — the TUI's front door
  │
  ├─ MY LIBRARY ─────────────────────────────────
  │     User's .bnto.json collection
  │     Run, edit, rename, delete, organize
  │     │
  │     ├─ [Enter] Run → Detail → Picker → Execution → Results
  │     ├─ [e] Edit → List Editor
  │     ├─ [r] Rename (edit name/description in .bnto.json)
  │     ├─ [d] Delete (with confirmation)
  │     ├─ [/] Search
  │     └��� [Esc] Back to Home
  │
  ├─ RECIPES (catalog) ──────────────────────────
  │     All predefined recipes from the engine
  │     Browse by category, search, preview
  │     │
  │     ├─ [Enter] Run → Detail → Picker → Execution → Results
  │     ├─ [a] Add to Library (copy .bnto.json)
  │     ├─ [Space] Preview (read-only detail)
  │     ├─ [/] Search
  │     └─ [Esc] Back to Home
  │
  ├─ NEW RECIPE ─────────────────────────────────
  │     Wizard → saves to library → List Editor
  │
  ├─ SETTINGS ───────────────────────────────────
  │     Theme, output dir, picker default path
  │
  ├─ DETAIL (pre-run config) ────────────────────
  │     Configure params before running
  │     ├─ [Enter] Confirm → Picker
  │     ├─ [e] Edit → List Editor
  │     └─ [Esc] Back
  │
  ├─ LIST EDITOR ────────────────────────────────
  │     Full recipe editing (add/remove/reorder nodes, edit params)
  │     ├─ [r] Run → Picker → Execution
  │     ├─ [w] Save
  │     └─ [Esc] Back (save prompt if dirty)
  │
  ├─ PICKER → EXECUTION → RESULTS ──────────────
  │     File selection → live progress → output summary
  │     Results: "Add to Library" if run from catalog
  │
  └─ WIZARD ─────────────────────────────────────
        Category → Operation → Config → save → List Editor
```

---

## Home Screen

The home screen is a **centered bento grid** — asymmetric compartments floating in the center of the terminal like a game main menu. See [tui-strategy.md — Home Screen: Centered Bento Grid](tui-strategy.md#home-screen-centered-bento-grid) for the full design vision and ratatui implementation approach.

### Phase 1 (shipped): Simple Menu

The current implementation is a centered text menu with cursor navigation. It works and is functional, but doesn't use the available screen real estate. This is the foundation that Phase 2 builds on.

```
╭─ bnto ──────────────────────────────────────────╮
│                                                  │
│   My Library           Your recipes              │
│   Recipes              Browse & discover         │
│   New Recipe           Create from scratch       │
│   Settings             Preferences               │
│                                                  │
╰──────────────────────────────────────────────────╯
  ↑↓ navigate  Enter select  q quit
```

### Phase 2 (planned): Centered Bento Grid

The target layout: a centered grid with asymmetric compartments. Library on the left (showing actual recipe names), Recipes catalog on the right with category grouping and search, and action panels (New Recipe, Settings) stacked below the library.

```
         ╭─────────────────────╮╭──────────────────────────────╮
         │ ▓▓ MY LIBRARY ▓▓▓▓ ││ ▓▓ RECIPES ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ │
         │                     ││                              │
         │  compress-images    ││  IMAGE                       │
         │  resize-photos      ││   ▸ Compress Images          │
         │  clean-csv          ││     Resize Photos            │
         │                     ││     Convert Format           │
         │  3 recipes          ││  FILE                        │
         ╰─────────────────────╯│     Rename Files             │
         ╭─────────────────────╮│     Clean CSV                │
         │ + New Recipe        ││  VIDEO                       │
         ╰─────────────────────╯│     Download Video           │
         ╭─────────────────────╮│                              │
         │ ⚙ Settings   tokyo ││  / search                    │
         ╰─────────────────────╯╰──────────────────────────────╯
```

**Key differences from Phase 1:**

- Content is information-dense — the Library shows actual recipe names, not just "My Library"
- The Recipes catalog is visible from the home screen with category grouping
- Navigation is compartment-based (Tab moves focus between panels, j/k within)
- Falls back to simple menu on small terminals (< 60 cols or < 20 rows)

**Model:**

```rust
struct HomeModel {
    selected: usize,       // cursor position (0-3)
    items: Vec<HomeItem>,  // My Library, Recipes, New Recipe, Settings
    library_count: usize,  // shown as badge: "My Library (3)"
}
```

**Messages:** `SelectNext`, `SelectPrev`, `Confirm`, `Quit`

**Tests:**

- Cursor wraps at boundaries
- Confirm dispatches correct screen transition
- Library count reflects actual file count in recipes directory

---

## My Library

Lists `.bnto.json` files from the user's recipe directory (`~/.local/share/bnto/recipes/` — see [tui-data-persistence.md](tui-data-persistence.md)). Each recipe shows its name, description, and category — all read from the `.bnto.json` file itself.

```
╭─ My Library (3) ────────────────────────────────╮
│                                                  │
│  Search: _                                       │
│                                                  │
│  > Compress for Web         quality: 60          │
│    My CSV Cleaner           trim whitespace      │
│    Instagram Resize         1080 x 1080          │
│                                                  │
╰──────────────────────────────────────────────────╯
  Enter run  e edit  r rename  d delete  / search  Esc back
```

**Empty state:**

```
╭─ My Library ────────────────────────────────────╮
│                                                  │
│  (●‿●) Your library is empty.                   │
│                                                  │
│  Press Esc to go back, then:                     │
│    Recipes    to browse and add recipes           │
│    New Recipe to create your own                  │
│                                                  │
╰──────────────────────────────────────────────────╯
```

**Model:**

```rust
struct LibraryModel {
    recipes: Vec<LibraryEntry>,    // loaded from ~/.local/share/bnto/recipes/
    filtered: Vec<usize>,          // indices after search
    selected: usize,               // cursor position
    search_query: String,
}

struct LibraryEntry {
    path: PathBuf,                 // full path to .bnto.json
    name: String,                  // from recipe JSON
    description: String,           // from recipe JSON
    category: Option<String>,      // inferred from first node type
}
```

**Key actions:**

| Key   | Action | What happens                                         |
| ----- | ------ | ---------------------------------------------------- |
| Enter | Run    | → Detail screen with recipe loaded                   |
| e     | Edit   | → List Editor with recipe loaded                     |
| r     | Rename | Inline edit of name/description, saved to .bnto.json |
| d     | Delete | Confirmation prompt → removes file from disk         |
| /     | Search | Filter by name/description                           |
| Esc   | Back   | → Home                                               |

**File loading:** On enter, scan the recipes directory for `*.bnto.json` files. Parse each file's top-level `name` and `description` fields. Sort by name (alphabetical). This is fast — we're talking about a handful of files, not thousands.

---

## Recipes (Catalog)

The predefined recipe catalog. This is what the current Browser screen does — browse all built-in recipes by category, search, and select. Two new actions: "Add to Library" and direct "Run."

```
╭─ Recipes ───────────────────────────────────────╮
│                                                  │
│  Search: _                                       │
│                                                  │
│  🍣 IMAGE                                        │
│  > Compress Images       Reduce file size        │
│    Resize Images         Change dimensions       │
│    Convert Format        PNG, JPEG, WebP         │
│                                                  │
│  🍱 SPREADSHEET                                  │
│    Clean CSV             Remove empty rows       │
│                                                  │
╰──────────────────────────────────────────────────╯
  Enter run  a add to library  Space preview  / search  Esc back
```

**Key actions:**

| Key   | Action         | What happens                                       |
| ----- | -------------- | -------------------------------------------------- |
| Enter | Run            | → Detail screen (same as today)                    |
| a     | Add to Library | Copy recipe .bnto.json to user's library directory |
| Space | Preview        | Read-only detail view (params, description)        |
| /     | Search         | Filter recipes                                     |
| Esc   | Back           | → Home                                             |

**"Add to Library" flow:**

1. User presses `a` on a predefined recipe
2. Engine's embedded recipe JSON is written to `~/.local/share/bnto/recipes/{slug}.bnto.json`
3. Status bar confirms: "Added 'Compress Images' to your library"
4. If file already exists: "Already in your library. Replace? [y/N]"

This is the existing Browser screen with two additions (the `a` and `Space` key actions) and a different parent (Home instead of being the root screen).

---

## Detail (Pre-Run Config)

Unchanged from Sprint 11. Shows recipe summary + editable parameters with type-aware controls. Reached from both My Library and Recipes.

One addition: `e` key to jump to the List Editor from Detail.

---

## List Editor

The recipe editing experience defined in [recipe-editors.md](recipe-editors.md). Structural editing: add/remove/reorder nodes, edit parameters inline, save changes.

**Entry points:**

- From My Library: `e` on a recipe → edit in place
- From Detail: `e` → edit before running
- From Wizard: end of wizard → drops into List Editor with new recipe
- From Results: "Edit recipe" → opens editor for the recipe that just ran

**Save behavior:**

- Library recipes: `w` saves directly to the `.bnto.json` file
- Catalog recipes (run from Recipes, then edit): prompts "Save to Library?" on first save
- Dirty state tracked: `Esc` with unsaved changes → "Save before leaving? [Y/n/cancel]"

---

## Results

Mostly unchanged. One new behavior: if the recipe was run from the catalog (not from the user's library), Results shows an "Add to Library" option alongside "Run again" and "Back."

---

## Wizard (New Recipe)

The guided creation flow from [recipe-editors.md](recipe-editors.md) Phase 2. Category → Operation → Config → Done → saves to library → drops into List Editor.

**Entry point:** Home screen → "New Recipe"

---

## Settings

Extends the existing Settings screen. User-configurable output preferences:

- **Theme** — los-angeles, tokyo, monaco (visual accent only)
- **Output directory** — where recipe results are saved
- **Picker default path** — where the file picker starts

Internal persistence (theme selection, telemetry consent) is handled by the data persistence layer — see [tui-data-persistence.md](tui-data-persistence.md). Settings shown here are the user-facing preferences that affect recipe execution.

---

## CLI Change: `bnto` = TUI

```
bnto                       → launches TUI (home screen)
bnto tui                   → same (backward compat)
bnto tui --theme tokyo     → TUI with theme flag
bnto run <recipe> [files]  → direct CLI execution (unchanged)
bnto list                  → CLI recipe listing (unchanged)
bnto info <recipe>         → CLI recipe info (unchanged)
bnto doctor                → dependency check (unchanged)
```

**Implementation:** In the CLI's `main.rs`, when no subcommand is provided (`clap` matches on no arguments), launch the TUI instead of printing help. The `tui` subcommand remains as an explicit alias.

---

## Web Client Parallel

The TUI defines the interaction model. The web inherits the same mental model:

| TUI                | Web                                             |
| ------------------ | ----------------------------------------------- |
| Home screen (menu) | Dashboard / homepage                            |
| My Library         | My Recipes page (localStorage or Convex-backed) |
| Recipes (catalog)  | `/recipes` gallery (existing SEO pages)         |
| Add to Library     | "Save to My Recipes" button                     |
| Run from catalog   | "Try it" on recipe pages (existing)             |
| List Editor        | List editor component (from recipe-editors.md)  |
| Wizard             | "New Recipe" wizard flow                        |
| Settings           | Account settings page                           |

Same journey, different rendering surface. Patterns validated in the TUI transfer directly.

---

## State Machine Updates

The current `AppModel` routes between 6 screens (Browser, Detail, Picker, Execution, Results, Settings). This adds:

| New Screen  | Replaces/Augments                                       |
| ----------- | ------------------------------------------------------- |
| **Home**    | New root screen (replaces Browser as default)           |
| **Library** | New screen                                              |
| **Recipes** | Renamed from Browser (same functionality + new actions) |
| **Editor**  | New screen (List Editor)                                |
| **Wizard**  | New screen                                              |

**Updated `Screen` enum:**

```rust
enum Screen {
    Home,           // NEW — main menu
    Library,        // NEW — user's recipes
    Recipes,        // was Browser — predefined catalog
    Detail,         // unchanged
    Picker,         // unchanged
    Execution,      // unchanged
    Results,        // unchanged
    Settings,       // unchanged
    Editor,         // NEW — List Editor (Sprint 12+)
    Wizard,         // NEW — guided creation (Sprint 13+)
}
```

**Navigation flow:**

```
Home → Library → [Enter] → Detail → Picker → Execution → Results → Home
Home → Recipes → [Enter] → Detail → Picker → Execution → Results → Home
Home → Library → [e] → Editor → Home
Home → New Recipe → Wizard → Editor → Home
Home → Settings → Home

Back navigation:
Library → Home
Recipes → Home
Detail → Library or Recipes (whichever it came from)
Editor → Library (save prompt if dirty)
Wizard → Home (confirm abandon if in progress)
Results → Home
Settings → Home
```

---

## Implementation Phasing

This work is sequenced across sprints. Each phase adds screens incrementally.

### Phase 1: Data Persistence + Home + Library (Sprint 12A)

**Prerequisite for everything else.** Establish the storage layer, home screen, and library screen.

1. `BntoPaths` — centralized path resolution (XDG-compliant)
2. Config migration — move from old location to new
3. Home screen — menu with navigation
4. Library screen — load/list/search `.bnto.json` files
5. "Add to Library" on Recipes screen
6. CLI change — `bnto` (no args) → TUI

### Phase 2: List Editor (Sprint 12)

As defined in [editor-implementation-plan.md](editor-implementation-plan.md). Editor state model, node list, inline config, add/remove/reorder, save.

### Phase 3: Wizard (Sprint 13)

As defined in [recipe-editors.md](recipe-editors.md) Phase 2. Guided creation flow.

### Phase 4: Polish (Sprint 14+)

- Execution history (state directory)
- Recently used recipes (state directory)
- Recipe import/export (drag `.bnto.json` from filesystem)
- Keyboard shortcut customization

---

## Test Strategy

| Screen            | Unit tests | What they cover                                   |
| ----------------- | ---------- | ------------------------------------------------- |
| Home              | ~5         | Cursor, navigation dispatch, library count        |
| Library           | ~12        | File loading, search, delete confirmation, rename |
| Recipes (updated) | ~5         | Add to library, preview, existing browser tests   |
| Editor            | ~75        | (Sprint 12 — see editor-implementation-plan.md)   |
| Wizard            | ~20        | (Sprint 13 — see recipe-editors.md)               |
| App (router)      | ~8         | New screen transitions, back navigation           |
| **New total**     | **~125**   | On top of existing 278 tests                      |

---

## Resolved Decisions

1. **Home screen is a menu, not a recipe list.** Intentional entry point. Frames the TUI as the user's tool, not a product demo.

2. **`bnto` (no args) = TUI.** The TUI is the product. CLI subcommands (`run`, `list`, `info`, `doctor`) remain for scriptable/non-interactive use.

3. **Library = filesystem directory.** `~/.local/share/bnto/recipes/` contains `.bnto.json` files. No database, no sidecar metadata. The recipe file IS the source of truth.

4. **Full ownership on copy.** When you add a predefined recipe to your library, you get a complete `.bnto.json` copy. Change anything — name, params, nodes. It's yours.

5. **Run from both Library and Catalog.** Users don't have to add a recipe to their library before running it. "Add to Library" is for recipes they want to keep, customize, or quick-access.

6. **Existing Browser screen becomes Recipes.** Same code, same UX, two new key bindings (`a` to add, `Space` to preview), different parent (Home instead of root).

7. **Data persistence is the foundation.** The storage layer (config, data, state) must be solid before library features are built. See [tui-data-persistence.md](tui-data-persistence.md).
