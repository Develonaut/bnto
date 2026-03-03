"use client";

import { useQuery } from "@tanstack/react-query";
import { core } from "../core";

/** List all recipes for the current user. */
export function useRecipes() {
  return useQuery(core.recipes.listQueryOptions());
}
