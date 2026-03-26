/** Node setters — simple store delegations for nodes, configs, selection, changes. */

import type { StoreApi } from "zustand";
import type { EditorStore } from "../store/types";
import type { NodeService } from "../editorTypes";
import type { BentoNode, NodeConfig, NodeConfigs } from "../adapters/types";
import type { NodeChange, EdgeChange } from "@xyflow/react";

type SetterMethods = Pick<
  NodeService,
  | "selectNode"
  | "setSelectedNodeId"
  | "setNodes"
  | "setConfig"
  | "setConfigs"
  | "removeConfig"
  | "onNodesChange"
  | "onEdgesChange"
  | "setInsertAfterNodeId"
  | "setInsertIntoContainerId"
>;

function buildNodeSetters(storeApi: StoreApi<EditorStore>): SetterMethods {
  return {
    selectNode: (id: string | null) => storeApi.getState().selectNode(id),
    setSelectedNodeId: (id: string | null) => storeApi.getState().setSelectedNodeId(id),
    setNodes: (nodes: BentoNode[]) => storeApi.getState().setNodes(nodes),
    setConfig: (nodeId: string, config: NodeConfig) =>
      storeApi.getState().setConfig(nodeId, config),
    setConfigs: (configs: NodeConfigs) => storeApi.getState().setConfigs(configs),
    removeConfig: (nodeId: string) => storeApi.getState().removeConfig(nodeId),
    onNodesChange: (changes: NodeChange<BentoNode>[]) => storeApi.getState().onNodesChange(changes),
    onEdgesChange: (changes: EdgeChange[]) => storeApi.getState().onEdgesChange(changes),
    setInsertAfterNodeId: (id: string | null) => storeApi.getState().setInsertAfterNodeId(id),
    setInsertIntoContainerId: (id: string | null) =>
      storeApi.getState().setInsertIntoContainerId(id),
  };
}

export { buildNodeSetters };
