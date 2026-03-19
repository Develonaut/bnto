/** Registry client — read-only public API for predefined recipes and node types. */

import { registryStore } from "../stores/registryStore";
import type { Recipe, NodeTypeInfo } from "@bnto/nodes";

export function createRegistryClient() {
  return {
    getRecipes: (): readonly Recipe[] => registryStore.getState().recipes,

    getRecipesByCategory: (category: string): Recipe[] =>
      registryStore.getState().recipes.filter((r) => r.category === category),

    getNodeTypes: () => registryStore.getState().nodeTypes,

    getBrowserNodeTypes: (): NodeTypeInfo[] =>
      Object.values(registryStore.getState().nodeTypes).filter((t) => t.browserCapable),

    getCategories: () => registryStore.getState().categories,

    getProcessors: () => registryStore.getState().processors,

    isInitialized: () => registryStore.getState().initialized,
  } as const;
}

export type RegistryClient = ReturnType<typeof createRegistryClient>;
