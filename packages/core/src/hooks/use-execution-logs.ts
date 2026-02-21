"use client";

import { useQuery } from "@tanstack/react-query";
import { executionLogsQueryOptions } from "../adapters";
import type { ExecutionId } from "../types";

/** List log entries for an execution. */
export function useExecutionLogs(executionId: ExecutionId) {
  return useQuery(executionLogsQueryOptions(executionId));
}
