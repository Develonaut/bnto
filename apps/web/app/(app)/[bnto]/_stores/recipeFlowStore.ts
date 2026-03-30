import { createEnhancedStore } from "@bnto/core";
import type { RunPhase } from "../_components/RunButton";

// ---------------------------------------------------------------------------
// Recipe flow types — page-scoped state for recipe (tool) page lifecycle.
// ---------------------------------------------------------------------------

/**
 * Recipe page flow state — data shape (no actions).
 *
 * Tracks the user's progress through a single recipe execution:
 * files selected -> config adjusted -> execution in progress -> results.
 */
interface RecipeFlow {
  /** Input files selected by the user (from drag-drop or file picker). */
  files: File[];
  /** Per-node config keyed by node ID (e.g., { "compress-image": { quality: 80 } }). */
  config: Record<string, Record<string, unknown>>;
  /** Cloud execution ID (null when idle or on the browser path). */
  executionId: string | null;
  /** Current phase in the cloud execution lifecycle. */
  cloudPhase: "idle" | "uploading" | "running" | "completed" | "failed";
  /** Client-side error (validation failure, upload error, etc.). */
  clientError: string | null;

  // Derived fields — synced by the provider via useLayoutEffect.
  /** 3-step phase indicator (1 = files, 2 = configure, 3 = results). */
  activeStep: 1 | 2 | 3;
  /** Unified execution phase for RunButton display. */
  resolvedPhase: RunPhase;
  /** True when uploading or running (disables back, file changes). */
  isProcessing: boolean;
}

// ---------------------------------------------------------------------------
// State shape — data + actions
// ---------------------------------------------------------------------------

interface RecipeFlowState extends RecipeFlow {
  /** Replace the file list. Clears any prior client error. */
  setFiles: (files: File[]) => void;
  /** Update a single param on a single node's config slice. */
  setNodeParam: (nodeId: string, paramName: string, value: unknown) => void;
  /** Cloud path: files are being uploaded to R2. */
  startUpload: () => void;
  /** Cloud path: upload done, execution record created. */
  startExecution: (id: string) => void;
  /** Cloud or client error — sets phase to failed with message. */
  failCloud: (error: string) => void;
  /** Sync derived phase fields from provider. */
  setDerivedStep: (active: 1 | 2 | 3, resolved: RunPhase) => void;
  /** Reset to initial state with new default config. */
  reset: (defaultConfig?: Record<string, Record<string, unknown>>) => void;
}

// ---------------------------------------------------------------------------
// Initial state
// ---------------------------------------------------------------------------

function createInitialState(defaultConfig: Record<string, Record<string, unknown>>): RecipeFlow {
  return {
    files: [],
    config: structuredClone(defaultConfig),
    executionId: null,
    cloudPhase: "idle",
    clientError: null,
    activeStep: 1,
    resolvedPhase: "idle",
    isProcessing: false,
  };
}

// ---------------------------------------------------------------------------
// Factory — creates a fresh store instance per provider mount
// ---------------------------------------------------------------------------

/**
 * Create a page-scoped recipe flow store.
 *
 * Each [bnto] page mount creates its own store instance via this factory.
 * The store tracks user progress through the recipe execution flow:
 * files -> config -> upload -> run -> results.
 *
 * @param defaultConfig - Per-node default config (e.g., { "compress-image": { quality: 80 } }).
 */
function createRecipeFlowStore(defaultConfig: Record<string, Record<string, unknown>> = {}) {
  const initial = createInitialState(defaultConfig);

  return createEnhancedStore<RecipeFlowState>()((set) => ({
    ...initial,

    setFiles: (files) => set({ files, clientError: null }),

    setNodeParam: (nodeId, paramName, value) =>
      set((s) => ({
        config: { ...s.config, [nodeId]: { ...s.config[nodeId], [paramName]: value } },
      })),

    startUpload: () => set({ cloudPhase: "uploading", clientError: null }),

    startExecution: (id) => set({ executionId: id, cloudPhase: "running" }),

    failCloud: (error) => set({ cloudPhase: "failed", clientError: error }),

    setDerivedStep: (active, resolved) =>
      set({
        activeStep: active,
        resolvedPhase: resolved,
        isProcessing: resolved === "uploading" || resolved === "running",
      }),

    reset: (cfg) => set(createInitialState(cfg ?? defaultConfig)),
  }));
}

export { createRecipeFlowStore };
export type { RecipeFlowState };
