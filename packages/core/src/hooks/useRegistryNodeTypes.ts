"use client";

import { useStore } from "zustand";
import { registryStore } from "../stores/registryStore";

/** Returns node type metadata record — reactive. Re-renders when registry changes. */
export function useRegistryNodeTypes() {
  return useStore(registryStore, (s) => s.nodeTypes);
}
