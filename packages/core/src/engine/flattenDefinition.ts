/**
 * flattenDefinition — converts a Definition tree into a PipelineDefinition.
 *
 * A Definition (from @bnto/nodes) is the authoring format — recursive,
 * with ports, edges, positions, and metadata. A PipelineDefinition is
 * the execution format — just what the executor needs.
 *
 * This function bridges the two: it walks the Definition tree and builds
 * PipelineNode objects, preserving nesting for container nodes (loop, group).
 */

import type { Definition } from "@bnto/nodes";
import type { PipelineDefinition, PipelineNode } from "./types";

/** Container node types that can have children. */
const CONTAINER_TYPES = new Set(["loop", "group", "parallel"]);

/**
 * Convert a Definition tree into a PipelineDefinition.
 *
 * The root Definition is a container (usually `group`) — its children
 * become the top-level nodes in the pipeline. Container children
 * are converted recursively, preserving the tree structure.
 */
export function flattenDefinition(definition: Definition): PipelineDefinition {
  const nodes = (definition.nodes ?? []).map(definitionToPipelineNode);
  return { nodes };
}

/** Convert a single Definition node to a PipelineNode. */
function definitionToPipelineNode(def: Definition): PipelineNode {
  const node: PipelineNode = {
    id: def.id,
    type: def.type,
    params: def.parameters ?? {},
  };

  if (CONTAINER_TYPES.has(def.type) && def.nodes?.length) {
    node.children = def.nodes.map(definitionToPipelineNode);
  }

  return node;
}
