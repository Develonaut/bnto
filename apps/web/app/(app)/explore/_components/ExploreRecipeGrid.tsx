/**
 * Masonry recipe grid for the explore page.
 *
 * Uses the shared RecipeMasonryGrid for layout and distributes
 * cards with alternating hero zone heights for a staggered look.
 */

"use client";

import { useCallback, useMemo } from "react";
import { useSearchParams } from "next/navigation";
import type { Recipe } from "@bnto/core";
import { getAllRecipes } from "@bnto/core";
import type { LucideIcon } from "@bnto/ui";
import { getBntoIcon } from "@/lib/bntoIcons";
import { RecipeMasonryGrid } from "@/components/sections";
import { ExploreRecipeCard } from "./ExploreRecipeCard";
import { ExploreEmptyState } from "./ExploreEmptyState";
import { filterRecipes } from "./filterRecipes";

type CardSize = "sm" | "md" | "lg";

/** Pre-resolved icon map keyed by slug — avoids per-render component creation. */
const RECIPE_ICONS: Record<string, LucideIcon> = Object.fromEntries(
  getAllRecipes().map((r) => [r.slug, getBntoIcon(r.slug)]),
);

/** Repeating size pattern — adjacent cards in the same column rarely share a size. */
const SIZE_PATTERN: CardSize[] = ["lg", "sm", "md", "sm", "lg", "md", "md", "lg", "sm"];

const getCardSize = (index: number) => SIZE_PATTERN[index % SIZE_PATTERN.length];
const getRecipeKey = (r: Recipe) => r.id;

export function ExploreRecipeGrid() {
  const searchParams = useSearchParams();
  const filtered = useMemo(
    () => filterRecipes(searchParams.get("q") ?? "", searchParams.get("category") ?? "all"),
    [searchParams],
  );

  const renderCard = useCallback(
    (recipe: Recipe, gridIndex: number) => (
      <ExploreRecipeCard
        recipe={recipe}
        icon={RECIPE_ICONS[recipe.slug]}
        size={getCardSize(gridIndex)}
      />
    ),
    [],
  );

  if (filtered.length === 0) return <ExploreEmptyState />;

  return (
    <RecipeMasonryGrid
      items={filtered}
      keyOf={getRecipeKey}
      renderCard={renderCard}
      testId="explore-recipe-grid"
    />
  );
}
