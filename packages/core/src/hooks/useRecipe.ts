"use client";

import { useQuery } from "@tanstack/react-query";
import { core } from "../core";

/** Get a single recipe by ID. */
export function useRecipe(id: string) {
  return useQuery(core.recipes.getQueryOptions(id));
}
