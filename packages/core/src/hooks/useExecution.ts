"use client";

import { useQuery } from "@tanstack/react-query";
import { core } from "../core";

/** Get a single execution by ID (real-time subscription). */
export function useExecution(id: string) {
  const options = core.executions.getQueryOptions(id);
  return useQuery({
    ...options,
    enabled: id.length > 0,
  });
}
