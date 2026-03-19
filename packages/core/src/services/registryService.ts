/**
 * Registry service — initializes the registry store from @bnto/registry.
 *
 * This import boundary is the seam: when predefined recipes come from an API
 * instead of static data, only this file changes.
 */

import { getAllRecipes, getAllNodeTypes, getAllCategories, getAllProcessors } from "@bnto/registry";
import { registryStore } from "../stores/registryStore";

export function createRegistryService() {
  return {
    /** Populate the registry store from @bnto/registry static lookups. */
    initialize: () => {
      if (registryStore.getState().initialized) return;
      registryStore.getState().populate({
        recipes: getAllRecipes(),
        nodeTypes: getAllNodeTypes(),
        categories: [...getAllCategories()],
        processors: [...getAllProcessors()],
      });
    },
  } as const;
}

export type RegistryService = ReturnType<typeof createRegistryService>;
