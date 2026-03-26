"use client";

import { useCallback } from "react";
import {
  Button,
  Toolbar,
  ToolbarGroup,
  ToolbarDivider,
  CircleHelpIcon,
  FileCogIcon,
  SlidersHorizontalIcon,
} from "@bnto/ui";
import { useEditor } from "../context";
import { RunButton } from "./RunButton";
import { RunPanel } from "./RunPanel";
import { NodePaletteDialog } from "./NodePaletteDialog";
import { HelpDialog } from "./HelpDialog";

/**
 * EditorToolbar — self-contained bottom-center toolbar.
 *
 * Recipe actions (rename, new, export, import) and recipe-level
 * settings live in the dedicated Settings panel — opened via the
 * Settings button (FileCog icon).
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
  const { toggle: toggleSettings } = editor.panels.usePanels("settings");
  const { isOpen: helpOpen, open: openHelp, close: closeHelp } = editor.panels.usePanels("help");
  const handlePaletteOpenChange = useCloseHandler(closePalette);
  const handleHelpOpenChange = useCloseHandler(closeHelp);

  return {
    toggleConfig,
    toggleSettings,
    openHelp,
    paletteOpen,
    handlePaletteOpenChange,
    helpOpen,
    handleHelpOpenChange,
  };
}

/** Properties + Help action buttons on the right side of the toolbar. */
function ToolbarActions({
  toggleConfig,
  openHelp,
}: {
  toggleConfig: () => void;
  openHelp: () => void;
}) {
  return (
    <>
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
    </>
  );
}

/** Toolbar button strip (settings, run, config, help). */
function ToolbarStrip({
  toggleSettings,
  toggleConfig,
  openHelp,
}: {
  toggleSettings: () => void;
  toggleConfig: () => void;
  openHelp: () => void;
}) {
  return (
    <Toolbar elevation="md" aria-label="Editor toolbar">
      <ToolbarGroup>
        <Button
          icon={<FileCogIcon />}
          variant="ghost"
          elevation="sm"
          onClick={toggleSettings}
          aria-label="Settings"
          data-testid="toolbar-settings"
        />
      </ToolbarGroup>
      <ToolbarDivider />
      <ToolbarGroup>
        <RunButton />
        <RunPanel />
      </ToolbarGroup>
      <ToolbarActions toggleConfig={toggleConfig} openHelp={openHelp} />
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
          toggleSettings={state.toggleSettings}
          toggleConfig={state.toggleConfig}
          openHelp={state.openHelp}
        />
      </div>
      <NodePaletteDialog open={state.paletteOpen} onOpenChange={state.handlePaletteOpenChange} />
      <HelpDialog open={state.helpOpen} onOpenChange={state.handleHelpOpenChange} />
    </>
  );
}

export { EditorToolbar };
