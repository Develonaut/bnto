/**
 * Filter dropdown for My Recipes — category filter + sort order.
 */

"use client";

import { Menu, MenuContent, MenuLabel, MenuSeparator } from "@bnto/ui";

import { FilterMenuItems } from "./RecipeFilterMenuItems";
import { RecipeFilterTrigger } from "./RecipeFilterTrigger";

export type RecipeCategory = "all" | "image" | "spreadsheet" | "file";
export type RecipeSortOrder = "newest" | "oldest";

const CATEGORIES: { value: RecipeCategory; label: string }[] = [
  { value: "all", label: "All Recipes" },
  { value: "image", label: "Image" },
  { value: "spreadsheet", label: "Data" },
  { value: "file", label: "File" },
];

const SORT_OPTIONS: { value: RecipeSortOrder; label: string }[] = [
  { value: "newest", label: "Newest first" },
  { value: "oldest", label: "Oldest first" },
];

interface RecipeFilterMenuProps {
  category: RecipeCategory;
  sort: RecipeSortOrder;
  onCategoryChange: (category: RecipeCategory) => void;
  onSortChange: (sort: RecipeSortOrder) => void;
}

export function RecipeFilterMenu({
  category,
  sort,
  onCategoryChange,
  onSortChange,
}: RecipeFilterMenuProps) {
  const activeLabel = CATEGORIES.find((c) => c.value === category)?.label ?? "All Recipes";

  return (
    <Menu>
      <RecipeFilterTrigger activeLabel={activeLabel} />
      <MenuContent align="end" className="w-48" data-testid="recipe-filter-menu">
        <MenuLabel>Category</MenuLabel>
        <FilterMenuItems
          options={CATEGORIES}
          selected={category}
          onSelect={onCategoryChange}
          testIdPrefix="filter-category"
        />
        <MenuSeparator />
        <MenuLabel>Sort</MenuLabel>
        <FilterMenuItems
          options={SORT_OPTIONS}
          selected={sort}
          onSelect={onSortChange}
          testIdPrefix="filter-sort"
        />
      </MenuContent>
    </Menu>
  );
}
