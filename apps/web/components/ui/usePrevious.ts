import { useEffect, useRef } from "react";

/**
 * Returns the value from the previous render.
 *
 * Useful for detecting changes between renders — e.g., comparing
 * previous vs current list length to know if an item was added or removed.
 *
 *   const prevCount = usePrevious(items.length);
 *   if (prevCount !== undefined && items.length > prevCount) { … }
 *
 * Uses useRef + useEffect. The ref updates after render so the current
 * render always sees the previous value. This avoids the setState-during-
 * render pattern which can cascade into infinite loops when combined with
 * multiple synchronous external store updates (Zustand + ReactFlow).
 */
function usePrevious<T>(value: T): T | undefined {
  const ref = useRef<T | undefined>(undefined);

  useEffect(() => {
    ref.current = value;
  });

  return ref.current;
}

export { usePrevious };
