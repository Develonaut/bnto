"use client";

import { useEffect, useRef, useState } from "react";

/**
 * Returns the value from the previous render.
 *
 * Useful for detecting changes between renders — e.g., comparing
 * previous vs current list length to know if an item was added or removed.
 *
 *   const prevCount = usePrevious(items.length);
 *   if (prevCount !== undefined && items.length > prevCount) { … }
 *
 * Uses useState to track the previous value. The state updates in an
 * effect after render so the current render always sees the previous value.
 * This avoids reading ref.current during render (which violates
 * react-hooks/refs) and avoids the setState-during-render pattern which
 * can cascade into infinite loops with external store updates.
 */
function usePrevious<T>(value: T): T | undefined {
  const [previous, setPrevious] = useState<T | undefined>(undefined);
  const currentRef = useRef(value);

  useEffect(() => {
    setPrevious(currentRef.current);
    currentRef.current = value;
  }, [value]);

  return previous;
}

export { usePrevious };
