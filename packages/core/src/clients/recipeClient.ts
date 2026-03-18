"use client";

import type { Definition } from "@bnto/nodes";
import { recipesStore } from "../stores/recipesStore";
import type { RecipeService } from "../services/recipeService";
import type { ExecutionService } from "../services/executionService";
import type { StartExecutionInput } from "../types";
import type { Recipe } from "../types/recipe";

/** Input shape for save — definition + recipe metadata. */
export interface SaveInput {
  id: string;
  name: string;
  type: string;
  version: string;
  cloudId?: string | null;
}

/** Recipe client — store-backed CRUD with cloud sync on top. */
export function createRecipeClient(recipes: RecipeService, executions: ExecutionService) {
  function upsert(recipe: Recipe) {
    recipesStore.getState().upsert(recipe);
  }

  return {
    get: (id: string): Recipe | undefined => recipesStore.getState().recipes[id],

    upsert,

    /**
     * Save a recipe: build Recipe, persist locally, sync to cloud.
     *
     * Core owns the Recipe shape — callers pass definition + metadata.
     * Layer 1: upsert into recipesStore (auto-persists to localStorage).
     * Layer 2: save to Convex if recipe has a cloudId (auth validated server-side).
     */
    save: (definition: Definition, metadata: SaveInput) => {
      const cloudId = metadata.cloudId ?? undefined;
      const recipe: Recipe = {
        id: metadata.id,
        name: metadata.name,
        definition,
        type: metadata.type,
        version: metadata.version,
        cloudId,
        savedAt: Date.now(),
        syncedAt: null,
      };

      // Layer 1: local store (always, synchronous)
      upsert(recipe);

      // Layer 2: cloud sync (async, cloudId only — Convex validates auth server-side)
      if (!cloudId) return;

      recipes
        .save({ id: cloudId, name: metadata.name, definition })
        .then(() => {
          upsert({ ...recipe, syncedAt: Date.now() });
          recipes.invalidateList();
        })
        .catch(() => {});
    },

    remove: (id: string) => {
      const recipe = recipesStore.getState().recipes[id];
      recipesStore.getState().remove(id);

      if (recipe?.cloudId) {
        recipes.remove(recipe.cloudId).catch(() => {});
      }
    },

    count: () => Object.keys(recipesStore.getState().recipes).length,

    hydrateFromCloud: (cloudRecipes: Recipe[]) => {
      recipesStore.getState().hydrateFromCloud(cloudRecipes);
    },

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

export type RecipeClient = ReturnType<typeof createRecipeClient>;
