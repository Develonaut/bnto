"use client";

import { useQuery } from "@tanstack/react-query";
import { core } from "../core";

/** List executions for a recipe (most recent first, up to 50). */
export function useExecutions(recipeId: string) {
  return useQuery(core.executions.listQueryOptions(recipeId));
}
