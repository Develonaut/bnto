/**
 * Node service — wraps pure node actions + storeApi.setState().
 *
 * Covers: nodes, edges, configs, selection, containers, insertion context,
 * and RF controlled-mode change handlers.
 *
 * Services call pure actions and apply results to the store. No
 * business logic lives here — it's all in the action files.
 */

import type { StoreApi } from "zustand";
import type { NodeTypeName } from "@bnto/core";
import type { EditorStore } from "../store/types";
import type { NodeService } from "../editorTypes";
import type { BentoNode, NodeConfig, NodeConfigs } from "../adapters/types";
import type { NodeChange, EdgeChange } from "@xyflow/react";
import { addNode } from "../actions/addNode";
import { removeNode } from "../actions/removeNode";

function createNodeService(storeApi: StoreApi<EditorStore>): NodeService {
  return {
    addNode(
      type: NodeTypeName,
      afterNodeId?: string | null,
      intoContainerId?: string | null,
      defaultParams?: Record<string, unknown>,
    ) {
      const result = addNode(
        storeApi.getState(),
        type,
        afterNodeId,
        intoContainerId,
        defaultParams,
      );
      if (!result) return null;
      storeApi.setState(result.nextState);
      return result.nodeId;
    },

    removeNode(id: string) {
      const nextState = removeNode(storeApi.getState(), id);
      if (!nextState) return false;
      storeApi.setState(nextState);
      return true;
    },

    selectNode(id: string | null) {
      storeApi.getState().selectNode(id);
    },

    setSelectedNodeId(id: string | null) {
      storeApi.getState().setSelectedNodeId(id);
    },

    setNodes(nodes: BentoNode[]) {
      storeApi.getState().setNodes(nodes);
    },

    setConfig(nodeId: string, config: NodeConfig) {
      storeApi.getState().setConfig(nodeId, config);
    },

    setConfigs(configs: NodeConfigs) {
      storeApi.getState().setConfigs(configs);
    },

    removeConfig(nodeId: string) {
      storeApi.getState().removeConfig(nodeId);
    },

    onNodesChange(changes: NodeChange<BentoNode>[]) {
      storeApi.getState().onNodesChange(changes);
    },

    onEdgesChange(changes: EdgeChange[]) {
      storeApi.getState().onEdgesChange(changes);
    },

    expandContainer(nodeId: string) {
      const state = storeApi.getState();
      state.expandContainer(nodeId);
      return storeApi.getState().expandedContainerIds.has(nodeId);
    },

    collapseContainer(nodeId: string) {
      const state = storeApi.getState();
      state.collapseContainer(nodeId);
      return !storeApi.getState().expandedContainerIds.has(nodeId);
    },

    toggleContainerExpanded(nodeId: string) {
      storeApi.getState().toggleContainerExpanded(nodeId);
    },

    setInsertAfterNodeId(id: string | null) {
      storeApi.getState().setInsertAfterNodeId(id);
    },

    setInsertIntoContainerId(id: string | null) {
      storeApi.getState().setInsertIntoContainerId(id);
    },
  };
}

export { createNodeService };
