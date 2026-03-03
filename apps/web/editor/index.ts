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
} from "./store/createEditorStore";

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

// Adapters
export { definitionToBento, SLOTS, CELL, GAP } from "./adapters/definitionToBento";
export type {
  BentoNode,
  BentoLayout,
  CompartmentData,
  CompartmentVariant,
} from "./adapters/definitionToBento";
export { bentoToDefinition } from "./adapters/bentoToDefinition";
