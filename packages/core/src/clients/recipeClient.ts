"use client";

import { recipesStore } from "../stores/recipesStore";
import type { RecipeService } from "../services/recipeService";
import type { ExecutionService } from "../services/executionService";
import type { StartExecutionInput } from "../types";
import type { StoredRecipe } from "../types/recipe";

/** Recipe client — store-backed CRUD with cloud sync on top. */
export function createRecipeClient(recipes: RecipeService, executions: ExecutionService) {
  return {
    get: (id: string): StoredRecipe | undefined => recipesStore.getState().recipes[id],

    upsert: (recipe: StoredRecipe) => recipesStore.getState().upsert(recipe),

    remove: (id: string) => {
      const recipe = recipesStore.getState().recipes[id];
      recipesStore.getState().remove(id);

      if (recipe?.metadata.cloudId) {
        recipes.remove(recipe.metadata.cloudId).catch(() => {});
      }
    },

    count: () => Object.keys(recipesStore.getState().recipes).length,

    syncToCloud: async (recipe: StoredRecipe) => {
      return recipes.save({
        id: recipe.metadata.cloudId,
        name: recipe.metadata.name,
        definition: recipe.definition,
      });
    },

    hydrateFromCloud: (cloudRecipes: StoredRecipe[]) => {
      recipesStore.getState().hydrateFromCloud(cloudRecipes);
    },

    listQueryOptions: () => recipes.listQueryOptions(),
    getQueryOptions: (id: string) => recipes.getQueryOptions(id),

    save: (args: { id?: string; name: string; definition: unknown; isPublic?: boolean }) =>
      recipes.save(args),

    run: async (input: StartExecutionInput) => {
      const executionId = await executions.start(input);
      executions.invalidateExecutions(input.recipeId);
      return executionId;
    },

    invalidateList: () => recipes.invalidateList(),
    invalidateRecipe: (id: string) => recipes.invalidateRecipe(id),
  } as const;
}

export type RecipeClient = ReturnType<typeof createRecipeClient>;
