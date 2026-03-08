/**
 * Convert a Definition (rich recipe format from @bnto/nodes) into a
 * PipelineDefinition (execution-oriented format for the WASM executor).
 *
 * Recursively walks the Definition tree, stripping editor metadata
 * (position, ports, edges) and optionally merging user config overrides
 * into leaf processing nodes.
 */

import type { Definition } from "@bnto/nodes";
import { isIoNodeType, isContainerNodeType } from "@bnto/nodes";
import type { PipelineDefinition, PipelineNode } from "../types/pipeline";

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/**
 * Convert a Definition tree into a PipelineDefinition.
 *
 * @param definition - Root definition node (recipe or editor output)
 * @param configOverrides - Optional user config to merge into leaf processing nodes
 */
export function definitionToPipeline(
  definition: Definition,
  configOverrides?: Record<string, unknown>,
): PipelineDefinition {
  const children = definition.nodes ?? [];
  return {
    nodes: children.map((child) => convertNode(child, configOverrides)),
  };
}

// ---------------------------------------------------------------------------
// Internal — recursive conversion
// ---------------------------------------------------------------------------

function convertNode(node: Definition, configOverrides?: Record<string, unknown>): PipelineNode {
  const isContainer = isContainerNodeType(node.type);
  const isIO = isIoNodeType(node.type);

  const params =
    !isIO && !isContainer && configOverrides
      ? { ...node.parameters, ...configOverrides }
      : node.parameters;

  const result: PipelineNode = { id: node.id, type: node.type, params };

  if (isContainer && node.nodes && node.nodes.length > 0) {
    result.children = node.nodes.map((child) => convertNode(child, configOverrides));
  }

  return result;
}
