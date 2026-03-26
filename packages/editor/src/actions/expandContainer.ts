/**
 * expandContainer action — materializes a container's children into the graph.
 *
 * Reads children from `state.definition` for the given container nodeId,
 * creates BentoNodes + NodeConfigs for each child (matching definitionToGraph
 * patterns), and adds them to `state.nodes` + `state.configs`.
 *
 * Returns null if the container has no definition, is already expanded,
 * or exceeds max nesting depth.
 */

import type { Definition } from "@bnto/core";
import type { EditorState } from "../store/types";
import type { BentoNode, NodeConfigs } from "../adapters/types";
import { MAX_CONTAINER_DEPTH } from "../adapters/bentoSlots";
import { findDefinitionById } from "../adapters/findDefinitionById";
import { withUndo } from "../store/withUndo";
import { childDefToNode } from "./childDefToNode";

/** Build NodeConfigs for an array of child definitions. */
function buildChildConfigs(children: Definition[]): NodeConfigs {
  const configs: NodeConfigs = {};
  for (const child of children) {
    const displayName = child.metadata?.customData?.displayName;
    configs[child.id] = {
      nodeType: child.type,
      name: child.name,
      ...(displayName ? { displayName } : {}),
      parameters: child.parameters,
    };
  }
  return configs;
}

export function expandContainer(state: EditorState, nodeId: string): Partial<EditorState> | null {
  if (!state.definition) return null;
  if (state.expandedContainerIds.has(nodeId)) return null;

  const parentNode = state.nodes.find((n) => n.id === nodeId);
  if (!parentNode) return null;

  const parentDepth = parentNode.data.depth ?? 0;
  if (parentDepth >= MAX_CONTAINER_DEPTH) return null;

  const containerDef = findDefinitionById(state.definition, nodeId);
  if (!containerDef) return null;

  const children = containerDef.nodes ?? [];
  const childNodes: BentoNode[] = children.map((c) => childDefToNode(c, nodeId, parentDepth + 1));
  const childConfigs = buildChildConfigs(children);

  const nextNodes = state.nodes.map((n) =>
    n.id === nodeId ? { ...n, data: { ...n.data, isExpanded: true } } : n,
  );

  const nextExpandedIds = new Set(state.expandedContainerIds);
  nextExpandedIds.add(nodeId);

  return withUndo(state, {
    nodes: [...nextNodes, ...childNodes],
    configs: { ...state.configs, ...childConfigs },
    expandedContainerIds: nextExpandedIds,
  });
}
