/**
 * createEditor — factory that assembles the full editor API.
 *
 * Creates a Zustand store, wraps it with services, composes clients,
 * and returns an EditorInstance with domain-namespaced access.
 *
 * Auto-persist: subscribes to store changes and saves the recipe
 * to core on every dirty mutation (debounced). No React effects —
 * this is a plain store subscription created at construction time.
 *
 * Usage:
 *   const editor = createEditor(definition);
 *   editor.nodes.addNode("image-compress");
 *   editor.history.undo();
 *   const def = editor.definition.exportAsDefinition();
 */

import { core } from "@bnto/core";
import type { Definition } from "@bnto/core";
import type { EditorState } from "./store/types";
import type { EditorInstance } from "./editorTypes";
import { createEditorStore } from "./store/createEditorStore";
import { createNodeService } from "./services/nodeService";
import { createDefinitionService } from "./services/definitionService";
import { createExecutionService } from "./services/executionService";
import { createHistoryService } from "./services/historyService";
import { createPanelService } from "./services/panelService";
import { createNodeClient } from "./clients/nodeClient";
import { createDefinitionClient } from "./clients/definitionClient";
import { createExecutionClient } from "./clients/executionClient";
import { createHistoryClient } from "./clients/historyClient";
import { createPanelClient } from "./clients/panelClient";
import { rfNodesToDefinition } from "./adapters/rfNodesToDefinition";
import { debounce } from "./draft/debounce";

const PERSIST_DELAY_MS = 1000;

type EditorStoreApi = ReturnType<typeof createEditorStore>;

/** Compose domain clients from services wired to the store. */
function createClients(storeApi: EditorStoreApi) {
  const nodes = createNodeClient(createNodeService(storeApi));
  const definition = createDefinitionClient(createDefinitionService(storeApi));
  const execution = createExecutionClient(createExecutionService(storeApi));
  const history = createHistoryClient(createHistoryService(storeApi));
  const panels = createPanelClient(createPanelService(storeApi));
  return { nodes, definition, execution, history, panels };
}

/** Subscribe to store changes and auto-persist dirty state to core. */
function subscribeAutoPersist(storeApi: EditorStoreApi) {
  const persistDebounced = debounce(PERSIST_DELAY_MS);
  const unsubscribe = storeApi.subscribe((state) => {
    if (!state.isDirty) return;
    persistDebounced.schedule(() => {
      const s = storeApi.getState();
      const exported = rfNodesToDefinition(s.nodes, s.recipeMetadata, s.configs, s.definition);
      core.recipes.save(exported, s.recipeMetadata);
    });
  });
  return { persistDebounced, unsubscribe };
}

function createEditor(definition?: Definition): EditorInstance {
  const storeApi = createEditorStore(definition);
  const clients = createClients(storeApi);
  const { persistDebounced, unsubscribe } = subscribeAutoPersist(storeApi);

  return {
    ...clients,
    getState: () => storeApi.getState(),
    subscribe: (listener: (state: EditorState) => void) => storeApi.subscribe(listener),
    destroy() {
      persistDebounced.flush();
      persistDebounced.destroy();
      unsubscribe();
    },
    _storeApi: storeApi,
  };
}

export { createEditor };
