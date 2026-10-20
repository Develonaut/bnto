/**
 * Convert a Definition (rich recipe format from @bnto/nodes) into a
 * PipelineDefinition (execution-oriented format for the WASM executor).
 *
 * Recursively walks the Definition tree, stripping editor metadata
 * (position, ports, port handles) and optionally merging per-node config
 * into leaf processing nodes. Edges and the recipe's settings are kept, so
 * the browser runs a recipe as the CLI does: an "auto" recipe iterates per
 * file, and a level's files follow its edges.
 */

import type { Definition, Edge } from "@bnto/registry";
import { isIoNodeType, isContainerNodeType } from "@bnto/registry";
import type { PipelineDefinition, PipelineEdge, PipelineNode } from "../types/pipeline";

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
  const pipeline: PipelineDefinition = {
    nodes: children.map((child) => convertNode(child, perNodeConfig)),
  };
  const edges = convertEdges(definition.edges);
  if (edges) pipeline.edges = edges;
  if (definition.settings) pipeline.settings = { ...definition.settings };
  return pipeline;
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
    const edges = convertEdges(node.edges);
    if (edges) result.edges = edges;
  }

  return result;
}

/** A level's edges without port handles (an editor concern), or none. */
function convertEdges(edges: Edge[] | undefined): PipelineEdge[] | undefined {
  if (!edges || edges.length === 0) return undefined;
  return edges.map(({ id, source, target }) => ({ id, source, target }));
}
