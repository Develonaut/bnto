"use client";

import { useMutation } from "@tanstack/react-query";
import { core } from "../core";

/** Create or update a recipe. */
export function useSaveRecipe() {
  return useMutation({
    mutationFn: (args: { name: string; definition: unknown; isPublic?: boolean }) =>
      core.recipes.save(args),
  });
}
