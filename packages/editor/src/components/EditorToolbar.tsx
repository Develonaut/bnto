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
} from "@bnto/ui";
import { useEditorUndoRedo } from "../hooks/useEditorUndoRedo";
import { useEditorStore } from "../hooks/useEditorStore";
import { useEditorStoreApi } from "../hooks/useEditorStoreApi";
import { useEditorExecutionContext } from "../hooks/EditorExecutionContext";
import { useNodeNavigation } from "../hooks/useNodeNavigation";
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
  const { phase, canRun, run, reset } = useEditorExecutionContext();
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleReset = useCallback(() => {
    const { loadRecipe, createBlank, slug } = storeApi.getState();
    if (slug) {
      loadRecipe(slug);
    } else {
      createBlank();
    }
  }, [storeApi]);

  /** Open the hidden file input to select files for execution. */
  const handleRunClick = useCallback(() => {
    if (phase === "completed" || phase === "failed") {
      reset();
      return;
    }
    fileInputRef.current?.click();
  }, [phase, reset]);

  /** Files selected — run the pipeline. */
  const handleFileChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const files = e.target.files;
      if (!files || files.length === 0) return;
      run(Array.from(files));
      // Reset the input so the same file can be re-selected.
      e.target.value = "";
    },
    [run],
  );

  const runIcon =
    phase === "running" ? (
      <LoaderIcon className="size-4 motion-safe:animate-spin" />
    ) : phase === "completed" || phase === "failed" ? (
      <RotateCcwIcon className="size-4" />
    ) : (
      <PlayIcon className="size-4" />
    );

  const runLabel =
    phase === "running"
      ? "Running"
      : phase === "completed" || phase === "failed"
        ? "Reset run"
        : "Run";

  return (
    <div
      className="pointer-events-auto absolute bottom-0 left-1/2 -translate-x-1/2"
      data-testid="editor-toolbar"
    >
      {/* Hidden file input for selecting input files */}
      <input
        ref={fileInputRef}
        type="file"
        multiple
        className="hidden"
        onChange={handleFileChange}
        data-testid="run-file-input"
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

        {/* Run / Reset execution */}
        <ToolbarGroup>
          <Button
            size="icon"
            variant={phase === "failed" ? "destructive" : "primary"}
            elevation="sm"
            onClick={handleRunClick}
            disabled={!canRun && phase !== "completed" && phase !== "failed"}
            aria-label={runLabel}
            data-testid="run-button"
          >
            {runIcon}
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
