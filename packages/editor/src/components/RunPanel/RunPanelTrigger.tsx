"use client";

import type { ComponentProps } from "react";
import { Button, TerminalIcon } from "@bnto/ui";
import { usePanels } from "../../hooks/usePanels";

/**
 * RunPanel.Trigger — toolbar button that toggles the run panel.
 *
 * Reads visibility from the editor store. No props needed.
 */
function RunPanelTrigger(props: Omit<ComponentProps<typeof Button>, "onClick" | "children">) {
  const { isOpen, toggle } = usePanels("run");

  return (
    <Button
      size="icon"
      variant="ghost"
      elevation="sm"
      onClick={toggle}
      aria-label="Run panel"
      aria-expanded={isOpen}
      {...props}
    >
      <TerminalIcon className="size-4" />
    </Button>
  );
}

export { RunPanelTrigger };
