"use client";

import { useQuery } from "@tanstack/react-query";
import { core } from "../core";

/** List all workflows for the current user. */
export function useWorkflows() {
  return useQuery(core.workflows.listQueryOptions());
}
