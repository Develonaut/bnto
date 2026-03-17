"use client";

import { core } from "@bnto/core";
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
  ListItem,
  ListItemContent,
  ListItemActions,
  RecipeCardIcon,
  Row,
  Text,
} from "@bnto/ui";
import { formatTimeAgo } from "@/lib/formatTimeAgo";
import { DeleteLocalRecipeButton } from "./DeleteLocalRecipeButton";
import { LocalRecipeUpsell } from "./LocalRecipeUpsell";

/**
 * Local recipe grid — shows localStorage drafts for unauthenticated users.
 *
 * All storage logic is delegated to core.recipes — this component
 * only handles rendering and delegates mutations to core hooks.
 */
export function LocalRecipeGrid() {
  const { data: recipes, refresh } = core.recipes.useLocalRecipes();
  const { mutate: removeLocal } = core.recipes.useRemoveLocalRecipe();

  if (recipes.length === 0) {
    return (
      <div className="min-h-[240px]">
        <EmptyState>
          <EmptyStateIcon>
            <FolderOpenIcon />
          </EmptyStateIcon>
          <EmptyStateTitle>No saved recipes yet</EmptyStateTitle>
          <EmptyStateDescription>
            Create a recipe in the editor — it will auto-save here.
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
    <>
      <LocalRecipeUpsell />
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
              <DeleteLocalRecipeButton
                recipeId={recipe.id}
                recipeName={recipe.name}
                onDelete={(id) => removeLocal(id, { onSuccess: refresh })}
              />
            </ListItemActions>
          </ListItem>
        ))}
      </BouncyStagger>
    </>
  );
}
