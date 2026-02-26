// ---------------------------------------------------------------------------
// Recipe flow types — shared state for recipe (tool) page lifecycle.
//
// These types are transport-agnostic: they describe what the recipe page
// needs to track regardless of whether execution happens in the browser
// (WASM) or in the cloud (Railway).
// ---------------------------------------------------------------------------

/**
 * Unified phase for the recipe page UI.
 *
 * Maps to the phases a user experiences on a tool page:
 *   idle → uploading → running → completed | failed
 *
 * Both browser and cloud execution paths converge to this type.
 * The phase mapping functions in the web app translate each path's
 * internal status into RunPhase for the UI.
 */
export type RunPhase =
  | "idle"
  | "uploading"
  | "running"
  | "completed"
  | "failed";

/**
 * Recipe page flow state — data shape (no actions).
 *
 * Tracks the user's progress through a single recipe execution:
 * files selected → config adjusted → execution in progress → results.
 *
 * Page-scoped: a new instance is created per [bnto] page mount.
 * Not a global singleton — each tool page owns its own flow state.
 */
export interface RecipeFlow {
  /** Input files selected by the user (from drag-drop or file picker). */
  files: File[];
  /** Per-recipe config (shape varies by slug — e.g., { quality: 80 }). */
  config: Record<string, unknown>;
  /** Cloud execution ID (null when idle or on the browser path). */
  executionId: string | null;
  /** Current phase in the cloud execution lifecycle. */
  cloudPhase: RunPhase;
  /** Client-side error (validation failure, upload error, etc.). */
  clientError: string | null;
}
