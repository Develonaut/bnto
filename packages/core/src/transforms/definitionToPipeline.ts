/**
 * Convert a Definition (rich recipe format from @bnto/nodes) into a
 * PipelineDefinition (execution-oriented format for the WASM executor).
 *
 * Recursively walks the Definition tree, stripping editor metadata
 * (position, ports, edges) and optionally merging per-node config
 * into leaf processing nodes.
 */

import type { Definition } from "@bnto/registry";
import { isIoNodeType, isContainerNodeType } from "@bnto/registry";
import type { PipelineDefinition, PipelineNode } from "../types/pipeline";

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/**
 * Convert a Definition tree into a PipelineDefinition.
 *
 * @param definition - Root definition node (recipe or editor output)
 * @param perNodeConfig - Optional per-node config keyed by node ID
 */
export function definitionToPipeline(
  definition: Definition,
  perNodeConfig?: Record<string, Record<string, unknown>>,
): PipelineDefinition {
  const children = definition.nodes ?? [];
  return {
    nodes: children.map((child) => convertNode(child, perNodeConfig)),
  };
}

// ---------------------------------------------------------------------------
// Internal — recursive conversion
// ---------------------------------------------------------------------------

function convertNode(
  node: Definition,
  perNodeConfig?: Record<string, Record<string, unknown>>,
): PipelineNode {
  const isContainer = isContainerNodeType(node.type);
  const isIO = isIoNodeType(node.type);

  const nodeOverrides = perNodeConfig?.[node.id];
  const params =
    !isIO && !isContainer && nodeOverrides
      ? { ...node.parameters, ...nodeOverrides }
      : node.parameters;

  const result: PipelineNode = { id: node.id, type: node.type, params };

  if (isContainer && node.nodes && node.nodes.length > 0) {
    result.children = node.nodes.map((child) => convertNode(child, perNodeConfig));
  }

  return result;
}
