"use client";

import { useQuery } from "@tanstack/react-query";
import { workflowQueryOptions } from "../adapters";
import type { WorkflowId } from "../types";

/** Get a single workflow by ID. */
export function useWorkflow(id: WorkflowId) {
  return useQuery(workflowQueryOptions(id));
}
