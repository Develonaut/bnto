"use client";

import {
  getRecipesQuery,
  getRecipeQuery,
  saveRecipe,
  removeRecipe,
} from "../adapters/convex/recipeAdapter";
import { toRecipe } from "../transforms/toRecipe";
import { toRecipeListItem } from "../transforms/toRecipeListItem";
import { getQueryClient } from "../client";
import type { RawRecipeDoc, RawRecipeListProjection } from "../types/raw";

export function createRecipeService() {
  function invalidateList() {
    getQueryClient().invalidateQueries({
      queryKey: getRecipesQuery().queryKey,
    });
  }

  function invalidateRecipe(id: string) {
    getQueryClient().invalidateQueries({
      queryKey: getRecipeQuery(id).queryKey,
    });
  }

  return {
    // ── Query Options ─────────────────────────────────────────────
    // Note: convexQuery returns opaque types, so select receives `unknown`.
    // The cast to Raw* types is a trust boundary — Convex docs match our
    // raw type definitions by construction (derived from the same schema).
    listQueryOptions: () => ({
      ...getRecipesQuery(),
      select: (data: unknown) => (data as RawRecipeListProjection[]).map(toRecipeListItem),
    }),

    getQueryOptions: (id: string) => ({
      ...getRecipeQuery(id),
      select: (data: unknown) => (data ? toRecipe(data as RawRecipeDoc) : null),
    }),

    // ── Mutations ─────────────────────────────────────────────────
    save: async (args: { id?: string; name: string; definition: unknown; isPublic?: boolean }) => {
      const result = await saveRecipe(args);
      invalidateList();
      if (args.id) invalidateRecipe(args.id);
      return result;
    },

    remove: async (id: string) => {
      await removeRecipe(id);
      invalidateList();
      invalidateRecipe(id);
    },

    // ── Cache Invalidation ────────────────────────────────────────
    invalidateList,
    invalidateRecipe,
  } as const;
}

export type RecipeService = ReturnType<typeof createRecipeService>;
