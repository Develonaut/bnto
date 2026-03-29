"use client";

/**
 * WatermarkPreviewControl — position grid with live watermark preview.
 *
 * Shows the first uploaded source image as background, with the watermark
 * overlay positioned at the selected grid position. This gives users a
 * realistic preview of how the watermark will look on their actual image.
 */

import { useCallback, useEffect, useMemo, useState } from "react";
import { useFormFiles, useFormOnChange, useFormValue } from "../FormStoreContext";
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
  const base = "relative z-10 size-4 rounded-full border border-white/40 transition-colors";
  if (isSelected && hasImage) return `${base} bg-transparent`;
  if (isSelected) return `${base} bg-primary`;
  return `${base} bg-[var(--surface-muted-wall)] hover:bg-muted-foreground/50`;
}

/** Create a stable object URL for the first image file. */
function useSourcePreview(files: File[]): string | undefined {
  const firstImage = useMemo(() => files.find((f) => f.type.startsWith("image/")), [files]);
  const [url, setUrl] = useState<string>();

  useEffect(() => {
    if (!firstImage) {
      setUrl(undefined);
      return;
    }
    const objectUrl = URL.createObjectURL(firstImage);
    setUrl(objectUrl);
    return () => URL.revokeObjectURL(objectUrl);
  }, [firstImage]);

  return url;
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
  const files = useFormFiles();
  const backgroundUrl = useSourcePreview(files);
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
      <PreviewGrid
        backgroundUrl={backgroundUrl}
        hasImage={!!imgSrc}
        current={current}
        onClick={handleClick}
      >
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
      </PreviewGrid>
      <span className="text-center text-xs text-muted-foreground">{label}</span>
    </div>
  );
}

/** The position grid container with optional source image background. */
function PreviewGrid({
  backgroundUrl,
  hasImage,
  current,
  onClick,
  children,
}: {
  backgroundUrl: string | undefined;
  hasImage: boolean;
  current: string;
  onClick: (e: React.MouseEvent<HTMLButtonElement>) => void;
  children: React.ReactNode;
}) {
  return (
    <div
      role="radiogroup"
      aria-label="Position"
      className="relative aspect-square w-full overflow-hidden rounded-md border border-[var(--surface-muted-wall)] bg-input"
      style={
        backgroundUrl
          ? {
              backgroundImage: `url(${backgroundUrl})`,
              backgroundSize: "cover",
              backgroundPosition: "center",
            }
          : undefined
      }
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
            onClick={onClick}
            className={dotClassName(pos === current, hasImage)}
          />
        ))}
        {children}
      </div>
    </div>
  );
}

export { WatermarkPreviewControl };
