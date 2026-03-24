"use client";

import { useState } from "react";

import { PageContentHeader } from "@/components/blocks/PageContentHeader";

import { RecipeFilterMenu } from "./RecipeFilterMenu";
import { RecipeGrid } from "./RecipeGrid";

import type { RecipeCategory, RecipeSortOrder } from "./RecipeFilterMenu";

/**
 * Client boundary for the My Recipes page.
 *
 * Owns filter + sort state and composes the page content header
 * (title + filter dropdown) with the recipe grid.
 */
export function MyRecipesTabs() {
  const [category, setCategory] = useState<RecipeCategory>("all");
  const [sort, setSort] = useState<RecipeSortOrder>("newest");

  return (
    <>
      <PageContentHeader title="My Recipes" data-testid="my-recipes-heading">
        <RecipeFilterMenu
          category={category}
          sort={sort}
          onCategoryChange={setCategory}
          onSortChange={setSort}
        />
      </PageContentHeader>
      <RecipeGrid category={category} sort={sort} />
    </>
  );
}
