/**
 * Editor module — public API for the recipe editor.
 *
 * Components (flat exports for RSC compatibility):
 *
 *   <EditorRoot slug="compress-images">
 *     <EditorCanvas />
 *   </EditorRoot>
 */

// --- Compound components ---

export {
  EditorRoot,
  EditorCanvas,
  EditorLayerPanel,
  EditorConfigPanel,
  EditorToolbar,
} from "./Editor";

// --- Store ---

export { createEditorStore } from "./store/createEditorStore";
export type {
  EditorStore,
  EditorState,
  EditorActions,
  EditorSnapshot,
  NodeExecutionStatus,
  ExecutionState,
  RecipeMetadata,
} from "./store/types";

// --- Context + Provider ---

export { EditorContext } from "./context";
export { EditorProvider } from "./EditorProvider";

// --- Hooks ---

export { useEditorStore } from "./hooks/useEditorStore";
export { useEditorStoreApi } from "./hooks/useEditorStoreApi";
export { useEditorActions } from "./hooks/useEditorActions";
export { useAddNode } from "./hooks/useAddNode";
export { useRemoveNode } from "./hooks/useRemoveNode";
export { useUpdateParams } from "./hooks/useUpdateParams";
export { useAutoSelect } from "./hooks/useAutoSelect";
export { useEditorNode } from "./hooks/useEditorNode";
export type { EditorNodeResult } from "./hooks/useEditorNode";
export { useNodePalette } from "./hooks/useNodePalette";
export type { PaletteGroup, NodePaletteResult } from "./hooks/useNodePalette";
export { useEditorExport } from "./hooks/useEditorExport";
export type { ExportResult, EditorExportResult } from "./hooks/useEditorExport";
export { useEditorSelection } from "./hooks/useEditorSelection";
export { useEditorUndoRedo } from "./hooks/useEditorUndoRedo";
export { useEditorPanels } from "./hooks/useEditorPanels";

// --- Adapters ---

export { definitionToBento } from "./adapters/definitionToBento";
export { rfNodesToDefinition } from "./adapters/rfNodesToDefinition";
export { createCompartmentNode } from "./adapters/createCompartmentNode";
export { SLOTS, CELL, GAP, STRIDE } from "./adapters/bentoSlots";
export type {
  BentoNode,
  BentoLayout,
  CompartmentNodeData,
  CompartmentVariant,
  NodeConfig,
  NodeConfigs,
} from "./adapters/types";
