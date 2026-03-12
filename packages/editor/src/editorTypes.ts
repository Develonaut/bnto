/**
 * Editor instance types — defines the public API shape for createEditor().
 *
 * Type-only file. No runtime code. These interfaces describe the
 * domain-namespaced API that components and tests consume.
 */

import type { StoreApi } from "zustand";
import type { NodeTypeName, Definition, RecipeMetadata as NodeRecipeMetadata, Recipe, ValidationError } from "@bnto/nodes";
import type { BrowserFileResult } from "@bnto/core";
import type { EditorStore, EditorState, ExecutionState, NodeExecutionStatus, PanelId, RecipeMetadata } from "./store/types";
import type { BentoNode, NodeConfig, NodeConfigs } from "./adapters/types";
import type { NodeChange, EdgeChange } from "@xyflow/react";

// ---------------------------------------------------------------------------
// Service types — thin wrappers around pure actions + storeApi.setState()
// ---------------------------------------------------------------------------

interface NodeService {
  addNode(type: NodeTypeName, afterNodeId?: string | null, intoContainerId?: string | null, defaultParams?: Record<string, unknown>): string | null;
  removeNode(id: string): boolean;
  selectNode(id: string | null): void;
  setSelectedNodeId(id: string | null): void;
  setNodes(nodes: BentoNode[]): void;
  setConfig(nodeId: string, config: NodeConfig): void;
  setConfigs(configs: NodeConfigs): void;
  removeConfig(nodeId: string): void;
  onNodesChange(changes: NodeChange<BentoNode>[]): void;
  onEdgesChange(changes: EdgeChange[]): void;
  expandContainer(nodeId: string): boolean;
  collapseContainer(nodeId: string): boolean;
  toggleContainerExpanded(nodeId: string): void;
  setInsertAfterNodeId(id: string | null): void;
  setInsertIntoContainerId(id: string | null): void;
}

interface DefinitionService {
  loadDefinition(def: Definition): void;
  createBlank(): void;
  updateParams(nodeId: string, params: Record<string, unknown>): boolean;
  updateSurfacedParam(leafNodeId: string, params: Record<string, unknown>): boolean;
  setRecipeMetadata(metadata: RecipeMetadata): void;
  markDirty(): void;
  resetDirty(): void;
  revalidate(): void;
  exportAsDefinition(): Definition;
}

interface ExecutionService {
  runExecution(files: File[]): Promise<void>;
  resetRun(): void;
  downloadResult(file: BrowserFileResult): void;
  downloadAllResults(): Promise<void>;
  setExecutionState(state: ExecutionState): void;
  resetNodeStatuses(): void;
  /** Force per-node execution status (dev tooling). */
  setNodeStatus(nodeId: string, status: NodeExecutionStatus): void;
  /** Force per-node progress percentage (dev tooling). */
  setNodeProgress(nodeId: string, percent: number): void;
  /** Force arbitrary execution state fields (dev tooling). */
  forceExecutionState(partial: Partial<EditorState>): void;
}

interface HistoryService {
  pushUndo(): void;
  undo(): void;
  redo(): void;
  resetHistory(): void;
}

interface PanelService {
  openPanel(id: PanelId): void;
  closePanel(id: PanelId): void;
  togglePanel(id: PanelId): void;
}

// ---------------------------------------------------------------------------
// Client types — compose services for cross-domain operations
// ---------------------------------------------------------------------------

/** Export result from definitionClient.exportAsRecipe(). */
interface ExportResult {
  recipe: Recipe | null;
  errors: ValidationError[];
}

type NodeClient = NodeService;

interface DefinitionClient extends DefinitionService {
  exportAsRecipe(metadata?: NodeRecipeMetadata): ExportResult;
}

type ExecutionClient = ExecutionService;

type HistoryClient = HistoryService;

type PanelClient = PanelService;

// ---------------------------------------------------------------------------
// EditorInstance — the public API returned by createEditor()
// ---------------------------------------------------------------------------

interface EditorInstance {
  nodes: NodeClient;
  definition: DefinitionClient;
  execution: ExecutionClient;
  history: HistoryClient;
  panels: PanelClient;
  getState(): EditorState;
  subscribe(listener: (state: EditorState) => void): () => void;
  destroy(): void;
  /** @internal — exposed for React bindings (useStore) */
  _storeApi: StoreApi<EditorStore>;
}

export type {
  NodeService,
  DefinitionService,
  ExecutionService,
  HistoryService,
  PanelService,
  NodeClient,
  DefinitionClient,
  ExecutionClient,
  HistoryClient,
  PanelClient,
  ExportResult,
  EditorInstance,
};
