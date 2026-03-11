/**
 * addTopLevel — adds a node at the top-level pipeline.
 *
 * Mode 3 of the addNode action. Inserts before the output node by
 * default, shifts subsequent nodes right by STRIDE, and captures
 * undo state.
 */

import type { EditorState } from "../store/types";
import type { BentoNode } from "../adapters/types";
import type { CompartmentNodeResult } from "../adapters/createCompartmentNode";
import { STRIDE } from "../adapters/bentoSlots";
import { withUndo } from "../store/withUndo";

interface AddNodeResult {
  nextState: Partial<EditorState>;
  nodeId: string;
}

function addTopLevel(
  state: EditorState,
  result: CompartmentNodeResult,
  newNode: BentoNode,
  deselected: BentoNode[],
  afterNodeId?: string | null,
  isContainer?: boolean,
): AddNodeResult {
  const globalInsertIndex = afterNodeId
    ? deselected.findIndex((n) => n.id === afterNodeId) + 1
    : (() => {
        const outputIdx = deselected.findIndex(
          (n) => state.configs[n.id]?.nodeType === "output",
        );
        return outputIdx >= 0 ? outputIdx : deselected.length;
      })();

  const shifted = deselected.slice(globalInsertIndex).map((n) =>
    !n.data.parentContainerId
      ? { ...n, position: { x: n.position.x + STRIDE, y: n.position.y } }
      : n,
  );

  const nextNodes = [...deselected.slice(0, globalInsertIndex), newNode, ...shifted];
  const nextConfigs = { ...state.configs, [result.node.id]: result.config };

  const nextExpandedIds = isContainer
    ? new Set([...state.expandedContainerIds, result.node.id])
    : undefined;

  return {
    nextState: withUndo(state, {
      nodes: nextNodes,
      configs: nextConfigs,
      ...(nextExpandedIds ? { expandedContainerIds: nextExpandedIds } : {}),
    }),
    nodeId: result.node.id,
  };
}

export { addTopLevel };
