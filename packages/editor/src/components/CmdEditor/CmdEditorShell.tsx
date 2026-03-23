"use client";

import type { ReactNode } from "react";
import { Surface } from "@bnto/ui";

/**
 * CmdEditorShell — terminal-style centered column layout.
 *
 * Both the node tree and command input sit on Surface cards,
 * vertically centered as a unified block. Max-width constrained
 * for readability.
 */

interface CmdEditorShellProps {
  children: ReactNode;
  commandSlot: ReactNode;
}

function CmdEditorShell({ children, commandSlot }: CmdEditorShellProps) {
  return (
    <div
      className="flex h-full flex-col items-center justify-center"
      data-testid="cmd-editor"
      role="application"
      aria-label="Recipe editor"
    >
      <div className="flex w-full max-w-2xl flex-col gap-4 px-4">
        <Surface elevation="sm" className="max-h-[60vh] p-2">
          <div className="h-full overflow-y-auto">{children}</div>
        </Surface>
        <Surface elevation="sm" className="shrink-0 p-2">
          {commandSlot}
        </Surface>
      </div>
    </div>
  );
}

export { CmdEditorShell };
