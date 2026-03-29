"use client";

import type { RecipeService } from "../services/recipeService";
import type { ExecutionService } from "../services/executionService";
import type { StartExecutionInput } from "../types";

/** Recipe client — query options, cache invalidation, and run(). */
export function createRecipeClient(recipes: RecipeService, executions: ExecutionService) {
  return {
    listQueryOptions: () => recipes.listQueryOptions(),
    getQueryOptions: (id: string) => recipes.getQueryOptions(id),
    run: async (input: StartExecutionInput) => {
      const executionId = await executions.start(input);
      executions.invalidateExecutions(input.recipeId);
      return executionId;
    },
    invalidateList: () => recipes.invalidateList(),
    invalidateRecipe: (id: string) => recipes.invalidateRecipe(id),
  } as const;
}
