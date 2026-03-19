"use client";

import { useStore } from "zustand";
import { registryStore } from "../stores/registryStore";

/** Returns processor definitions — reactive. Re-renders when registry changes. */
export function useRegistryProcessors() {
  return useStore(registryStore, (s) => s.processors);
}
