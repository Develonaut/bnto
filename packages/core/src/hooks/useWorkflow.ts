"use client";

import { useQuery } from "@tanstack/react-query";
import { core } from "../core";

/** Get a single workflow by ID. */
export function useWorkflow(id: string) {
  return useQuery(core.workflows.getQueryOptions(id));
}
