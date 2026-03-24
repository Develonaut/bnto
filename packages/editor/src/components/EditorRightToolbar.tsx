"use client";

import { ConfigPanel } from "./ConfigPanel";

/**
 * EditorRightToolbar — offscreen anchor for Radix Menu positioning.
 *
 * Shifted 48px past the right viewport edge so it's invisible and
 * non-interactive, but the EditorMenuPanel trigger remains in the DOM
 * for Radix to position MenuContent relative to it (side="left"
 * makes the panel land near the right edge). MenuContent renders via
 * Portal so pointer-events-none doesn't affect it.
 *
 * Toggle button lives in EditorToolbar (bottom bar).
 */
function EditorRightToolbar() {
  return (
    <div
      className="pointer-events-none absolute -right-12 top-1/2 -translate-y-1/2 opacity-0"
      aria-hidden="true"
    >
      <ConfigPanel />
    </div>
  );
}

export { EditorRightToolbar };
