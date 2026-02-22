"use client";

import { useMutation } from "@tanstack/react-query";
import { core } from "../core";

/** Delete a workflow by ID. */
export function useRemoveWorkflow() {
  return useMutation({
    mutationFn: (id: string) => core.workflows.remove(id),
  });
}
