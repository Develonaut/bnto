"use client";

import { useMutation } from "@tanstack/react-query";
import { useRemoveWorkflowMutation } from "../adapters";

/** Delete a workflow by ID. */
export function useRemoveWorkflow() {
  return useMutation({ mutationFn: useRemoveWorkflowMutation() });
}
