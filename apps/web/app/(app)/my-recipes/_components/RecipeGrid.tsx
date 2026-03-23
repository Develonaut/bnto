"use client";

import { useMemo } from "react";
import { core, NODE_TYPE_INFO } from "@bnto/core";

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

import type { RecipeCategory, RecipeSortOrder } from "./RecipeFilterMenu";

/** Build a reverse map: node type label → category. */
const LABEL_TO_CATEGORY: Record<string, string> = {};
for (const info of Object.values(NODE_TYPE_INFO)) {
  LABEL_TO_CATEGORY[info.label] = info.category;
}

interface RecipeGridProps {
  category: RecipeCategory;
  sort: RecipeSortOrder;
}

/**
 * Unified recipe grid — store-backed, reactive.
 *
 * Reads from the Zustand recipesStore via core.recipes.useRecipes().
 * Applies category filter and sort order from the parent.
 */
export function RecipeGrid({ category, sort }: RecipeGridProps) {
  const { isAuthenticated } = core.auth.useAuth();
  const { data: recipes } = core.recipes.useRecipes();

  const filtered = useMemo(() => {
    let result = recipes;

    if (category !== "all") {
      result = result.filter((r) =>
        r.nodeTypes.some((label) => LABEL_TO_CATEGORY[label] === category),
      );
    }

    if (sort === "oldest") {
      result = [...result].sort((a, b) => a.updatedAt - b.updatedAt);
    }
    // "newest" is the default sort from useRecipes()

    return result;
  }, [recipes, category, sort]);

  if (filtered.length === 0) {
    const isFiltered = category !== "all";
    return (
      <div className="min-h-[240px]">
        <EmptyState>
          <EmptyStateIcon>
            <FolderOpenIcon />
          </EmptyStateIcon>
          <EmptyStateTitle>
            {isFiltered ? "No matching recipes" : "No saved recipes yet"}
          </EmptyStateTitle>
          <EmptyStateDescription>
            {isFiltered
              ? "Try a different filter or create a new recipe."
              : "Create a recipe in the editor — it will auto-save here."}
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
        {filtered.map((recipe) => (
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
