"use client";

import { useMutation } from "@tanstack/react-query";
import { core } from "../core";

/** Delete a recipe by ID. */
export function useRemoveRecipe() {
  return useMutation({
    mutationFn: (id: string) => core.recipes.remove(id),
  });
}
