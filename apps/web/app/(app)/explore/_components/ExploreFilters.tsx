/**
 * Explore page category filter — Motorway Select dropdown.
 *
 * Reads and writes URL search params (?category=...).
 */

"use client";

import { useCallback } from "react";
import { getAllCategories, getRecipesByCategory } from "@bnto/registry";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@bnto/ui";
import { useExploreParams } from "./useExploreParams";

/** Categories relevant for recipe filtering (exclude internal-only categories). */
const RECIPE_CATEGORIES = getAllCategories().filter(
  (c) => c.name !== "io" && c.name !== "control" && c.name !== "system",
);

export function ExploreFilters() {
  const { category, update } = useExploreParams();

  const handleChange = useCallback((value: string) => update("category", value), [update]);

  return (
    <div className="mt-1 shrink-0" data-testid="explore-category-filter">
      <Select value={category} onValueChange={handleChange}>
        <SelectTrigger>
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="all">All categories</SelectItem>
          {RECIPE_CATEGORIES.map((cat) => {
            const count = getRecipesByCategory(cat.name).length;
            return (
              <SelectItem key={cat.name} value={cat.name} disabled={count === 0}>
                {cat.label} ({count})
              </SelectItem>
            );
          })}
        </SelectContent>
      </Select>
    </div>
  );
}
