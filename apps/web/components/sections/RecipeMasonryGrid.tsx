"use client";

import type { ReactNode } from "react";
import { useMemo } from "react";
import { ScaleIn, Stagger } from "@bnto/ui";

import { useColumnCount } from "./useColumnCount";

/** Distribute items round-robin into N columns. */
// eslint-disable-next-line no-restricted-syntax -- generic helper, internal to this component
function distributeToColumns<T>(items: readonly T[], columnCount: number): T[][] {
  const columns: T[][] = Array.from({ length: columnCount }, () => []);
  for (let i = 0; i < items.length; i++) {
    columns[i % columnCount].push(items[i]);
  }
  return columns;
}

interface RecipeMasonryGridProps<T> {
  items: readonly T[];
  /** Unique key for each item. */
  keyOf: (item: T) => string;
  /** Render a single card. `gridIndex` is the item's position in the flat list. */
  renderCard: (item: T, gridIndex: number) => ReactNode;
  /** data-testid for the grid container. */
  testId?: string;
}

export function RecipeMasonryGrid<T>({
  items,
  keyOf,
  renderCard,
  testId,
}: RecipeMasonryGridProps<T>) {
  const columnCount = useColumnCount();
  const columns = useMemo(() => distributeToColumns(items, columnCount), [items, columnCount]);

  return (
    <Stagger className="flex items-start gap-4" data-testid={testId}>
      {columns.map((col, ci) => (
        <div key={ci} className="flex flex-1 flex-col gap-4">
          {col.map((item, rowIndex) => {
            const gridIndex = ci + rowIndex * columnCount;
            return (
              <ScaleIn key={keyOf(item)} index={gridIndex} from={0.9}>
                {renderCard(item, gridIndex)}
              </ScaleIn>
            );
          })}
        </div>
      ))}
    </Stagger>
  );
}
