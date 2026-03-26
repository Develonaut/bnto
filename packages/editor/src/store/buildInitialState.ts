/** Build the initial state shape from resolved initial values. */

import type { EditorState } from "./types";

interface ResolvedInitial {
  nodes: EditorState["nodes"];
  configs: EditorState["configs"];
  definition: EditorState["definition"];
  metadata: EditorState["recipeMetadata"];
  selectedNodeId: string | null;
}

type InitialState = Pick<
  EditorState,
  | "nodes"
  | "edges"
  | "configs"
  | "definition"
  | "recipeMetadata"
  | "isDirty"
  | "validationErrors"
  | "undoStack"
  | "redoStack"
  | "selectedNodeId"
  | "panels"
  | "insertAfterNodeId"
  | "insertIntoContainerId"
  | "expandedContainerIds"
>;

function buildInitialState(initial: ResolvedInitial): InitialState {
  return {
    nodes: initial.selectedNodeId
      ? initial.nodes.map((n) => (n.id === initial.selectedNodeId ? { ...n, selected: true } : n))
      : initial.nodes,
    edges: [],
    configs: initial.configs,
    definition: initial.definition,
    recipeMetadata: initial.metadata,
    isDirty: false,
    validationErrors: [],
    undoStack: [],
    redoStack: [],
    selectedNodeId: initial.selectedNodeId,
    panels: {
      config: initial.selectedNodeId !== null,
      palette: false,
      run: false,
      help: false,
      settings: false,
    },
    insertAfterNodeId: null,
    insertIntoContainerId: null,
    expandedContainerIds: new Set(),
  };
}

export { buildInitialState };
