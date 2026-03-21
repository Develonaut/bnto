"use client";

import type { Definition } from "@bnto/registry";
import { definitionToRecipe } from "@bnto/registry";
import { recipesStore } from "../stores/recipesStore";
import type { RecipeService } from "../services/recipeService";
import type { ExecutionService } from "../services/executionService";
import type { AuthClient } from "./authClient";
import type { StartExecutionInput } from "../types";
import type { UserRecipe } from "../types/recipe";

/** Input shape for save — definition + recipe metadata. */
export interface SaveInput {
  id: string;
  name: string;
  slug: string;
  cloudId?: string | null;
}

/** Recipe client — store-backed CRUD with cloud sync on top. */
export function createRecipeClient(
  recipes: RecipeService,
  executions: ExecutionService,
  auth: AuthClient,
) {
  function upsert(recipe: UserRecipe) {
    recipesStore.getState().upsert(recipe);
  }

  return {
    get: (id: string): UserRecipe | undefined => recipesStore.getState().recipes[id],

    upsert,

    /**
     * Create a personal recipe from any Definition.
     *
     * Stamps a fresh UUID, derives display metadata (name, slug, category,
     * accept spec) from the definition, persists to localStorage.
     * Returns the new recipe ID for navigation.
     */
    createFromDefinition: (definition: Definition): string => {
      const id = crypto.randomUUID();
      const cloned: Definition = { ...definition, id };
      const recipe = definitionToRecipe(cloned, { id });

      const userRecipe: UserRecipe = {
        ...recipe,
        cloudId: null,
        savedAt: Date.now(),
        syncedAt: null,
      };

      upsert(userRecipe);
      return id;
    },

    /**
     * Save a recipe: build UserRecipe, persist locally, sync to cloud.
     *
     * Core owns the UserRecipe shape — callers pass definition + metadata.
     * Layer 1: upsert into recipesStore (auto-persists to localStorage).
     * Layer 2: attempt cloud sync (Convex validates auth server-side).
     *   - Existing cloud recipe (cloudId): ID-based update.
     *   - New recipe (no cloudId): name-based upsert. Stamps cloudId on success.
     *   - Unauthenticated: skipped, recipe remains local-only.
     */
    save: (definition: Definition, metadata: SaveInput) => {
      const cloudId = metadata.cloudId ?? undefined;
      const recipe = definitionToRecipe(definition, {
        id: metadata.id,
        slug: metadata.slug,
        name: metadata.name,
      });

      // Wrap Recipe into UserRecipe with persistence fields
      const userRecipe: UserRecipe = {
        ...recipe,
        cloudId: cloudId ?? null,
        savedAt: Date.now(),
        syncedAt: null,
      };

      // Layer 1: local store (always, synchronous)
      upsert(userRecipe);

      // Layer 2: cloud sync (skip entirely if not authenticated)
      if (!auth.isAuthenticated()) return;

      recipes
        .save({
          ...(cloudId ? { id: cloudId } : {}),
          name: metadata.name,
          definition,
        })
        .then((resultId) => {
          upsert({
            ...userRecipe,
            cloudId: String(resultId),
            syncedAt: Date.now(),
          });
          recipes.invalidateList();
        })
        .catch(() => {});
    },

    /**
     * Sync all local-only recipes to cloud.
     *
     * Called on unauth→auth transition (sign-in / sign-up).
     * For each recipe without a cloudId: save to Convex (name-based upsert),
     * then stamp cloudId + syncedAt back onto the local recipe.
     */
    syncToCloud: async () => {
      if (!auth.isAuthenticated()) return;

      const allRecipes = Object.values(recipesStore.getState().recipes);
      const unsynced = allRecipes.filter((r) => !r.cloudId);
      if (unsynced.length === 0) return;

      await Promise.allSettled(
        unsynced.map(async (recipe) => {
          const cloudId = await recipes.save({
            name: recipe.name,
            definition: recipe.definition,
          });
          upsert({
            ...recipe,
            cloudId: String(cloudId),
            syncedAt: Date.now(),
          });
        }),
      );

      recipes.invalidateList();
    },

    remove: (id: string) => {
      const recipe = recipesStore.getState().recipes[id];
      recipesStore.getState().remove(id);

      if (recipe?.cloudId && auth.isAuthenticated()) {
        recipes.remove(recipe.cloudId).catch(() => {});
      }
    },

    count: () => Object.keys(recipesStore.getState().recipes).length,

    hydrateFromCloud: (cloudRecipes: UserRecipe[]) => {
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
