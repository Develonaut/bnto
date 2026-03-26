/**
 * Virtualized recipe grid for the explore page.
 */

"use client";

import { getAllRecipes } from "@bnto/registry";
import { Stagger } from "@bnto/ui";
import { getBntoIcon } from "@/lib/bntoIcons";
import { ExploreGridRow } from "./ExploreGridRow";
import { ExploreEmptyState } from "./ExploreEmptyState";
import { useExploreVirtualizer } from "./useExploreVirtualizer";

/** Pre-resolved icon map keyed by slug — avoids per-render component creation. */
const RECIPE_ICONS = Object.fromEntries(getAllRecipes().map((r) => [r.slug, getBntoIcon(r.slug)]));

export function ExploreRecipeGrid() {
  const { filtered, columnCount, totalItems, virtualizer } = useExploreVirtualizer();

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
