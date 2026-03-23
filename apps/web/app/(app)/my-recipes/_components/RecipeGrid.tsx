"use client";

import { useMemo } from "react";
import { core, NODE_TYPE_INFO } from "@bnto/core";

import {
  BlocksIcon,
  Button,
  CardActionArea,
  ClockIcon,
  CloudIcon,
  CloudOffIcon,
  EmptyState,
  EmptyStateIcon,
  EmptyStateTitle,
  EmptyStateDescription,
  FileIcon,
  FolderOpenIcon,
  GlobeIcon,
  Grid,
  GridItem,
  ImageIcon,
  PlusIcon,
  RecipeCard,
  RecipeCardContent,
  RecipeCardHeader,
  RecipeCardIcon,
  RecipeCardTags,
  RecipeCardTitle,
  Row,
  ScaleIn,
  SheetIcon,
  Stagger,
  Text,
} from "@bnto/ui";
import type { LucideIcon } from "@bnto/ui";
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

/** Map category → card icon. Fallback to BlocksIcon (RecipeCardIcon default). */
const CATEGORY_ICON: Record<string, LucideIcon> = {
  image: ImageIcon,
  file: FileIcon,
  spreadsheet: SheetIcon,
  network: GlobeIcon,
};

/** Find the dominant domain category icon from a recipe's node type labels. */
function getCategoryIcon(nodeTypes: string[]): LucideIcon | undefined {
  for (const label of nodeTypes) {
    const icon = CATEGORY_ICON[LABEL_TO_CATEGORY[label]];
    if (icon) return icon;
  }
  return undefined;
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
                    <RecipeCardIcon icon={getCategoryIcon(recipe.nodeTypes)} />
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
                      <Row className="gap-1 items-center">
                        {recipe.syncedAt !== null ? (
                          <CloudIcon className="size-3 text-muted-foreground" />
                        ) : (
                          <CloudOffIcon className="size-3 text-muted-foreground" />
                        )}
                        <Text as="span" size="xs" color="muted">
                          {recipe.syncedAt !== null ? "Synced" : "Local"}
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
