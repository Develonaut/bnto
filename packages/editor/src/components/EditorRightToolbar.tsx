"use client";

import { Toolbar, ToolbarGroup } from "@bnto/ui";
import { ConfigPanel } from "./ConfigPanel";
import { RunPanel } from "./RunPanel";

/**
 * EditorRightToolbar — vertical toolbar on the right edge.
 *
 * Contains the config and run panel menus that open to the left.
 * Positioned by the overlay — vertically centered, right edge.
 */
function EditorRightToolbar() {
  return (
    <div className="pointer-events-auto absolute right-0 top-1/2 -translate-y-1/2">
      <Toolbar elevation="md" className="flex-col px-1.5 py-2 gap-1">
        <ToolbarGroup className="flex-col">
          <ConfigPanel />
          <RunPanel />
        </ToolbarGroup>
      </Toolbar>
    </div>
  );
}

export { EditorRightToolbar };
