"use client";

import { useMemo } from "react";
import { core } from "@bnto/core";
import { Grid, Stagger } from "@bnto/ui";
import { LocalRecipeUpsell } from "./LocalRecipeUpsell";
import { EmptyRecipeGrid } from "./EmptyRecipeGrid";
import { RecipeCardItem } from "./RecipeCardItem";
import { filterAndSortRecipes, getCategoryIcon } from "./recipeGridHelpers";
import type { RecipeCategory, RecipeSortOrder } from "./RecipeFilterMenu";

interface RecipeGridProps {
  category: RecipeCategory;
  sort: RecipeSortOrder;
}

/**
 * Recipe card grid -- store-backed, reactive.
 *
 * Reads from the Zustand recipesStore via core.recipes.useRecipes().
 * Applies category filter and sort order from the parent.
 */
export function RecipeGrid({ category, sort }: RecipeGridProps) {
  const { isAuthenticated } = core.auth.useAuth();
  const { data: recipes } = core.recipes.useRecipes();
  const filtered = useMemo(
    () => filterAndSortRecipes(recipes, category, sort),
    [recipes, category, sort],
  );

  if (filtered.length === 0) {
    return <EmptyRecipeGrid isFiltered={category !== "all"} />;
  }

  return (
    <>
      {!isAuthenticated && <LocalRecipeUpsell />}
      <Stagger asChild>
        <Grid cols={{ mobile: 1, tablet: 2, desktop: 3 }} gap="md" animated>
          {filtered.map((recipe, i) => (
            <RecipeCardItem
              key={recipe.id}
              recipe={recipe}
              index={i}
              categoryIcon={getCategoryIcon(recipe.nodeTypes)}
            />
          ))}
        </Grid>
      </Stagger>
    </>
  );
}
