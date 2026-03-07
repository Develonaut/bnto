/**
 * Slug → PipelineDefinition resolution.
 *
 * Converts a predefined bnto slug (e.g., "compress-images") into
 * a 3-node PipelineDefinition (input → process → output) that the
 * pipeline executor can run directly.
 *
 * This separates slug resolution from execution — callers resolve
 * the slug once, then pass the definition to runPipeline().
 */

import { getBrowserNodeType } from "./slugCapability";
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
  const nodeType = getBrowserNodeType(slug);
  if (!nodeType) return null;

  return {
    nodes: [
      { id: "input", type: "input", params: {} },
      { id: "process", type: nodeType, params },
      { id: "output", type: "output", params: {} },
    ],
  };
}
