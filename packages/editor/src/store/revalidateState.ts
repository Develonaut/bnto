/**
 * Reconstruct a Definition from current editor state and run validation.
 *
 * Used after any state change that could affect validity (add/remove nodes,
 * update params, undo/redo).
 */

import { validateDefinition } from "@bnto/nodes";
import { rfNodesToDefinition } from "../adapters/rfNodesToDefinition";
import type { BentoNode, NodeConfigs } from "../adapters/types";
import type { RecipeMetadata } from "./types";

function revalidateState(nodes: BentoNode[], configs: NodeConfigs, metadata: RecipeMetadata) {
  const definition = rfNodesToDefinition(nodes, metadata, configs);
  return validateDefinition(definition);
}

export { revalidateState };
