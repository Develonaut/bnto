"use client";

import { useContext } from "react";
import { SessionContext } from "../providers/SessionContext";

/**
 * Returns `true` when the provider stack is fully mounted and ready.
 *
 * Guards Convex hooks that crash before ConvexProvider mounts.
 * Used by paginated query hooks to skip queries until the provider is available.
 *
 * @example
 * ```ts
 * const ready = useReady();
 * const { results } = usePaginatedQuery(funcRef, ready ? args : "skip", opts);
 * ```
 */
export function useReady() {
  const ctx = useContext(SessionContext);
  if (!ctx) {
    throw new Error("useReady must be used within a BntoCoreProvider");
  }
  return ctx.ready;
}
