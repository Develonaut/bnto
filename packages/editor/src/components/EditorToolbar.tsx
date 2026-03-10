"use client";

import { useCallback, useState } from "react";
import {
  Button,
  Toolbar,
  ToolbarGroup,
  ToolbarDivider,
  Menu,
  MenuTrigger,
  MenuContent,
  MenuItem,
  PlusIcon,
  RotateCcwIcon,
  Undo2Icon,
  Redo2Icon,
  FolderOpenIcon,
  SlidersHorizontalIcon,
  TerminalIcon,
} from "@bnto/ui";
import { useEditorUndoRedo } from "../hooks/useEditorUndoRedo";
import { useEditorStore } from "../hooks/useEditorStore";
import { useEditorStoreApi } from "../hooks/useEditorStoreApi";
import { useEditorExport } from "../hooks/useEditorExport";
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
        {/* File menu */}
        <ToolbarGroup>
          <Menu>
            <MenuTrigger icon={<FolderOpenIcon />} variant="ghost" elevation="sm" aria-label="File menu" />
            <MenuContent className="w-44 p-1">
              <MenuItem onClick={() => setOpenDialogOpen(true)}>Open</MenuItem>
              <MenuItem onClick={() => download()} disabled={!canDownload}>Export</MenuItem>
            </MenuContent>
          </Menu>
        </ToolbarGroup>

        <ToolbarDivider />

        {/* Add */}
        <ToolbarGroup>
          <Button icon={<PlusIcon />} variant="primary" elevation="sm" onClick={togglePalette} aria-label="Add node" />
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

        {/* Config */}
        <ToolbarGroup>
          <Button icon={<SlidersHorizontalIcon />} variant="ghost" elevation="sm" onClick={toggleConfig} aria-label="Properties" />
        </ToolbarGroup>
      </Toolbar>
    </div>
    <OpenRecipeDialog open={openDialogOpen} onOpenChange={setOpenDialogOpen} />
    <NodePaletteDialog open={paletteOpen} onOpenChange={(open) => { if (!open) closePalette(); }} />
    </>
  );
}

export { EditorToolbar };
