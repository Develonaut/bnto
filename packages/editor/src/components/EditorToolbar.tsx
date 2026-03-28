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

/** Wire up all panel/dialog state the toolbar needs. */
function useToolbarState() {
  const editor = useEditor();
  const { isOpen: paletteOpen, close: closePalette } = editor.panels.usePanels("palette");
  const { toggle: toggleConfig } = editor.panels.usePanels("config");
  const { isOpen: helpOpen, open: openHelp, close: closeHelp } = editor.panels.usePanels("help");
  const settingsDialog = useDialog();
  const openRecipeDialog = useDialog();
  const handlePaletteOpenChange = useCloseHandler(closePalette);
  const handleHelpOpenChange = useCloseHandler(closeHelp);

  return {
    toggleConfig,
    openHelp,
    settingsDialog,
    openRecipeDialog,
    paletteOpen,
    handlePaletteOpenChange,
    helpOpen,
    handleHelpOpenChange,
  };
}

/** Help action button on the right side of the toolbar. */
function ToolbarActions({ openHelp }: { openHelp: () => void }) {
  return (
    <>
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
    </>
  );
}

/** Toolbar button strip (file, run + config, help). */
function ToolbarStrip({
  toggleConfig,
  openHelp,
  onRename,
  onImport,
}: {
  toggleConfig: () => void;
  openHelp: () => void;
  onRename: () => void;
  onImport: () => void;
}) {
  return (
    <Toolbar elevation="md" aria-label="Editor toolbar">
      <ToolbarGroup>
        <FileMenuButton onRename={onRename} onImport={onImport} />
      </ToolbarGroup>
      <ToolbarDivider />
      <ToolbarGroup>
        <RunButton />
        <RunPanel />
        <Button
          icon={<SlidersHorizontalIcon />}
          variant="ghost"
          elevation="sm"
          onClick={toggleConfig}
          aria-label="Properties"
          data-testid="toolbar-properties"
        />
      </ToolbarGroup>
      <ToolbarActions openHelp={openHelp} />
    </Toolbar>
  );
}

function EditorToolbar() {
  const state = useToolbarState();

  return (
    <>
      <div
        className="pointer-events-auto absolute bottom-0 left-1/2 -translate-x-1/2"
        data-testid="editor-toolbar"
      >
        <ToolbarStrip
          toggleConfig={state.toggleConfig}
          openHelp={state.openHelp}
          onRename={state.settingsDialog.openDialog}
          onImport={state.openRecipeDialog.openDialog}
        />
      </div>
      <RecipeDialog
        open={state.settingsDialog.open}
        onOpenChange={state.settingsDialog.onOpenChange}
      />
      <OpenRecipeDialog
        open={state.openRecipeDialog.open}
        onOpenChange={state.openRecipeDialog.onOpenChange}
      />
      <NodePaletteDialog open={state.paletteOpen} onOpenChange={state.handlePaletteOpenChange} />
      <HelpDialog open={state.helpOpen} onOpenChange={state.handleHelpOpenChange} />
    </>
  );
}

export { EditorToolbar };
