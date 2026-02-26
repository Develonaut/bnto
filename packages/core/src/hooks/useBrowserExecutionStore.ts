"use client";

import { useStore } from "zustand";
import {
  browserExecutionStore,
  type BrowserExecutionState,
} from "../stores/browserExecutionStore";

/** React hook for selecting slices from the browser execution store. */
export function useBrowserExecutionStore<T>(
  selector: (state: BrowserExecutionState) => T,
): T {
  return useStore(browserExecutionStore, selector);
}
