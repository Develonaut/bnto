"use client";

import { useMutation } from "@tanstack/react-query";
import { useSaveWorkflowMutation } from "../adapters";

/** Create or update a workflow. */
export function useSaveWorkflow() {
  return useMutation({ mutationFn: useSaveWorkflowMutation() });
}
