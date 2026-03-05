"use client";

import { useCallback } from "react";
import {
  Button,
  Toolbar,
  ToolbarGroup,
  ToolbarDivider,
  PlusIcon,
  TrashIcon,
  RotateCcwIcon,
  Undo2Icon,
  Redo2Icon,
} from "@bnto/ui";
import { useEditorUndoRedo } from "../hooks/useEditorUndoRedo";
import { useEditorActions } from "../hooks/useEditorActions";
import { useEditorStore } from "../hooks/useEditorStore";
import { useEditorStoreApi } from "../hooks/useEditorStoreApi";
import { LayerPanelTrigger } from "./LayerPanel";
import { ConfigPanelTrigger } from "./ConfigPanel";
import { NodePaletteMenu, NodePaletteMenuTrigger, NodePaletteMenuContent } from "./NodePaletteMenu";

/**
 * EditorToolbar — self-contained bottom-center toolbar.
 *
 * Includes its own overlay positioning. Reads all state from the store.
 * Panel triggers read visibility from the editor store — no prop drilling.
 */

function EditorToolbar() {
  const { removeSelectedNode, selectedNodeId: selId } = useEditorActions();
  const { undo, redo, canUndo, canRedo } = useEditorUndoRedo();
  const isDirty = useEditorStore((s) => s.isDirty);
  const storeApi = useEditorStoreApi();

  const handleReset = useCallback(() => {
    const { loadRecipe, createBlank, slug } = storeApi.getState();
    if (slug) {
      loadRecipe(slug);
    } else {
      createBlank();
    }
  }, [storeApi]);

  return (
    <div className="pointer-events-auto absolute bottom-0 left-1/2 -translate-x-1/2">
      <Toolbar elevation="md">
        <ToolbarGroup>
          <LayerPanelTrigger />
        </ToolbarGroup>

        <ToolbarDivider />

        {/* Add / Remove */}
        <ToolbarGroup>
          <NodePaletteMenu>
            <NodePaletteMenuTrigger
              size="icon"
              variant="ghost"
              elevation="sm"
              aria-label="Add node"
            >
              <PlusIcon className="size-4" />
            </NodePaletteMenuTrigger>
            <NodePaletteMenuContent side="top" offset="lg" />
          </NodePaletteMenu>
          <Button
            size="icon"
            variant="ghost"
            elevation="sm"
            onClick={removeSelectedNode}
            disabled={!selId}
            aria-label="Remove selected node"
          >
            <TrashIcon className="size-4" />
          </Button>
        </ToolbarGroup>

        <ToolbarDivider />

        {/* Undo / Redo / Reset */}
        <ToolbarGroup>
          <Button
            size="icon"
            variant="ghost"
            elevation="sm"
            onClick={undo}
            disabled={!canUndo}
            aria-label="Undo"
          >
            <Undo2Icon className="size-4" />
          </Button>
          <Button
            size="icon"
            variant="ghost"
            elevation="sm"
            onClick={redo}
            disabled={!canRedo}
            aria-label="Redo"
          >
            <Redo2Icon className="size-4" />
          </Button>
          <Button
            size="icon"
            variant="ghost"
            elevation="sm"
            onClick={handleReset}
            disabled={!isDirty}
            aria-label="Reset"
          >
            <RotateCcwIcon className="size-4" />
          </Button>
        </ToolbarGroup>

        <ToolbarDivider />

        <ToolbarGroup>
          <ConfigPanelTrigger />
        </ToolbarGroup>
      </Toolbar>
    </div>
  );
}

export { EditorToolbar };
