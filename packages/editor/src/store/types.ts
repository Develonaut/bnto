/**
 * Editor store types.
 *
 * The store holds two complementary views of the recipe:
 * - **Graph** (nodes, edges, configs) — flat RF working state for visual editing
 * - **Definition** — the nested tree the engine takes, with container children
 *
 * There is no overlap. Top-level params live in the graph (configs).
 * Nested container children live in the definition. On export,
 * rfNodesToDefinition() merges both into a complete Definition.
 */

import type { Edge, NodeChange, EdgeChange } from "@xyflow/react";
import type { Definition, ValidationError, PipelineEvent, BrowserFileResult } from "@bnto/core";
import type { BentoNode, NodeConfig, NodeConfigs } from "../adapters/types";

// ---------------------------------------------------------------------------
// Execution state — per-node status tracking
// ---------------------------------------------------------------------------

type NodeExecutionStatus = "idle" | "pending" | "active" | "completed" | "failed";

type ExecutionState = Record<string, NodeExecutionStatus>;

/** Editor-local event for validation failures (not a pipeline event). */
interface ValidationFailedEvent {
  type: "ValidationFailed";
  nodeId: string;
  field: string;
  error: string;
}

/** All events that can appear in the run log. */
type RunLogEvent = PipelineEvent | ValidationFailedEvent;

/** A single log entry — a pipeline or editor event with a capture timestamp. */
interface RunLogEntry {
  timestamp: number;
  event: RunLogEvent;
}

// ---------------------------------------------------------------------------
// Execution lifecycle — overall run state (phase, results, logs)
// ---------------------------------------------------------------------------

type ExecutionPhase = "idle" | "running" | "completed" | "failed";

interface FileProgress {
  fileIndex: number;
  totalFiles: number;
  overallPercent: number;
  message: string;
}

// ---------------------------------------------------------------------------
// Recipe metadata — root definition fields without child nodes
// ---------------------------------------------------------------------------

interface RecipeMetadata {
  id: string;
  name: string;
  slug: string;
  /** Convex document _id. Null if not yet saved to cloud. */
  cloudId: string | null;
}

// ---------------------------------------------------------------------------
// Undo/redo snapshot — captures both nodes and configs atomically
// ---------------------------------------------------------------------------

interface EditorSnapshot {
  nodes: BentoNode[];
  configs: NodeConfigs;
  definition: Definition | null;
  expandedContainerIds: Set<string>;
}

// ---------------------------------------------------------------------------
// Panel visibility — centralized panel state
// ---------------------------------------------------------------------------

/** All panel identifiers. Add new panels here — one place to extend. */
type PanelId = "config" | "palette" | "run" | "help" | "recipe";

/** Map of panel IDs to their open/closed state. */
type PanelState = Record<PanelId, boolean>;

/** Side slot — panels on the same side are mutually exclusive. */
type PanelSide = "left" | "right";

/** Which side each panel lives on. Panels sharing a side are exclusive. */
const PANEL_SIDES: Partial<Record<PanelId, PanelSide>> = {
  config: "right",
  recipe: "left",
};

// ---------------------------------------------------------------------------
// State shape
// ---------------------------------------------------------------------------

interface EditorState {
  // --- Graph: flat RF working state (store owns, RF renders as controlled props) ---
  nodes: BentoNode[];
  edges: Edge[];
  configs: NodeConfigs;

  // --- Definition: nested tree the engine takes (container children live here) ---
  definition: Definition | null;

  // --- Metadata ---
  recipeMetadata: RecipeMetadata;
  isDirty: boolean;
  validationErrors: ValidationError[];
  executionState: ExecutionState;
  nodeProgress: Record<string, number>;
  undoStack: EditorSnapshot[];
  redoStack: EditorSnapshot[];

  // --- Selection ---
  selectedNodeId: string | null;

  // --- Panel visibility ---
  panels: PanelState;

  // --- Container expansion ---
  expandedContainerIds: Set<string>;

  // --- Insertion context ---
  /** When set, the next addNode inserts after this node ID instead of at the end. */
  insertAfterNodeId: string | null;
  /** When set, the next addNode inserts as a child inside this container. */
  insertIntoContainerId: string | null;

  // --- Execution lifecycle ---
  executionPhase: ExecutionPhase;
  executionResults: BrowserFileResult[];
  executionErrors: string[];
  executionLogs: RunLogEntry[];
  executionFileProgress: FileProgress | null;
  executionInputFiles: File[];
}

// ---------------------------------------------------------------------------
// Actions
// ---------------------------------------------------------------------------

interface EditorActions {
  // --- Entry points ---
  loadDefinition: (def: Definition) => void;
  createBlank: () => void;

  // --- RF controlled-mode change handlers ---
  onNodesChange: (changes: NodeChange<BentoNode>[]) => void;
  onEdgesChange: (changes: EdgeChange[]) => void;

  // --- Graph setters ---
  setNodes: (nodes: BentoNode[]) => void;
  selectNode: (id: string | null) => void;

  // --- Config setters ---
  setConfigs: (configs: NodeConfigs) => void;
  setConfig: (nodeId: string, config: NodeConfig) => void;
  removeConfig: (nodeId: string) => void;

  // --- History ---
  pushUndo: () => void;
  undo: () => void;
  redo: () => void;

  // --- Selection ---
  setSelectedNodeId: (id: string | null) => void;

  // --- Panel visibility ---
  openPanel: (id: PanelId) => void;
  closePanel: (id: PanelId) => void;
  togglePanel: (id: PanelId) => void;

  // --- Execution lifecycle ---
  runExecution: (files: File[]) => Promise<void>;
  resetRun: () => void;
  setInputFiles: (files: File[]) => void;
  downloadResult: (file: BrowserFileResult) => void;
  downloadAllResults: () => Promise<void>;

  // --- Container expansion ---
  toggleContainerExpanded: (nodeId: string) => void;
  expandContainer: (nodeId: string) => void;
  collapseContainer: (nodeId: string) => void;

  // --- Insertion context ---
  setInsertAfterNodeId: (id: string | null) => void;
  setInsertIntoContainerId: (id: string | null) => void;

  // --- Utility ---
  revalidate: () => void;
  setExecutionState: (state: ExecutionState) => void;
  /** Reset per-node execution statuses only (not the full run lifecycle). */
  resetNodeStatuses: () => void;
  setRecipeMetadata: (metadata: RecipeMetadata) => void;
  resetHistory: () => void;
}

// ---------------------------------------------------------------------------
// Full store type
// ---------------------------------------------------------------------------

type EditorStore = EditorState & EditorActions;

export type {
  EditorStore,
  EditorState,
  EditorActions,
  EditorSnapshot,
  NodeExecutionStatus,
  ExecutionState,
  ExecutionPhase,
  FileProgress,
  RunLogEntry,
  RunLogEvent,
  ValidationFailedEvent,
  RecipeMetadata,
  PanelId,
  PanelState,
  PanelSide,
};
export { PANEL_SIDES };
