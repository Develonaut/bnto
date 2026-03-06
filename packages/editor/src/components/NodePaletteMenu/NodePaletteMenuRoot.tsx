"use client";

import type { ComponentProps } from "react";
import { Menu, MenuTrigger } from "@bnto/ui";
import { useEditorPanels } from "../../hooks/useEditorPanels";

/**
 * NodePaletteMenu root — controlled by editor store state.
 *
 * The palette's open/closed state lives in the editor store so
 * it can be opened programmatically (e.g., from a placeholder node click).
 *
 *   <NodePaletteMenu>
 *     <NodePaletteMenuTrigger>Add</NodePaletteMenuTrigger>
 *     <NodePaletteMenuContent side="top" />
 *   </NodePaletteMenu>
 */

function NodePaletteMenuRoot({
  children,
  ...props
}: Omit<ComponentProps<typeof Menu>, "open" | "onOpenChange">) {
  const { paletteOpen, openPalette, closePalette } = useEditorPanels();

  return (
    <Menu
      open={paletteOpen}
      onOpenChange={(open) => (open ? openPalette() : closePalette())}
      {...props}
    >
      {children}
    </Menu>
  );
}

/** Pass-through to MenuTrigger. */
const NodePaletteMenuTrigger = MenuTrigger;

export { NodePaletteMenuRoot, NodePaletteMenuTrigger };
