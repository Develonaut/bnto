/**
 * Single virtualized row in the explore grid.
 */

import type { VirtualItem } from "@tanstack/react-virtual";
import type { Recipe } from "@bnto/core";
import type { LucideIcon } from "@bnto/ui";
import { ScaleIn } from "@bnto/ui";
import { ExploreRecipeCard } from "./ExploreRecipeCard";
import { ExploreCtaCard } from "./ExploreCtaCard";

interface ExploreGridRowProps {
  virtualRow: VirtualItem;
  columnCount: number;
  recipes: readonly Recipe[];
  totalItems: number;
  icons: Record<string, LucideIcon>;
  measureElement: (el: Element | null) => void;
}

function renderCell(
  index: number,
  recipes: readonly Recipe[],
  icons: Record<string, LucideIcon>,
  total: number,
) {
  if (index >= total) return null;
  if (index >= recipes.length) {
    return (
      <ScaleIn key="cta" index={index}>
        <ExploreCtaCard />
      </ScaleIn>
    );
  }
  const r = recipes[index];
  return (
    <ScaleIn key={r.id} index={index} from={0.9}>
      <ExploreRecipeCard recipe={r} icon={icons[r.slug]} />
    </ScaleIn>
  );
}

export function ExploreGridRow({
  virtualRow,
  columnCount,
  recipes,
  totalItems,
  icons,
  measureElement,
}: ExploreGridRowProps) {
  const start = virtualRow.index * columnCount;
  return (
    <div
      data-index={virtualRow.index}
      ref={measureElement}
      className="grid gap-4 pb-4"
      style={{ gridTemplateColumns: `repeat(${columnCount}, minmax(0, 1fr))` }}
    >
      {Array.from({ length: columnCount }, (_, i) =>
        renderCell(start + i, recipes, icons, totalItems),
      )}
    </div>
  );
}
