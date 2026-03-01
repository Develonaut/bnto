"use client";

import { useQuery } from "@tanstack/react-query";
import { core } from "../core";

/**
 * Server-node execution quota: monthly usage, limit, remaining.
 *
 * Browser executions are unlimited — this only tracks server-node runs.
 * Replaces the old `useRunsRemaining` which implied all runs were capped.
 */
export function useServerQuota() {
  return useQuery(core.analytics.serverQuotaQueryOptions());
}
