/**
 * Node service — wraps pure node actions + storeApi.setState().
 *
 * Covers: nodes, edges, configs, selection, containers, insertion context,
 * and RF controlled-mode change handlers.
 *
 * Services call pure actions and apply results to the store. No
 * business logic lives here — it's all in the action files.
 */

import type { StoreApi } from "zustand";
import type { EditorStore } from "../store/types";
import type { NodeService } from "../editorTypes";
import { buildNodeMutations } from "./nodeServiceMutations";
import { buildNodeSetters } from "./nodeServiceSetters";
import { buildContainerMethods } from "./nodeServiceContainers";

function createNodeService(storeApi: StoreApi<EditorStore>): NodeService {
  return {
    ...buildNodeMutations(storeApi),
    ...buildNodeSetters(storeApi),
    ...buildContainerMethods(storeApi),
  };
}

export { createNodeService };
