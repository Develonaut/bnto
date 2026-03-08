"use client";

import { Toolbar, ToolbarGroup } from "@bnto/ui";
import { LayerPanel } from "./LayerPanel";
import { NodePalettePanel } from "./NodePalettePanel";

/**
 * EditorLeftToolbar — vertical toolbar on the left edge.
 *
 * Contains the layer panel and node palette panel menus that open to the right.
 * Positioned by the overlay — vertically centered, left edge.
 */
function EditorLeftToolbar() {
  return (
    <div className="pointer-events-auto absolute left-0 top-1/2 -translate-y-1/2">
      <Toolbar elevation="md" className="flex-col px-1.5 py-2 gap-1">
        <ToolbarGroup className="flex-col">
          <LayerPanel />
          <NodePalettePanel />
        </ToolbarGroup>
      </Toolbar>
    </div>
  );
}

export { EditorLeftToolbar };
