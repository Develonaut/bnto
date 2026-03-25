/** Construct the EditorDebugApi facade from a store and domain target. */

import type { StoreApi } from "zustand";
import type { EditorStore } from "../store/types";
import type { EditorDebugApi } from "./editorDebugApi";
import type { DebugTarget } from "./registerEditorDebug";
import {
  forceError,
  forceComplete,
  reset as resetToBlank,
  clear as clearExecution,
  step,
  run,
} from "./debugHelpers";

/** Deep clone state using structuredClone, with fallback for non-cloneable values. */
function deepCloneState<T>(state: T): T {
  try {
    return structuredClone(state);
  } catch {
    return JSON.parse(JSON.stringify(state));
  }
}

/** Strip functions from state — returns a data-only snapshot for console inspection. */
function stripFunctions(obj: Record<string, unknown>): Record<string, unknown> {
  const result: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(obj)) {
    if (typeof value !== "function") result[key] = value;
  }
  return result;
}

function buildDebugApi(storeApi: StoreApi<EditorStore>, target: DebugTarget): EditorDebugApi {
  return {
    getState: () => deepCloneState(target.getState()),
    setState: (partial) => storeApi.setState(partial),
    subscribe: (fn) => target.subscribe(fn),

    nodes: target.nodes,
    definition: target.definition,
    execution: target.execution,
    history: target.history,
    panels: target.panels,

    run: (delayMs) => run(storeApi, delayMs),
    step: () => step(storeApi),
    error: (message) => forceError(storeApi, message),
    complete: () => forceComplete(storeApi),
    snapshot: () => stripFunctions(target.getState() as unknown as Record<string, unknown>),
    reset: () => clearExecution(storeApi),
    clear: () => resetToBlank(storeApi),
  };
}

export { buildDebugApi };
