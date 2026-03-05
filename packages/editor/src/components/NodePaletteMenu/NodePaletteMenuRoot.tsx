"use client";

import type { ComponentProps } from "react";
import { Menu, MenuTrigger } from "@bnto/ui";

/**
 * NodePaletteMenu root — delegates to Menu.
 *
 * Compound component (dot-notation) that wraps the generic Menu with
 * editor-specific node palette content.
 *
 *   <NodePaletteMenu>
 *     <NodePaletteMenuTrigger variant="ghost">
 *       <PlusIcon /> Add
 *     </NodePaletteMenuTrigger>
 *     <NodePaletteMenu.Content side="top" />
 *   </NodePaletteMenu>
 */

function NodePaletteMenuRoot({ children, ...props }: ComponentProps<typeof Menu>) {
  return <Menu {...props}>{children}</Menu>;
}

/** Pass-through to MenuTrigger. */
const NodePaletteMenuTrigger = MenuTrigger;

export { NodePaletteMenuRoot, NodePaletteMenuTrigger };
