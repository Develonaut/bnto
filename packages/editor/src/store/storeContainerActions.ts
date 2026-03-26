/** Store actions: container expansion (expand, collapse, toggle). */

import type { EditorStore } from "./types";
import { expandContainer } from "../actions/expandContainer";
import { collapseContainer } from "../actions/collapseContainer";

type Setter = (partial: Partial<EditorStore> | ((s: EditorStore) => Partial<EditorStore>)) => void;
type Getter = () => EditorStore;

function createContainerActions(set: Setter, get: Getter) {
  return {
    expandContainer: (nodeId: string) => {
      const result = expandContainer(get(), nodeId);
      if (result) set(result);
    },

    collapseContainer: (nodeId: string) => {
      const result = collapseContainer(get(), nodeId);
      if (result) set(result);
    },

    toggleContainerExpanded: (nodeId: string) => {
      const state = get();
      const action = state.expandedContainerIds.has(nodeId) ? collapseContainer : expandContainer;
      const result = action(state, nodeId);
      if (result) set(result);
    },
  };
}

export { createContainerActions };
