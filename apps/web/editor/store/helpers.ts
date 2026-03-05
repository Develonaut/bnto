/**
 * Pure helper functions for the editor store.
 *
 * Shared by both the store (undo/redo, entry points) and action hooks
 * (addNode, removeNode, updateConfigParams). Keeps mutation logic DRY
 * without coupling store and hooks.
 */

import { validateDefinition } from "@bnto/nodes";
import { rfNodesToDefinition } from "../adapters/rfNodesToDefinition";
import type { BentoNode, NodeConfigs } from "../adapters/types";
import type { EditorSnapshot, RecipeMetadata } from "./types";

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const MAX_UNDO_HISTORY = 50;

// ---------------------------------------------------------------------------
// Snapshot helpers
// ---------------------------------------------------------------------------

/** Deep-clone nodes + configs into an immutable snapshot for undo/redo. */
function captureSnapshot(nodes: BentoNode[], configs: NodeConfigs): EditorSnapshot {
  const clonedConfigs: NodeConfigs = {};
  for (const [id, config] of Object.entries(configs)) {
    clonedConfigs[id] = {
      ...config,
      parameters: { ...config.parameters },
    };
  }
  return { nodes: [...nodes], configs: clonedConfigs };
}

/** Push a snapshot onto an undo stack, capping at MAX_UNDO_HISTORY. */
function pushToStack(stack: EditorSnapshot[], snapshot: EditorSnapshot): EditorSnapshot[] {
  const next = [...stack, snapshot];
  return next.length > MAX_UNDO_HISTORY ? next.slice(-MAX_UNDO_HISTORY) : next;
}

// ---------------------------------------------------------------------------
// Validation helper
// ---------------------------------------------------------------------------

/** Reconstruct a Definition from current state and run validation. */
function revalidateState(nodes: BentoNode[], configs: NodeConfigs, metadata: RecipeMetadata) {
  const definition = rfNodesToDefinition(nodes, metadata, configs);
  return validateDefinition(definition);
}

export { MAX_UNDO_HISTORY, captureSnapshot, pushToStack, revalidateState };
