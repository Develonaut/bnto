"use client";

import type { RecipeService } from "../services/recipeService";
import type { DraftRecipeService } from "../services/localRecipeService";
import type { ExecutionService } from "../services/executionService";
import type { StartExecutionInput } from "../types";

/**
 * Recipe client — public API for recipe operations.
 *
 * Composes recipe, draft, and execution services for cross-domain orchestration.
 * Running a recipe creates an execution (cross-domain side effect).
 * Draft operations read/remove localStorage-backed editor drafts.
 */
export function createRecipeClient(
  recipes: RecipeService,
  executions: ExecutionService,
  drafts: DraftRecipeService,
) {
  return {
    // ── Query Options ─────────────────────────────────────────────
    listQueryOptions: () => recipes.listQueryOptions(),
    getQueryOptions: (id: string) => recipes.getQueryOptions(id),

    // ── Mutations ─────────────────────────────────────────────────
    save: (args: { id?: string; name: string; definition: unknown; isPublic?: boolean }) =>
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

    // ── Draft Operations (localStorage-backed) ───────────────────
    /** List all device-local draft recipes as RecipeListItem[]. */
    listDrafts: () => drafts.list(),
    /** Remove a device-local draft by recipe ID. */
    removeDraft: (recipeId: string) => drafts.remove(recipeId),
    /** Count device-local drafts. */
    countDrafts: () => drafts.count(),
  } as const;
}

export type RecipeClient = ReturnType<typeof createRecipeClient>;
