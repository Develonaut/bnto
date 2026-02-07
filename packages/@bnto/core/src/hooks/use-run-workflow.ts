"use client";

import { useMutation } from "@tanstack/react-query";
import { useStartExecutionMutation } from "../adapters";

/** Start a workflow execution. Returns the new execution ID. */
export function useRunWorkflow() {
  return useMutation({ mutationFn: useStartExecutionMutation() });
}
