# Editor Journey — E2E Test Matrix

**Goal:** Verify the complete editor user journey — from entry to export/save. Every test maps to a journey stage from [editor-user-journey.md](../strategy/editor-user-journey.md).

**Last Updated:** March 4, 2026

---

## The Stack

```
User navigates to /editor
  -> AccountGate (sign-in if unauthenticated)
    -> EditorProvider + ReactFlowProvider
      -> EditorCanvas (BentoCanvas + panels)
        -> useEditorStore (Zustand — nodes, configs, undo/redo)
          -> @bnto/nodes (definition CRUD, validation)
            -> core.executions (browser WASM engine)
```

---

## Test Tags

- `@editor` — Editor-specific tests (no backend needed for most)
- `@browser` — Browser execution within editor (WASM engine)
- `@auth` — Needs Convex running (save, My Recipes)

---

## Entry & Navigation Tests

| ID | Test | Journey Stage | Tag | What it verifies |
|----|------|--------------|-----|-----------------|
| EN1 | `/editor` loads blank canvas with Input + Output nodes | Enter | `@editor` | Auto-scaffold, route accessible |
| EN2 | `/editor?from=compress-images` loads predefined recipe | Enter | `@editor` | Recipe loading, all nodes present |
| EN3 | `/editor?from=invalid-slug` shows blank canvas (graceful fallback) | Enter | `@editor` | Invalid slug handling |
| EN4 | "Open in Editor" button on recipe page navigates to `/editor?from={slug}` | Discover | `@editor` | Bridge button wiring |
| EN5 | Unauthenticated user sees AccountGate on `/editor` | Discover | `@auth` | Access control |
| EN6 | Nav link to `/editor` visible for authenticated users | Discover | `@auth` | Navigation integration |

---

## Build & Configure Tests

| ID | Test | Journey Stage | Tag | What it verifies |
|----|------|--------------|-----|-----------------|
| BC1 | Add node from palette — compartment appears on canvas | Build | `@editor` | Node creation, palette interaction |
| BC2 | Remove node — compartment disappears, neighbor auto-selected | Build | `@editor` | Node removal, selection management |
| BC3 | Select node — config panel opens with correct schema fields | Build | `@editor` | Schema-driven config panel |
| BC4 | Update parameter — value persists in config, dirty flag set | Build | `@editor` | Parameter editing, state tracking |
| BC5 | Undo after adding node — node removed | Build | `@editor` | Undo/redo stack |
| BC6 | Redo after undo — node restored | Build | `@editor` | Undo/redo stack |
| BC7 | Canvas full (10 nodes) — add button disabled | Build | `@editor` | Slot limit enforcement |
| BC8 | Input/Output nodes cannot be deleted | Build | `@editor` | I/O node protection |
| BC9 | `visibleWhen` conditions show/hide fields in config panel | Build | `@editor` | Conditional field visibility |
| BC10 | All 10 node types can be added from palette | Build | `@editor` | Complete node type coverage |

---

## Execution Tests

| ID | Test | Journey Stage | Tag | What it verifies |
|----|------|--------------|-----|-----------------|
| EX1 | Drop files into Input node → files appear in config panel | Test | `@editor` `@browser` | Input node file acceptance |
| EX2 | Click Run → compartments show elevation progress (idle → active → completed) | Test | `@editor` `@browser` | Execution visualization |
| EX3 | Execution completes → Output node config panel shows download list | Test | `@editor` `@browser` | Result delivery |
| EX4 | Download single file from Output node | Test | `@editor` `@browser` | File download |
| EX5 | Download All as ZIP from Output node | Test | `@editor` `@browser` | Batch download |
| EX6 | Auto-download toggle — files download automatically on completion | Test | `@editor` `@browser` | Auto-download |
| EX7 | Run Again — resets results, re-executes | Refine | `@editor` `@browser` | Reset/re-run flow |
| EX8 | Error during execution — failed node shows destructive state | Refine | `@editor` `@browser` | Error handling |
| EX9 | Unsupported file type in Input node — clear error message | Test | `@editor` | Input validation |

---

## Export Tests

| ID | Test | Journey Stage | Tag | What it verifies |
|----|------|--------------|-----|-----------------|
| XP1 | Export downloads valid `.bnto.json` | Export | `@editor` | Export functionality |
| XP2 | Exported JSON round-trips — re-import produces identical state | Export | `@editor` | Round-trip fidelity (Success Criterion 2) |
| XP3 | Export with validation errors — export disabled or shows warning | Export | `@editor` | Validation gate |
| XP4 | Exported recipe includes Input/Output nodes with correct parameters | Export | `@editor` | I/O node serialization |

---

## Save Tests (Authenticated)

| ID | Test | Journey Stage | Tag | What it verifies |
|----|------|--------------|-----|-----------------|
| SV1 | Save recipe — persisted to Convex, appears in My Recipes | Save | `@auth` | Persistence |
| SV2 | Free tier: 4th recipe save shows upgrade prompt | Save | `@auth` | Tier limit enforcement |
| SV3 | Load saved recipe from My Recipes into editor | Save | `@auth` | Saved recipe loading |
| SV4 | Unsaved changes warning on navigation away | Save | `@editor` | Dirty state tracking |

---

## Predefined Recipe Parity Tests (Success Criterion 3)

| ID | Test | Recipe | Tag | What it verifies |
|----|------|--------|-----|-----------------|
| PR1 | compress-images via editor produces valid compressed output | compress-images | `@editor` `@browser` | Parity with recipe page |
| PR2 | resize-images via editor produces valid resized output | resize-images | `@editor` `@browser` | Parity with recipe page |
| PR3 | convert-image-format via editor produces valid converted output | convert-image-format | `@editor` `@browser` | Parity with recipe page |
| PR4 | clean-csv via editor produces valid cleaned output | clean-csv | `@editor` `@browser` | Parity with recipe page |
| PR5 | rename-csv-columns via editor produces valid renamed output | rename-csv-columns | `@editor` `@browser` | Parity with recipe page |
| PR6 | rename-files via editor produces valid renamed output | rename-files | `@editor` `@browser` | Parity with recipe page |

---

## Success Criteria Coverage

| Criterion | Tests | How verified |
|-----------|-------|-------------|
| **Task completion (< 5 min from scratch)** | EN1, BC1-BC4, EX1-EX4 | Blank canvas → add node → configure → run → download. Programmatic timing optional |
| **Round-trip fidelity** | XP1, XP2, XP4 | Export → re-import → compare state. Programmatic deep equality |
| **Predefined recipe parity** | PR1-PR6, EN2 | Load via `?from={slug}` → run → compare output to recipe page execution |

---

## Verification Convention

Editor E2E tests follow the same 4-phase approach as [browser-execution.md](browser-execution.md):

```
Phase 1: SETUP   -> Navigate to /editor, load recipe or blank canvas, verify canvas state
Phase 2: BUILD   -> Add/configure nodes, drop files (data attributes, element counts)
Phase 3: EXECUTE -> Click Run, wait for completion (elevation states, data-phase attributes)
Phase 4: VERIFY  -> Download output, validate content (file size, magic bytes, JSON structure)
```

All phases use programmatic assertions. Screenshots are reserved for page-level layout verification only.

---

## Shared Helpers (to create)

| Helper | Purpose |
|--------|---------|
| `navigateToEditor(page, slug?)` | Navigate to `/editor` or `/editor?from={slug}`, wait for canvas |
| `addNodeFromPalette(page, nodeType)` | Open palette, click node type, wait for compartment |
| `selectNode(page, nodeId)` | Click compartment, wait for config panel |
| `updateParameter(page, paramName, value)` | Change a parameter in the config panel |
| `dropFilesIntoInput(page, filePaths[])` | Select Input node, upload files via config panel dropzone |
| `runAndWaitForCompletion(page)` | Click Run, wait for all compartments to reach completed state |
| `exportAndDownload(page)` | Click Export, capture downloaded `.bnto.json` |

---

## Priority Order

### Wave 1: Core (editor MVP verification)
- EN1, EN2: Entry points work
- BC1-BC4: Basic build flow
- EX1-EX4: Execution works end-to-end

### Wave 2: Completeness (full coverage)
- BC5-BC10: Undo/redo, limits, all node types
- EX5-EX9: Batch, auto-download, errors
- XP1-XP4: Export verification
- PR1-PR6: Predefined recipe parity

### Wave 3: Persistence (auth-dependent)
- EN4-EN6: Bridge button, AccountGate, nav
- SV1-SV4: Save/load/limits
