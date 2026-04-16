---
name: editor-api
description: Pick up and execute the next task in the Editor API Layer sprint (5D)
args: "[--status]"
---

# Editor API Layer — Pickup Skill

You are working on the **Editor API Layer** sprint (Sprint 5D). This sprint adds a `client → service → store` abstraction layer to `packages/editor/`, mirroring `@bnto/core`'s pattern.

## Arguments

| Flag        | Description                                                                                  |
| ----------- | -------------------------------------------------------------------------------------------- |
| _(no flag)_ | **Default.** Pick up the next unclaimed task in the current wave and execute it.             |
| `--status`  | **Status check.** Report progress — what's done, what's next, any blockers. No code changes. |

## Before Every Session

1. **Read the plan document:** `.claude/strategy/editor-api.md` — full architecture, phases, file lists, design decisions.
2. **Read PLAN.md Sprint 5D:** `.claude/PLAN.md` — find Sprint 5D, check which tasks are done (`[x]`), claimed (`**CLAIMED**`), or available (`[ ]`).
3. **Read the editor store types:** `packages/editor/src/store/types.ts` — `EditorState`, `EditorActions`, `EditorStore`.
4. **Read the store factory:** `packages/editor/src/store/createEditorStore.ts` — understand how actions are wired.
5. **Read the instance singleton:** `packages/editor/src/store/instance.ts` — this is what we're replacing with context.
6. **Check git status:** Ensure no uncommitted changes in `packages/editor/`.

## Architecture Quick Reference

```
createEditor(definition?)
  ├── Zustand store (createEditorStore)
  ├── Services (nodeService, definitionService, executionService, historyService, panelService)
  ├── Clients (nodeClient, definitionClient, executionClient, historyClient, panelClient)
  └── EditorInstance { nodes, definition, execution, history, panels, getState(), subscribe(), destroy() }
```

**Key rules:**

- Services wrap **existing pure actions** + `storeApi.setState()` — do NOT duplicate action logic
- Clients compose services for **cross-domain** operations (e.g., selectNode writes nodes + opens config panel)
- Services NEVER call other services — cross-domain lives in clients only
- Old hooks continue to work until Phase 5 cleanup
- Each phase is independently shippable and verifiable

## Wave → Phase Mapping

| Wave   | Phase   | Summary                                                 |
| ------ | ------- | ------------------------------------------------------- |
| Wave 1 | Phase 1 | Services + `createEditor()` factory + tests             |
| Wave 2 | Phase 2 | EditorProvider + context + compat bridge                |
| Wave 3 | Phase 3 | Domain-namespaced hooks (useNodes, useDefinition, etc.) |
| Wave 4 | Phase 4 | Component migration (file-by-file)                      |
| Wave 5 | Phase 5 | Delete deprecated hooks + singleton                     |

## How to Pick Up a Task

### If `--status` flag:

Read PLAN.md Sprint 5D. Report:

- Completed tasks (count + list)
- Current wave status (all done? any claimed?)
- Next available task(s)
- Any blockers or dependencies
- Estimated remaining effort

Then STOP.

### If no flag (default — execute):

1. **Find the current wave** in PLAN.md Sprint 5D — the first wave with unclaimed tasks.
2. **Do NOT skip waves.** If Wave 1 has unclaimed tasks, work on Wave 1 even if Wave 3 looks more interesting.
3. **Claim the task:** Change `- [ ]` to `- [ ] **CLAIMED**` in PLAN.md.
4. **Create a branch** (if not already on one): `git checkout -b refactor/editor-api main`
5. **Activate personas:** `/reactflow-expert` for all waves. Add `/frontend-engineer` for Wave 4.
6. **Read the relevant files** listed in the task before writing code.
7. **Implement** following the patterns in `.claude/strategy/editor-api.md`.
8. **Test:** `task ui:build && task ui:test` after every file. Add `task e2e` for Waves 2+.
9. **Run `/pre-commit`** before committing.
10. **Mark done:** Change `- [ ] **CLAIMED**` to `- [x]` in PLAN.md.
11. **If the wave is complete**, note that the next wave is unblocked.

## Service Pattern (copy this)

```typescript
// services/nodeService.ts
import type { StoreApi } from "zustand";
import type { EditorStore } from "../store/types";
import { addNode } from "../actions/addNode";

export function createNodeService(storeApi: StoreApi<EditorStore>) {
  return {
    addNode(type: NodeTypeName, afterNodeId?: string | null, intoContainerId?: string | null) {
      const result = addNode(storeApi.getState(), type, afterNodeId, intoContainerId);
      if (!result) return null;
      storeApi.setState(result.nextState);
      return result.nodeId;
    },
    // ... other methods follow the same pattern
  };
}

export type NodeService = ReturnType<typeof createNodeService>;
```

## Client Pattern (copy this)

```typescript
// clients/nodeClient.ts
import type { NodeService } from "../services/nodeService";
import type { PanelService } from "../services/panelService";

export function createNodeClient(nodeService: NodeService, panelService: PanelService) {
  return {
    ...nodeService,
    // Cross-domain: selectNode also manages config panel
    // Note: The store's selectNode action already writes nodes + panels atomically.
    // The service wraps that single atomic call. The client just delegates.
    selectNode: nodeService.selectNode,
  };
}

export type NodeClient = ReturnType<typeof createNodeClient>;
```

## Hook Pattern (copy this)

```typescript
// hooks/useNodes.ts
import { useStore } from "zustand";
import { useEditor } from "../context";

export function useNodes() {
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

## Test Pattern (copy this)

```typescript
// createEditor.test.ts
import { describe, it, expect } from "vitest";
import { createEditor } from "./createEditor";

describe("createEditor", () => {
  it("creates an instance with domain clients", () => {
    const editor = createEditor();
    expect(editor.nodes).toBeDefined();
    expect(editor.definition).toBeDefined();
    expect(editor.history).toBeDefined();
    expect(editor.panels).toBeDefined();
    expect(editor.execution).toBeDefined();
  });

  it("addNode via nodes client works imperatively", () => {
    const editor = createEditor();
    const id = editor.nodes.addNode("image-compress");
    expect(id).toBeTruthy();
    expect(editor.getState().nodes.length).toBeGreaterThan(0);
  });

  it("undo reverses addNode", () => {
    const editor = createEditor();
    const before = editor.getState().nodes.length;
    editor.nodes.addNode("image-compress");
    expect(editor.getState().nodes.length).toBe(before + 1);
    editor.history.undo();
    expect(editor.getState().nodes.length).toBe(before);
  });
});
```

## DO NOT

- Do NOT modify pure action files (`actions/*`) — services wrap them, not replace them
- Do NOT modify adapter files (`adapters/*`) — services call them as-is
- Do NOT delete old hooks until Phase 5 — they're the compat bridge
- Do NOT skip waves — sequential dependency
- Do NOT create new Zustand stores — reuse `createEditorStore`
- Do NOT duplicate logic that already exists in actions — services are thin wrappers
- Do NOT add `@bnto/core` as a dependency for this work — the editor API is self-contained within `packages/editor/`

## Reference

- **Full plan:** `.claude/strategy/editor-api.md`
- **Editor architecture:** `.claude/strategy/editor-architecture.md`
- **Core API pattern (reference):** `.claude/scopes/backend/core-api.md`
- **Code standards:** `.claude/rules/code-standards.md`
- **Editor store types:** `packages/editor/src/store/types.ts`
- **Editor actions:** `packages/editor/src/actions/` (all files)
- **Editor hooks (current):** `packages/editor/src/hooks/` (all files)
