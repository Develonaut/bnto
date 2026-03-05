"use client";

import { useEffect, useState } from "react";

/**
 * Count seconds since a given start time, ticking every second while active.
 *
 * @param startedAt - Epoch ms when the timer started (undefined = not started)
 * @param isActive  - Whether the timer should be ticking
 */
export function useElapsedTime(startedAt: number | undefined, isActive: boolean): number {
  const [elapsed, setElapsed] = useState(0);

  useEffect(() => {
    if (!startedAt || !isActive) {
      return;
    }

    const compute = () => Math.floor((Date.now() - startedAt) / 1000);
    // eslint-disable-next-line react-hooks/set-state-in-effect -- timer sync with Date.now()
    setElapsed(compute());
    const interval = setInterval(() => setElapsed(compute()), 1000);
    return () => clearInterval(interval);
  }, [startedAt, isActive]);

  return elapsed;
}

/** Format elapsed seconds as "Xs" or "Xm Ys". */
export function formatElapsed(seconds: number): string {
  if (seconds < 60) return `${seconds}s`;
  const m = Math.floor(seconds / 60);
  const s = seconds % 60;
  return `${m}m ${s}s`;
}
