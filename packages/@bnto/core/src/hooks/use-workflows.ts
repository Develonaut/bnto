"use client";

import { useQuery } from "@tanstack/react-query";
import { workflowsQueryOptions } from "../adapters";

/** List all workflows for the current user. */
export function useWorkflows() {
  return useQuery(workflowsQueryOptions());
}
