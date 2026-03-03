"use client";

import { useQuery } from "@tanstack/react-query";
import { core } from "../core";

/** Per-slug aggregate stats: most-used bntos, completion rates, duration. */
export function useSlugAggregates() {
  return useQuery(core.user.slugAggregatesQueryOptions());
}
