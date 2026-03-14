/**
 * useKeyDown — low-level hook for document-level keydown listeners.
 *
 * Attaches a `keydown` listener to `document` and cleans up on unmount.
 * Use this when you need to intercept browser-level shortcuts (e.g.,
 * Cmd+S to prevent the Save dialog) or listen for keys outside of a
 * specific DOM element's focus scope.
 *
 * For canvas-scoped shortcuts inside ReactFlow, prefer `useKeyPress`
 * from `@xyflow/react` instead — it handles input field exclusion
 * and modifier key detection automatically.
 *
 *   useKeyDown((e) => {
 *     if ((e.metaKey || e.ctrlKey) && e.key === "s") {
 *       e.preventDefault();
 *       save();
 *     }
 *   });
 */

"use client";

import { useEffect } from "react";

function useKeyDown(handler: (event: KeyboardEvent) => void): void {
  useEffect(() => {
    document.addEventListener("keydown", handler);
    return () => document.removeEventListener("keydown", handler);
  }, [handler]);
}

export { useKeyDown };
