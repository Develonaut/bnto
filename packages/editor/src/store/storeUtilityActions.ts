/** Store actions: revalidation, execution state, metadata, history reset. */

import type { EditorStore, ExecutionState, RecipeMetadata } from "./types";
import { revalidateState } from "./revalidateState";

type Setter = (partial: Partial<EditorStore> | ((s: EditorStore) => Partial<EditorStore>)) => void;
type Getter = () => EditorStore;

function createUtilityActions(set: Setter, get: Getter) {
  return {
    revalidate: () => {
      const state = get();
      set({
        validationErrors: revalidateState(state.nodes, state.configs, state.recipeMetadata),
      });
    },

    setExecutionState: (executionState: ExecutionState) => {
      set({ executionState });
    },

    resetNodeStatuses: () => {
      set({ executionState: {} });
    },

    setRecipeMetadata: (newMetadata: RecipeMetadata) => {
      set({ recipeMetadata: newMetadata, isDirty: true });
    },

    resetHistory: () => {
      set({ undoStack: [], redoStack: [] });
    },
  };
}

export { createUtilityActions };
