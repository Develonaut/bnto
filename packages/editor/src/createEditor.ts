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

function createEditor(definition?: Definition, cloudId?: string): EditorInstance {
  const storeApi = createEditorStore(definition, cloudId);

  // --- Services: thin wrappers around pure actions + store ---
  const nodeService = createNodeService(storeApi);
  const definitionService = createDefinitionService(storeApi);
  const executionService = createExecutionService(storeApi);
  const historyService = createHistoryService(storeApi);
  const panelService = createPanelService(storeApi);

  // --- Clients: domain-namespaced API (compose services for cross-domain) ---
  const nodes = createNodeClient(nodeService);
  const def = createDefinitionClient(definitionService);
  const execution = createExecutionClient(executionService);
  const history = createHistoryClient(historyService);
  const panels = createPanelClient(panelService);

  // --- Auto-persist: save recipe to core on dirty mutations ---
  const persistDebounced = debounce(PERSIST_DELAY_MS);
  const unsubscribePersist = storeApi.subscribe((state) => {
    if (!state.isDirty) return;
    persistDebounced.schedule(() => {
      const s = storeApi.getState();
      const exported = rfNodesToDefinition(s.nodes, s.recipeMetadata, s.configs, s.definition);
      core.recipes.save(exported, s.recipeMetadata);
    });
  });

  const instance: EditorInstance = {
    nodes,
    definition: def,
    execution,
    history,
    panels,

    getState(): EditorState {
      return storeApi.getState();
    },

    subscribe(listener: (state: EditorState) => void) {
      return storeApi.subscribe(listener);
    },

    destroy() {
      persistDebounced.flush();
      persistDebounced.destroy();
      unsubscribePersist();
    },

    _storeApi: storeApi,
  };

  return instance;
}

export { createEditor };
