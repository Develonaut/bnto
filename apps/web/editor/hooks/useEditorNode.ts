/**
 * Hook for accessing a single node's data, schema, and visible params.
 *
 * Returns the node from the definition tree, its schema, and the
 * parameters that should be visible given the current parameter values
 * (conditional visibility via visibleWhen).
 */

"use client";

import { useMemo } from "react";
import type { Definition } from "@bnto/nodes";
import type { NodeTypeName, NodeSchema, ParameterSchema } from "@bnto/nodes";
import { NODE_TYPE_INFO, getNodeSchema } from "@bnto/nodes";
import { useEditorStore } from "./useEditorStore";

// ---------------------------------------------------------------------------
// Node finder — recursive tree search
// ---------------------------------------------------------------------------

function findNodeById(definition: Definition, nodeId: string): Definition | null {
  for (const child of definition.nodes ?? []) {
    if (child.id === nodeId) return child;
    const found = findNodeById(child, nodeId);
    if (found) return found;
  }
  return null;
}

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
  /** The node definition, or null if not found. */
  node: Definition | null;
  /** Node type info (label, category, capabilities). */
  typeInfo: typeof NODE_TYPE_INFO[NodeTypeName] | null;
  /** Full schema for the node type. */
  schema: NodeSchema | null;
  /** Parameters visible given current parameter values. */
  visibleParams: ParameterSchema[];
}

/**
 * Access a node by ID with its schema and conditionally visible params.
 *
 * Subscribes to the editor store — re-renders when the definition changes.
 */
function useEditorNode(nodeId: string | null): EditorNodeResult {
  const definition = useEditorStore((s) => s.definition);

  return useMemo(() => {
    if (!nodeId) {
      return { node: null, typeInfo: null, schema: null, visibleParams: [] };
    }

    const node = findNodeById(definition, nodeId);
    if (!node) {
      return { node: null, typeInfo: null, schema: null, visibleParams: [] };
    }

    const typeInfo = NODE_TYPE_INFO[node.type as NodeTypeName] ?? null;
    const schema = getNodeSchema(node.type) ?? null;
    const visibleParams = schema
      ? resolveVisibleParams(schema, node.parameters)
      : [];

    return { node, typeInfo, schema, visibleParams };
  }, [definition, nodeId]);
}

export { useEditorNode, findNodeById };
export type { EditorNodeResult };
