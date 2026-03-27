/**
 * Masonry recipe grid for the explore page.
 *
 * Distributes cards across columns (round-robin) with alternating
 * hero zone heights to create a Pinterest-style staggered layout.
 */

"use client";

import { useMemo } from "react";
import { useSearchParams } from "next/navigation";
import { getAllRecipes } from "@bnto/core";
import { ScaleIn, Stagger } from "@bnto/ui";
import { getBntoIcon } from "@/lib/bntoIcons";
import { ExploreRecipeCard } from "./ExploreRecipeCard";

type CardSize = "sm" | "md" | "lg";
import { ExploreEmptyState } from "./ExploreEmptyState";
import { filterRecipes } from "./filterRecipes";
import { useColumnCount } from "./useColumnCount";

/** Pre-resolved icon map keyed by slug — avoids per-render component creation. */
const RECIPE_ICONS = Object.fromEntries(getAllRecipes().map((r) => [r.slug, getBntoIcon(r.slug)]));

/**
 * Repeating size pattern across the grid (read left-to-right, row by row).
 * Designed so adjacent cards in the same column rarely share a size.
 */
const SIZE_PATTERN: CardSize[] = ["lg", "sm", "md", "sm", "lg", "md", "md", "lg", "sm"];

function getCardSize(index: number) {
  return SIZE_PATTERN[index % SIZE_PATTERN.length];
}

/** Distribute items round-robin into N columns. */
function distributeToColumns<T>(items: readonly T[], columnCount: number): T[][] {
  const columns: T[][] = Array.from({ length: columnCount }, () => []);
  for (let i = 0; i < items.length; i++) {
    columns[i % columnCount].push(items[i]);
  }
  return columns;
}

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
    <Stagger className="flex items-start gap-4">
      {columns.map((column, colIndex) => (
        <div key={colIndex} className="flex flex-1 flex-col gap-4">
          {column.map((recipe, rowIndex) => {
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
      ))}
    </Stagger>
  );
}
