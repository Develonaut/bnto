/**
 * Registry service — initializes the registry store from @bnto/nodes static data.
 *
 * This import boundary is the seam: when predefined recipes come from an API
 * instead of static data, only this file changes.
 */

import { RECIPES, NODE_TYPE_INFO, CATEGORIES, PROCESSORS } from "@bnto/nodes";
import { registryStore } from "../stores/registryStore";

export function createRegistryService() {
  return {
    /** Populate the registry store from @bnto/nodes static exports. */
    initialize: () => {
      if (registryStore.getState().initialized) return;
      registryStore.getState().populate({
        recipes: RECIPES,
        nodeTypes: NODE_TYPE_INFO,
        categories: [...CATEGORIES],
        processors: [...PROCESSORS],
      });
    },
  } as const;
}

export type RegistryService = ReturnType<typeof createRegistryService>;
