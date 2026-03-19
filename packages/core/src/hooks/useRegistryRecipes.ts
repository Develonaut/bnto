"use client";

import { useStore } from "zustand";
import { registryStore } from "../stores/registryStore";

/** Returns all predefined recipes — reactive. Re-renders when registry changes. */
export function useRegistryRecipes() {
  return useStore(registryStore, (s) => s.recipes);
}
