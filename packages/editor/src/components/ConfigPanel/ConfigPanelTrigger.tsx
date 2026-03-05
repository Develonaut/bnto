"use client";

import type { ComponentProps } from "react";
import { Button, SlidersHorizontalIcon } from "@bnto/ui";
import { useEditorPanels } from "../../hooks/useEditorPanels";

/**
 * ConfigPanel.Trigger — toolbar button that toggles the config panel.
 *
 * Reads visibility from the editor store. No props needed.
 */
function ConfigPanelTrigger(props: Omit<ComponentProps<typeof Button>, "onClick" | "children">) {
  const { configOpen, toggleConfig } = useEditorPanels();

  return (
    <Button
      size="icon"
      variant="ghost"
      elevation="sm"
      onClick={toggleConfig}
      aria-label="Properties"
      aria-expanded={configOpen}
      {...props}
    >
      <SlidersHorizontalIcon className="size-4" />
    </Button>
  );
}

export { ConfigPanelTrigger };
