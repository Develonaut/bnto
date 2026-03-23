"use client";

import { useMemo } from "react";
import { core, NODE_TYPE_INFO } from "@bnto/core";

import {
  BlocksIcon,
  Button,
  CardActionArea,
  ClockIcon,
  EmptyState,
  EmptyStateIcon,
  EmptyStateTitle,
  EmptyStateDescription,
  FolderOpenIcon,
  Grid,
  GridItem,
  PlusIcon,
  RecipeCard,
  RecipeCardContent,
  RecipeCardHeader,
  RecipeCardIcon,
  RecipeCardTags,
  RecipeCardTitle,
  Row,
  ScaleIn,
  Stagger,
  Text,
} from "@bnto/ui";
import { editorUrl } from "@/lib/routes";
import { formatTimeAgo } from "@/lib/formatTimeAgo";
import { LocalRecipeUpsell } from "./LocalRecipeUpsell";
import { RecipeCardMenu } from "./RecipeCardMenu";

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
 * Recipe card grid — store-backed, reactive.
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
      <Stagger asChild>
        <Grid cols={{ mobile: 1, tablet: 2, desktop: 3 }} gap="md" animated>
          {filtered.map((recipe, i) => (
            <GridItem key={recipe.id}>
              <ScaleIn
                index={i}
                from={0.85}
                easing="spring-bouncy"
                className="h-full"
                data-testid="recipe-card"
              >
                <RecipeCard href={editorUrl(recipe.id)}>
                  <RecipeCardHeader>
                    <RecipeCardIcon />
                    <CardActionArea>
                      <RecipeCardMenu recipeId={recipe.id} recipeName={recipe.name} />
                    </CardActionArea>
                  </RecipeCardHeader>
                  <RecipeCardContent>
                    <RecipeCardTitle>{recipe.name}</RecipeCardTitle>
                    <Row className="gap-3 items-center">
                      <Row className="gap-1 items-center">
                        <BlocksIcon className="size-3 text-muted-foreground" />
                        <Text as="span" size="xs" color="muted">
                          {recipe.nodeCount === 1 ? "1 node" : `${recipe.nodeCount} nodes`}
                        </Text>
                      </Row>
                      <Row className="gap-1 items-center">
                        <ClockIcon className="size-3 text-muted-foreground" />
                        <Text as="span" size="xs" color="muted">
                          {formatTimeAgo(recipe.updatedAt)}
                        </Text>
                      </Row>
                    </Row>
                    {recipe.nodeTypes.length > 0 && (
                      <RecipeCardTags tags={recipe.nodeTypes} limit={3} />
                    )}
                  </RecipeCardContent>
                </RecipeCard>
              </ScaleIn>
            </GridItem>
          ))}
        </Grid>
      </Stagger>
    </>
  );
}
