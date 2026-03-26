/**
 * Window-based row virtualizer for the explore recipe grid.
 */

"use client";

import { useMemo } from "react";
import { useSearchParams } from "next/navigation";
import { useWindowVirtualizer } from "@bnto/ui";
import { useColumnCount } from "./useColumnCount";
import { filterRecipes } from "./filterRecipes";

const CARD_HEIGHT = 340;
const GAP = 16;

export function useExploreVirtualizer() {
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

  return { filtered, columnCount, totalItems, virtualizer };
}
