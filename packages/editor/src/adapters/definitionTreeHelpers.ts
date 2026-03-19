/**
 * definitionTreeHelpers — immutable operations on the nested Definition tree.
 *
 * Pure functions for adding, removing, and updating child nodes in the
 * recursive Definition structure. Used by expand/collapse and child-editing
 * actions to keep `state.definition` in sync with graph changes.
 */

import type { Definition } from "@bnto/core";

/** Immutably add a child Definition to a container node in the tree. */
function addChildToContainer(
  root: Definition,
  containerId: string,
  childDef: Definition,
  afterChildId?: string,
): Definition {
  if (root.id === containerId) {
    const existing = root.nodes ?? [];
    if (afterChildId) {
      const idx = existing.findIndex((n) => n.id === afterChildId);
      const insertAt = idx >= 0 ? idx + 1 : existing.length;
      return {
        ...root,
        nodes: [...existing.slice(0, insertAt), childDef, ...existing.slice(insertAt)],
      };
    }
    // No afterChildId — prepend as first child (button is on the container)
    return { ...root, nodes: [childDef, ...existing] };
  }

  if (!root.nodes?.length) return root;

  return {
    ...root,
    nodes: root.nodes.map((child) =>
      addChildToContainer(child, containerId, childDef, afterChildId),
    ),
  };
}

/** Immutably remove a child Definition from a container node in the tree. */
function removeChildFromContainer(
  root: Definition,
  containerId: string,
  childId: string,
): Definition {
  if (root.id === containerId) {
    return {
      ...root,
      nodes: (root.nodes ?? []).filter((n) => n.id !== childId),
    };
  }

  if (!root.nodes?.length) return root;

  return {
    ...root,
    nodes: root.nodes.map((child) => removeChildFromContainer(child, containerId, childId)),
  };
}

/** Immutably update a node's parameters anywhere in the tree. */
function updateNodeInTree(
  root: Definition,
  nodeId: string,
  params: Record<string, unknown>,
): Definition {
  if (root.id === nodeId) {
    return { ...root, parameters: { ...root.parameters, ...params } };
  }

  if (!root.nodes?.length) return root;

  return {
    ...root,
    nodes: root.nodes.map((child) => updateNodeInTree(child, nodeId, params)),
  };
}

export { addChildToContainer, removeChildFromContainer, updateNodeInTree };
