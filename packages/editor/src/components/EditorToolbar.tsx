"use client";

import { useCallback } from "react";
import {
  Button,
  Toolbar,
  ToolbarGroup,
  ToolbarDivider,
  Menu,
  MenuTrigger,
  MenuContent,
  MenuItem,
  FolderOpenIcon,
  SlidersHorizontalIcon,
  PlusIcon,
  DownloadIcon,
  FileUpIcon,
  MenuSeparator,
  CircleHelpIcon,
  PenLineIcon,
  Text,
  useDialog,
} from "@bnto/ui";
import { useEditor } from "../context";
import { downloadDefinition } from "../actions/downloadDefinition";
import { RunButton } from "./RunButton";
import { RunPanel } from "./RunPanel";
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
  const { isOpen: helpOpen, open: openHelp, close: closeHelp } = editor.panels.usePanels("help");
  const { validationErrors, recipeMetadata } = editor.definition.useDefinition();
  const { nodes } = editor.nodes.useNodes();

  const hasNodes = nodes.length > 0;
  const canExport = validationErrors.length === 0;

  const settingsDialog = useDialog();
  const openRecipeDialog = useDialog();

  const handleNew = useCallback(() => {
    editor.definition.createBlank();
  }, [editor]);

  const download = useCallback(() => {
    downloadDefinition(editor.definition);
  }, [editor]);

  const handlePaletteOpenChange = useCallback(
    (open: boolean) => {
      if (!open) closePalette();
    },
    [closePalette],
  );

  const handleHelpOpenChange = useCallback(
    (open: boolean) => {
      if (!open) closeHelp();
    },
    [closeHelp],
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
                data-testid="toolbar-file-menu"
              />
              <MenuContent side="top" className="w-52 p-1">
                <div className="px-3 py-2">
                  <Text weight="medium" size="sm" className="truncate">
                    {recipeMetadata.name}
                  </Text>
                </div>
                <MenuSeparator />
                <MenuItem onClick={settingsDialog.openDialog} data-testid="menu-rename">
                  <PenLineIcon /> Rename
                </MenuItem>
                <MenuItem onClick={handleNew} data-testid="menu-new">
                  <PlusIcon /> New Recipe
                </MenuItem>
                <MenuSeparator />
                <MenuItem onClick={download} disabled={!canDownload} data-testid="menu-export">
                  <DownloadIcon /> Export <ShortcutHint shortcutId="export" />
                </MenuItem>
                <MenuItem onClick={openRecipeDialog.openDialog} data-testid="menu-import">
                  <FileUpIcon /> Import
                </MenuItem>
              </MenuContent>
            </Menu>
          </ToolbarGroup>

          <ToolbarDivider />

          {/* Run / Run panel */}
          <ToolbarGroup>
            <RunButton />
            <RunPanel />
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
              data-testid="toolbar-properties"
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
              data-testid="toolbar-help"
            />
          </ToolbarGroup>
        </Toolbar>
      </div>
      <RecipeDialog open={settingsDialog.open} onOpenChange={settingsDialog.onOpenChange} />
      <OpenRecipeDialog open={openRecipeDialog.open} onOpenChange={openRecipeDialog.onOpenChange} />
      <NodePaletteDialog open={paletteOpen} onOpenChange={handlePaletteOpenChange} />
      <HelpDialog open={helpOpen} onOpenChange={handleHelpOpenChange} />
    </>
  );
}

export { EditorToolbar };
