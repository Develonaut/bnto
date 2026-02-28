"use client";

import { useStore } from "zustand";
import { core } from "../core";
import type { WasmExecutionState } from "../stores/wasmExecutionStore";

/** React hook for selecting slices from the WASM execution store. */
export function useWasmExecutionStore<T>(
  selector: (state: WasmExecutionState) => T,
): T {
  return useStore(core.wasm.store, selector);
}
