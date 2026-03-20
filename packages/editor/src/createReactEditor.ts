/**
 * createReactEditor — factory that wraps createEditor() with React hooks.
 *
 * Mirrors @bnto/core's reactCore pattern: hooks are closures that capture
 * storeApi at creation time, merged onto each domain client.
 *
 * Returns both the ReactEditorInstance (public) and the raw storeApi
 * (for the rendering pipeline hooks that need direct store access).
 *
 * Usage:
 *   const { instance, storeApi } = createReactEditor(definition);
 *   // In a React component:
 *   const { nodes, selectedNodeId } = instance.nodes.useNodes();
 *   instance.nodes.addNode("image-compress");
 */

import type { StoreApi } from "zustand";
import type { Definition } from "@bnto/core";
import type { EditorStore } from "./store/types";
import type { ReactEditorInstance } from "./reactEditorTypes";
import { createEditor } from "./createEditor";
import { createUseNodes } from "./hooks/factories/createUseNodes";
import { createUseDefinition } from "./hooks/factories/createUseDefinition";
import { createUseExecution } from "./hooks/factories/createUseExecution";
import { createUseHistory } from "./hooks/factories/createUseHistory";
import { createUsePanels } from "./hooks/factories/createUsePanels";
import { createUseNode } from "./hooks/factories/createUseNode";

interface CreateReactEditorResult {
  instance: ReactEditorInstance;
  storeApi: StoreApi<EditorStore>;
}

function createReactEditor(definition?: Definition, cloudId?: string): CreateReactEditorResult {
  const base = createEditor(definition, cloudId);
  const storeApi = base._storeApi;

  const instance: ReactEditorInstance = {
    nodes: { ...base.nodes, useNodes: createUseNodes(storeApi), useNode: createUseNode(storeApi) },
    definition: { ...base.definition, useDefinition: createUseDefinition(storeApi) },
    execution: { ...base.execution, useExecution: createUseExecution(storeApi) },
    history: { ...base.history, useHistory: createUseHistory(storeApi) },
    panels: { ...base.panels, usePanels: createUsePanels(storeApi, base.panels) },
    getState: base.getState,
    subscribe: base.subscribe,
    destroy: base.destroy,
  };

  return { instance, storeApi };
}

export { createReactEditor };
export type { CreateReactEditorResult };
