"use client";

import { useCallback, useState } from "react";
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
  FolderOpenIcon,
  DownloadIcon,
  SlidersHorizontalIcon,
  TerminalIcon,
} from "@bnto/ui";
import { useEditorUndoRedo } from "../hooks/useEditorUndoRedo";
import { useEditorStore } from "../hooks/useEditorStore";
import { useEditorStoreApi } from "../hooks/useEditorStoreApi";
import { useEditorExport } from "../hooks/useEditorExport";
import { useNodeNavigation } from "../hooks/useNodeNavigation";
import { usePanel } from "../hooks/usePanel";
import { RunButton } from "./RunButton";
import { OpenRecipeDialog } from "./OpenRecipeDialog";
import { NodePaletteDialog } from "./NodePaletteDialog";

/**
 * EditorToolbar — self-contained bottom-center toolbar.
 *
 * Includes its own overlay positioning. Reads all state from the store.
 * Panel triggers read visibility from the editor store — no prop drilling.
 */

function EditorToolbar() {
  const { isOpen: paletteOpen, toggle: togglePalette, close: closePalette } = usePanel("palette");
  const { toggle: toggleConfig } = usePanel("config");
  const { toggle: toggleRunPanel } = usePanel("run");
  const { canPrev, canNext, canDelete, handlePrev, handleNext, removeSelectedNode } =
    useNodeNavigation();
  const { undo, redo, canUndo, canRedo } = useEditorUndoRedo();
  const { download, canExport } = useEditorExport();
  const isDirty = useEditorStore((s) => s.isDirty);
  const hasRun = useEditorStore((s) => s.executionPhase !== "idle");
  const hasNodes = useEditorStore((s) => s.nodes.length > 0);
  const storeApi = useEditorStoreApi();
  const [openDialogOpen, setOpenDialogOpen] = useState(false);

  const handleReset = useCallback(() => {
    const { loadDefinition, createBlank, definition } = storeApi.getState();
    if (definition) {
      loadDefinition(definition);
    } else {
      createBlank();
    }
  }, [storeApi]);

  const canDownload = canExport && hasNodes;

  return (
    <>
    <div
      className="pointer-events-auto absolute bottom-0 left-1/2 -translate-x-1/2"
      data-testid="editor-toolbar"
    >
      <Toolbar elevation="md">
        {/* Open / Add / Navigate / Remove */}
        <ToolbarGroup>
          <Button icon={<FolderOpenIcon />} variant="ghost" elevation="sm" onClick={() => setOpenDialogOpen(true)} aria-label="Open recipe" />
          <Button icon={<PlusIcon />} variant="primary" elevation="sm" onClick={togglePalette} aria-label="Add node" />
          <Button icon={<ChevronLeftIcon />} variant="ghost" elevation="sm" onClick={handlePrev} disabled={!canPrev} aria-label="Previous node" />
          <Button icon={<ChevronRightIcon />} variant="ghost" elevation="sm" onClick={handleNext} disabled={!canNext} aria-label="Next node" />
          <Button icon={<TrashIcon />} variant="ghost" elevation="sm" onClick={removeSelectedNode} disabled={!canDelete} aria-label="Remove selected node" />
        </ToolbarGroup>

        <ToolbarDivider />

        {/* Run / Run panel */}
        <ToolbarGroup>
          <RunButton />
          <Button icon={<TerminalIcon />} variant="ghost" elevation="sm" onClick={toggleRunPanel} aria-label="Run panel" />
        </ToolbarGroup>

        <ToolbarDivider />

        {/* Undo / Redo / Reset */}
        <ToolbarGroup>
          <Button icon={<Undo2Icon />} variant="ghost" elevation="sm" onClick={undo} disabled={!canUndo} aria-label="Undo" />
          <Button icon={<Redo2Icon />} variant="ghost" elevation="sm" onClick={redo} disabled={!canRedo} aria-label="Redo" />
          <Button icon={<RotateCcwIcon />} variant="ghost" elevation="sm" onClick={handleReset} disabled={!isDirty && !hasRun} aria-label="Reset" />
        </ToolbarGroup>

        <ToolbarDivider />

        {/* Config / Download */}
        <ToolbarGroup>
          <Button icon={<SlidersHorizontalIcon />} variant="ghost" elevation="sm" onClick={toggleConfig} aria-label="Properties" />
          <Button variant="ghost" elevation="sm" onClick={() => download()} disabled={!canDownload} aria-label="Download recipe">
            <DownloadIcon className="size-4" />
            Download
          </Button>
        </ToolbarGroup>
      </Toolbar>
    </div>
    <OpenRecipeDialog open={openDialogOpen} onOpenChange={setOpenDialogOpen} />
    <NodePaletteDialog open={paletteOpen} onOpenChange={(open) => { if (!open) closePalette(); }} />
    </>
  );
}

export { EditorToolbar };
