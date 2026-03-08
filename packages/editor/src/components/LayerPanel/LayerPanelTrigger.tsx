"use client";

import type { ComponentProps } from "react";
import { Button, PanelLeftIcon, PanelLeftCloseIcon } from "@bnto/ui";
import { usePanel } from "../../hooks/usePanel";

/**
 * LayerPanel.Trigger — toolbar button that toggles the layers panel.
 *
 * Reads visibility from the editor store. No props needed.
 */
function LayerPanelTrigger(props: Omit<ComponentProps<typeof Button>, "onClick" | "children">) {
  const { isOpen, toggle } = usePanel("layers");

  return (
    <Button
      size="icon"
      variant="ghost"
      elevation="sm"
      onClick={toggle}
      aria-label={isOpen ? "Close panel" : "Open panel"}
      aria-expanded={isOpen}
      {...props}
    >
      {isOpen ? <PanelLeftCloseIcon className="size-4" /> : <PanelLeftIcon className="size-4" />}
    </Button>
  );
}

export { LayerPanelTrigger };
