/** Container expansion methods — expand, collapse, toggle. */

import type { StoreApi } from "zustand";
import type { EditorStore } from "../store/types";
import type { NodeService } from "../editorTypes";

type ContainerMethods = Pick<
  NodeService,
  "expandContainer" | "collapseContainer" | "toggleContainerExpanded"
>;

function buildContainerMethods(storeApi: StoreApi<EditorStore>): ContainerMethods {
  return {
    expandContainer(nodeId: string) {
      storeApi.getState().expandContainer(nodeId);
      return storeApi.getState().expandedContainerIds.has(nodeId);
    },

    collapseContainer(nodeId: string) {
      storeApi.getState().collapseContainer(nodeId);
      return !storeApi.getState().expandedContainerIds.has(nodeId);
    },

    toggleContainerExpanded(nodeId: string) {
      storeApi.getState().toggleContainerExpanded(nodeId);
    },
  };
}

export { buildContainerMethods };
