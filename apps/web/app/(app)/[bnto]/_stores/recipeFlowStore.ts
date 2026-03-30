import { createEnhancedStore } from "@bnto/core";

// ---------------------------------------------------------------------------
// Recipe flow types — page-scoped state for recipe (tool) page lifecycle.
// ---------------------------------------------------------------------------

/**
 * Recipe page flow state — data shape (no actions).
 *
 * Tracks the user's progress through a single recipe execution:
 * files selected -> config adjusted -> execution in progress -> results.
 *
 * Page-scoped: a new instance is created per [bnto] page mount.
 * Not a global singleton — each tool page owns its own flow state.
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
  /** Reset to initial state (default config for the slug). */
  reset: () => void;
}

// ---------------------------------------------------------------------------
// Initial state factory — captures default config at creation time
// ---------------------------------------------------------------------------

function createInitialState(defaultConfig: Record<string, Record<string, unknown>>): RecipeFlow {
  return {
    files: [],
    config: structuredClone(defaultConfig),
    executionId: null,
    cloudPhase: "idle",
    clientError: null,
  };
}

// ---------------------------------------------------------------------------
// Store factory
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
export function createRecipeFlowStore(defaultConfig: Record<string, Record<string, unknown>> = {}) {
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

    reset: () => set(createInitialState(defaultConfig)),
  }));
}

export type { RecipeFlowState };
