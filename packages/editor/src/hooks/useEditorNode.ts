/**
 * Hook for accessing a single node's visual data, config, schema,
 * and visible params.
 *
 * Visual data comes from the store's nodes array.
 * Domain data (nodeType, parameters) comes from the store's configs map.
 *
 * Must be inside an EditorProvider.
 */

"use client";

import { useMemo } from "react";
import type { NodeTypeName, NodeSchemaDefinition } from "@bnto/nodes";
import { NODE_TYPE_INFO, getNodeSchema, getVisibleParams } from "@bnto/nodes";
import { useEditorStore } from "./useEditorStore";
import type { CompartmentNodeData, NodeConfig } from "../adapters/types";

// ---------------------------------------------------------------------------
// Hook
// ---------------------------------------------------------------------------

interface EditorNodeResult {
  /** The node's visual data, or null if not found. */
  node: CompartmentNodeData | null;
  /** The node's domain config (nodeType, name, parameters), or null. */
  config: NodeConfig | null;
  /** Node type info (label, category, capabilities). */
  typeInfo: (typeof NODE_TYPE_INFO)[NodeTypeName] | null;
  /** Full schema definition for the node type. */
  schemaDef: NodeSchemaDefinition | null;
  /** Parameter names visible given current parameter values. */
  visibleParams: string[];
}

/**
 * Access a node by ID with its config, schema, and conditionally visible params.
 *
 * Visual data from store nodes, domain data from store configs.
 */
function useEditorNode(nodeId: string | null): EditorNodeResult {
  const nodeData = useEditorStore((s) => {
    if (!nodeId) return null;
    return s.nodes.find((n) => n.id === nodeId)?.data ?? null;
  });

  const config = useEditorStore((s) => {
    if (!nodeId) return null;
    return s.configs[nodeId] ?? null;
  });

  return useMemo(() => {
    if (!nodeData || !config) {
      return { node: null, config: null, typeInfo: null, schemaDef: null, visibleParams: [] };
    }

    const typeInfo = NODE_TYPE_INFO[config.nodeType as NodeTypeName] ?? null;
    const schemaDef = getNodeSchema(config.nodeType) ?? null;
    const visibleParams = schemaDef ? getVisibleParams(config.nodeType, config.parameters) : [];

    return { node: nodeData, config, typeInfo, schemaDef, visibleParams };
  }, [nodeData, config]);
}

export { useEditorNode };
export type { EditorNodeResult };
