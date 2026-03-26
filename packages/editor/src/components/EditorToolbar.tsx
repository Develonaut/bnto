"use client";

import { useCallback } from "react";
import {
  Button,
  Toolbar,
  ToolbarGroup,
  ToolbarDivider,
  CircleHelpIcon,
  SlidersHorizontalIcon,
  useDialog,
} from "@bnto/ui";
import { useEditor } from "../context";
import { RunButton } from "./RunButton";
import { RunPanel } from "./RunPanel";
import { OpenRecipeDialog } from "./OpenRecipeDialog";
import { NodePaletteDialog } from "./NodePaletteDialog";
import { HelpDialog } from "./HelpDialog";
import { RecipeDialog } from "./RecipeDialog";
import { FileMenuButton } from "./FileMenuButton";

/**
 * EditorToolbar — self-contained bottom-center toolbar.
 */

/** Close-only open change handler factory. */
function useCloseHandler(close: () => void) {
  return useCallback(
    (open: boolean) => {
      if (!open) close();
    },
    [close],
  );
}

function EditorToolbar() {
  const editor = useEditor();
  const { isOpen: paletteOpen, close: closePalette } = editor.panels.usePanels("palette");
  const { toggle: toggleConfig } = editor.panels.usePanels("config");
  const { isOpen: helpOpen, open: openHelp, close: closeHelp } = editor.panels.usePanels("help");
  const settingsDialog = useDialog();
  const openRecipeDialog = useDialog();
  const handlePaletteOpenChange = useCloseHandler(closePalette);
  const handleHelpOpenChange = useCloseHandler(closeHelp);

  return (
    <>
      <div
        className="pointer-events-auto absolute bottom-0 left-1/2 -translate-x-1/2"
        data-testid="editor-toolbar"
      >
        <Toolbar elevation="md" aria-label="Editor toolbar">
          <ToolbarGroup>
            <FileMenuButton
              onRename={settingsDialog.openDialog}
              onImport={openRecipeDialog.openDialog}
            />
          </ToolbarGroup>
          <ToolbarDivider />
          <ToolbarGroup>
            <RunButton />
            <RunPanel />
          </ToolbarGroup>
          <ToolbarDivider />
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
