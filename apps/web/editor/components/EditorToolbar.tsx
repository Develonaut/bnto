"use client";

import { useCallback } from "react";
import { Button } from "@/components/ui/Button";
import { Toolbar } from "@/components/ui/Toolbar";
import {
  PlusIcon,
  TrashIcon,
  RotateCcwIcon,
  Undo2Icon,
  Redo2Icon,
} from "@/components/ui/icons";
import { useEditorUndoRedo } from "@/editor/hooks/useEditorUndoRedo";
import { useEditorActions } from "@/editor/hooks/useEditorActions";
import { useEditorStore } from "@/editor/hooks/useEditorStore";
import { useEditorStoreApi } from "@/editor/hooks/useEditorStoreApi";
import { LayerPanel } from "./LayerPanel";
import { ConfigPanel } from "./ConfigPanel";
import { NodePaletteMenu } from "./NodePaletteMenu";

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
        <Toolbar.Group>
          <LayerPanel.Trigger />
        </Toolbar.Group>

        <Toolbar.Divider />

        {/* Add / Remove */}
        <Toolbar.Group>
          <NodePaletteMenu>
            <NodePaletteMenu.Trigger
              size="icon"
              variant="ghost"
              elevation="sm"
              aria-label="Add node"
            >
              <PlusIcon className="size-4" />
            </NodePaletteMenu.Trigger>
            <NodePaletteMenu.Content side="top" offset="lg" />
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
        </Toolbar.Group>

        <Toolbar.Divider />

        {/* Undo / Redo / Reset */}
        <Toolbar.Group>
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
        </Toolbar.Group>

        <Toolbar.Divider />

        <Toolbar.Group>
          <ConfigPanel.Trigger />
        </Toolbar.Group>
      </Toolbar>
    </div>
  );
}

export { EditorToolbar };
