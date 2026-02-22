"use client";

import { useQuery } from "@tanstack/react-query";
import { core } from "../core";

/** List executions for a workflow (most recent first, up to 50). */
export function useExecutions(workflowId: string) {
  return useQuery(core.executions.listQueryOptions(workflowId));
}
