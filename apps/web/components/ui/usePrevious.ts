import { useState } from "react";

/**
 * Returns the value from the previous render.
 *
 * Useful for detecting changes between renders — e.g., comparing
 * previous vs current list length to know if an item was added or removed.
 *
 *   const prevCount = usePrevious(items.length);
 *   if (prevCount !== undefined && items.length > prevCount) { … }
 *
 * Uses the "update state during render" pattern (React-approved) instead
 * of reading a ref during render (flagged by react-hooks/refs in React 19).
 */
function usePrevious<T>(value: T): T | undefined {
  const [pair, setPair] = useState<[T | undefined, T]>([undefined, value]);

  if (pair[1] !== value) {
    setPair([pair[1], value]);
  }

  return pair[0];
}

export { usePrevious };
