/**
 * Hook for accessing a single node's data, schema, and visible params.
 *
 * Reads from ReactFlow's internal nodeLookup (O(1) access) — RF is the
 * single source of truth for node domain state during editing.
 *
 * Returns the node data, its type info, schema, and the parameters
 * that should be visible given the current parameter values
 * (conditional visibility via visibleWhen).
 *
 * Must be inside a ReactFlowProvider.
 */

"use client";

import { useMemo } from "react";
import { useStore } from "@xyflow/react";
import type { NodeTypeName, NodeSchema, ParameterSchema } from "@bnto/nodes";
import { NODE_TYPE_INFO, getNodeSchema } from "@bnto/nodes";
import type { CompartmentNodeData } from "../adapters/types";

// ---------------------------------------------------------------------------
// Visible params resolver
// ---------------------------------------------------------------------------

function resolveVisibleParams(
  schema: NodeSchema,
  parameters: Record<string, unknown>,
): ParameterSchema[] {
  return schema.parameters.filter((param) => {
    if (!param.visibleWhen) return true;

    const conditions = Array.isArray(param.visibleWhen)
      ? param.visibleWhen
      : [param.visibleWhen];

    // OR logic — any matching condition makes the param visible
    return conditions.some(
      (cond) => String(parameters[cond.param] ?? "") === cond.equals,
    );
  });
}

// ---------------------------------------------------------------------------
// Hook
// ---------------------------------------------------------------------------

interface EditorNodeResult {
  /** The node's domain data from RF, or null if not found. */
  node: CompartmentNodeData | null;
  /** Node type info (label, category, capabilities). */
  typeInfo: (typeof NODE_TYPE_INFO)[NodeTypeName] | null;
  /** Full schema for the node type. */
  schema: NodeSchema | null;
  /** Parameters visible given current parameter values. */
  visibleParams: ParameterSchema[];
}

/**
 * Access a node by ID with its schema and conditionally visible params.
 *
 * Reads from RF's internal nodeLookup Map — O(1) access, no recursive
 * tree search. Subscribes to RF store — re-renders when node data changes.
 */
function useEditorNode(nodeId: string | null): EditorNodeResult {
  const nodeData = useStore(
    (s: { nodeLookup: Map<string, { data: Record<string, unknown> }> }) => {
      if (!nodeId) return null;
      const rfNode = s.nodeLookup.get(nodeId);
      return rfNode ? (rfNode.data as CompartmentNodeData) : null;
    },
  );

  return useMemo(() => {
    if (!nodeData) {
      return { node: null, typeInfo: null, schema: null, visibleParams: [] };
    }

    const typeInfo = NODE_TYPE_INFO[nodeData.nodeType as NodeTypeName] ?? null;
    const schema = getNodeSchema(nodeData.nodeType) ?? null;
    const visibleParams = schema
      ? resolveVisibleParams(schema, nodeData.parameters)
      : [];

    return { node: nodeData, typeInfo, schema, visibleParams };
  }, [nodeData]);
}

export { useEditorNode };
export type { EditorNodeResult };
