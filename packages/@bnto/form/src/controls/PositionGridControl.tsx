"use client";

import { useCallback } from "react";
import type { ControlProps } from "./types";

/**
 * 3x3 position grid for 9-position enum fields.
 *
 * Maps positions to a grid layout:
 * ```
 * top-left     top-center     top-right
 * middle-left  center         middle-right
 * bottom-left  bottom-center  bottom-right
 * ```
 */
const POSITIONS = [
  "top-left",
  "top-center",
  "top-right",
  "middle-left",
  "center",
  "middle-right",
  "bottom-left",
  "bottom-center",
  "bottom-right",
] as const;

function PositionGridControl({ id, value, onChange }: ControlProps) {
  const selected = typeof value === "string" ? value : "bottom-right";

  const handleClick = useCallback((position: string) => () => onChange(position), [onChange]);

  return (
    <div
      className="grid grid-cols-3 gap-1 w-fit"
      role="radiogroup"
      aria-label="Position"
      data-testid={`control-position-grid-${id}`}
    >
      {POSITIONS.map((pos) => (
        <button
          key={pos}
          type="button"
          role="radio"
          aria-checked={selected === pos}
          aria-label={pos.replace("-", " ")}
          onClick={handleClick(pos)}
          data-active={selected === pos}
          className="size-7 rounded-sm border border-border transition-colors data-[active=true]:bg-primary data-[active=true]:border-primary hover:bg-muted"
          data-testid={`position-${pos}`}
        />
      ))}
    </div>
  );
}

export { PositionGridControl };
