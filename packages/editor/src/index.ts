/**
 * Editor module — public API for the recipe editor.
 *
 * Components (flat exports for RSC compatibility):
 *
 *   <EditorRoot definition={recipe.definition}>
 *     <EditorCanvas />
 *   </EditorRoot>
 *
 * Factory (headless, no React):
 *
 *   const editor = createEditor(definition);
 *   editor.nodes.addNode("image");
 *   editor.definition.exportAsRecipe();
 */

// --- Compound components ---

export { EditorRoot, EditorCanvas, EditorToolbar, EditorRightToolbar } from "./Editor";

// --- Editor factory + context ---

export { createEditor } from "./createEditor";
export { createReactEditor } from "./createReactEditor";
export { EditorProvider } from "./EditorProvider";
export { useEditor } from "./context";
export type {
  EditorInstance,
  NodeService,
  NodeClient,
  DefinitionService,
  DefinitionClient,
  ExecutionService,
  ExecutionClient,
  HistoryService,
  HistoryClient,
  PanelService,
  PanelClient,
  ExportResult,
} from "./editorTypes";
export type {
  ReactEditorInstance,
  ReactNodeClient,
  ReactDefinitionClient,
  ReactExecutionClient,
  ReactHistoryClient,
  ReactPanelClient,
  NodeHookResult,
  NodesHookResult,
  DefinitionHookResult,
  ExecutionHookResult,
  HistoryHookResult,
  PanelsHookResult,
} from "./reactEditorTypes";

// --- Domain hooks ---

export { useNodes } from "./hooks/useNodes";
export { useDefinition } from "./hooks/useDefinition";
export { useExecution } from "./hooks/useExecution";
export { useHistory } from "./hooks/useHistory";
export { usePanels } from "./hooks/usePanels";
export { useUnsavedWarning } from "./hooks/useUnsavedWarning";
export { useEditorShortcuts } from "./hooks/useEditorShortcuts";
export { useAutosave } from "./hooks/useAutosave";
export { useDraftHydration } from "./hooks/useDraftHydration";
export type { UseAutosaveOptions } from "./hooks/useAutosave";

// --- Draft persistence ---

export type { Draft } from "./draft/draftTypes";
export { deserializeDraft } from "./draft/deserializeDraft";
export { serializeDraft } from "./draft/serializeDraft";
export { DRAFT_KEY, saveDraft, loadDraft, clearDraft } from "./draft/draftStorage";
export { formatLastSaved } from "./draft/formatLastSaved";

// --- Internal hooks (rendering pipeline) ---

export { useAutoSelect } from "./hooks/useAutoSelect";
export { useEditorNode } from "./hooks/useEditorNode";
export type { EditorNodeResult } from "./hooks/useEditorNode";
export { useEditorSelection } from "./hooks/useEditorSelection";
export { useExecutionNodes } from "./hooks/useExecutionNodes";

// --- Store (internal — prefer createEditor for new code) ---

export { createEditorStore } from "./store/createEditorStore";
export type {
  EditorStore,
  EditorState,
  EditorActions,
  EditorSnapshot,
  NodeExecutionStatus,
  ExecutionState,
  ExecutionPhase,
  FileProgress,
  RecipeMetadata,
  PanelId,
  PanelState,
  RunLogEntry,
} from "./store/types";

// --- Canvas ---

export { Canvas } from "./components/EditorCanvas/Canvas";
export { CompartmentNode } from "./components/nodes/CompartmentNode";
export { IoNode } from "./components/nodes/IoNode";
export { PlaceholderNode } from "./components/nodes/PlaceholderNode";

// --- Schema-driven forms ---

export { SchemaForm } from "./components/SchemaForm";
export type { SchemaFormProps } from "./components/SchemaForm";
export { SchemaField } from "./components/SchemaField";
export type { SchemaFieldProps } from "./components/SchemaField";

// --- Dialogs ---

export { OpenRecipeDialog } from "./components/OpenRecipeDialog";
export { NodePaletteDialog } from "./components/NodePaletteDialog";
export { HelpDialog } from "./components/HelpDialog";

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

export { definitionToGraph } from "./adapters/definitionToGraph";
export { rfNodesToDefinition } from "./adapters/rfNodesToDefinition";
export { createCompartmentNode } from "./adapters/createCompartmentNode";
export {
  SLOTS,
  CELL,
  GAP_X,
  GAP_Y,
  STRIDE,
  ROW_OFFSET,
  MAX_CONTAINER_DEPTH,
} from "./adapters/bentoSlots";
export { layoutNodes } from "./adapters/layoutNodes";
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
