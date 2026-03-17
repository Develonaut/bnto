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
  PenLineIcon,
  PlusIcon,
  List,
  ListItem,
  ListItemContent,
  ListItemActions,
  RecipeCardIcon,
  Row,
  Text,
  Skeleton,
} from "@bnto/ui";
import { formatTimeAgo } from "@/lib/formatTimeAgo";
import { DeleteRecipeButton } from "./DeleteRecipeButton";
import { LocalRecipeGrid } from "./LocalRecipeGrid";

/**
 * Saved recipes list — self-fetching.
 *
 * When authenticated: shows Convex-backed recipes with cloud sync.
 * When unauthenticated: delegates to LocalRecipeGrid for localStorage drafts.
 */
export function RecipeGrid() {
  const { isAuthenticated } = core.auth.useAuth();
  const { data: recipes, isLoading } = core.recipes.useRecipes();
  const showSkeleton = useDelayedLoading(isLoading && isAuthenticated);

  if (!isAuthenticated) {
    return <LocalRecipeGrid />;
  }

  if (showSkeleton) {
    return (
      <List>
        {Array.from({ length: 3 }).map((_, i) => (
          <ListItem key={i} loading>
            <Skeleton className="size-10 rounded-lg" />
            <ListItemContent>
              <Skeleton className="h-5 w-3/4" />
              <Skeleton className="h-4 w-1/3" />
            </ListItemContent>
          </ListItem>
        ))}
      </List>
    );
  }
  if (isLoading) return null;

  if (!recipes || recipes.length === 0) {
    return (
      <div className="min-h-[240px]">
        <EmptyState>
          <EmptyStateIcon>
            <FolderOpenIcon />
          </EmptyStateIcon>
          <EmptyStateTitle>No saved recipes yet</EmptyStateTitle>
          <EmptyStateDescription>
            Create a recipe in the editor and save it to find it here.
          </EmptyStateDescription>
          <Button href="/editor" variant="primary" elevation="sm" className="mt-2">
            <PlusIcon />
            New Recipe
          </Button>
        </EmptyState>
      </div>
    );
  }

  return (
    <BouncyStagger className="flex flex-col gap-3" from={0.85}>
      {recipes.map((recipe) => (
        <ListItem key={recipe.id} className="group">
          <RecipeCardIcon />
          <ListItemContent>
            <Text weight="medium">{recipe.name}</Text>
            <Row className="gap-2">
              <Text as="span" size="xs" color="muted">
                {recipe.nodeCount === 1 ? "1 node" : `${recipe.nodeCount} nodes`}
              </Text>
              <Text as="span" size="xs" color="muted">
                &middot;
              </Text>
              <Text as="span" size="xs" color="muted">
                {formatTimeAgo(recipe.updatedAt)}
              </Text>
            </Row>
          </ListItemContent>
          <ListItemActions>
            <Button
              icon={<PenLineIcon />}
              variant="primary"
              dormant
              href={`/editor?recipe=${recipe.id}`}
              aria-label={`Edit ${recipe.name}`}
            />
            <DeleteRecipeButton recipeId={recipe.id} recipeName={recipe.name} />
          </ListItemActions>
        </ListItem>
      ))}
    </BouncyStagger>
  );
}
