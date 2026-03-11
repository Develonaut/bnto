# Editor API Layer — Architecture & Implementation Plan

**Last Updated:** March 11, 2026
**Status:** Planned — Sprint 5D
**Prerequisite:** Sprint 4E (editor extraction) complete
**Tracks:** [PLAN.md](../PLAN.md) Sprint 5D
**Pickup skill:** [/editor-api](../skills/editor-api/SKILL.md)

---

## Problem

The editor currently has good bones — pure actions, adapters, a Zustand store — but components access them through 3 inconsistent patterns:

1. `useEditorStore(selector)` — direct store subscription
2. `useEditorActions()` — composed hook returning all actions
3. `storeApi.setState()` with imported pure actions — manual get/call/set

The module-level singleton (`store/instance.ts`) prevents per-instance usage and makes testing awkward. Every test must either mock the singleton or create a store manually and call actions imperatively.

## Goal

Add a **client → service → store** abstraction layer inside `packages/editor/`, mirroring `@bnto/core`'s proven pattern. The result:

1. **Clean per-instance API** — `createEditor(definition?)` returns an `EditorInstance` that components consume via context and tests consume directly without React
2. **Domain-namespaced access** — `editor.nodes.addNode()`, `editor.history.undo()`, `editor.definition.export()` instead of a flat grab-bag
3. **Cross-domain atomicity preserved** — `selectNode` writes both nodes AND panels in a single `storeApi.setState()` call (no double render)
4. **Zero breaking changes during migration** — old hooks work alongside new ones until Phase 5 cleanup

This is step 1 of the Editor API initiative:

1. **Editor API abstraction** (this plan — Sprint 5D)
2. Node Palette + Config UI tweaks
3. Recipe round-trip tests
4. E2E journey tests

---

## Design

### Factory Pattern

```
createEditor(definition?) → EditorInstance
  ├── creates Zustand store (reuses existing createEditorStore)
  ├── creates services (imperative wrappers around pure actions + store)
  ├── creates clients (domain-namespaced, compose services for cross-domain ops)
  └── returns EditorInstance { nodes, definition, execution, history, panels }
```

### Domain Decomposition

| Domain | Covers |
|--------|--------|
| **nodes** | nodes, edges, configs, selection, containers, insertion context |
| **definition** | definition tree, metadata, params, validation, export, dirty flag |
| **execution** | run pipeline, reset, download results, logs, progress |
| **history** | undo, redo, stack management |
| **panels** | open, close, toggle panel visibility |

### EditorInstance Shape

```typescript
interface EditorInstance {
  nodes: NodeClient;
  definition: DefinitionClient;
  execution: ExecutionClient;
  history: HistoryClient;
  panels: PanelClient;
  getState(): EditorState;
  subscribe(listener: (state: EditorState) => void): () => void;
  destroy(): void;
  /** @internal — exposed for React bindings (useStore) */
  _storeApi: StoreApi<EditorStore>;
}
```

### Cross-Domain Atomicity

`selectNode` currently writes both nodes AND panels in one `set()` call. The **nodeClient** handles this by composing state patches from both services into a single `storeApi.setState()` — no double render.

The service layer accesses `storeApi.setState` directly with the combined patch. The client delegates to the service (which handles the atomic write internally), not two separate service calls.

### Layering Rules

```
Clients → Services → Store (via storeApi)
   |
   └── compose multiple services for cross-domain ops
       (services NEVER call other services)
```

- **Services** wrap pure action functions + `storeApi.setState()`
- **Clients** compose services for cross-domain behavior
- **Hooks** use `useEditor()` context + `useStore(editor._storeApi, selector)`
- **Pure actions** remain unchanged — services call them

---

## Phases

### Phase 1: Services + Factory (no component changes)

**Scope:** ~30 new files, zero modified files
**Estimate:** Medium (2-3 hours)
**Persona:** `/reactflow-expert`
**Package:** `[editor]`

Create the service layer and `createEditor()` factory. All existing code continues to work unchanged.

**New files:**

| File | Purpose | Lines (est.) |
|------|---------|-------------|
| `src/services/nodeService.ts` | Wraps addNode, removeNode, selectNode, container ops, RF change handlers | ~60 |
| `src/services/definitionService.ts` | Wraps loadDefinition, createBlank, updateParams, updateSurfacedParam, export, metadata | ~50 |
| `src/services/executionService.ts` | Wraps runExecution, resetRun, downloads | ~30 |
| `src/services/historyService.ts` | Wraps undo, redo, pushUndo, resetHistory | ~20 |
| `src/services/panelService.ts` | Wraps openPanel, closePanel, togglePanel | ~20 |
| `src/clients/nodeClient.ts` | Composes nodeService + panelService (selectNode auto-opens config) | ~30 |
| `src/clients/definitionClient.ts` | Composes definitionService (export includes validation + rfNodesToDefinition) | ~25 |
| `src/clients/executionClient.ts` | Thin passthrough to executionService | ~15 |
| `src/clients/historyClient.ts` | Thin passthrough | ~15 |
| `src/clients/panelClient.ts` | Thin passthrough | ~15 |
| `src/createEditor.ts` | Factory: creates store → services → clients → returns EditorInstance | ~40 |
| `src/editorTypes.ts` | EditorInstance, client type exports | ~50 |

**Tests:**

| File | Covers |
|------|--------|
| `src/createEditor.test.ts` | Factory creates instance, imperative API works end-to-end |
| `src/services/nodeService.test.ts` | add/remove/select via service |
| `src/services/definitionService.test.ts` | load/create/updateParams via service |
| `src/services/historyService.test.ts` | undo/redo via service |

**Reuses (no changes):**

- `actions/*` — all pure actions called by services
- `adapters/*` — all converters called by services
- `store/createEditorStore.ts` — factory called by `createEditor()`
- `store/types.ts` — EditorState/EditorActions types

**Verify:** `task ui:build && task ui:test`

---

### Phase 2: EditorProvider + Context

**Scope:** 2 new files, 2 modified files
**Estimate:** Small (< 1 hour)
**Persona:** `/reactflow-expert`
**Package:** `[editor]`

Add React context so components can access the editor instance. Backwards-compatible.

**New files:**

| File | Purpose |
|------|---------|
| `src/context.ts` | `EditorContext`, `useEditor()` |
| `src/EditorProvider.tsx` | Creates instance, provides via context, wraps ReactFlowProvider |

**Modified files:**

| File | Change |
|------|--------|
| `src/store/instance.ts` | Add `setEditorStore()` — EditorProvider calls it so old hooks work |
| `src/components/EditorCanvas/EditorCanvasRoot.tsx` | Use EditorProvider internally (backwards compatible) |

**Key:** EditorProvider calls `setEditorStore(instance._storeApi)` on init, so the existing `getEditorStore()` singleton still works. This is the compat bridge.

**Verify:** `task ui:build && task ui:test && task e2e`

---

### Phase 3: Domain-Namespaced Hooks

**Scope:** 5 new files
**Estimate:** Small (< 1 hour)
**Persona:** `/reactflow-expert`
**Package:** `[editor]`

New hooks that use context and delegate to client methods. Old hooks still work.

**New files:**

| File | Purpose |
|------|---------|
| `src/hooks/useNodes.ts` | nodes, configs, selectedNodeId + node actions |
| `src/hooks/useDefinition.ts` | definition, metadata, isDirty + definition actions |
| `src/hooks/useExecution.ts` | Replaces useEditorExecution, uses context |
| `src/hooks/useHistory.ts` | undo, redo, canUndo, canRedo |
| `src/hooks/usePanels.ts` | isOpen, open, close, toggle for a panel ID |

**Hook pattern:**

```typescript
function useNodes() {
  const editor = useEditor();
  const nodes = useStore(editor._storeApi, (s) => s.nodes);
  const selectedNodeId = useStore(editor._storeApi, (s) => s.selectedNodeId);
  return {
    nodes,
    selectedNodeId,
    addNode: editor.nodes.addNode,
    removeNode: editor.nodes.removeNode,
    selectNode: editor.nodes.selectNode,
  };
}
```

**Verify:** `task ui:build && task ui:test`

---

### Phase 4: Component Migration

**Scope:** ~10 modified files
**Estimate:** Medium-Large (3-4 hours)
**Persona:** `/reactflow-expert` + `/frontend-engineer`
**Package:** `[editor]`

Migrate components from old hooks to new domain hooks. File-by-file, one commit each.

**Migration order (least risk first):**

1. `EditorToolbar.tsx` — uses useEditorUndoRedo, useEditorStore, useEditorExport, usePanel → `useHistory()`, `useDefinition()`, `usePanels()`
2. `CanvasShell.tsx` — uses useEditorStore selectors → `useNodes()`
3. `NodePaletteDialogRoot.tsx` — uses useEditorActions, useEditorStore, useEditorStoreApi → `useNodes()`, `useEditor()`
4. `ConfigPanelRoot.tsx` — uses useEditorStore, useEditorStoreApi, useEditorActions, raw updateSurfacedParam import → `useNodes()`, `useDefinition()`
5. `RunPanel/*` — uses useEditorExecution → `useExecution()`
6. Remaining: NodeDeleteButton, EditorRightToolbar, other small consumers

**Verify after each file:** `task ui:build && task ui:test && task e2e`

---

### Phase 5: Cleanup

**Scope:** ~12 deleted files, 1 modified file
**Estimate:** Small (< 1 hour)
**Persona:** `/reactflow-expert`
**Package:** `[editor]`

Remove deprecated hooks and module-level singleton.

**Delete:**

- `src/store/instance.ts`
- `src/hooks/useEditorStore.ts`
- `src/hooks/useEditorStoreApi.ts`
- `src/hooks/useEditorActions.ts`
- `src/hooks/useAddNode.ts`
- `src/hooks/useRemoveNode.ts`
- `src/hooks/useUpdateParams.ts`
- `src/hooks/useEditorUndoRedo.ts`
- `src/hooks/usePanel.ts`
- `src/hooks/useEditorExport.ts`
- `src/hooks/useEditorExecution.ts`

**Update:**

- `src/index.ts` — remove old exports, add new ones (`createEditor`, `EditorProvider`, `useEditor`, domain hooks)

**Final verification:**

```bash
task ui:build && task ui:test && task e2e
# Grep for deleted hook names — zero references
```

---

## Testability Improvement

### Before

```typescript
const store = createEditorStore(definition);
const result = addNode(store.getState(), "image-compress");
store.setState(result.nextState);
// Manual: get state, call action, apply result — every time
```

### After

```typescript
const editor = createEditor(definition);
editor.nodes.addNode("image-compress");
expect(editor.getState().nodes).toHaveLength(4);
editor.history.undo();
expect(editor.getState().nodes).toHaveLength(3);
// Clean: imperative API, no React, no store ceremony
```

---

## Critical Files Reference

| File | Role |
|------|------|
| `packages/editor/src/store/createEditorStore.ts` | Reused by createEditor — creates the Zustand store |
| `packages/editor/src/store/types.ts` | EditorState + EditorActions — defines all state/actions to decompose |
| `packages/editor/src/store/instance.ts` | Current singleton — replaced by context in Phase 5 |
| `packages/editor/src/actions/addNode.ts` | Representative pure action — services call these |
| `packages/editor/src/actions/runExecution.ts` | Async action using set/get — needs special service handling |
| `packages/editor/src/hooks/useEditorActions.ts` | Current composed hook — replaced by domain hooks |
| `packages/editor/src/hooks/useEditorExecution.ts` | Current execution hook — replaced by useExecution |
| `packages/editor/src/hooks/useEditorExport.ts` | Export logic — moves into definitionClient |
| `packages/editor/src/adapters/rfNodesToDefinition.ts` | Export adapter — called by definitionService |
| `packages/editor/src/components/EditorCanvas/EditorCanvasRoot.tsx` | Current init point — wraps EditorProvider |
| `packages/core/src/core.ts` | Reference pattern for factory/service/client wiring |

---

## Decisions

| Decision | Rationale |
|----------|-----------|
| Mirror `@bnto/core` pattern (client → service → store) | Proven in production, team knows it, consistent vocabulary |
| 5 domains (nodes, definition, execution, history, panels) | Maps to user mental model, not implementation details |
| Services call pure actions (no duplication) | Actions are the tested unit — services are thin wrappers |
| `_storeApi` exposed on instance | React hooks need `useStore(storeApi, selector)` for subscriptions |
| Compat bridge via `setEditorStore()` | Old hooks keep working during migration — zero big-bang risk |
| Phase-by-phase migration | Each phase is independently shippable and verifiable |
