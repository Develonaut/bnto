import type { StoreApi } from "zustand";
import type { EditorStore } from "./types";

/**
 * Auto-expand all container nodes on store init so children are visible.
 * Clears undo/redo after expansion since initial expansion isn't undoable.
 */
function autoExpandContainers(store: StoreApi<EditorStore>) {
  const containerNodes = store.getState().nodes.filter((n) => n.data.isContainer);
  for (const node of containerNodes) {
    store.getState().expandContainer(node.id);
  }
  store.setState({ undoStack: [], redoStack: [], isDirty: false });
}

export { autoExpandContainers };
