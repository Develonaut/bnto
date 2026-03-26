/**
 * addTopLevel — adds a node at the top-level pipeline.
 *
 * Mode 3 of the addNode action. Inserts before the output node by
 * default, shifts subsequent nodes right by STRIDE, updates the
 * definition tree, and captures undo state.
 */

import type { NodeTypeName, Definition } from "@bnto/core";
import type { EditorState } from "../store/types";
import type { BentoNode } from "../adapters/types";
import type { AddNodeResult } from "./types";
import type { CompartmentNodeResult } from "../adapters/createCompartmentNode";
import { STRIDE } from "../adapters/bentoSlots";
import { withUndo } from "../store/withUndo";
import { buildChildDefinition } from "./buildChildDefinition";

/** Shift top-level nodes after the insertion point right by STRIDE. */
function shiftNodesRight(nodes: BentoNode[], fromIndex: number): BentoNode[] {
  return nodes
    .slice(fromIndex)
    .map((n) =>
      !n.data.parentContainerId
        ? { ...n, position: { x: n.position.x + STRIDE, y: n.position.y } }
        : n,
    );
}

function addTopLevel(
  state: EditorState,
  result: CompartmentNodeResult,
  newNode: BentoNode,
  deselected: BentoNode[],
  afterNodeId?: string | null,
  isContainer?: boolean,
): AddNodeResult {
  const idx = findInsertIndex(deselected, state, afterNodeId);
  const nextNodes = [...deselected.slice(0, idx), newNode, ...shiftNodesRight(deselected, idx)];
  const nextDef = insertIntoDefinition(state.definition, result, afterNodeId);
  const nextExpanded = isContainer
    ? new Set([...state.expandedContainerIds, result.node.id])
    : undefined;

  return {
    nextState: withUndo(state, {
      nodes: nextNodes,
      configs: { ...state.configs, [result.node.id]: result.config },
      ...(nextDef !== state.definition ? { definition: nextDef } : {}),
      ...(nextExpanded ? { expandedContainerIds: nextExpanded } : {}),
    }),
    nodeId: result.node.id,
  };
}

/** Find the graph insertion index — after afterNodeId, or before the output node. */
function findInsertIndex(
  nodes: BentoNode[],
  state: EditorState,
  afterNodeId?: string | null,
): number {
  if (afterNodeId) return nodes.findIndex((n) => n.id === afterNodeId) + 1;
  const outputIdx = nodes.findIndex((n) => state.configs[n.id]?.nodeType === "output");
  return outputIdx >= 0 ? outputIdx : nodes.length;
}

/** Insert a Definition node into the definition tree at the correct position. */
function insertIntoDefinition(
  definition: Definition | null,
  result: CompartmentNodeResult,
  afterNodeId?: string | null,
): Definition | null {
  if (!definition) return definition;

  const childDef = buildChildDefinition(
    result.node.id,
    result.config.nodeType as NodeTypeName,
    result.config,
  );

  const rootNodes = definition.nodes ?? [];
  let defInsertIndex: number;
  if (afterNodeId) {
    const idx = rootNodes.findIndex((n) => n.id === afterNodeId);
    defInsertIndex = idx >= 0 ? idx + 1 : rootNodes.length;
  } else {
    const outputIdx = rootNodes.findIndex((n) => n.type === "output");
    defInsertIndex = outputIdx >= 0 ? outputIdx : rootNodes.length;
  }

  return {
    ...definition,
    nodes: [...rootNodes.slice(0, defInsertIndex), childDef, ...rootNodes.slice(defInsertIndex)],
  };
}

export { addTopLevel };
