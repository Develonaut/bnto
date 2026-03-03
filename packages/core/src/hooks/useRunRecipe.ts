"use client";

import { useMutation } from "@tanstack/react-query";
import { core } from "../core";
import type { StartExecutionInput } from "../types";

/** Start a recipe execution. Returns the new execution ID. */
export function useRunRecipe() {
  return useMutation({
    mutationFn: (input: StartExecutionInput) => core.recipes.run(input),
  });
}
