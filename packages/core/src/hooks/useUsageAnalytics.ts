"use client";

import { useQuery } from "@tanstack/react-query";
import { core } from "../core";

/** Lifetime usage analytics: plan, total runs, last activity. */
export function useUsageAnalytics() {
  return useQuery(core.analytics.analyticsQueryOptions());
}
