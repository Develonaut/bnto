/**
 * useNodes — standalone domain hook for node state + actions.
 *
 * Delegates to editor.nodes.useNodes() for state. Actions come directly
 * from the editor instance. Prefer editor.nodes.useNodes() in new code.
 */

"use client";

import { useEditor } from "../context";

function useNodes() {
  const editor = useEditor();
  const state = editor.nodes.useNodes();

  return {
    ...state,
    addNode: editor.nodes.addNode,
    removeNode: editor.nodes.removeNode,
    selectNode: editor.nodes.selectNode,
    setSelectedNodeId: editor.nodes.setSelectedNodeId,
    setNodes: editor.nodes.setNodes,
    setConfig: editor.nodes.setConfig,
    setConfigs: editor.nodes.setConfigs,
    removeConfig: editor.nodes.removeConfig,
    expandContainer: editor.nodes.expandContainer,
    collapseContainer: editor.nodes.collapseContainer,
    toggleContainerExpanded: editor.nodes.toggleContainerExpanded,
    onNodesChange: editor.nodes.onNodesChange,
    onEdgesChange: editor.nodes.onEdgesChange,
    setInsertAfterNodeId: editor.nodes.setInsertAfterNodeId,
    setInsertIntoContainerId: editor.nodes.setInsertIntoContainerId,
  };
}

export { useNodes };
