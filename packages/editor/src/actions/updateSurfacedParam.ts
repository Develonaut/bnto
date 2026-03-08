/**
 * updateSurfacedParam action — pure function that writes a surfaced param
 * change through to the leaf node in `definition`.
 *
 * When a user changes a surfaced parameter (e.g., quality slider on a
 * group node's config panel), this updates the actual leaf node's params
 * in the full definition tree. The change is write-through: it modifies
 * `definition` so that export preserves the updated value.
 *
 * Uses the same undo/dirty pattern as updateParams.
 */

import type { Definition } from "@bnto/nodes";
import type { EditorState } from "../store/types";
import { withUndo } from "../store/withUndo";

/** Recursively find and update a leaf node's params in a definition tree. */
function updateLeafInTree(
  def: Definition,
  leafNodeId: string,
  params: Record<string, unknown>,
): Definition {
  if (def.id === leafNodeId) {
    return { ...def, parameters: { ...def.parameters, ...params } };
  }

  if (!def.nodes?.length) return def;

  return {
    ...def,
    nodes: def.nodes.map((child) => updateLeafInTree(child, leafNodeId, params)),
  };
}

export function updateSurfacedParam(
  state: EditorState,
  leafNodeId: string,
  params: Record<string, unknown>,
): Partial<EditorState> | null {
  if (!state.definition) return null;

  const nextDefinition = updateLeafInTree(state.definition, leafNodeId, params);

  return withUndo(state, { definition: nextDefinition });
}
