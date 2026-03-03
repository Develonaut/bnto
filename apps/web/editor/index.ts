/**
 * Editor module — public API for the recipe editor.
 *
 * This barrel export is the single import point for editor functionality.
 * Consumers import from "@/editor" (alias) or "./editor".
 */

// Store
export { createEditorStore } from "./store/createEditorStore";
export type {
  EditorStore,
  EditorState,
  EditorActions,
  NodeExecutionStatus,
  ExecutionState,
  UndoSnapshot,
  PositionGetter,
} from "./store/types";

// Context + Provider
export { EditorContext } from "./context";
export { EditorProvider } from "./EditorProvider";

// Hooks
export { useEditorStore } from "./hooks/useEditorStore";
export { useEditorActions } from "./hooks/useEditorActions";
export { useEditorNode } from "./hooks/useEditorNode";
export type { EditorNodeResult } from "./hooks/useEditorNode";
export { useNodePalette } from "./hooks/useNodePalette";
export type { PaletteGroup, NodePaletteResult } from "./hooks/useNodePalette";
export { useEditorExport } from "./hooks/useEditorExport";
export type { ExportResult, EditorExportResult } from "./hooks/useEditorExport";
export { useEditorSelection } from "./hooks/useEditorSelection";
export { useDefinitionSync } from "./hooks/useDefinitionSync";
export { useEditorUndoRedo } from "./hooks/useEditorUndoRedo";

// Adapters
export { definitionToBento } from "./adapters/definitionToBento";
export { syncPositionsToDefinition } from "./adapters/syncPositionsToDefinition";
export { definitionNodeToRfNode } from "./adapters/definitionNodeToRfNode";
export { SLOTS, CELL, GAP, STRIDE } from "./adapters/bentoSlots";
export type {
  BentoNode,
  BentoLayout,
  CompartmentNodeData,
  CompartmentVariant,
} from "./adapters/types";

// Utils
export { findNodeById } from "./utils/findNodeById";
