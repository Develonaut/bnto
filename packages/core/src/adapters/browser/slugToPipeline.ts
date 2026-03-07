/**
 * Slug → PipelineDefinition resolution.
 *
 * Converts a predefined bnto slug (e.g., "compress-images") into
 * a 3-node PipelineDefinition (input → process → output) that the
 * pipeline executor can run directly.
 *
 * This separates slug resolution from execution — callers resolve
 * the slug once, then pass the definition to runPipeline().
 *
 * @deprecated Use Definition → flattenDefinition() → executePipeline() instead.
 * Kept for backward compat with recipe pages during transition.
 */

import { getNodeOperation } from "./slugCapability";
import type { PipelineDefinition } from "../../engine/types";

/**
 * Build a PipelineDefinition from a predefined slug + params.
 *
 * Returns null if the slug has no browser implementation.
 */
export function slugToPipeline(
  slug: string,
  params: Record<string, unknown> = {},
): PipelineDefinition | null {
  const nodeOp = getNodeOperation(slug);
  if (!nodeOp) return null;

  return {
    nodes: [
      { id: "input", type: "input", params: {} },
      {
        id: "process",
        type: nodeOp.nodeType,
        params: { ...params, operation: nodeOp.operation },
      },
      { id: "output", type: "output", params: {} },
    ],
  };
}
