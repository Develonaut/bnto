"use client";

import { useStore } from "zustand";
import { registryStore } from "../stores/registryStore";

/** Returns all node categories — reactive. Re-renders when registry changes. */
export function useRegistryCategories() {
  return useStore(registryStore, (s) => s.categories);
}
