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

// --- Provider ---

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

// --- Canvas ---

export { CompartmentNode } from "./components/canvas/CompartmentNode";
export type { CompartmentStatus } from "./components/canvas/CompartmentNode";
export { BentoCanvas } from "./components/canvas/BentoCanvas";

// --- Schema-driven forms ---

export { SchemaForm } from "./components/SchemaForm";
export type { SchemaFormProps } from "./components/SchemaForm";
export { SchemaField } from "./components/SchemaField";
export type { SchemaFieldProps } from "./components/SchemaField";

// --- Renderers ---

export { InputRenderer } from "./components/InputRenderer";
export { OutputRenderer } from "./components/OutputRenderer";

// --- Archive: Conveyor ---

export {
  ConveyorCanvas,
  StationNode,
  ConveyorEdge,
  BeltPiece,
  PieceShape,
  VARIANT_PIECE_MAP,
  SALMON_CLIP,
} from "./components/archive/conveyor";
export type {
  StationData,
  StationNodeType,
  ConveyorEdgeData,
  ConveyorEdgeType,
  PieceType,
} from "./components/archive/conveyor";

// --- Adapters ---

export { definitionToBento } from "./adapters/definitionToBento";
export { rfNodesToDefinition } from "./adapters/rfNodesToDefinition";
export { createCompartmentNode } from "./adapters/createCompartmentNode";
export { SLOTS, CELL, GAP, STRIDE } from "./adapters/bentoSlots";
export { ICON_COMPONENTS } from "./adapters/nodeIcons";
export { CATEGORY_VARIANT } from "./adapters/categoryVariant";
export type {
  BentoNode,
  BentoLayout,
  CompartmentNodeData,
  CompartmentVariant,
  NodeConfig,
  NodeConfigs,
} from "./adapters/types";
