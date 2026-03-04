import { useEffect, useRef } from "react";

/**
 * Returns the value from the previous render.
 *
 * Useful for detecting changes between renders — e.g., comparing
 * previous vs current list length to know if an item was added or removed.
 *
 *   const prevCount = usePrevious(items.length);
 *   if (prevCount !== undefined && items.length > prevCount) { … }
 */
function usePrevious<T>(value: T): T | undefined {
  const ref = useRef<T>(undefined);

  useEffect(() => {
    ref.current = value;
  }, [value]);

  return ref.current;
}

export { usePrevious };
