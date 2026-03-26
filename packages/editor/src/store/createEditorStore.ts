/**
 * Editor store factory — state layer with simple setters (controlled mode).
 *
 * Business logic lives in pure action functions (editor/actions/).
 * Hooks are thin wrappers bridging actions to the store.
 *
 *   Pure actions -> Services -> Clients (EditorInstance)
 */

import { createEnhancedStore } from "@bnto/core";
import type { Definition } from "@bnto/core";
import type { EditorStore } from "./types";
import { resolveInitialState } from "./resolveInitialState";
import { loadDefinition } from "../actions/loadDefinition";
import { createBlank } from "../actions/createBlank";
import { pushUndoAction, undoAction, redoAction } from "./historyActions";
import { EXECUTION_DEFAULTS } from "./executionDefaults";
import { autoExpandContainers } from "./autoExpandContainers";
import { buildInitialState } from "./buildInitialState";
import { createGraphActions } from "./storeGraphActions";
import { createConfigActions } from "./storeConfigActions";
import { createPanelActions } from "./storePanelActions";
import { createContainerActions } from "./storeContainerActions";
import { createExecutionActions } from "./storeExecutionActions";
import { createUtilityActions } from "./storeUtilityActions";

function createEditorStore(definition?: Definition, cloudId?: string) {
  const initial = resolveInitialState(definition, cloudId);

  const store = createEnhancedStore<EditorStore>()((set, get) => ({
    ...buildInitialState(initial),
    ...EXECUTION_DEFAULTS,

    loadDefinition: (def) => set(loadDefinition(def)),
    createBlank: () => set(createBlank()),

    ...createGraphActions(set),
    ...createConfigActions(set),

    pushUndo: () => set(pushUndoAction(get())),
    undo: () => {
      const r = undoAction(get());
      if (r) set(r);
    },
    redo: () => {
      const r = redoAction(get());
      if (r) set(r);
    },

    ...createPanelActions(set),
    ...createContainerActions(set, get),

    setInsertAfterNodeId: (id) => set({ insertAfterNodeId: id }),
    setInsertIntoContainerId: (id) => set({ insertIntoContainerId: id }),

    ...createExecutionActions(set, get),
    ...createUtilityActions(set, get),
  }));

  if (definition) autoExpandContainers(store);
  return store;
}

export { createEditorStore };
