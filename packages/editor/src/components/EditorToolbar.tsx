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
  RotateCcwIcon,
  Undo2Icon,
  Redo2Icon,
  FolderOpenIcon,
  SlidersHorizontalIcon,
  TerminalIcon,
  PlusIcon,
  SaveIcon,
  DownloadIcon,
  FileUpIcon,
  MenuSeparator,
} from "@bnto/ui";
import { core } from "@bnto/core";
import { useEditor } from "../context";
import { downloadDefinition } from "../actions/downloadDefinition";
import { RunButton } from "./RunButton";
import { OpenRecipeDialog } from "./OpenRecipeDialog";
import { SaveRecipeDialog } from "./SaveRecipeDialog";
import { NodePaletteDialog } from "./NodePaletteDialog";

/**
 * EditorToolbar — self-contained bottom-center toolbar.
 *
 * Includes its own overlay positioning. Reads all state from the store.
 * Panel triggers read visibility from the editor store — no prop drilling.
 */

function EditorToolbar() {
  const editor = useEditor();
  const { isOpen: paletteOpen, close: closePalette } = editor.panels.usePanels("palette");
  const { toggle: toggleConfig } = editor.panels.usePanels("config");
  const { toggle: toggleRunPanel } = editor.panels.usePanels("run");
  const { canUndo, canRedo } = editor.history.useHistory();
  const { isDirty, validationErrors } = editor.definition.useDefinition();
  const { phase } = editor.execution.useExecution();
  const { nodes } = editor.nodes.useNodes();
  const { isAuthenticated } = core.auth.useAuth();
  const saveMutation = core.recipes.useSaveRecipe();

  const hasRun = phase !== "idle";
  const hasNodes = nodes.length > 0;
  const canExport = validationErrors.length === 0;
  const canSave = canExport && hasNodes && isAuthenticated;

  const [openDialogOpen, setOpenDialogOpen] = useState(false);
  const [saveDialogOpen, setSaveDialogOpen] = useState(false);

  const handleReset = useCallback(() => {
    const { definition } = editor.getState();
    if (definition) {
      editor.definition.loadDefinition(definition);
    } else {
      editor.definition.createBlank();
    }
  }, [editor]);

  const handleNew = useCallback(() => {
    editor.definition.createBlank();
  }, [editor]);

  const download = useCallback(() => {
    downloadDefinition(editor.definition);
  }, [editor]);

  const handleSave = useCallback(
    async (args: { name: string; definition: unknown }) => {
      await saveMutation.mutateAsync(args);
    },
    [saveMutation],
  );

  const canDownload = canExport && hasNodes;

  return (
    <>
      <div
        className="pointer-events-auto absolute bottom-0 left-1/2 -translate-x-1/2"
        data-testid="editor-toolbar"
      >
        <Toolbar elevation="md" aria-label="Editor toolbar">
          {/* File menu */}
          <ToolbarGroup>
            <Menu>
              <MenuTrigger
                icon={<FolderOpenIcon />}
                variant="ghost"
                elevation="sm"
                aria-label="File menu"
              />
              <MenuContent side="top" className="w-44 p-1">
                <MenuItem onClick={handleNew}>
                  <PlusIcon /> New
                </MenuItem>
                <MenuItem onClick={() => setOpenDialogOpen(true)}>
                  <FileUpIcon /> Open
                </MenuItem>
                <MenuSeparator />
                <MenuItem onClick={() => setSaveDialogOpen(true)} disabled={!canSave}>
                  <SaveIcon /> Save
                </MenuItem>
                <MenuItem onClick={download} disabled={!canDownload}>
                  <DownloadIcon /> Export
                </MenuItem>
              </MenuContent>
            </Menu>
          </ToolbarGroup>

          <ToolbarDivider />

          {/* Run / Run panel */}
          <ToolbarGroup>
            <RunButton />
            <Button
              icon={<TerminalIcon />}
              variant="ghost"
              elevation="sm"
              onClick={toggleRunPanel}
              aria-label="Run panel"
            />
          </ToolbarGroup>

          <ToolbarDivider />

          {/* Undo / Redo / Reset */}
          <ToolbarGroup>
            <Button
              icon={<Undo2Icon />}
              variant="ghost"
              elevation="sm"
              onClick={editor.history.undo}
              disabled={!canUndo}
              aria-label="Undo"
            />
            <Button
              icon={<Redo2Icon />}
              variant="ghost"
              elevation="sm"
              onClick={editor.history.redo}
              disabled={!canRedo}
              aria-label="Redo"
            />
            <Button
              icon={<RotateCcwIcon />}
              variant="ghost"
              elevation="sm"
              onClick={handleReset}
              disabled={!isDirty && !hasRun}
              aria-label="Reset"
            />
          </ToolbarGroup>

          <ToolbarDivider />

          {/* Config */}
          <ToolbarGroup>
            <Button
              icon={<SlidersHorizontalIcon />}
              variant="ghost"
              elevation="sm"
              onClick={toggleConfig}
              aria-label="Properties"
            />
          </ToolbarGroup>
        </Toolbar>
      </div>
      <OpenRecipeDialog open={openDialogOpen} onOpenChange={setOpenDialogOpen} />
      <SaveRecipeDialog
        open={saveDialogOpen}
        onOpenChange={setSaveDialogOpen}
        onSave={handleSave}
        isSaving={saveMutation.isPending}
      />
      <NodePaletteDialog
        open={paletteOpen}
        onOpenChange={(open) => {
          if (!open) closePalette();
        }}
      />
    </>
  );
}

export { EditorToolbar };
