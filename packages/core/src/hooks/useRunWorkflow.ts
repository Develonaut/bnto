"use client";

import { useMutation } from "@tanstack/react-query";
import { core } from "../core";

/** Start a workflow execution. Returns the new execution ID. */
export function useRunWorkflow() {
  return useMutation({
    mutationFn: (workflowId: string) => core.workflows.run(workflowId),
  });
}
