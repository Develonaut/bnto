/**
 * Definition service — wraps definition-related actions + storeApi.setState().
 *
 * Covers: loadDefinition, createBlank, updateParams,
 * metadata, dirty flag, revalidation, and export.
 *
 * Export reads state imperatively and calls rfNodesToDefinition.
 */

import type { StoreApi } from "zustand";
import type { Definition } from "@bnto/nodes";
import type { EditorStore, RecipeMetadata } from "../store/types";
import type { DefinitionService } from "../editorTypes";
import { updateParams } from "../actions/updateParams";
import { rfNodesToDefinition } from "../adapters/rfNodesToDefinition";

function createDefinitionService(storeApi: StoreApi<EditorStore>): DefinitionService {
  return {
    loadDefinition(def: Definition) {
      storeApi.getState().loadDefinition(def);
    },

    createBlank() {
      storeApi.getState().createBlank();
    },

    updateParams(nodeId: string, params: Record<string, unknown>) {
      const nextState = updateParams(storeApi.getState(), nodeId, params);
      if (!nextState) return false;
      storeApi.setState(nextState);
      return true;
    },

    setRecipeMetadata(metadata: RecipeMetadata) {
      storeApi.getState().setRecipeMetadata(metadata);
    },

    revalidate() {
      storeApi.getState().revalidate();
    },

    exportAsDefinition() {
      const { nodes, configs, recipeMetadata, definition } = storeApi.getState();
      return rfNodesToDefinition(nodes, recipeMetadata, configs, definition);
    },
  };
}

export { createDefinitionService };
