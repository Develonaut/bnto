/**
 * generatePipelineEdges — derives visual pipeline edges from node order.
 *
 * Pure function: reads positioned nodes + execution state, returns Edge[].
 * Edges are NOT stored in the graph — they're a rendering concern only.
 *
 * Key rule: no edges inside containers. Containers are opaque boxes;
 * their internal structure is the bento grid. Edges only connect
 * top-level pipeline steps left-to-right.
 */

import type { Edge } from "@xyflow/react";
import type { BentoNode } from "./types";
import type { ExecutionState } from "../store/types";

/** Node types that participate in the top-level pipeline road. */
const PIPELINE_TYPES = new Set(["compartment", "io", "containerGroup"]);

/** Derive progress (0–1) for an edge based on its source node's execution state. */
function deriveProgress(
  sourceId: string,
  executionState: ExecutionState,
  nodeProgress: Record<string, number>,
): number {
  const status = executionState[sourceId];
  if (status === "completed" || status === "failed") return 1;
  if (status === "active") return (nodeProgress[sourceId] ?? 0) / 100;
  return 0;
}

function generatePipelineEdges(
  nodes: BentoNode[],
  executionState: ExecutionState = {},
  nodeProgress: Record<string, number> = {},
): Edge[] {
  // Filter to top-level pipeline nodes only
  const pipelineNodes = nodes
    .filter((n) => PIPELINE_TYPES.has(n.type!) && !n.data.parentContainerId)
    .sort((a, b) => (a.position?.x ?? 0) - (b.position?.x ?? 0));

  if (pipelineNodes.length < 2) return [];

  const edges: Edge[] = [];
  for (let i = 0; i < pipelineNodes.length - 1; i++) {
    const source = pipelineNodes[i]!;
    const target = pipelineNodes[i + 1]!;
    edges.push({
      id: `pipe-${source.id}-${target.id}`,
      source: source.id,
      target: target.id,
      type: "pipeline",
      data: {
        progress: deriveProgress(source.id, executionState, nodeProgress),
      },
    });
  }

  return edges;
}

export { generatePipelineEdges };
