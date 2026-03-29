"use client";

import { useCallback, useMemo } from "react";
import type { ControlProps } from "./types";

/**
 * 3x3 visual grid for selecting a position (e.g., watermark placement).
 *
 * Renders a grid of 9 dots matching the positions:
 *   top-left    top-center    top-right
 *   middle-left center        middle-right
 *   bottom-left bottom-center bottom-right
 */

const GRID: readonly string[][] = [
  ["top-left", "top-center", "top-right"],
  ["middle-left", "center", "middle-right"],
  ["bottom-left", "bottom-center", "bottom-right"],
];

function formatLabel(pos: string) {
  return pos
    .split("-")
    .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
    .join(" ");
}

function PositionGridControl({ id, value, onChange }: ControlProps) {
  const current = typeof value === "string" ? value : "bottom-right";

  const handleClick = useCallback(
    (e: React.MouseEvent<HTMLButtonElement>) => {
      const pos = e.currentTarget.dataset.pos;
      if (pos) onChange(pos);
    },
    [onChange],
  );

  const label = useMemo(() => formatLabel(current), [current]);

  return (
    <div className="flex flex-col gap-2" data-testid={`control-position-grid-${id}`}>
      <div
        role="radiogroup"
        aria-label="Position"
        className="grid aspect-square w-full grid-cols-3 place-items-center rounded-md border border-border bg-muted/50 p-3"
      >
        {GRID.map((row) =>
          row.map((pos) => {
            const isSelected = pos === current;
            return (
              <button
                key={pos}
                type="button"
                role="radio"
                aria-checked={isSelected}
                aria-label={formatLabel(pos)}
                data-pos={pos}
                onClick={handleClick}
                className={`size-4 rounded-full transition-colors ${
                  isSelected ? "bg-primary" : "bg-border hover:bg-muted-foreground/50"
                }`}
              />
            );
          }),
        )}
      </div>
      <span className="text-center text-xs text-muted-foreground">{label}</span>
    </div>
  );
}

export { PositionGridControl };
