/**
 * Filter dropdown for My Recipes — category filter + sort order.
 *
 * Two sections separated by a divider:
 *   Top:    Category (All, Image, Data, File)
 *   Bottom: Sort (Newest first, Oldest first)
 */

"use client";

import {
  CheckIcon,
  ChevronDownIcon,
  Menu,
  MenuContent,
  MenuItem,
  MenuLabel,
  MenuSeparator,
  MenuTrigger,
  SlidersHorizontalIcon,
} from "@bnto/ui";

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
      <MenuTrigger variant="outline" elevation="sm" data-testid="recipe-filter-trigger">
        <SlidersHorizontalIcon />
        {activeLabel}
        <ChevronDownIcon />
      </MenuTrigger>
      <MenuContent align="end" className="w-48" data-testid="recipe-filter-menu">
        <MenuLabel>Category</MenuLabel>
        {CATEGORIES.map((cat) => (
          <MenuItem
            key={cat.value}
            onClick={() => onCategoryChange(cat.value)}
            data-testid={`filter-category-${cat.value}`}
          >
            {category === cat.value ? (
              <CheckIcon className="size-4 text-primary" />
            ) : (
              <span className="size-4" />
            )}
            {cat.label}
          </MenuItem>
        ))}
        <MenuSeparator />
        <MenuLabel>Sort</MenuLabel>
        {SORT_OPTIONS.map((opt) => (
          <MenuItem
            key={opt.value}
            onClick={() => onSortChange(opt.value)}
            data-testid={`filter-sort-${opt.value}`}
          >
            {sort === opt.value ? (
              <CheckIcon className="size-4 text-primary" />
            ) : (
              <span className="size-4" />
            )}
            {opt.label}
          </MenuItem>
        ))}
      </MenuContent>
    </Menu>
  );
}
