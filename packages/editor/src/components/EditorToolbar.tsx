"use client";

import { useCallback, useRef } from "react";
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
  PlayIcon,
  LoaderIcon,
  RefreshCwIcon,
} from "@bnto/ui";
import { useEditorUndoRedo } from "../hooks/useEditorUndoRedo";
import { useEditorStore } from "../hooks/useEditorStore";
import { useEditorStoreApi } from "../hooks/useEditorStoreApi";
import { useNodeNavigation } from "../hooks/useNodeNavigation";
import { useEditorExecution } from "../hooks/useEditorExecution";
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
  const { canPrev, canNext, canDelete, handlePrev, handleNext, removeSelectedNode } =
    useNodeNavigation();
  const { undo, redo, canUndo, canRedo } = useEditorUndoRedo();
  const isDirty = useEditorStore((s) => s.isDirty);
  const storeApi = useEditorStoreApi();
  const execution = useEditorExecution();
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleReset = useCallback(() => {
    const { loadRecipe, createBlank, slug } = storeApi.getState();
    if (slug) {
      loadRecipe(slug);
    } else {
      createBlank();
    }
  }, [storeApi]);

  const handleRunClick = useCallback(() => {
    if (execution.phase === "completed" || execution.phase === "failed") {
      execution.reset();
      return;
    }
    fileInputRef.current?.click();
  }, [execution]);

  const handleFilesSelected = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const fileList = e.target.files;
      if (!fileList || fileList.length === 0) return;
      const files = Array.from(fileList);
      execution.run(files);
      // Reset the input so the same files can be re-selected
      e.target.value = "";
    },
    [execution],
  );

  const isRunning = execution.phase === "running";
  const isCompleted = execution.phase === "completed";
  const isFailed = execution.phase === "failed";
  const showReset = isCompleted || isFailed;

  return (
    <div className="pointer-events-auto absolute bottom-0 left-1/2 -translate-x-1/2">
      {/* Hidden file input for triggering file selection */}
      <input
        ref={fileInputRef}
        type="file"
        multiple
        className="hidden"
        onChange={handleFilesSelected}
        data-testid="editor-file-input"
      />

      <Toolbar elevation="md">
        <ToolbarGroup>
          <LayerPanelTrigger />
        </ToolbarGroup>

        <ToolbarDivider />

        {/* Add / Navigate / Remove */}
        <ToolbarGroup>
          <NodePaletteMenu>
            <NodePaletteMenuTrigger
              size="icon"
              variant="primary"
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

        {/* Run / Reset */}
        <ToolbarGroup>
          <Button
            size="icon"
            variant={showReset ? "ghost" : "primary"}
            elevation="sm"
            onClick={handleRunClick}
            disabled={!execution.canRun && !showReset}
            aria-label={showReset ? "Reset" : isRunning ? "Running" : "Run"}
            data-testid="editor-run-button"
            data-phase={execution.phase}
          >
            {isRunning ? (
              <LoaderIcon className="size-4 animate-spin" />
            ) : showReset ? (
              <RefreshCwIcon className="size-4" />
            ) : (
              <PlayIcon className="size-4" />
            )}
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
