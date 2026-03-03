"use client";

import { core } from "@bnto/core";

import { useDelayedLoading } from "../_hooks/useDelayedLoading";
import { Animate } from "@/components/ui/Animate";
import { EmptyState } from "@/components/ui/EmptyState";
import { Grid } from "@/components/ui/Grid";
import { Skeleton } from "@/components/ui/Skeleton";
import { RecipeCard } from "@/components/blocks/RecipeCard";
import { FolderOpenIcon } from "@/components/ui/icons";

/**
 * Saved recipes grid — self-fetching.
 * Shows RecipeCards for saved recipes, or an EmptyState when empty.
 *
 * Loading state uses Card loading={true} with skeleton content inside,
 * so the card surface springs up when data arrives. Skeleton content
 * mirrors the loaded layout: icon + title + meta.
 */
export function RecipeGrid() {
  const { data: recipes, isLoading } = core.recipes.useRecipes();
  const showSkeleton = useDelayedLoading(isLoading);

  if (showSkeleton) {
    return (
      <Grid cols={{ mobile: 1, tablet: 2, desktop: 3 }} gap="md">
        {Array.from({ length: 3 }).map((_, i) => (
          <Grid.Item key={i}>
            <RecipeCard loading>
              <RecipeCard.Header>
                <Skeleton className="size-10 rounded-lg" />
              </RecipeCard.Header>
              <RecipeCard.Content>
                <Skeleton className="h-5 w-3/4" />
                <Skeleton className="h-4 w-1/2" />
              </RecipeCard.Content>
            </RecipeCard>
          </Grid.Item>
        ))}
      </Grid>
    );
  }
  if (isLoading) return null;

  if (!recipes || recipes.length === 0) {
    return (
      <div className="min-h-[240px]">
        <EmptyState size="md">
          <EmptyState.Icon>
            <FolderOpenIcon />
          </EmptyState.Icon>
          <EmptyState.Title>No saved recipes yet</EmptyState.Title>
          <EmptyState.Description>
            Run a recipe and save it to see it here.
          </EmptyState.Description>
        </EmptyState>
      </div>
    );
  }

  return (
    <Animate.BouncyStagger asChild from={0.85}>
      <Grid cols={{ mobile: 1, tablet: 2, desktop: 3 }} gap="md">
        {recipes.map((recipe) => (
          <Grid.Item key={recipe.id}>
            <RecipeCard>
              <RecipeCard.Header>
                <RecipeCard.Icon />
              </RecipeCard.Header>
              <RecipeCard.Content>
                <RecipeCard.Title>{recipe.name}</RecipeCard.Title>
                <RecipeCard.Meta
                  nodeCount={recipe.nodeCount}
                  updatedAt={recipe.updatedAt}
                />
              </RecipeCard.Content>
            </RecipeCard>
          </Grid.Item>
        ))}
      </Grid>
    </Animate.BouncyStagger>
  );
}
