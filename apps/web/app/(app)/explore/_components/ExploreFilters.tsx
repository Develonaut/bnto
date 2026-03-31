/**
 * Explore page filters — horizontal scrolling category cards.
 *
 * Reads and writes URL search params (?category=...).
 */

"use client";

import { useCallback } from "react";
import { getAllCategories, getRecipesByCategory } from "@bnto/registry";
import { LayersIcon } from "@bnto/ui";
import { CATEGORY_ICON } from "@/constants/categoryIcons";
import { CategoryCard } from "./CategoryCard";
import { useExploreParams } from "./useExploreParams";

/** Categories relevant for recipe filtering (exclude internal-only categories). */
const RECIPE_CATEGORIES = getAllCategories().filter(
  (c) => c.name !== "io" && c.name !== "control" && c.name !== "system",
);

export function ExploreFilters() {
  const { category, update } = useExploreParams();
  const handleCategorySelect = useCallback((c: string) => update("category", c), [update]);
  const handleAll = useCallback(() => handleCategorySelect("all"), [handleCategorySelect]);

  return (
    <div className="-mx-6 overflow-x-auto px-6 scrollbar-none">
      <div className="flex gap-3 py-3">
        <CategoryCard
          label="All"
          icon={LayersIcon}
          active={category === "all"}
          onClick={handleAll}
        />
        {RECIPE_CATEGORIES.map((cat) => (
          <CategoryBarCard
            key={cat.name}
            cat={cat}
            active={category === cat.name}
            onSelect={handleCategorySelect}
          />
        ))}
      </div>
    </div>
  );
}

export function CategoryBarCard({
  cat,
  active,
  onSelect,
}: {
  cat: { name: string; label: string };
  active: boolean;
  onSelect: (c: string) => void;
}) {
  const count = getRecipesByCategory(cat.name).length;
  const handleClick = useCallback(() => onSelect(cat.name), [onSelect, cat.name]);

  return (
    <CategoryCard
      label={cat.label}
      icon={CATEGORY_ICON[cat.name]}
      count={count}
      active={active}
      muted={count === 0}
      onClick={handleClick}
    />
  );
}
