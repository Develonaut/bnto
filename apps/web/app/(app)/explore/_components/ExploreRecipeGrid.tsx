/**
 * Masonry recipe grid for the explore page.
 *
 * Distributes cards across columns (round-robin) with alternating
 * hero zone heights to create a Pinterest-style staggered layout.
 */

"use client";

import { useMemo } from "react";
import { useSearchParams } from "next/navigation";
import type { Recipe } from "@bnto/core";
import { getAllRecipes } from "@bnto/core";
import type { LucideIcon } from "@bnto/ui";
import { ScaleIn, Stagger } from "@bnto/ui";
import { getBntoIcon } from "@/lib/bntoIcons";
import { ExploreRecipeCard } from "./ExploreRecipeCard";
import { ExploreEmptyState } from "./ExploreEmptyState";
import { filterRecipes } from "./filterRecipes";
import { useColumnCount } from "./useColumnCount";

type CardSize = "sm" | "md" | "lg";

/** Pre-resolved icon map keyed by slug — avoids per-render component creation. */
const RECIPE_ICONS: Record<string, LucideIcon> = Object.fromEntries(
  getAllRecipes().map((r) => [r.slug, getBntoIcon(r.slug)]),
);

/** Repeating size pattern — adjacent cards in the same column rarely share a size. */
const SIZE_PATTERN: CardSize[] = ["lg", "sm", "md", "sm", "lg", "md", "md", "lg", "sm"];

const getCardSize = (index: number) => SIZE_PATTERN[index % SIZE_PATTERN.length];

/** Distribute items round-robin into N columns. */
const distributeToColumns = <T,>(items: readonly T[], columnCount: number): T[][] => {
  const columns: T[][] = Array.from({ length: columnCount }, () => []);
  for (let i = 0; i < items.length; i++) {
    columns[i % columnCount].push(items[i]);
  }
  return columns;
};

export function ExploreRecipeGrid() {
  const searchParams = useSearchParams();
  const columnCount = useColumnCount();
  const filtered = useMemo(
    () => filterRecipes(searchParams.get("q") ?? "", searchParams.get("category") ?? "all"),
    [searchParams],
  );
  const columns = useMemo(
    () => distributeToColumns(filtered, columnCount),
    [filtered, columnCount],
  );

  if (filtered.length === 0) return <ExploreEmptyState />;

  return (
    <Stagger className="flex items-start gap-4" data-testid="explore-recipe-grid">
      {columns.map((col, ci) => (
        <MasonryColumn key={ci} recipes={col} colIndex={ci} columnCount={columnCount} />
      ))}
    </Stagger>
  );
}

export function MasonryColumn({
  recipes,
  colIndex,
  columnCount,
}: {
  recipes: Recipe[];
  colIndex: number;
  columnCount: number;
}) {
  return (
    <div className="flex flex-1 flex-col gap-4">
      {recipes.map((recipe, rowIndex) => {
        const gridIndex = colIndex + rowIndex * columnCount;
        return (
          <ScaleIn key={recipe.id} index={gridIndex} from={0.9}>
            <ExploreRecipeCard
              recipe={recipe}
              icon={RECIPE_ICONS[recipe.slug]}
              size={getCardSize(gridIndex)}
            />
          </ScaleIn>
        );
      })}
    </div>
  );
}
