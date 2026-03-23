"use client";

import { useCallback } from "react";
import type { ReactNode } from "react";
import {
  Button,
  Toolbar,
  ToolbarGroup,
  ToolbarDivider,
  RotateCcwIcon,
  Undo2Icon,
  Redo2Icon,
  SlidersHorizontalIcon,
  TerminalIcon,
  CircleHelpIcon,
} from "@bnto/ui";
import { useEditor } from "../context";
import { RunButton } from "./RunButton";
import { NodePaletteDialog } from "./NodePaletteDialog";
import { HelpDialog } from "./HelpDialog";

/**
 * EditorToolbar — self-contained bottom-center toolbar.
 *
 * Includes its own overlay positioning. Reads all state from the store.
 * Panel triggers read visibility from the editor store — no prop drilling.
 * Help dialog state comes from the panel system (components stay dumb).
 *
 * File menu moved to EditorLeftPanel (RecipeFileMenu).
 *
 * `trailing` — optional slot rendered after the last toolbar group
 * (used by apps/web to inject NavUser without editor knowing about it).
 */

interface EditorToolbarProps {
  /** Content rendered after the last toolbar group (e.g. user avatar). */
  trailing?: ReactNode;
}

function EditorToolbar({ trailing }: EditorToolbarProps) {
  const editor = useEditor();
  const { isOpen: paletteOpen, close: closePalette } = editor.panels.usePanels("palette");
  const { toggle: toggleConfig } = editor.panels.usePanels("config");
  const { toggle: toggleRunPanel } = editor.panels.usePanels("run");
  const { isOpen: helpOpen, open: openHelp, close: closeHelp } = editor.panels.usePanels("help");
  const { canUndo, canRedo } = editor.history.useHistory();
  const { isDirty } = editor.definition.useDefinition();
  const { phase } = editor.execution.useExecution();

  const hasRun = phase !== "idle";

  const handleReset = useCallback(() => {
    const { definition } = editor.getState();
    if (definition) {
      editor.definition.loadDefinition(definition);
    } else {
      editor.definition.createBlank();
    }
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

  return (
    <>
      <div
        className="pointer-events-auto absolute bottom-0 left-1/2 -translate-x-1/2"
        data-testid="editor-toolbar"
      >
        <Toolbar elevation="md" aria-label="Editor toolbar">
          {/* Run / Run panel */}
          <ToolbarGroup>
            <RunButton />
            <Button
              icon={<TerminalIcon />}
              variant="ghost"
              elevation="sm"
              onClick={toggleRunPanel}
              aria-label="Run panel"
              data-testid="toolbar-run-panel"
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
              data-testid="toolbar-undo"
            />
            <Button
              icon={<Redo2Icon />}
              variant="ghost"
              elevation="sm"
              onClick={editor.history.redo}
              disabled={!canRedo}
              aria-label="Redo"
              data-testid="toolbar-redo"
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

          {trailing && (
            <>
              <ToolbarDivider />
              <ToolbarGroup>{trailing}</ToolbarGroup>
            </>
          )}
        </Toolbar>
      </div>
      <NodePaletteDialog open={paletteOpen} onOpenChange={handlePaletteOpenChange} />
      <HelpDialog open={helpOpen} onOpenChange={handleHelpOpenChange} />
    </>
  );
}

export { EditorToolbar };
export type { EditorToolbarProps };
