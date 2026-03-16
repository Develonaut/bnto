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
  DownloadIcon,
  FileUpIcon,
  MenuSeparator,
  CircleHelpIcon,
  PenLineIcon,
  Text,
} from "@bnto/ui";
import { useEditor } from "../context";
import { downloadDefinition } from "../actions/downloadDefinition";
import { formatLastSaved } from "../draft/formatLastSaved";
import { RunButton } from "./RunButton";
import { OpenRecipeDialog } from "./OpenRecipeDialog";
import { NodePaletteDialog } from "./NodePaletteDialog";
import { HelpDialog } from "./HelpDialog";
import { RecipeDialog } from "./RecipeDialog";
import { ShortcutHint } from "./ShortcutHint";

/**
 * EditorToolbar — self-contained bottom-center toolbar.
 *
 * Includes its own overlay positioning. Reads all state from the store.
 * Panel triggers read visibility from the editor store — no prop drilling.
 * Help dialog state comes from the panel system (components stay dumb).
 */

function EditorToolbar() {
  const editor = useEditor();
  const { isOpen: paletteOpen, close: closePalette } = editor.panels.usePanels("palette");
  const { toggle: toggleConfig } = editor.panels.usePanels("config");
  const { toggle: toggleRunPanel } = editor.panels.usePanels("run");
  const { isOpen: helpOpen, open: openHelp, close: closeHelp } = editor.panels.usePanels("help");
  const { canUndo, canRedo } = editor.history.useHistory();
  const { isDirty, validationErrors, lastSavedAt, recipeMetadata } =
    editor.definition.useDefinition();
  const { phase } = editor.execution.useExecution();
  const { nodes } = editor.nodes.useNodes();

  const hasRun = phase !== "idle";
  const hasNodes = nodes.length > 0;
  const canExport = validationErrors.length === 0;

  const [openDialogOpen, setOpenDialogOpen] = useState(false);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const lastSavedLabel = formatLastSaved(lastSavedAt);

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
              <MenuContent side="top" className="w-52 p-1">
                <div className="px-3 py-2">
                  <Text weight="medium" size="sm" className="truncate">
                    {recipeMetadata.name}
                  </Text>
                  <Text size="xs" className="text-muted-foreground">
                    {lastSavedLabel}
                  </Text>
                </div>
                <MenuSeparator />
                <MenuItem onClick={() => setSettingsOpen(true)}>
                  <PenLineIcon /> Rename
                </MenuItem>
                <MenuItem onClick={handleNew}>
                  <PlusIcon /> New
                </MenuItem>
                <MenuItem onClick={() => setOpenDialogOpen(true)}>
                  <FileUpIcon /> Open
                </MenuItem>
                <MenuSeparator />
                <MenuItem onClick={download} disabled={!canDownload}>
                  <DownloadIcon /> Export <ShortcutHint shortcutId="export" />
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

          <ToolbarDivider />

          {/* Help */}
          <ToolbarGroup>
            <Button
              icon={<CircleHelpIcon />}
              variant="ghost"
              elevation="sm"
              onClick={openHelp}
              aria-label="Help"
            />
          </ToolbarGroup>
        </Toolbar>
      </div>
      <RecipeDialog open={settingsOpen} onOpenChange={setSettingsOpen} />
      <OpenRecipeDialog open={openDialogOpen} onOpenChange={setOpenDialogOpen} />
      <NodePaletteDialog
        open={paletteOpen}
        onOpenChange={(open) => {
          if (!open) closePalette();
        }}
      />
      <HelpDialog
        open={helpOpen}
        onOpenChange={(open) => {
          if (!open) closeHelp();
        }}
      />
    </>
  );
}

export { EditorToolbar };
