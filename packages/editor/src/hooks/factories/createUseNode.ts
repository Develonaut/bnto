/**
 * createUseNode — hook factory for single-node state.
 *
 * Returns a useNode(nodeId) hook closure that captures storeApi at
 * creation time. Subscribes to the specific node's visual data and
 * domain config, plus derives schema/type metadata.
 */

"use client";

import { useMemo } from "react";
import type { StoreApi } from "zustand";
import { useStore } from "zustand";
import type { NodeParamFields, NodeTypeName, NodeSchema } from "@bnto/core";
import { NODE_TYPE_INFO, getNodeSchema, getNodeParamFields, getVisibleParams } from "@bnto/core";
import type { EditorStore } from "../../store/types";
import type { CompartmentNodeData, NodeConfig } from "../../adapters/types";

interface NodeHookResult {
  /** The node's visual data, or null if not found. */
  node: CompartmentNodeData | null;
  /** The node's domain config (nodeType, name, parameters), or null. */
  config: NodeConfig | null;
  /** Node type info (label, category, capabilities). */
  typeInfo: (typeof NODE_TYPE_INFO)[NodeTypeName] | null;
  /** Full schema definition for the node type. */
  schemaDef: NodeSchema | null;
  /** UI field config map for the node type. */
  fieldConfigs: NodeParamFields | undefined;
  /** Parameter names visible given current parameter values. */
  visibleParams: string[];
}

const EMPTY_RESULT: NodeHookResult = {
  node: null,
  config: null,
  typeInfo: null,
  schemaDef: null,
  fieldConfigs: undefined,
  visibleParams: [],
};

function createUseNode(storeApi: StoreApi<EditorStore>) {
  return function useNode(nodeId: string | null): NodeHookResult {
    const nodeData = useStore(storeApi, (s) => {
      if (!nodeId) return null;
      return s.nodes.find((n) => n.id === nodeId)?.data ?? null;
    });

    const config = useStore(storeApi, (s) => {
      if (!nodeId) return null;
      return s.configs[nodeId] ?? null;
    });

    return useMemo(() => {
      if (!nodeData || !config) return EMPTY_RESULT;

      const typeInfo = NODE_TYPE_INFO[config.nodeType as NodeTypeName] ?? null;
      const schemaDef = getNodeSchema(config.nodeType) ?? null;
      const fieldConfigs = getNodeParamFields(config.nodeType);
      const visibleParams = schemaDef ? getVisibleParams(config.nodeType, config.parameters) : [];

      return { node: nodeData, config, typeInfo, schemaDef, fieldConfigs, visibleParams };
    }, [nodeData, config]);
  };
}

export { createUseNode };
export type { NodeHookResult };
