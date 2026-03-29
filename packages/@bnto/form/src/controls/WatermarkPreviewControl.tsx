"use client";

/**
 * WatermarkPreviewControl — position grid with live watermark preview.
 *
 * Shows the first uploaded source image as background, with the watermark
 * overlay positioned at the selected grid position. This gives users a
 * realistic preview of how the watermark will look on their actual image.
 *
 * The outer container is always square. When a source image is uploaded,
 * the image is letterboxed (contain) inside the square, and the grid dots
 * + watermark overlay are constrained to the image area so positioning
 * accurately represents the output.
 *
 * IMPORTANT: The anchor positions (0%, 50%, 100%) must match the Rust engine's
 * `calculate_position()` in watermark.rs. Both use edge/center anchors — NOT
 * grid cell centers. See watermarkAnchors.test.ts for the contract test.
 */

import { useCallback, useEffect, useMemo, useState } from "react";
import { useFormFiles, useFormOnChange, useFormValue } from "../FormStoreContext";
import type { ControlProps } from "./types";
import { ANCHOR_PERCENT, imageAreaStyle } from "./watermarkLayout";

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

function dotClassName(isSelected: boolean, hasImage: boolean) {
  const base = "relative z-10 size-4 rounded-full border border-white/40 transition-colors";
  if (isSelected && hasImage) return `${base} bg-transparent`;
  if (isSelected) return `${base} bg-primary`;
  return `${base} bg-[var(--surface-muted-wall)] hover:bg-muted-foreground/50`;
}

/** Create a stable object URL + natural aspect ratio for the first image file. */
function useSourcePreview(files: File[]): { url?: string; ratio?: number } {
  const firstImage = useMemo(() => files.find((f) => f.type.startsWith("image/")), [files]);
  const [url, setUrl] = useState<string>();
  const [ratio, setRatio] = useState<number>();

  useEffect(() => {
    if (!firstImage) {
      setUrl(undefined);
      setRatio(undefined);
      return;
    }
    const objectUrl = URL.createObjectURL(firstImage);
    setUrl(objectUrl);

    const img = new Image();
    img.onload = () => setRatio(img.naturalWidth / img.naturalHeight);
    img.src = objectUrl;

    return () => URL.revokeObjectURL(objectUrl);
  }, [firstImage]);

  return { url, ratio };
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
  const { url: backgroundUrl, ratio: imageRatio } = useSourcePreview(files);
  const anchor = ANCHOR_PERCENT[current] ?? ANCHOR_PERCENT["bottom-right"];

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
        imageRatio={imageRatio}
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
              top: `${anchor.y}%`,
              left: `${anchor.x}%`,
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
  imageRatio,
  hasImage,
  current,
  onClick,
  children,
}: {
  backgroundUrl: string | undefined;
  imageRatio: number | undefined;
  hasImage: boolean;
  current: string;
  onClick: (e: React.MouseEvent<HTMLButtonElement>) => void;
  children: React.ReactNode;
}) {
  const areaStyle = useMemo(() => imageAreaStyle(imageRatio), [imageRatio]);

  return (
    <div
      role="radiogroup"
      aria-label="Position"
      className="relative aspect-square w-full overflow-hidden rounded-md border border-[var(--surface-muted-wall)] bg-input"
    >
      {backgroundUrl && (
        <img
          src={backgroundUrl}
          alt=""
          className="pointer-events-none absolute inset-0 m-auto h-full w-full object-contain"
        />
      )}
      <div
        className="absolute grid grid-cols-3 grid-rows-3 place-items-center"
        style={areaStyle}
      >
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
