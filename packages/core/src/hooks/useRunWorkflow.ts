"use client";

import { useMutation } from "@tanstack/react-query";
import { core } from "../core";
import type { StartExecutionInput } from "../types";

/** Start a workflow execution. Returns the new execution ID. */
export function useRunWorkflow() {
  return useMutation({
    mutationFn: (input: StartExecutionInput) => core.workflows.run(input),
  });
}
