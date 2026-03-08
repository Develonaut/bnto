"use client";

import type { ComponentProps } from "react";
import { Button, SlidersHorizontalIcon } from "@bnto/ui";
import { usePanel } from "../../hooks/useEditorPanels";

/**
 * ConfigPanel.Trigger — toolbar button that toggles the config panel.
 *
 * Reads visibility from the editor store. No props needed.
 */
function ConfigPanelTrigger(props: Omit<ComponentProps<typeof Button>, "onClick" | "children">) {
  const { isOpen, toggle } = usePanel("config");

  return (
    <Button
      size="icon"
      variant="ghost"
      elevation="sm"
      onClick={toggle}
      aria-label="Properties"
      aria-expanded={isOpen}
      {...props}
    >
      <SlidersHorizontalIcon className="size-4" />
    </Button>
  );
}

export { ConfigPanelTrigger };
