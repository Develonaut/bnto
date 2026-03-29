"use client";

/**
 * WatermarkPreviewControl — position grid with live watermark preview.
 *
 * The preview image floats over the grid at the selected position,
 * sized as a percentage of the container width (mirroring the engine's
 * "percentage of source image width" semantics). Opacity tracks the sibling
 * opacity param. When no watermark is uploaded, renders a plain position grid.
 */

import { useCallback, useMemo } from "react";
import { useFormOnChange, useFormValue } from "../FormStoreContext";
import type { ControlProps } from "./types";

const GRID: readonly string[][] = [
  ["top-left", "top-center", "top-right"],
  ["middle-left", "center", "middle-right"],
  ["bottom-left", "bottom-center", "bottom-right"],
];

const POS_ANCHOR: Record<string, { row: number; col: number }> = {
  "top-left": { row: 0, col: 0 },
  "top-center": { row: 0, col: 1 },
  "top-right": { row: 0, col: 2 },
  "middle-left": { row: 1, col: 0 },
  center: { row: 1, col: 1 },
  "middle-right": { row: 1, col: 2 },
  "bottom-left": { row: 2, col: 0 },
  "bottom-center": { row: 2, col: 1 },
  "bottom-right": { row: 2, col: 2 },
};

/** Convert grid row/col (0-2) to percentage of content area. */
function gridPercent(index: number): string {
  const fraction = (2 * index + 1) / 6;
  return `${(fraction * 100).toFixed(2)}%`;
}

function formatLabel(pos: string) {
  return pos
    .split("-")
    .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
    .join(" ");
}

function dotClassName(isSelected: boolean, hasImage: boolean) {
  if (isSelected && hasImage)
    return "relative z-10 size-4 rounded-full transition-colors bg-transparent";
  if (isSelected) return "relative z-10 size-4 rounded-full transition-colors bg-primary";
  return "relative z-10 size-4 rounded-full transition-colors bg-[var(--surface-muted-wall)] hover:bg-muted-foreground/50";
}

function useWatermarkSiblings() {
  const watermark = useFormValue("watermark") as string | undefined;
  const size = (useFormValue("size") as number | undefined) ?? 25;
  const opacity = (useFormValue("opacity") as number | undefined) ?? 80;
  const offsetX = (useFormValue("offsetX") as number | undefined) ?? 0;
  const offsetY = (useFormValue("offsetY") as number | undefined) ?? 0;
  return { imgSrc: watermark || undefined, size, opacity, offsetX, offsetY };
}

function WatermarkPreviewControl({ id, value, onChange }: ControlProps) {
  const current = typeof value === "string" ? value : "bottom-right";
  const { imgSrc, size, opacity, offsetX, offsetY } = useWatermarkSiblings();
  const formOnChange = useFormOnChange();
  const anchor = POS_ANCHOR[current] ?? POS_ANCHOR["bottom-right"];

  const handleClick = useCallback(
    (e: React.MouseEvent<HTMLButtonElement>) => {
      const pos = e.currentTarget.dataset.pos;
      if (!pos) return;
      onChange(pos);
      formOnChange("offsetX", 0);
      formOnChange("offsetY", 0);
    },
    [onChange, formOnChange],
  );

  const label = useMemo(() => formatLabel(current), [current]);

  return (
    <div
      className="sticky top-0 z-30 flex flex-col gap-2 bg-[var(--card)] pb-2"
      data-testid={`control-watermark-preview-${id}`}
    >
      <div
        role="radiogroup"
        aria-label="Position"
        className="relative aspect-square w-full overflow-hidden rounded-md border border-[var(--surface-muted-wall)] bg-input p-3"
      >
        <div className="relative grid h-full w-full grid-cols-3 place-items-center">
          {GRID.flat().map((pos) => (
            <button
              key={pos}
              type="button"
              role="radio"
              aria-checked={pos === current}
              aria-label={formatLabel(pos)}
              data-pos={pos}
              onClick={handleClick}
              className={dotClassName(pos === current, !!imgSrc)}
            />
          ))}
          {imgSrc && (
            <img
              src={imgSrc}
              alt=""
              className="pointer-events-none absolute z-20"
              style={{
                width: `${size}%`,
                opacity: opacity / 100,
                top: gridPercent(anchor.row),
                left: gridPercent(anchor.col),
                transform: `translate(calc(-50% + ${offsetX}px), calc(-50% + ${offsetY}px))`,
              }}
            />
          )}
        </div>
      </div>
      <span className="text-center text-xs text-muted-foreground">{label}</span>
    </div>
  );
}

export { WatermarkPreviewControl };
