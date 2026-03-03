"use client";

import { useMutation } from "@tanstack/react-query";
import { core } from "../core";
import type { StartPredefinedInput } from "../types";

/** Start a predefined bnto execution (no stored recipe needed). */
export function useRunPredefined() {
  return useMutation({
    mutationFn: (input: StartPredefinedInput) =>
      core.executions.startPredefined(input),
  });
}
