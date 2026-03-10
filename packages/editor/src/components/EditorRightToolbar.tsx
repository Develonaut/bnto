"use client";

import { ConfigPanel } from "./ConfigPanel";
import { RunPanel } from "./RunPanel";

/**
 * EditorRightToolbar — offscreen anchor for Radix Menu positioning.
 *
 * Shifted 48px past the right viewport edge so it's invisible and
 * non-interactive, but the EditorMenuPanel triggers remain in the DOM
 * for Radix to position MenuContent relative to them (side="left"
 * makes panels land near the right edge). MenuContent renders via
 * Portal so pointer-events-none doesn't affect it.
 *
 * Both panels are stacked in the same grid cell so they share a
 * single anchor point — panels open at a consistent height
 * regardless of which one is active.
 *
 * Toggle buttons live in EditorToolbar (bottom bar).
 */
function EditorRightToolbar() {
  return (
    <div
      className="pointer-events-none absolute -right-12 top-1/2 -translate-y-1/2 opacity-0"
      aria-hidden="true"
    >
      <div className="grid">
        <div className="[grid-area:1/1]">
          <ConfigPanel />
        </div>
        <div className="[grid-area:1/1]">
          <RunPanel />
        </div>
      </div>
    </div>
  );
}

export { EditorRightToolbar };
