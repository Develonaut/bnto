"use client";

import type { ReactNode } from "react";
import { Surface } from "@bnto/ui";
import { useFocusZones } from "../../hooks/useFocusZones";
import { useCmdEditorShortcuts } from "../../hooks/useCmdEditorShortcuts";
import { FocusZoneContext } from "./focusZoneContext";
import { useFocusZoneContext } from "./focusZoneContext";

/**
 * CmdEditorShell — terminal-style centered column layout.
 *
 *   <CmdEditorShell>
 *     <CmdEditorTree><NodeTree /></CmdEditorTree>
 *     <CmdEditorCommand><CmdInput /></CmdEditorCommand>
 *   </CmdEditorShell>
 */

function CmdEditorShell({ children }: { children: ReactNode }) {
  const { treeRef, commandRef, onShellKeyDown } = useFocusZones();
  useCmdEditorShortcuts();

  return (
    <FocusZoneContext value={{ treeRef, commandRef }}>
      <div
        className="flex h-full flex-col items-center justify-center"
        data-testid="cmd-editor"
        role="application"
        aria-label="Recipe editor"
        onKeyDown={onShellKeyDown}
      >
        <div className="flex w-full max-w-2xl flex-col gap-4 px-4">{children}</div>
      </div>
    </FocusZoneContext>
  );
}

/** Scrollable tree region within the CmdEditor shell. */
function CmdEditorTree({ children }: { children: ReactNode }) {
  const ctx = useFocusZoneContext();

  return (
    <Surface
      ref={ctx?.treeRef}
      variant="transparent"
      elevation="none"
      dashed
      className="max-h-[40vh] p-4"
      data-testid="cmd-editor-node-tree"
    >
      <div className="h-full overflow-y-auto overflow-x-hidden pb-1 pl-1 pr-1">{children}</div>
    </Surface>
  );
}

/** Non-scrollable command input region at the bottom. */
function CmdEditorCommand({ children }: { children: ReactNode }) {
  const ctx = useFocusZoneContext();

  return (
    <div ref={ctx?.commandRef} className="shrink-0 pb-2" data-testid="cmd-editor-command-slot">
      {children}
    </div>
  );
}

export { CmdEditorShell, CmdEditorTree, CmdEditorCommand };
