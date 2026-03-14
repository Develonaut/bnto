"use client";

import { core } from "@bnto/core";
import type { Definition } from "@bnto/nodes";
import { Skeleton, Stack, Text } from "@bnto/ui";

/**
 * SavedRecipeLoader — fetches a saved recipe by ID from Convex.
 *
 * Renders children with the loaded definition once ready.
 * Shows a loading state while the recipe is being fetched.
 * Shows an error state if the recipe is not found.
 */

interface SavedRecipeLoaderProps {
  recipeId: string;
  children: (definition: Definition) => React.ReactNode;
}

export function SavedRecipeLoader({ recipeId, children }: SavedRecipeLoaderProps) {
  const { data: recipe, isLoading } = core.recipes.useRecipe(recipeId);

  if (isLoading) {
    return (
      <Stack className="items-center justify-center h-full gap-4">
        <Skeleton className="h-8 w-48" />
        <Text color="muted" size="sm">
          Loading recipe…
        </Text>
      </Stack>
    );
  }

  if (!recipe?.definition) {
    return (
      <Stack className="items-center justify-center h-full gap-2">
        <Text color="muted">Recipe not found</Text>
      </Stack>
    );
  }

  return <>{children(recipe.definition as Definition)}</>;
}
