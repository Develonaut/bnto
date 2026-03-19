/** registryStore — read-only Zustand store for predefined recipes and node type metadata. */

import { createEnhancedStore } from "./createEnhancedStore";
import type { NodeTypeInfo, NodeTypeName } from "@bnto/registry";
import type { RegistryData } from "@bnto/registry";

interface RegistryStoreState extends RegistryData {
  initialized: boolean;
  populate: (data: RegistryData) => void;
  reset: () => void;
}

const EMPTY: RegistryData = {
  recipes: [],
  nodeTypes: {} as Record<NodeTypeName, NodeTypeInfo>,
  categories: [],
  processors: [],
};

export const registryStore: import("zustand/vanilla").StoreApi<RegistryStoreState> =
  createEnhancedStore<RegistryStoreState>()((set) => ({
    ...EMPTY,
    initialized: false,

    populate: (data) =>
      set(() => ({
        ...data,
        initialized: true,
      })),

    reset: () =>
      set(() => ({
        ...EMPTY,
        initialized: false,
      })),
  }));

export type { RegistryStoreState, RegistryData };
