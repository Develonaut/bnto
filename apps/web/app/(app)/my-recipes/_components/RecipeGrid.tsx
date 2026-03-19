"use client";

import { core } from "@bnto/core";

import {
  BlocksIcon,
  BouncyStagger,
  Button,
  ClockIcon,
  CloudIcon,
  CloudOffIcon,
  EmptyState,
  EmptyStateIcon,
  EmptyStateTitle,
  EmptyStateDescription,
  FolderOpenIcon,
  PlusIcon,
  RecipeCard,
  RecipeCardTags,
  Row,
  Stack,
  Text,
} from "@bnto/ui";
import { editorUrl } from "@/lib/routes";
import { formatTimeAgo } from "@/lib/formatTimeAgo";
import { DeleteRecipeButton } from "./DeleteRecipeButton";
import { LocalRecipeUpsell } from "./LocalRecipeUpsell";
import { RecipeInfoButton } from "./RecipeInfoButton";
import { SyncStatus } from "./SyncStatus";

/**
 * Unified recipe grid — store-backed, reactive.
 *
 * Reads from the Zustand recipesStore via core.recipes.useRecipes().
 * No auth branching for storage — all recipes come from one source.
 * Shows a sync upsell banner for unauthenticated users.
 */
export function RecipeGrid() {
  const { isAuthenticated } = core.auth.useAuth();
  const { data: recipes } = core.recipes.useRecipes();

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
      {!isAuthenticated && <LocalRecipeUpsell />}
      <BouncyStagger className="flex flex-col gap-3" from={0.85}>
        {recipes.map((recipe) => (
          <Row key={recipe.id} align="stretch" className="gap-2 group" data-testid="recipe-card">
            <RecipeCard compact href={editorUrl(recipe.id)} className="flex-1">
              <SyncStatus syncedAt={recipe.syncedAt} />
              <Stack className="flex-1 gap-0.5">
                <Text weight="medium">{recipe.name}</Text>
                <Row className="gap-3 items-center">
                  <Row className="gap-1 items-center">
                    {recipe.syncedAt ? (
                      <CloudIcon className="size-3 text-muted-foreground" />
                    ) : (
                      <CloudOffIcon className="size-3 text-muted-foreground" />
                    )}
                    <Text as="span" size="xs" color="muted">
                      {recipe.syncedAt ? formatTimeAgo(recipe.syncedAt) : "Not synced"}
                    </Text>
                  </Row>
                  <Row className="gap-1 items-center">
                    <ClockIcon className="size-3 text-muted-foreground" />
                    <Text as="span" size="xs" color="muted">
                      {formatTimeAgo(recipe.updatedAt)}
                    </Text>
                  </Row>
                  <Row className="gap-1 items-center">
                    <BlocksIcon className="size-3 text-muted-foreground" />
                    <Text as="span" size="xs" color="muted">
                      {recipe.nodeCount === 1 ? "1 node" : `${recipe.nodeCount} nodes`}
                    </Text>
                  </Row>
                </Row>
                {recipe.nodeTypes.length > 0 && (
                  <RecipeCardTags tags={recipe.nodeTypes} limit={3} />
                )}
              </Stack>
            </RecipeCard>
            <Stack className="gap-2">
              <RecipeInfoButton recipeId={recipe.id} recipeName={recipe.name} />
              <DeleteRecipeButton recipeId={recipe.id} recipeName={recipe.name} />
            </Stack>
          </Row>
        ))}
      </BouncyStagger>
    </>
  );
}
