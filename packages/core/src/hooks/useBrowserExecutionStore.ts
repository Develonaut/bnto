"use client";

import { useStore } from "zustand";
import { core } from "../core";
import type { BrowserExecutionState } from "../stores/browserExecutionStore";

/** React hook for selecting slices from the browser execution store. */
export function useBrowserExecutionStore<T>(
  selector: (state: BrowserExecutionState) => T,
): T {
  return useStore(core.browser.store, selector);
}
