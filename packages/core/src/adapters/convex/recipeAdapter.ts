"use client";

import { convexQuery } from "@convex-dev/react-query";
import { api } from "@bnto/backend/convex/_generated/api";
import type { Id } from "@bnto/backend/convex/_generated/dataModel";
import { getConvexClient } from "../../client";

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

export function getRecipesQuery() {
  return convexQuery(api.recipes.list, {});
}

export function getRecipeQuery(id: string) {
  return convexQuery(api.recipes.get, id ? { id: id as Id<"recipes"> } : "skip");
}

// ---------------------------------------------------------------------------
// Mutations (imperative — no React hooks)
// ---------------------------------------------------------------------------

export function saveRecipe(args: {
  name: string;
  definition: unknown;
  isPublic?: boolean;
}) {
  return getConvexClient().mutation(api.recipes.save, args);
}

export function removeRecipe(id: string) {
  return getConvexClient().mutation(api.recipes.remove, {
    id: id as Id<"recipes">,
  });
}
