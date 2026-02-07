"use client";

import { useQuery } from "@tanstack/react-query";
import { executionsQueryOptions } from "../adapters";
import type { WorkflowId } from "../types";

/** List executions for a workflow (most recent first, up to 50). */
export function useExecutions(workflowId: WorkflowId) {
  return useQuery(executionsQueryOptions(workflowId));
}
