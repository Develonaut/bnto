/**
 * Virtualized recipe grid for the explore page.
 *
 * Reads URL search params for filtering, uses @tanstack/react-virtual
 * for window-based row virtualization with responsive columns.
 */

"use client";

import { useMemo } from "react";
import { useSearchParams } from "next/navigation";
import { useWindowVirtualizer } from "@tanstack/react-virtual";
import { getAllRecipes } from "@bnto/registry";
import { Stagger } from "@bnto/ui";
import { getBntoIcon } from "@/lib/bntoIcons";
import { ExploreGridRow } from "./ExploreGridRow";
import { ExploreEmptyState } from "./ExploreEmptyState";
import { useColumnCount } from "./useColumnCount";
import { filterRecipes } from "./filterRecipes";

const CARD_HEIGHT = 340;
const GAP = 16;

/** Pre-resolved icon map keyed by slug — avoids per-render component creation. */
const RECIPE_ICONS = Object.fromEntries(getAllRecipes().map((r) => [r.slug, getBntoIcon(r.slug)]));

export function ExploreRecipeGrid() {
  const searchParams = useSearchParams();
  const filtered = useMemo(
    () => filterRecipes(searchParams.get("q") ?? "", searchParams.get("category") ?? "all"),
    [searchParams],
  );
  const columnCount = useColumnCount();
  const totalItems = filtered.length + 1;

  const virtualizer = useWindowVirtualizer({
    count: Math.ceil(totalItems / columnCount),
    estimateSize: () => CARD_HEIGHT + GAP,
    overscan: 3,
  });

  if (filtered.length === 0) return <ExploreEmptyState />;

  const rows = virtualizer.getVirtualItems();
  return (
    <div className="relative w-full" style={{ height: virtualizer.getTotalSize() }}>
      <Stagger
        className="absolute left-0 top-0 w-full"
        style={{ transform: `translateY(${rows[0]?.start ?? 0}px)` }}
      >
        {rows.map((vr) => (
          <ExploreGridRow
            key={vr.key}
            virtualRow={vr}
            columnCount={columnCount}
            recipes={filtered}
            totalItems={totalItems}
            icons={RECIPE_ICONS}
            measureElement={virtualizer.measureElement}
          />
        ))}
      </Stagger>
    </div>
  );
}
