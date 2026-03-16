/**
 * createUseDefinition — hook factory for definition state.
 *
 * Returns a useDefinition hook closure that captures storeApi at creation time.
 * Subscribes to definition-related state slices.
 */

"use client";

import type { StoreApi } from "zustand";
import { useStore } from "zustand";
import type { EditorStore } from "../../store/types";
import type { DefinitionHookResult } from "../../reactEditorTypes";

function createUseDefinition(storeApi: StoreApi<EditorStore>) {
  return function useDefinition(): DefinitionHookResult {
    const definition = useStore(storeApi, (s) => s.definition);
    const recipeMetadata = useStore(storeApi, (s) => s.recipeMetadata);
    const isDirty = useStore(storeApi, (s) => s.isDirty);
    const validationErrors = useStore(storeApi, (s) => s.validationErrors);
    const lastSavedAt = useStore(storeApi, (s) => s.lastSavedAt);

    return { definition, recipeMetadata, isDirty, validationErrors, lastSavedAt };
  };
}

export { createUseDefinition };
