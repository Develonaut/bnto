"use client";

import { useQuery } from "@tanstack/react-query";
import { executionQueryOptions } from "../adapters";
import type { ExecutionId } from "../types";

/** Get a single execution by ID (real-time subscription). */
export function useExecution(id: ExecutionId) {
  return useQuery(executionQueryOptions(id));
}
