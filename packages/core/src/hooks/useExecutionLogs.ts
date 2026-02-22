"use client";

import { useQuery } from "@tanstack/react-query";
import { core } from "../core";

/** List log entries for an execution. */
export function useExecutionLogs(executionId: string) {
  return useQuery(core.executions.logsQueryOptions(executionId));
}
