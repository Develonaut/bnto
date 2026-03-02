import { useEffect, useState } from "react";

/**
 * Suppress a loading skeleton for a short delay.
 *
 * If the data arrives before the delay (common for empty states),
 * the skeleton never shows — avoiding a jarring flash-then-empty.
 * If loading takes longer than the delay, the skeleton appears as normal.
 *
 * @param isLoading - Whether data is still loading.
 * @param delay - Milliseconds to wait before showing skeleton (default 300ms).
 * @returns `true` only when isLoading AND the delay has elapsed.
 */
export function useDelayedLoading(isLoading: boolean, delay = 300): boolean {
  const [delayElapsed, setDelayElapsed] = useState(false);

  useEffect(() => {
    if (!isLoading) return;

    const timer = setTimeout(() => setDelayElapsed(true), delay);
    return () => clearTimeout(timer);
  }, [isLoading, delay]);

  return isLoading && delayElapsed;
}
