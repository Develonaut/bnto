"use client";

import { useMutation } from "@tanstack/react-query";
import { core } from "../core";

/** Create or update a workflow. */
export function useSaveWorkflow() {
  return useMutation({
    mutationFn: (args: { name: string; definition: unknown; isPublic?: boolean }) =>
      core.workflows.save(args),
  });
}
