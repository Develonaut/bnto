# Editor Implementation Plan

**Created:** April 16, 2026
**Status:** Backlog — added to PLAN.md as Sprints 12-18 (April 16, 2026)
**Related:** [recipe-editors.md](recipe-editors.md), [tui-strategy.md](tui-strategy.md), [editor-architecture.md](editor-architecture.md)

---

## Context

The recipe editor is the make-or-break product feature. This plan breaks down the editor work from [recipe-editors.md](recipe-editors.md) into PR-sized tasks with TDD red-first strategy. **TUI first, web second** — matching the CLI-first pivot.

### What exists today

**TUI (Rust):**

- 6 screens (browser, detail, picker, execution, results, settings), 278 tests
- Detail screen: text-only parameter editing (all params rendered as text inputs)
- **Zero recipe creation/editing** — no add/remove/reorder nodes, no save/load, no custom recipes
- Sprint 11 (in progress): enriching Detail screen with type-aware controls (boolean toggles, enum selects, number sliders, validation, conditional visibility)

**Web (TypeScript):**

- `@bnto/editor` package: Visual editor only (React Flow canvas, CompartmentNodes, ConfigPanel)
- `@bnto/form` package: Schema-driven forms auto-generating controls from `ParameterDef`
- Zustand store with full undo/redo, controlled mode, 90+ tests
- Editor API layer: `createEditor()`, 5 domain clients, services
- Frozen since CLI/TUI-first pivot — maintenance mode

### Prerequisites

Sprint 11 (Engine-Owned Node Schema + TUI Schema-Driven Config) must complete before editor work begins. It delivers:

- Engine as single source of truth for all parameter metadata
- Type-aware TUI controls (boolean, enum, number)
- `@bnto/nodes` collapsed to barrel over engine-generated code
- Custom recipe loading (`bnto tui recipe.bnto.json`)

---

## Dependency Chain

```
Sprint 11 (in progress)         ← prerequisite for all editor work
  │
  ├── Sprint 12: TUI List Editor  ← Phase 1 TUI
  │     │
  │     ├── Sprint 13: TUI Wizard     ← Phase 2 TUI
  │     ├── Sprint 14: TUI Code + Graph  ← Phase 3 TUI
  │     └── Sprint 15: bnto-editor crate extraction  ← package boundary
  │
  └── Sprint 16: Web List Editor  ← Phase 1 Web
        │
        ├── Sprint 17: Web Wizard     ← Phase 2 Web
        └── Sprint 18: Web Code Editor  ← Phase 3 Web (Visual already exists)
```

TUI sprints are independent of web sprints. Web work can start after Sprint 11, but TUI is higher priority.

---

## Phase 1: TUI List Editor (Sprint 12)

**Goal:** Transform the TUI from a read-only runner into a recipe editor. The List editor is the center of gravity — it handles 90% of editing needs and establishes the editor state model that all other editor types share.

**What changes:** New "Editor" screen (System 6) with the List editor view. This is distinct from the existing Detail screen, which remains as the "configure + run" flow for predefined recipes. The Editor screen is for creating and modifying recipe structures.

**Entry points:**

- `bnto tui --new` → blank recipe → Editor screen (Wizard in Phase 2)
- `bnto tui recipe.bnto.json` → load file → Editor screen
- Browser screen: `e` on a predefined recipe → clone into Editor

### Wave 1: Editor State Model + Recipe I/O

**PR 1: Editor state model (`bnto-core` or `bnto` crate)**

The shared recipe editing state that all editor types operate on. Pure data, no UI.

```
EditorModel {
    recipe_name: String,
    recipe_description: String,
    nodes: Vec<EditorNode>,         // ordered list of nodes
    selected_index: Option<usize>,  // focused node
    dirty: bool,                    // unsaved changes
    undo_stack: Vec<EditorSnapshot>,
    redo_stack: Vec<EditorSnapshot>,
    source: EditorSource,           // New | File(PathBuf) | Predefined(slug)
}

EditorNode {
    id: String,
    node_type: String,              // e.g., "image-compress"
    label: String,                  // from NodeTypeInfo
    params: HashMap<String, Value>, // current parameter values
    expanded: bool,                 // collapsed/expanded in list view
}
```

RED tests (define the API before implementing):

- `test_new_editor_has_empty_nodes`
- `test_add_node_appends_with_defaults`
- `test_remove_node_by_index`
- `test_reorder_swap_adjacent`
- `test_reorder_bounds_check`
- `test_undo_restores_previous_state`
- `test_redo_after_undo`
- `test_undo_stack_clears_redo_on_new_action`
- `test_dirty_flag_set_on_mutation`
- `test_dirty_flag_cleared_on_save`
- `test_node_defaults_from_metadata` — adding "image-compress" auto-fills quality=80

Scope: ~15-20 tests, pure Rust, no TUI dependency.

**PR 2: Recipe file I/O**

Load `.bnto.json` from disk → `EditorModel`. Serialize `EditorModel` → `.bnto.json` on save.

RED tests:

- `test_load_valid_recipe` — roundtrip a known `.bnto.json`
- `test_load_invalid_json_returns_error`
- `test_load_missing_file_returns_error`
- `test_save_produces_valid_json` — output parses as valid Definition
- `test_save_roundtrip_fidelity` — load → save → load produces identical model
- `test_save_creates_parent_dirs` — save to nested path
- `test_load_predefined_recipe` — clone from builtin registry

Scope: ~8-10 tests, file I/O (integration tests).

### Wave 2: List Editor Screen (TUI)

**PR 3: Editor screen shell + navigation**

New `screens/editor.rs` with basic list rendering and navigation. No editing yet — just viewing.

```
EditorMessage {
    FocusNext,
    FocusPrev,
    ExpandToggle,       // Enter on a node
    Back,               // Esc → confirm discard if dirty
    SwitchEditor(type), // Tab → cycle List/Graph/Code
}
```

RED tests:

- `test_focus_next_moves_down`
- `test_focus_prev_moves_up`
- `test_focus_wraps_at_boundaries`
- `test_expand_toggle_opens_node`
- `test_expand_toggle_closes_node`
- `test_back_when_clean_returns_to_browser`
- `test_back_when_dirty_shows_confirm`
- `test_editor_renders_node_list` — each node shows label + hero param

Scope: ~10 tests. TEA pattern: EditorModel + EditorMessage + update().

**PR 4: Node add/remove**

Add nodes via a picker overlay, remove with confirmation.

```
EditorMessage {
    ...existing...
    OpenPicker,          // 'a' key → node type picker
    PickerSelect(type),  // Enter in picker → add node
    PickerCancel,        // Esc in picker
    DeleteNode,          // 'd' key on focused node
    ConfirmDelete,       // 'y' in confirm
    CancelDelete,        // 'n' / Esc in confirm
}
```

RED tests:

- `test_add_node_inserts_after_cursor`
- `test_add_node_opens_picker_overlay`
- `test_picker_shows_all_browser_node_types`
- `test_picker_search_filters_by_name`
- `test_picker_select_adds_node_with_defaults`
- `test_delete_node_shows_confirmation`
- `test_confirm_delete_removes_node`
- `test_cancel_delete_preserves_node`
- `test_add_triggers_undo_snapshot`
- `test_delete_triggers_undo_snapshot`

Scope: ~12 tests. Picker is a sub-model within EditorModel (like search in BrowserModel).

**PR 5: Node reorder**

Move nodes up/down with keyboard shortcuts.

RED tests:

- `test_shift_j_moves_node_down`
- `test_shift_k_moves_node_up`
- `test_reorder_at_top_noop`
- `test_reorder_at_bottom_noop`
- `test_reorder_updates_cursor_to_follow`
- `test_reorder_triggers_undo_snapshot`

Scope: ~6 tests. Simple swap operations on the node vector.

### Wave 3: Inline Config + Schema Controls

**PR 6: Inline parameter editing**

When a node is expanded, render its parameters using the type-aware controls from Sprint 11. Reuse the control dispatch logic already built for the Detail screen.

RED tests:

- `test_expanded_node_shows_parameters`
- `test_number_param_renders_with_constraints`
- `test_enum_param_renders_options`
- `test_boolean_param_renders_toggle`
- `test_param_edit_updates_editor_model`
- `test_param_edit_triggers_undo_snapshot`
- `test_visible_when_hides_param`
- `test_description_shown_in_help_area`
- `test_preset_shortcut_jumps_value`

Scope: ~10 tests. Leverages Sprint 11's control widgets.

### Wave 4: Save + Entry Points

**PR 7: Save workflow**

Save edited recipe to disk. Confirm overwrite for existing files, prompt for path on new recipes.

RED tests:

- `test_save_writes_to_source_path`
- `test_save_new_recipe_prompts_for_path`
- `test_save_clears_dirty_flag`
- `test_save_as_creates_new_file`
- `test_ctrl_s_triggers_save`

Scope: ~5 tests.

**PR 8: Entry points + app integration**

Wire the Editor screen into the app state machine.

- `bnto tui --new` → Editor with blank recipe
- `bnto tui recipe.bnto.json` → Editor with loaded recipe
- Browser: `e` on recipe → clone into Editor
- Detail: `e` → open in Editor (carry params)

RED tests:

- `test_new_flag_opens_editor_blank`
- `test_file_arg_opens_editor_loaded`
- `test_browser_e_key_clones_to_editor`
- `test_editor_back_returns_to_source_screen`

Scope: ~5 tests. Updates to `app.rs` Screen enum and routing.

**Sprint 12 totals: ~8 PRs, ~75 tests, ~1500-2000 LOC**

---

## Phase 2: TUI Wizard (Sprint 13)

**Goal:** Guided recipe creation for first-time users. "What do you want to do?" → category → operation → config → done.

**Depends on:** Sprint 12 (editor state model + List editor)

### Wave 1: Wizard Flow

**PR 9: Wizard state model**

```
WizardModel {
    step: WizardStep,              // Category | Operation | Config | Complete
    category: Option<NodeCategory>,
    selected_type: Option<String>,
    params: HashMap<String, Value>,
    available_categories: Vec<NodeCategory>,
    available_types: Vec<NodeTypeInfo>,
}

WizardStep { Category, Operation, Config, Complete }
```

RED tests:

- `test_wizard_starts_at_category`
- `test_select_category_advances_to_operation`
- `test_select_operation_advances_to_config`
- `test_config_complete_builds_recipe`
- `test_back_returns_to_previous_step`
- `test_skip_to_end_uses_defaults`
- `test_categories_from_engine_metadata` — not hardcoded
- `test_operations_filtered_by_category`
- `test_wizard_produces_valid_editor_model`

Scope: ~10 tests.

**PR 10: Wizard screen + rendering**

TUI screen with step-by-step prompts.

RED tests:

- `test_category_step_shows_all_categories`
- `test_operation_step_shows_filtered_types`
- `test_config_step_shows_type_aware_controls`
- `test_complete_step_shows_summary`
- `test_enter_advances_step`
- `test_esc_goes_back`

Scope: ~8 tests.

### Wave 2: Wizard-to-Editor Handoff

**PR 11: Auto-name + handoff to List editor**

Wizard completes → generates recipe name → populates EditorModel → switches to List editor.

RED tests:

- `test_wizard_generates_name_from_operation` — "Compress Images v1"
- `test_handoff_populates_editor_model`
- `test_handoff_switches_to_list_editor`
- `test_wizard_accessible_from_browser_n_key`

Scope: ~5 tests.

**Sprint 13 totals: ~3 PRs, ~25 tests**

---

## Phase 3: TUI Code + Graph Views (Sprint 14)

**Goal:** Power-user and read-only views.

### Wave 1: Code Editor ($EDITOR Integration)

**PR 12: $EDITOR integration**

Press `c` in Editor → export to temp `.bnto.json` → open in `$EDITOR` → validate on return → update state.

RED tests:

- `test_code_view_creates_temp_file`
- `test_code_view_opens_editor_env_var` — respects $EDITOR, falls back to $VISUAL, then vi
- `test_valid_json_on_return_updates_model`
- `test_invalid_json_on_return_shows_error`
- `test_unchanged_file_no_dirty_flag`
- `test_code_view_roundtrip_fidelity`

Scope: ~6 tests.

### Wave 2: Read-Only Graph View

**PR 13: ASCII graph renderer**

Press `g` in Editor → read-only box-drawing view of recipe structure. `l` or Esc returns to List.

```
┌─────────┐    ┌─────────────┐    ┌──────────┐
│  Input   │───▶│  Compress   │───▶│  Output  │
│  (files) │    │  (q: 80%)   │    │  (zip)   │
└─────────┘    └─────────────┘    └──────────┘
```

RED tests:

- `test_graph_renders_linear_pipeline`
- `test_graph_renders_container_children_indented`
- `test_graph_shows_hero_param_in_box`
- `test_graph_press_l_returns_to_list`
- `test_graph_is_read_only` — no mutations possible

Scope: ~5 tests.

**Sprint 14 totals: ~2 PRs, ~11 tests**

---

## Phase 4: bnto-editor Crate Extraction (Sprint 15)

**Goal:** Extract the shared editor state model into a standalone `bnto-editor` crate.

**Why now:** After Sprint 12-14, the editor state model, I/O, and operations are proven in production. The TUI `screens/editor.rs` currently owns the model. Extracting to a crate makes it reusable for desktop (Tauri) and potentially a library for third-party integrations.

**PR 14: Extract `bnto-editor` crate**

Move from `bnto/src/tui/screens/` to `engine/crates/bnto-editor/`:

- `EditorModel`, `EditorNode`, `EditorSnapshot`
- `EditorCommand` enum (add, remove, reorder, configure, undo, redo)
- Recipe I/O (load/save `.bnto.json`)
- Wizard state model
- Validation logic

TUI becomes a consumer: `bnto-editor` state + TUI rendering.

RED tests (crate-level):

- All existing editor unit tests move to the crate
- New: `test_editor_model_is_send_sync` — required for async consumers
- New: `test_editor_command_apply_is_pure` — no side effects

**Sprint 15 totals: ~1 PR, migration + ~5 new tests**

---

## Phase 5: Web List Editor (Sprint 16)

**Goal:** Add the List editor to the web `@bnto/editor` package alongside the existing Visual editor.

**Depends on:** Sprint 11 only (web doesn't depend on TUI sprints).

### Wave 1: List View Component

**PR 15: List editor component**

New `ListEditor` component in `@bnto/editor`. Renders store nodes as an ordered step list.

RED tests:

- `test_list_editor_renders_all_nodes`
- `test_node_expand_shows_parameters`
- `test_node_collapse_hides_parameters`
- `test_collapsed_shows_label_and_hero_param`
- `test_keyboard_navigation` — Arrow keys, Enter to expand

Scope: ~6 tests.

**PR 16: Reorder + Add/Remove in List**

DnD reorder with `@dnd-kit`, plus keyboard (Shift+Arrow). Node picker popover for adding.

RED tests:

- `test_drag_reorder_updates_store`
- `test_shift_arrow_reorder`
- `test_add_node_from_picker`
- `test_delete_node_with_undo`

Scope: ~5 tests.

### Wave 2: Editor Switcher

**PR 17: Editor type switcher**

Toolbar control to switch between Visual (existing), List (new), and Code (Phase 7). State preserved across switches.

RED tests:

- `test_switch_visual_to_list_preserves_state`
- `test_switch_list_to_visual_preserves_state`
- `test_default_editor_from_preference`
- `test_preference_persisted_to_localstorage`

Scope: ~5 tests.

**PR 18: Per-node JSON toggle**

In the List editor, each expanded step has a "Show JSON" toggle that reveals the raw JSON for that node.

RED tests:

- `test_show_json_toggle_renders_node_json`
- `test_json_is_read_only_in_list_view`
- `test_toggle_remembers_state_per_node`

Scope: ~3 tests.

**Sprint 16 totals: ~4 PRs, ~19 tests**

---

## Phase 6: Web Wizard (Sprint 17)

**Goal:** Guided recipe creation for web users.

**PR 19: Wizard flow component**

Step-by-step form: category → operation → config → done. Card grid for categories, radio list for operations. Config step reuses `@bnto/form` SchemaForm.

RED tests:

- `test_wizard_renders_category_step`
- `test_category_select_shows_operations`
- `test_operation_select_shows_config`
- `test_config_uses_schema_form`
- `test_complete_populates_store`
- `test_wizard_to_list_handoff`
- `test_back_navigation`
- `test_skip_to_end`

Scope: ~8 tests.

**Sprint 17 totals: ~1 PR, ~8 tests**

---

## Phase 7: Web Code Editor (Sprint 18)

**Goal:** JSON editor with CodeMirror 6, following the existing [code-editor.md](code-editor.md) strategy.

### Wave 1: CM6 Integration

**PR 20: CodeMirror 6 editor view**

JSON editing with validation, hover info, and autocompletion from engine JSON Schema.

RED tests:

- `test_code_editor_renders_json`
- `test_json_validation_shows_errors`
- `test_autocompletion_suggests_properties`
- `test_store_sync_debounced`
- `test_external_update_annotation`

Scope: ~6 tests.

### Wave 2: Slash Commands

**PR 21: Slash command insertion**

`/` trigger inserts a complete, valid node JSON block.

RED tests:

- `test_slash_shows_node_type_menu`
- `test_select_node_inserts_json`
- `test_inserted_json_has_defaults`
- `test_slash_at_valid_position_only`

Scope: ~4 tests.

**Sprint 18 totals: ~2 PRs, ~10 tests**

---

## Summary

| Sprint | Phase  | Platform          | Focus                        | PRs         | Tests          |
| ------ | ------ | ----------------- | ---------------------------- | ----------- | -------------- |
| 11     | Prereq | Engine + TUI + TS | Schema-driven config         | 7           | (in progress)  |
| 12     | 1      | TUI               | List Editor                  | 8           | ~75            |
| 13     | 2      | TUI               | Wizard                       | 3           | ~25            |
| 14     | 3      | TUI               | Code + Graph views           | 2           | ~11            |
| 15     | 4      | Engine            | bnto-editor crate extraction | 1           | ~5             |
| 16     | 5      | Web               | List Editor                  | 4           | ~19            |
| 17     | 6      | Web               | Wizard                       | 1           | ~8             |
| 18     | 7      | Web               | Code Editor (CM6)            | 2           | ~10            |
|        |        |                   | **Totals**                   | **~28 PRs** | **~153 tests** |

### Velocity estimate

Each PR is sized for a single agent session (< 15 files, < 400 lines). Assuming 1-2 PRs per day with review:

- **Sprint 12 (TUI List):** ~4-5 days
- **Sprint 13 (TUI Wizard):** ~2 days
- **Sprint 14 (TUI Code+Graph):** ~1-2 days
- **Sprint 15 (Crate extraction):** ~1 day
- **Sprint 16 (Web List):** ~2-3 days
- **Sprint 17 (Web Wizard):** ~1 day
- **Sprint 18 (Web Code):** ~1-2 days

**Total: ~12-16 days after Sprint 11 completes.**

---

## Key Design Decisions

### 1. Editor screen vs. Detail screen

The existing Detail screen stays as-is — it's the "configure parameters for a predefined recipe and run it" flow. The new Editor screen is "create, modify, and restructure recipes." They share the type-aware controls (Sprint 11) but serve different user intents.

### 2. State model lives in Rust

The `EditorModel` is a pure Rust struct with `update()` function (TEA pattern). This means:

- Testable without a terminal (pure unit tests)
- Extractable into `bnto-editor` crate (Sprint 15)
- Reusable for desktop (Tauri) and potentially WASM
- Same pattern as the existing TUI screens

### 3. Web mirrors Rust concepts, doesn't share code

The web `@bnto/editor` package has its own Zustand store (already built). We don't transpile the Rust EditorModel to WASM for the web. Instead, both platforms implement the same operations (add, remove, reorder, configure, undo, redo) against the same recipe JSON format. The strategy doc [recipe-editors.md](recipe-editors.md) defines the shared concepts.

### 4. Phase ordering rationale

List Editor first because:

- Highest user impact (90% of editing needs)
- Simplest to implement (ordered list, no spatial layout)
- Establishes the state model that all other editors use
- Works equally well on TUI and web

Wizard second because:

- On-ramp for new users
- Reuses List editor controls for the config step
- Small incremental effort once the state model exists

Code/Graph last because:

- Power-user features (lower priority for initial launch)
- Graph is read-only on TUI (minimal investment)
- Code editor (CM6) is web-only and already has a strategy doc

### 5. Per-node JSON toggle

Resolved: Yes. In the List editor, each expanded step has a "Show JSON" toggle. This bridges the gap between List and Code editors without forcing a mode switch. Power users get raw access, casual users ignore it.

### 6. Auto-iteration preserved (CRITICAL)

All 15 predefined recipes use `settings.iteration: "auto"` — the engine automatically wraps contiguous `PerFile` processors in implicit per-file loops. This means `Input → Compress → Output` processes each file individually without needing explicit loop/group containers.

**The editor MUST preserve this:**

- **New recipes** default to `settings.iteration: "auto"` — users never manually add loop nodes for simple pipelines
- **The List editor** shows a flat step list that matches this mental model — no artificial loop nesting
- **Recipe I/O** roundtrips the full `Definition` including `settings` — never strip or lose this field
- **The Wizard** produces recipes with `settings.iteration: "auto"` — flat and simple
- **Export** preserves `settings` in `.bnto.json` output

**When users DO need explicit containers:** Multi-node branching, conditional paths, or batch-then-per-file sequences (e.g., merge CSVs → transform each row). The editor should make containers available but never require them for the 90% case.

**Contract:** `InputCardinality` on `NodeMetadata` drives the behavior. `PerFile` nodes auto-iterate, `Batch` nodes receive all files, `Source` nodes run once. See [smart-iteration.md](smart-iteration.md) for the full design.

**RED tests for this (Sprint 12, PR 2):**

- `test_new_recipe_has_auto_iteration` — blank recipe defaults to `settings.iteration: "auto"`
- `test_save_preserves_iteration_setting` — roundtrip doesn't lose settings
- `test_load_recipe_without_settings_defaults_explicit` — backward compat

---

## Open Items

- **hero_param inference:** Need to decide whether to add `hero_param: Option<String>` to `ParameterDef` in the engine or infer it in consumers (first required param, or first param with non-default value). Engine-side is cleaner.
- **Node type palette categories:** The wizard needs categories from engine metadata. Sprint 11 already exposes `NodeTypeInfo.category`. Verify this is sufficient or if we need subcategories.
- **Recipe validation on save:** The engine already has `DEFINITION_JSON_SCHEMA`. The editor should validate against it before saving, surfacing errors inline rather than on save failure.

---

## References

| Document                                         | Covers                                                           |
| ------------------------------------------------ | ---------------------------------------------------------------- |
| [recipe-editors.md](recipe-editors.md)           | Strategy — 4 editor types, interaction models, design philosophy |
| [tui-strategy.md](tui-strategy.md)               | TUI architecture — TEA, screens, theming                         |
| [editor-architecture.md](editor-architecture.md) | Web editor — store, hooks, package strategy                      |
| [code-editor.md](code-editor.md)                 | CodeMirror 6 — tech choice, slash commands                       |
| [config-controls.md](config-controls.md)         | Schema-to-control mapping                                        |
| [PLAN.md](../PLAN.md)                            | Sprint 11 details (prerequisite)                                 |
