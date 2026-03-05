"use client";

import type { ComponentProps } from "react";
import { Button, PanelLeftIcon, PanelLeftCloseIcon } from "@bnto/ui";
import { useEditorPanels } from "../../hooks/useEditorPanels";

/**
 * LayerPanel.Trigger — toolbar button that toggles the layers panel.
 *
 * Reads visibility from the editor store. No props needed.
 */
function LayerPanelTrigger(props: Omit<ComponentProps<typeof Button>, "onClick" | "children">) {
  const { layersOpen, toggleLayers } = useEditorPanels();

  return (
    <Button
      size="icon"
      variant="ghost"
      elevation="sm"
      onClick={toggleLayers}
      aria-label={layersOpen ? "Close panel" : "Open panel"}
      aria-expanded={layersOpen}
      {...props}
    >
      {layersOpen ? (
        <PanelLeftCloseIcon className="size-4" />
      ) : (
        <PanelLeftIcon className="size-4" />
      )}
    </Button>
  );
}

export { LayerPanelTrigger };
