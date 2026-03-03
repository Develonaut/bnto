"use client";

import { useIsAuthenticated } from "./useIsAuthenticated";
import { useServerExecutionHistory } from "./useServerExecutionHistory";
import { useLocalExecutionHistory } from "./useLocalExecutionHistory";

/**
 * Routes to server (Convex) or local (IndexedDB) history based on auth state.
 * Both sub-hooks are called unconditionally; the inactive path is skipped via `enabled`.
 */
export function useExecutionHistory(options?: { pageSize?: number }) {
  const isAuthenticated = useIsAuthenticated();

  const server = useServerExecutionHistory({ ...options, enabled: isAuthenticated });
  const local = useLocalExecutionHistory({ enabled: !isAuthenticated });

  return isAuthenticated ? server : local;
}
