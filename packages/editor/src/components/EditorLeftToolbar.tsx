"use client";

import { RecipePanel } from "./RecipePanel";

/**
 * EditorLeftToolbar — offscreen anchor for Radix Menu positioning.
 *
 * Mirrors EditorRightToolbar but on the left side. Shifted 48px past
 * the left viewport edge so it's invisible and non-interactive, but
 * the EditorMenuPanel trigger remains in the DOM for Radix to position
 * MenuContent relative to it (side="right" makes the panel land near
 * the left edge). MenuContent renders via Portal so pointer-events-none
 * doesn't affect it.
 *
 * Toggle button lives in EditorToolbar (bottom bar).
 */
function EditorLeftToolbar() {
  return (
    <div className="pointer-events-none absolute -left-12 top-1/2 -translate-y-1/2 opacity-0" inert>
      <RecipePanel />
    </div>
  );
}

export { EditorLeftToolbar };
