"use client";

import { core } from "@bnto/core";

import { useDelayedLoading } from "../_hooks/useDelayedLoading";
import {
  BouncyStagger,
  Button,
  EmptyState,
  EmptyStateIcon,
  EmptyStateTitle,
  EmptyStateDescription,
  FolderOpenIcon,
  Grid,
  GridItem,
  Skeleton,
} from "@bnto/ui";
import {
  RecipeCard,
  RecipeCardHeader,
  RecipeCardContent,
  RecipeCardIcon,
  RecipeCardTitle,
  RecipeCardMeta,
} from "@/components/blocks/RecipeCard";

/**
 * Saved recipes grid — self-fetching.
 * Shows RecipeCards for saved recipes, or an EmptyState when empty.
 *
 * Unauthenticated users see a sign-in nudge since saved recipes
 * require an account. Authenticated users with no recipes see
 * a standard empty state.
 *
 * Loading state uses Card loading={true} with skeleton content inside,
 * so the card surface springs up when data arrives. Skeleton content
 * mirrors the loaded layout: icon + title + meta.
 */
export function RecipeGrid() {
  const { isAuthenticated } = core.auth.useAuth();
  const { data: recipes, isLoading } = core.recipes.useRecipes();
  const showSkeleton = useDelayedLoading(isLoading && isAuthenticated);

  if (!isAuthenticated) {
    return (
      <div className="min-h-[240px]">
        <EmptyState size="md">
          <EmptyStateIcon>
            <FolderOpenIcon />
          </EmptyStateIcon>
          <EmptyStateTitle>Save your recipes</EmptyStateTitle>
          <EmptyStateDescription>
            Sign in to save recipes, build custom workflows, and pick up where you left off.
          </EmptyStateDescription>
          <Button href="/signin" variant="primary" elevation="sm" className="mt-2">
            Sign in
          </Button>
        </EmptyState>
      </div>
    );
  }

  if (showSkeleton) {
    return (
      <Grid cols={{ mobile: 1, tablet: 2, desktop: 3 }} gap="md">
        {Array.from({ length: 3 }).map((_, i) => (
          <GridItem key={i}>
            <RecipeCard loading>
              <RecipeCardHeader>
                <Skeleton className="size-10 rounded-lg" />
              </RecipeCardHeader>
              <RecipeCardContent>
                <Skeleton className="h-5 w-3/4" />
                <Skeleton className="h-4 w-1/2" />
              </RecipeCardContent>
            </RecipeCard>
          </GridItem>
        ))}
      </Grid>
    );
  }
  if (isLoading) return null;

  if (!recipes || recipes.length === 0) {
    return (
      <div className="min-h-[240px]">
        <EmptyState size="md">
          <EmptyStateIcon>
            <FolderOpenIcon />
          </EmptyStateIcon>
          <EmptyStateTitle>No saved recipes yet</EmptyStateTitle>
          <EmptyStateDescription>Run a recipe and save it to see it here.</EmptyStateDescription>
        </EmptyState>
      </div>
    );
  }

  return (
    <BouncyStagger asChild from={0.85}>
      <Grid cols={{ mobile: 1, tablet: 2, desktop: 3 }} gap="md">
        {recipes.map((recipe) => (
          <GridItem key={recipe.id}>
            <RecipeCard>
              <RecipeCardHeader>
                <RecipeCardIcon />
              </RecipeCardHeader>
              <RecipeCardContent>
                <RecipeCardTitle>{recipe.name}</RecipeCardTitle>
                <RecipeCardMeta nodeCount={recipe.nodeCount} updatedAt={recipe.updatedAt} />
              </RecipeCardContent>
            </RecipeCard>
          </GridItem>
        ))}
      </Grid>
    </BouncyStagger>
  );
}
