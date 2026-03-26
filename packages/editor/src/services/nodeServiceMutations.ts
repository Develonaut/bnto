/** Node mutations — add/remove nodes (involve pure action functions). */

import type { StoreApi } from "zustand";
import type { NodeTypeName } from "@bnto/core";
import type { EditorStore } from "../store/types";
import type { NodeService } from "../editorTypes";
import { addNode } from "../actions/addNode";
import { removeNode } from "../actions/removeNode";

type MutationMethods = Pick<NodeService, "addNode" | "removeNode">;

function buildNodeMutations(storeApi: StoreApi<EditorStore>): MutationMethods {
  return {
    addNode(
      type: NodeTypeName,
      afterNodeId?: string | null,
      intoContainerId?: string | null,
      defaultParams?: Record<string, unknown>,
    ) {
      const result = addNode(
        storeApi.getState(),
        type,
        afterNodeId,
        intoContainerId,
        defaultParams,
      );
      if (!result) return null;
      storeApi.setState(result.nextState);
      return result.nodeId;
    },

    removeNode(id: string) {
      const nextState = removeNode(storeApi.getState(), id);
      if (!nextState) return false;
      storeApi.setState(nextState);
      return true;
    },
  };
}

export { buildNodeMutations };
