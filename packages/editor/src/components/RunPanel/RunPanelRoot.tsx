"use client";

import { TerminalIcon } from "@bnto/ui";
import { RunTab } from "./RunTab";
import { EditorMenuPanel } from "../EditorMenuPanel";

/**
 * RunPanel — menu-based input staging + execution results panel.
 *
 * Opens upward from the toolbar trigger. Closes via toolbar toggle only.
 * Children consume execution state directly from the editor store.
 */
function RunPanelRoot() {
  return (
    <EditorMenuPanel
      panelId="run"
      side="top"
      width="w-full"
      boundaryPadding={16}
      label="Run panel"
      icon={<TerminalIcon className="size-4" />}
      className="!h-[300px] !w-[50vw] !min-w-0"
    >
      <RunTab />
    </EditorMenuPanel>
  );
}

export { RunPanelRoot };
