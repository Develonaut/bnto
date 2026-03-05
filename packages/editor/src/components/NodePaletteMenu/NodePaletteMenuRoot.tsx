"use client";

import type { ComponentProps } from "react";
import { Menu } from "@bnto/ui";

/**
 * NodePaletteMenu root — delegates to Menu.
 *
 * Compound component (dot-notation) that wraps the generic Menu with
 * editor-specific node palette content.
 *
 *   <NodePaletteMenu>
 *     <NodePaletteMenu.Trigger variant="ghost">
 *       <PlusIcon /> Add
 *     </NodePaletteMenu.Trigger>
 *     <NodePaletteMenu.Content side="top" />
 *   </NodePaletteMenu>
 */

function NodePaletteMenuRoot({ children, ...props }: ComponentProps<typeof Menu>) {
  return <Menu {...props}>{children}</Menu>;
}

/** Pass-through to Menu.Trigger. */
const NodePaletteMenuTrigger = Menu.Trigger;

export { NodePaletteMenuRoot, NodePaletteMenuTrigger };
