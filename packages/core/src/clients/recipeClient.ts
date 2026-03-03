"use client";

import type { RecipeService } from "../services/recipeService";
import type { ExecutionService } from "../services/executionService";
import type { StartExecutionInput } from "../types";

/**
 * Recipe client — public API for recipe operations.
 *
 * Composes recipe and execution services for cross-domain orchestration.
 * Running a recipe creates an execution (cross-domain side effect).
 */
export function createRecipeClient(
  recipes: RecipeService,
  executions: ExecutionService,
) {
  return {
    // ── Query Options ─────────────────────────────────────────────
    listQueryOptions: () => recipes.listQueryOptions(),
    getQueryOptions: (id: string) => recipes.getQueryOptions(id),

    // ── Mutations ─────────────────────────────────────────────────
    save: (args: { name: string; definition: unknown; isPublic?: boolean }) =>
      recipes.save(args),

    remove: (id: string) => recipes.remove(id),

    /** Cross-domain: starts execution and invalidates both caches. */
    run: async (input: StartExecutionInput) => {
      const executionId = await executions.start(input);
      executions.invalidateExecutions(input.recipeId);
      return executionId;
    },

    // ── Cache Invalidation ────────────────────────────────────────
    invalidateList: () => recipes.invalidateList(),
    invalidateRecipe: (id: string) => recipes.invalidateRecipe(id),
  } as const;
}

export type RecipeClient = ReturnType<typeof createRecipeClient>;
