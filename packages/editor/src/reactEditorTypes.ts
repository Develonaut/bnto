/**
 * React editor types — extends imperative EditorInstance with hook factories.
 *
 * Type-only file. Defines ReactEditorInstance and the React-aware client
 * interfaces that merge imperative methods with hooks.
 */

import type { Edge } from "@xyflow/react";
import type { Definition, ValidationError } from "@bnto/nodes";
import type { BrowserFileResult } from "@bnto/core";
import type {
  NodeClient,
  DefinitionClient,
  ExecutionClient,
  HistoryClient,
  PanelClient,
} from "./editorTypes";
import type {
  EditorState,
  ExecutionPhase,
  ExecutionState,
  FileProgress,
  PanelId,
  RecipeMetadata,
  RunLogEntry,
} from "./store/types";
import type { BentoNode, NodeConfigs } from "./adapters/types";
import type { NodeHookResult } from "./hooks/factories/createUseNode";

// ---------------------------------------------------------------------------
// Hook result types — what each domain hook returns
// ---------------------------------------------------------------------------

interface NodesHookResult {
  nodes: BentoNode[];
  edges: Edge[];
  configs: NodeConfigs;
  selectedNodeId: string | null;
  insertAfterNodeId: string | null;
  insertIntoContainerId: string | null;
}

interface DefinitionHookResult {
  definition: Definition | null;
  recipeMetadata: RecipeMetadata;
  isDirty: boolean;
  validationErrors: ValidationError[];
  lastSavedAt: number | null;
}

interface ExecutionHookResult {
  phase: ExecutionPhase;
  results: BrowserFileResult[];
  errors: string[];
  logs: RunLogEntry[];
  fileProgress: FileProgress | null;
  inputFiles: File[];
  canRun: boolean;
  fileAccept: string | undefined;
  executionState: ExecutionState;
  nodeProgress: Record<string, number>;
}

interface HistoryHookResult {
  canUndo: boolean;
  canRedo: boolean;
}

interface PanelsHookResult {
  isOpen: boolean;
  open: () => void;
  close: () => void;
  toggle: () => void;
}

// ---------------------------------------------------------------------------
// React-aware client types — imperative + hooks
// ---------------------------------------------------------------------------

interface ReactNodeClient extends NodeClient {
  useNodes(): NodesHookResult;
  useNode(nodeId: string | null): NodeHookResult;
}

interface ReactDefinitionClient extends DefinitionClient {
  useDefinition(): DefinitionHookResult;
}

interface ReactExecutionClient extends ExecutionClient {
  useExecution(): ExecutionHookResult;
}

interface ReactHistoryClient extends HistoryClient {
  useHistory(): HistoryHookResult;
}

interface ReactPanelClient extends PanelClient {
  usePanels(panelId: PanelId): PanelsHookResult;
}

// ---------------------------------------------------------------------------
// ReactEditorInstance — the public API with hooks on each domain
// ---------------------------------------------------------------------------

interface ReactEditorInstance {
  nodes: ReactNodeClient;
  definition: ReactDefinitionClient;
  execution: ReactExecutionClient;
  history: ReactHistoryClient;
  panels: ReactPanelClient;
  getState(): EditorState;
  subscribe(listener: (state: EditorState) => void): () => void;
  destroy(): void;
}

export type {
  NodeHookResult,
  NodesHookResult,
  DefinitionHookResult,
  ExecutionHookResult,
  HistoryHookResult,
  PanelsHookResult,
  ReactNodeClient,
  ReactDefinitionClient,
  ReactExecutionClient,
  ReactHistoryClient,
  ReactPanelClient,
  ReactEditorInstance,
};
