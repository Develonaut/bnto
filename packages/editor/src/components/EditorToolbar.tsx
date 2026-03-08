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
  ChevronLeftIcon,
  ChevronRightIcon,
} from "@bnto/ui";
import { useEditorUndoRedo } from "../hooks/useEditorUndoRedo";
import { useEditorStore } from "../hooks/useEditorStore";
import { useEditorStoreApi } from "../hooks/useEditorStoreApi";
import { useNodeNavigation } from "../hooks/useNodeNavigation";
import { usePanel } from "../hooks/usePanel";
import { RunButton } from "./RunButton";

/**
 * EditorToolbar — self-contained bottom-center toolbar.
 *
 * Includes its own overlay positioning. Reads all state from the store.
 * Panel triggers read visibility from the editor store — no prop drilling.
 */

function EditorToolbar() {
  const { toggle: togglePalette } = usePanel("palette");
  const { canPrev, canNext, canDelete, handlePrev, handleNext, removeSelectedNode } =
    useNodeNavigation();
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
    <div
      className="pointer-events-auto absolute bottom-0 left-1/2 -translate-x-1/2"
      data-testid="editor-toolbar"
    >
      <Toolbar elevation="md">
        {/* Add / Navigate / Remove */}
        <ToolbarGroup>
          <Button
            size="icon"
            variant="primary"
            elevation="sm"
            onClick={togglePalette}
            aria-label="Add node"
          >
            <PlusIcon className="size-4" />
          </Button>
          <Button
            size="icon"
            variant="ghost"
            elevation="sm"
            onClick={handlePrev}
            disabled={!canPrev}
            aria-label="Previous node"
          >
            <ChevronLeftIcon className="size-4" />
          </Button>
          <Button
            size="icon"
            variant="ghost"
            elevation="sm"
            onClick={handleNext}
            disabled={!canNext}
            aria-label="Next node"
          >
            <ChevronRightIcon className="size-4" />
          </Button>
          <Button
            size="icon"
            variant="ghost"
            elevation="sm"
            onClick={removeSelectedNode}
            disabled={!canDelete}
            aria-label="Remove selected node"
          >
            <TrashIcon className="size-4" />
          </Button>
        </ToolbarGroup>

        <ToolbarDivider />

        {/* Run / Reset execution */}
        <ToolbarGroup>
          <RunButton />
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
      </Toolbar>
    </div>
  );
}

export { EditorToolbar };
